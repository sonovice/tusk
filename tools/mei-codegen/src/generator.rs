//! Rust code generator from MEI ODD definitions.
//!
//! Generates Rust types that map 1:1 to MEI elements and attribute classes.

use anyhow::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use crate::ast::*;

/// Generate all Rust code from ODD definitions.
pub fn generate_all(defs: &OddDefinitions, output: &Path) -> Result<()> {
    fs::create_dir_all(output)?;

    // Generate data types
    generate_data_types(defs, output)?;

    // Generate attribute classes
    generate_att_classes(defs, output)?;

    // Generate model class traits
    generate_model_classes(defs, output)?;

    // Generate pattern entities
    generate_pattern_entities(defs, output)?;

    // Generate elements
    generate_elements(defs, output)?;

    // Generate validation module
    generate_validation(defs, output)?;

    // Generate lib.rs
    generate_mod_rs(defs, output)?;

    // Print constraint statistics
    print_constraint_stats(defs);

    Ok(())
}

/// Print statistics about parsed constraints.
fn print_constraint_stats(defs: &OddDefinitions) {
    let mut total_constraints = 0;
    let mut list_attrs = 0;
    let mut interleave_count = 0;

    // Count constraints
    for ac in defs.att_classes.values() {
        total_constraints += ac.constraints.len();
        for attr in &ac.attributes {
            total_constraints += attr.constraints.len();
            if matches!(attr.datatype, Some(AttributeDataType::List { .. })) {
                list_attrs += 1;
            }
        }
    }
    for elem in defs.elements.values() {
        total_constraints += elem.constraints.len();
        count_interleaves(&elem.content, &mut interleave_count);
    }

    println!("  Parsed: {} constraints", total_constraints);
    println!("  Parsed: {} rng:list attributes", list_attrs);
    println!(
        "  Parsed: {} rng:interleave content models",
        interleave_count
    );
}

/// Count Interleave items in content model.
fn count_interleaves(content: &ContentModel, count: &mut usize) {
    for item in content {
        match item {
            ContentItem::Interleave(choices) => {
                *count += 1;
                for choice in choices {
                    count_interleaves(choice, count);
                }
            }
            ContentItem::ZeroOrMore(inner)
            | ContentItem::OneOrMore(inner)
            | ContentItem::Optional(inner)
            | ContentItem::Group(inner)
            | ContentItem::List(inner) => {
                count_interleaves(inner, count);
            }
            ContentItem::Choice(choices) => {
                for choice in choices {
                    count_interleaves(choice, count);
                }
            }
            _ => {}
        }
    }
}

// ============================================================================
// Data Types
// ============================================================================

fn generate_data_types(defs: &OddDefinitions, output: &Path) -> Result<()> {
    let path = output.join("data.rs");

    let mut tokens = TokenStream::new();

    tokens.extend(quote! {
        //! MEI data types (generated from ODD).
        //!
        //! DO NOT EDIT - regenerate with: cargo run -p mei-codegen

        use serde::{Deserialize, Serialize};
        use crate::generated::validation::{ValidationContext, Validate};
        use once_cell::sync::Lazy;
        use regex::Regex;

    });

    // Group by module for organization
    let mut by_module: HashMap<String, Vec<&DataType>> = HashMap::new();
    for dt in defs.data_types.values() {
        by_module.entry(dt.module.clone()).or_default().push(dt);
    }

    for (_module, types) in by_module {
        for dt in types {
            if let Some(type_tokens) = generate_data_type(dt, defs) {
                tokens.extend(type_tokens);
                tokens.extend(quote! {});
            }
        }
    }

    write_tokens_to_file(&tokens, &path)?;
    println!("  Generated: {}", path.display());

    Ok(())
}

fn generate_data_type(dt: &DataType, defs: &OddDefinitions) -> Option<TokenStream> {
    let name = mei_ident_to_type(&dt.ident);
    let doc = &dt.desc;

    match &dt.kind {
        DataTypeKind::ValList(values) if !values.is_empty() => {
            let variants: Vec<_> = values
                .iter()
                .map(|v| {
                    let var_name = mei_value_to_variant(&v.ident);
                    let var_doc = &v.desc;
                    let mei_ident = &v.ident;
                    let rename = if var_name.to_string() != v.ident {
                        Some(quote! { #[serde(rename = #mei_ident)] })
                    } else {
                        None
                    };
                    quote! {
                        #[doc = #var_doc]
                        #rename
                        #var_name,
                    }
                })
                .collect();

            Some(quote! {
                #[doc = #doc]
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
                pub enum #name {
                    #(#variants)*
                }
            })
        }
        DataTypeKind::Primitive {
            type_name,
            pattern,
            min_inclusive,
            max_inclusive,
        } => {
            // Generate newtype struct for primitives (allows use in Choice enums)
            let rust_type = rng_data_to_rust(type_name);
            let rust_type_tokens: TokenStream = rust_type.parse().unwrap();

            // Build doc comment with constraints
            let mut doc_parts = vec![doc.clone()];
            if let Some(p) = pattern {
                doc_parts.push(format!("\n\nPattern: `{}`", p));
            }
            if let Some(min) = min_inclusive {
                doc_parts.push(format!("\n\nMin: {}", min));
            }
            if let Some(max) = max_inclusive {
                doc_parts.push(format!("\n\nMax: {}", max));
            }
            let full_doc = doc_parts.join("");

            // f64 doesn't implement Eq/Hash, so use different derives
            // For f64, we don't use #[derive(Serialize, Deserialize)] because we need
            // custom impls that format whole numbers without decimal points
            let derives = if rust_type == "f64" {
                quote! { #[derive(Debug, Clone, PartialEq)] }
            } else {
                quote! { #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)] }
            };

            // Generate validation implementation if there are constraints
            let validation_impl = generate_primitive_validation(
                &name,
                pattern,
                min_inclusive,
                max_inclusive,
                rust_type,
            );

            // For f64 types, we need a special Display impl that formats whole numbers
            // without the decimal point (e.g., "1" instead of "1.0")
            // We also need custom Serialize/Deserialize impls that use this formatting
            let (display_impl, serde_impls, serde_attr) = if rust_type == "f64" {
                let display = quote! {
                    impl std::fmt::Display for #name {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            // Format whole numbers without decimal point
                            if self.0.fract() == 0.0 && self.0.is_finite() {
                                write!(f, "{}", self.0 as i64)
                            } else {
                                write!(f, "{}", self.0)
                            }
                        }
                    }
                };
                // Custom serde impls that serialize using Display (whole numbers without decimals)
                let serde = quote! {
                    impl serde::Serialize for #name {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                            S: serde::Serializer,
                        {
                            // Use Display impl which formats whole numbers without decimals
                            serializer.serialize_str(&self.to_string())
                        }
                    }

                    impl<'de> serde::Deserialize<'de> for #name {
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            let s = String::deserialize(deserializer)?;
                            s.parse().map_err(serde::de::Error::custom)
                        }
                    }
                };
                (display, serde, quote! {})
            } else {
                let display = quote! {
                    impl std::fmt::Display for #name {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f, "{}", self.0)
                        }
                    }
                };
                (display, quote! {}, quote! { #[serde(transparent)] })
            };

            Some(quote! {
                #[doc = #full_doc]
                #derives
                #serde_attr
                pub struct #name(pub #rust_type_tokens);

                impl From<#rust_type_tokens> for #name {
                    fn from(v: #rust_type_tokens) -> Self {
                        Self(v)
                    }
                }

                #display_impl

                #serde_impls

                impl std::str::FromStr for #name {
                    type Err = <#rust_type_tokens as std::str::FromStr>::Err;

                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        Ok(Self(s.parse()?))
                    }
                }

                #validation_impl
            })
        }
        DataTypeKind::Reference(ref_name) => {
            // Type alias to referenced type
            let ref_type = mei_ident_to_type(ref_name);
            Some(quote! {
                #[doc = #doc]
                pub type #name = #ref_type;
            })
        }
        DataTypeKind::Alternate(refs) | DataTypeKind::Choice(refs) => {
            // Generate enum for union types
            let variants: Vec<_> = refs
                .iter()
                .filter_map(|r| {
                    let key = match r {
                        DataTypeRef::MacroRef(k) | DataTypeRef::RngRef(k) => k,
                    };
                    // Skip model class references (they're traits, not types)
                    if key.starts_with("model.") {
                        return None;
                    }
                    // Skip self-references to avoid infinite types
                    if key == &dt.ident {
                        return None;
                    }
                    // Only include references to types that actually exist
                    if key.starts_with("data.") {
                        match defs.data_types.get(key) {
                            Some(ref_dt) => {
                                // Skip type aliases (Reference) - they're not distinct types
                                // Skip empty value lists
                                match &ref_dt.kind {
                                    DataTypeKind::Reference(_) => return None,
                                    DataTypeKind::ValList(vals) if vals.is_empty() => return None,
                                    _ => {} // Primitives are now newtypes, so include them
                                }
                            }
                            None => return None,
                        }
                    }
                    let var_name = mei_ident_to_type(key);
                    let type_name = var_name.clone();
                    Some(quote! { #var_name(#type_name), })
                })
                .collect();

            if variants.is_empty() {
                // All variants were filtered (references) - generate as newtype String
                return Some(quote! {
                    #[doc = #doc]
                    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
                    #[serde(transparent)]
                    pub struct #name(pub String);
                });
            }

            Some(quote! {
                #[doc = #doc]
                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
                #[serde(untagged)]
                pub enum #name {
                    #(#variants)*
                }
            })
        }
        _ => None,
    }
}

/// Generate validation implementation for primitive data types.
fn generate_primitive_validation(
    name: &Ident,
    pattern: &Option<String>,
    min_inclusive: &Option<String>,
    max_inclusive: &Option<String>,
    rust_type: &str,
) -> TokenStream {
    let has_pattern = pattern.is_some();
    let has_min = min_inclusive.is_some();
    let has_max = max_inclusive.is_some();

    if !has_pattern && !has_min && !has_max {
        // No constraints - simple validation that always passes
        return quote! {
            impl Validate for #name {
                fn validate_with_context(&self, _ctx: &mut ValidationContext) {
                    // No constraints to validate
                }
            }
        };
    }

    let name_str = name.to_string();

    // Pattern validation
    let pattern_check = if let Some(p) = pattern {
        let regex_name = format_ident!("{}_PATTERN", name.to_string().to_uppercase());
        // Escape the pattern for use in Rust string
        let pattern_escaped = p.replace('\\', "\\\\").replace('"', "\\\"");
        quote! {
            static #regex_name: Lazy<Regex> = Lazy::new(|| {
                Regex::new(#p).expect("Invalid regex pattern in MEI spec")
            });

            let value_str = self.0.to_string();
            if !#regex_name.is_match(&value_str) {
                ctx.add_pattern_mismatch(#name_str, None, #name_str, &value_str, #pattern_escaped);
            }
        }
    } else {
        quote! {}
    };

    // Range validation (only for numeric types)
    let range_check = if rust_type == "i64" || rust_type == "u64" || rust_type == "f64" {
        let min_check = if let Some(min) = min_inclusive {
            let min_val: TokenStream = min.parse().unwrap_or_else(|_| quote! { 0 });
            let min_str = min.clone();
            quote! {
                if (self.0 as f64) < (#min_val as f64) {
                    ctx.add_error(crate::generated::validation::ValidationError::RangeViolation {
                        location: ctx.location(#name_str, None),
                        attribute: #name_str.to_string(),
                        value: self.0.to_string(),
                        min: #min_str.to_string(),
                        max: "∞".to_string(),
                    });
                }
            }
        } else {
            quote! {}
        };

        let max_check = if let Some(max) = max_inclusive {
            let max_val: TokenStream = max.parse().unwrap_or_else(|_| quote! { 0 });
            let max_str = max.clone();
            quote! {
                if (self.0 as f64) > (#max_val as f64) {
                    ctx.add_error(crate::generated::validation::ValidationError::RangeViolation {
                        location: ctx.location(#name_str, None),
                        attribute: #name_str.to_string(),
                        value: self.0.to_string(),
                        min: "-∞".to_string(),
                        max: #max_str.to_string(),
                    });
                }
            }
        } else {
            quote! {}
        };

        quote! {
            #min_check
            #max_check
        }
    } else {
        quote! {}
    };

    quote! {
        impl Validate for #name {
            fn validate_with_context(&self, ctx: &mut ValidationContext) {
                #pattern_check
                #range_check
            }
        }
    }
}

// ============================================================================
// Attribute Classes
// ============================================================================

fn generate_att_classes(defs: &OddDefinitions, output: &Path) -> Result<()> {
    let att_dir = output.join("att");
    fs::create_dir_all(&att_dir)?;

    let mut mod_items = Vec::new();

    for ac in defs.att_classes.values() {
        let file_name = escape_keyword_filename(&ac.ident.to_snake_case().replace('.', "_"));
        let file_path = att_dir.join(format!("{}.rs", file_name));

        let tokens = generate_att_class(ac, defs);
        write_tokens_to_file(&tokens, &file_path)?;

        let mod_name = format_ident!("{}", file_name);
        mod_items.push(quote! {
            mod #mod_name;
            pub use #mod_name::*;
        });
    }

    // Generate mod.rs
    let mod_tokens = quote! {
        //! MEI attribute classes (generated from ODD).
        //!
        //! DO NOT EDIT - regenerate with: cargo run -p mei-codegen

        #(#mod_items)*
    };

    write_tokens_to_file(&mod_tokens, &att_dir.join("mod.rs"))?;
    println!("  Generated: {} attribute classes", defs.att_classes.len());

    Ok(())
}

fn generate_att_class(ac: &AttClass, defs: &OddDefinitions) -> TokenStream {
    let name = mei_ident_to_type(&ac.ident);
    let doc = &ac.desc;

    // Collect all attributes including inherited ones
    let all_attrs = defs.collect_attributes(&ac.ident);

    // First pass: generate inline enums for value lists
    let mut inline_enums = Vec::new();
    let mut enum_names: std::collections::HashMap<String, Ident> = std::collections::HashMap::new();

    for attr in &all_attrs {
        if let Some(AttributeDataType::InlineValList(values)) = &attr.datatype {
            if !values.is_empty() {
                // Generate enum name from attribute ident (sanitize special chars)
                let sanitized_ident = attr.ident.replace(['.', '-', ':'], "_");
                let enum_name = format_ident!("{}{}", name, sanitized_ident.to_upper_camel_case());

                let variants: Vec<_> = values
                    .iter()
                    .map(|v| {
                        let variant = mei_value_to_variant(&v.ident);
                        let mei_ident = &v.ident;
                        let variant_doc = &v.desc;
                        quote! {
                            #[doc = #variant_doc]
                            #[serde(rename = #mei_ident)]
                            #variant,
                        }
                    })
                    .collect();

                inline_enums.push(quote! {
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
                    pub enum #enum_name {
                        #(#variants)*
                    }
                });

                enum_names.insert(attr.ident.clone(), enum_name);
            }
        }
    }

    // Generate fields for each attribute
    let fields: Vec<_> = all_attrs
        .iter()
        .map(|attr| {
            let field_name = make_safe_ident(&attr.ident.replace(['.', '-', ':'], "_").to_snake_case());
            let field_doc = &attr.desc;

            // Check if this is a multi-valued attribute
            let is_unbounded = attr.max_occurs.as_deref() == Some("unbounded");

            // Determine serde rename (with @ prefix for XML attributes)
            let xml_name = &attr.ident;
            let rename = if is_unbounded {
                // Vec fields use default and is_empty
                if xml_name.starts_with("xml:") {
                    quote! { #[serde(rename = #xml_name, default, skip_serializing_if = "Vec::is_empty")] }
                } else {
                    let attr_name = format!("@{}", xml_name);
                    quote! { #[serde(rename = #attr_name, default, skip_serializing_if = "Vec::is_empty")] }
                }
            } else {
                if xml_name.starts_with("xml:") {
                    quote! { #[serde(rename = #xml_name, skip_serializing_if = "Option::is_none")] }
                } else {
                    let attr_name = format!("@{}", xml_name);
                    quote! { #[serde(rename = #attr_name, skip_serializing_if = "Option::is_none")] }
                }
            };

            // Determine field type
            let field_type = if let Some(enum_name) = enum_names.get(&attr.ident) {
                if is_unbounded {
                    quote! { Vec<#enum_name> }
                } else {
                    quote! { Option<#enum_name> }
                }
            } else {
                attribute_type_tokens(&attr.datatype, &attr.max_occurs, defs)
            };

            quote! {
                #[doc = #field_doc]
                #rename
                pub #field_name: #field_type,
            }
        })
        .collect();

    quote! {
        //! Attribute class: `#ac.ident`

        use serde::{Deserialize, Serialize};

        #(#inline_enums)*

        #[doc = #doc]
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct #name {
            #(#fields)*
        }
    }
}

/// Get the raw inner type for an attribute (without Option wrapper).
/// Used for List inner types.
fn attribute_inner_type_tokens(datatype: &AttributeDataType, defs: &OddDefinitions) -> TokenStream {
    match datatype {
        AttributeDataType::Ref(ref_name) => {
            if defs.data_types.contains_key(ref_name) {
                let type_name = mei_ident_to_type(ref_name);
                quote! { crate::generated::data::#type_name }
            } else {
                quote! { String }
            }
        }
        AttributeDataType::InlineValList(_) => {
            quote! { String }
        }
        AttributeDataType::Primitive { type_name, .. } => {
            let rust_type = rng_data_to_rust(type_name);
            let rust_type_tokens: TokenStream = rust_type.parse().unwrap();
            quote! { #rust_type_tokens }
        }
        AttributeDataType::List { inner, .. } => {
            // Nested list - recurse
            let inner_type = attribute_inner_type_tokens(inner, defs);
            quote! { Vec<#inner_type> }
        }
    }
}

fn attribute_type_tokens(
    datatype: &Option<AttributeDataType>,
    max_occurs: &Option<String>,
    defs: &OddDefinitions,
) -> TokenStream {
    let is_unbounded = max_occurs.as_deref() == Some("unbounded");

    match datatype {
        Some(AttributeDataType::Ref(ref_name)) => {
            // Check if the referenced type actually exists in our definitions
            if defs.data_types.contains_key(ref_name) {
                let type_name = mei_ident_to_type(ref_name);
                if is_unbounded {
                    quote! { Vec<crate::generated::data::#type_name> }
                } else {
                    quote! { Option<crate::generated::data::#type_name> }
                }
            } else {
                // Type not found, fall back to String
                if is_unbounded {
                    quote! { Vec<String> }
                } else {
                    quote! { Option<String> }
                }
            }
        }
        Some(AttributeDataType::InlineValList(_)) => {
            // Inline value lists are handled separately in generate_att_class
            if is_unbounded {
                quote! { Vec<String> }
            } else {
                quote! { Option<String> }
            }
        }
        Some(AttributeDataType::Primitive { type_name, .. }) => {
            let rust_type = rng_data_to_rust(type_name);
            let rust_type_tokens: TokenStream = rust_type.parse().unwrap();
            if is_unbounded {
                quote! { Vec<#rust_type_tokens> }
            } else {
                quote! { Option<#rust_type_tokens> }
            }
        }
        Some(AttributeDataType::List { inner, .. }) => {
            // Space-separated list - get the raw inner type without Option wrapper
            let inner_type = attribute_inner_type_tokens(inner, defs);
            // We use a custom wrapper for space-separated serialization
            quote! { Option<crate::generated::SpaceSeparated<#inner_type>> }
        }
        None => {
            if is_unbounded {
                quote! { Vec<String> }
            } else {
                quote! { Option<String> }
            }
        }
    }
}

// ============================================================================
// Model Classes
// ============================================================================

fn generate_model_classes(defs: &OddDefinitions, output: &Path) -> Result<()> {
    let path = output.join("model.rs");

    let mut tokens = TokenStream::new();

    tokens.extend(quote! {
        //! MEI model classes (generated from ODD).
        //!
        //! Model classes group elements that can appear in specific content model positions.
        //!
        //! DO NOT EDIT - regenerate with: cargo run -p mei-codegen

    });

    for mc in defs.model_classes.values() {
        let name = mei_ident_to_type(&mc.ident);
        let doc = &mc.desc;

        tokens.extend(quote! {
            #[doc = #doc]
            pub trait #name {}

        });
    }

    write_tokens_to_file(&tokens, &path)?;
    println!("  Generated: {} model classes", defs.model_classes.len());

    Ok(())
}

// ============================================================================
// Pattern Entities
// ============================================================================

fn generate_pattern_entities(defs: &OddDefinitions, output: &Path) -> Result<()> {
    let path = output.join("pattern_entities.rs");

    let mut tokens = TokenStream::new();

    tokens.extend(quote! {
        //! MEI pattern entities (generated from ODD).
        //!
        //! Pattern entities define reusable content patterns that can be referenced
        //! by element content models via macroRef.
        //!
        //! DO NOT EDIT - regenerate with: cargo run -p mei-codegen

        use serde::{Deserialize, Serialize};

    });

    // Group by module for organization
    let mut by_module: HashMap<String, Vec<&PatternEntity>> = HashMap::new();
    for pe in defs.pattern_entities.values() {
        by_module.entry(pe.module.clone()).or_default().push(pe);
    }

    for (_module, entities) in by_module {
        for pe in entities {
            let pe_tokens = generate_pattern_entity(pe, defs);
            tokens.extend(pe_tokens);
            tokens.extend(quote! {});
        }
    }

    write_tokens_to_file(&tokens, &path)?;
    println!(
        "  Generated: {} pattern entities",
        defs.pattern_entities.len()
    );

    Ok(())
}

fn generate_pattern_entity(pe: &PatternEntity, defs: &OddDefinitions) -> TokenStream {
    let name = mei_ident_to_type(&pe.ident);
    let doc = &pe.desc;

    // Collect all possible child element types from the content model
    let mut child_types: HashSet<String> = HashSet::new();
    let mut has_text = false;
    collect_content_refs(&pe.content, defs, &mut child_types, &mut has_text);

    if child_types.is_empty() && !has_text {
        // Empty pattern entity - generate unit struct
        return quote! {
            #[doc = #doc]
            #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
            pub struct #name;
        };
    }

    // Generate enum for pattern entity content
    let enum_name = format_ident!("{}Content", name);

    let mut variants = Vec::new();

    if has_text {
        variants.push(quote! {
            /// Text content.
            #[serde(rename = "$text")]
            Text(String),
        });
    }

    for child in &child_types {
        let var_name = mei_ident_to_type(child);
        let xml_name = child;
        variants.push(quote! {
            #[serde(rename = #xml_name)]
            #var_name(Box<crate::generated::elements::#var_name>),
        });
    }

    let content_doc = format!("Content for pattern entity `{}`.", pe.ident);

    quote! {
        #[doc = #content_doc]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "lowercase")]
        pub enum #enum_name {
            #(#variants)*
        }

        #[doc = #doc]
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
        pub struct #name {
            /// Pattern entity content.
            #[serde(default, rename = "$value")]
            pub content: Vec<#enum_name>,
        }
    }
}

// ============================================================================
// Elements
// ============================================================================

fn generate_elements(defs: &OddDefinitions, output: &Path) -> Result<()> {
    let elem_dir = output.join("elements");
    fs::create_dir_all(&elem_dir)?;

    let mut mod_items = Vec::new();

    for elem in defs.elements.values() {
        let file_name = escape_keyword_filename(&elem.ident.to_snake_case());
        let file_path = elem_dir.join(format!("{}.rs", file_name));

        let tokens = generate_element(elem, defs);
        write_tokens_to_file(&tokens, &file_path)?;

        let mod_name = format_ident!("{}", file_name);
        mod_items.push(quote! {
            mod #mod_name;
            pub use #mod_name::*;
        });
    }

    // Generate mod.rs
    let mod_tokens = quote! {
        //! MEI elements (generated from ODD).
        //!
        //! DO NOT EDIT - regenerate with: cargo run -p mei-codegen

        #(#mod_items)*
    };

    write_tokens_to_file(&mod_tokens, &elem_dir.join("mod.rs"))?;
    println!("  Generated: {} elements", defs.elements.len());

    Ok(())
}

fn generate_element(elem: &Element, defs: &OddDefinitions) -> TokenStream {
    let name = mei_ident_to_type(&elem.ident);
    let xml_name = &elem.ident;
    let doc = if elem.gloss.is_empty() {
        elem.desc.clone()
    } else {
        format!("{} - {}", elem.gloss, elem.desc)
    };

    // Collect attribute class fields with flatten
    let att_class_fields: Vec<_> = elem
        .member_of
        .iter()
        .filter(|m| m.starts_with("att."))
        .map(|m| {
            let field_name = format_ident!(
                "{}",
                m.strip_prefix("att.")
                    .unwrap()
                    .replace('.', "_")
                    .to_snake_case()
            );
            let type_name = mei_ident_to_type(m);
            quote! {
                #[serde(flatten)]
                pub #field_name: crate::generated::att::#type_name,
            }
        })
        .collect();

    // Generate local attribute fields (element-specific attributes not from classes)
    let local_attr_fields: Vec<_> = elem
        .local_attributes
        .iter()
        .map(|attr| {
            let field_name =
                make_safe_ident(&attr.ident.replace(['.', '-', ':'], "_").to_snake_case());
            let field_doc = &attr.desc;
            let xml_name = &attr.ident;

            // Check if this is a multi-valued attribute
            let is_unbounded = attr.max_occurs.as_deref() == Some("unbounded");

            // Determine serde rename (with @ prefix for XML attributes)
            let rename = if is_unbounded {
                // Vec fields use default and is_empty
                if xml_name.starts_with("xml:") {
                    quote! { #[serde(rename = #xml_name, default, skip_serializing_if = "Vec::is_empty")] }
                } else {
                    let attr_name = format!("@{}", xml_name);
                    quote! { #[serde(rename = #attr_name, default, skip_serializing_if = "Vec::is_empty")] }
                }
            } else if xml_name.starts_with("xml:") {
                quote! { #[serde(rename = #xml_name, skip_serializing_if = "Option::is_none")] }
            } else {
                let attr_name = format!("@{}", xml_name);
                quote! { #[serde(rename = #attr_name, skip_serializing_if = "Option::is_none")] }
            };

            // Determine field type
            let field_type = attribute_type_tokens(&attr.datatype, &attr.max_occurs, defs);

            quote! {
                #[doc = #field_doc]
                #rename
                pub #field_name: #field_type,
            }
        })
        .collect();

    // Generate child element enum and field if content model is non-empty
    let (child_enum, child_field) = generate_child_content(elem, defs);

    // Generate model class trait implementations
    let model_impls: Vec<_> = elem
        .member_of
        .iter()
        .filter(|m| m.starts_with("model."))
        .map(|m| {
            let trait_name = mei_ident_to_type(m);
            quote! {
                impl crate::generated::model::#trait_name for #name {}
            }
        })
        .collect();

    // Generate validation implementation
    let validation_impl = generate_element_validation(elem, defs);

    let child_field_tokens = child_field.unwrap_or_else(|| quote! {});
    let module_doc = format!("Element: `<{}>`", elem.ident);

    quote! {
        #![doc = #module_doc]

        use serde::{Deserialize, Serialize};
        use crate::generated::validation::{ValidationContext, Validate};

        #child_enum

        #[doc = #doc]
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
        #[serde(rename = #xml_name)]
        pub struct #name {
            #(#att_class_fields)*
            #(#local_attr_fields)*
            #child_field_tokens
        }

        #(#model_impls)*

        #validation_impl
    }
}

/// Generate validation implementation for an element.
fn generate_element_validation(elem: &Element, defs: &OddDefinitions) -> TokenStream {
    let name = mei_ident_to_type(&elem.ident);
    let _elem_name_str = &elem.ident;

    // Generate constraint checks
    let constraint_checks: Vec<TokenStream> = elem
        .constraints
        .iter()
        .map(|c| {
            let (code, _requires_doc) = translate_constraint(c);
            code
        })
        .collect();

    // Generate attribute class validation calls
    let att_class_validations: Vec<TokenStream> = elem
        .member_of
        .iter()
        .filter(|m| m.starts_with("att."))
        .map(|m| {
            let _field_name = format_ident!(
                "{}",
                m.strip_prefix("att.")
                    .unwrap()
                    .replace('.', "_")
                    .to_snake_case()
            );
            quote! {
                // Validate attribute class: #m
                // self.#_field_name.validate_with_context(ctx);
            }
        })
        .collect();

    // Check if element has children field (same logic as generate_child_content)
    let mut child_types: HashSet<String> = HashSet::new();
    let mut has_text = false;
    collect_content_refs(&elem.content, defs, &mut child_types, &mut has_text);
    let has_children_field = !child_types.is_empty() || has_text;

    let child_validation = if has_children_field {
        quote! {
            // Validate children
            for (i, child) in self.children.iter().enumerate() {
                child.validate_with_context(ctx, i);
            }
        }
    } else {
        quote! {}
    };

    // Try to get xml:id from common attributes if available
    let xml_id_access = if elem.member_of.iter().any(|m| m == "att.common") {
        quote! { self.common.xml_id.as_deref() }
    } else {
        quote! { None }
    };

    quote! {
        impl Validate for #name {
            fn validate_with_context(&self, ctx: &mut ValidationContext) {
                let _xml_id: Option<&str> = #xml_id_access;

                #(#constraint_checks)*

                #(#att_class_validations)*

                #child_validation
            }
        }
    }
}

fn generate_child_content(
    elem: &Element,
    defs: &OddDefinitions,
) -> (TokenStream, Option<TokenStream>) {
    if elem.content.is_empty() {
        return (quote! {}, None);
    }

    // Collect all possible child element types
    let mut child_types: HashSet<String> = HashSet::new();
    let mut has_text = false;

    collect_content_refs(&elem.content, defs, &mut child_types, &mut has_text);

    if child_types.is_empty() && !has_text {
        return (quote! {}, None);
    }

    let elem_name = mei_ident_to_type(&elem.ident);
    let enum_name = format_ident!("{}Child", elem_name);

    // Generate enum variants
    let mut variants = Vec::new();
    let mut validation_arms = Vec::new();

    if has_text {
        variants.push(quote! {
            /// Text content.
            #[serde(rename = "$text")]
            Text(String),
        });
        validation_arms.push(quote! {
            #enum_name::Text(_) => {
                // Text content - no validation needed
            }
        });
    }

    for child in &child_types {
        let var_name = mei_ident_to_type(child);
        let xml_name = child;
        variants.push(quote! {
            #[serde(rename = #xml_name)]
            #var_name(Box<crate::generated::elements::#var_name>),
        });
        validation_arms.push(quote! {
            #enum_name::#var_name(elem) => {
                ctx.enter(#xml_name, index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        });
    }

    if variants.is_empty() {
        return (quote! {}, None);
    }

    let child_doc = format!("Child content for `<{}>`.", elem.ident);
    let child_enum = quote! {
        #[doc = #child_doc]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "lowercase")]
        pub enum #enum_name {
            #(#variants)*
        }

        impl #enum_name {
            /// Validate this child element.
            pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
                match self {
                    #(#validation_arms)*
                }
            }
        }
    };

    // Always use Vec for children to support Default derive on element structs
    // Cardinality constraints (required, optional, etc.) can be enforced at validation time
    let child_field = quote! {
        /// Child elements.
        #[serde(default, rename = "$value")]
        pub children: Vec<#enum_name>,
    };

    (child_enum, Some(child_field))
}

fn collect_content_refs(
    content: &ContentModel,
    defs: &OddDefinitions,
    refs: &mut HashSet<String>,
    has_text: &mut bool,
) {
    for item in content {
        match item {
            ContentItem::Text => *has_text = true,
            ContentItem::Ref(name) => {
                if name.starts_with("model.") {
                    // Resolve model class to elements
                    for elem_name in defs.resolve_model_to_elements(name) {
                        refs.insert(elem_name);
                    }
                } else if name.starts_with("macro.") {
                    // Resolve pattern entity referenced via rng:ref
                    if let Some(pe) = defs.pattern_entities.get(name) {
                        collect_content_refs(&pe.content, defs, refs, has_text);
                    }
                } else if defs.elements.contains_key(name) {
                    refs.insert(name.clone());
                }
            }
            ContentItem::MacroRef(key) => {
                // Resolve pattern entity
                if let Some(pe) = defs.pattern_entities.get(key) {
                    collect_content_refs(&pe.content, defs, refs, has_text);
                }
            }
            ContentItem::ZeroOrMore(inner)
            | ContentItem::OneOrMore(inner)
            | ContentItem::Optional(inner)
            | ContentItem::Group(inner)
            | ContentItem::List(inner) => {
                collect_content_refs(inner, defs, refs, has_text);
            }
            ContentItem::Choice(choices) | ContentItem::Interleave(choices) => {
                for choice in choices {
                    collect_content_refs(choice, defs, refs, has_text);
                }
            }
            ContentItem::Empty | ContentItem::AnyElement => {}
        }
    }
}

// ============================================================================
// Constraint Translation
// ============================================================================

/// Translate a Schematron constraint to Rust code.
/// Returns (condition_code, requires_document_context).
fn translate_constraint(constraint: &crate::ast::Constraint) -> (TokenStream, bool) {
    let test = &constraint.test;
    let message = &constraint.message;
    let ident = &constraint.ident;

    // Try to translate simple patterns
    if let Some(code) = try_translate_simple_constraint(test, message, ident) {
        return (code, false);
    }

    // For complex XPath, generate a TODO comment
    let _test_escaped = test.replace('"', "\\\"");
    let code = quote! {
        // TODO: Complex constraint requires document context
        // Constraint: #ident
        // XPath test: #test
        // Message: #message
    };
    (code, true)
}

/// Try to translate simple constraint patterns.
fn try_translate_simple_constraint(
    test: &str,
    _message: &str,
    _ident: &str,
) -> Option<TokenStream> {
    // Pattern: @attr (attribute must exist)
    if test.starts_with('@') && !test.contains(' ') && !test.contains('[') {
        let attr_name = test.trim_start_matches('@');
        let _field_name = format_ident!(
            "{}",
            attr_name.replace(['.', '-', ':'], "_").to_snake_case()
        );
        return Some(quote! {
            // Constraint: attribute must exist
            // (This is typically checked via usage="req" in attribute definition)
        });
    }

    // Pattern: not(@attr) (attribute must NOT exist)
    if test.starts_with("not(@") && test.ends_with(')') {
        // This is an inverse assertion, typically used with sch:report
        return Some(quote! {
            // Constraint: #ident - inverse check, see context
        });
    }

    // Pattern: @attr1 and @attr2 (both must exist)
    // Pattern: @attr1 or @attr2 (one must exist)
    // These need more context about what fields exist

    // Pattern: @attr = 'value' or @attr eq 'value'
    let eq_patterns = [" = '", " eq '", "='", "eq'"];
    for pat in eq_patterns {
        if test.contains(pat) {
            // Extract attribute and value
            if let Some((attr_part, rest)) = test.split_once(pat) {
                let attr_name = attr_part
                    .trim()
                    .trim_start_matches('@')
                    .trim_start_matches("not(");
                if let Some(value) = rest.strip_suffix("')").or(rest.strip_suffix("'")) {
                    let _field_name = format_ident!(
                        "{}",
                        attr_name.replace(['.', '-', ':'], "_").to_snake_case()
                    );
                    let _value_str = value;
                    // This requires knowing the field type, return generic TODO for now
                }
            }
        }
    }

    // Pattern: not(. eq ../@attr) - check that values don't duplicate
    if test.contains("not(. eq ") || test.contains("not(. = ") {
        return Some(quote! {
            // Constraint: #ident - duplication check
            // #message
        });
    }

    // Pattern: count(mei:X) = N
    if test.contains("count(") {
        return Some(quote! {
            // Constraint: #ident - child count check
            // Test: #test
            // #message
        });
    }

    None
}

// ============================================================================
// Validation Module
// ============================================================================

fn generate_validation(defs: &OddDefinitions, output: &Path) -> Result<()> {
    let path = output.join("validation.rs");

    // Collect all unique constraints for documentation
    let mut all_constraints: Vec<&crate::ast::Constraint> = Vec::new();
    for ac in defs.att_classes.values() {
        all_constraints.extend(ac.constraints.iter());
        for attr in &ac.attributes {
            all_constraints.extend(attr.constraints.iter());
        }
    }
    for elem in defs.elements.values() {
        all_constraints.extend(elem.constraints.iter());
    }

    // Generate constraint list as doc comments
    let constraint_docs: Vec<_> = all_constraints
        .iter()
        .take(20) // Just show first 20 in doc comment
        .map(|c| {
            let ident = &c.ident;
            let ctx = &c.context;
            format!("/// - `{}` (context: `{}`)", ident, ctx)
        })
        .collect();

    let constraint_doc_str = if constraint_docs.is_empty() {
        String::new()
    } else {
        format!(
            "/// \n/// # Sample constraints (showing 20 of {}):\n{}",
            all_constraints.len(),
            constraint_docs.join("\n")
        )
    };

    let tokens = quote! {
        //! MEI validation support (generated from ODD).
        //!
        //! Provides opt-in validation for MEI documents. Validation is NOT performed
        //! during deserialization - call `validate()` explicitly after loading.
        //!
        #![doc = #constraint_doc_str]
        //!
        //! DO NOT EDIT - regenerate with: cargo run -p mei-codegen

        use std::fmt;

        /// Location in the MEI document tree.
        #[derive(Debug, Clone)]
        pub struct Location {
            /// Element path from root (e.g., "mei/music/body/mdiv[0]/score/section[2]/measure[5]/note[3]")
            pub path: String,
            /// Element type name (e.g., "note")
            pub element: String,
            /// xml:id if present
            pub xml_id: Option<String>,
        }

        impl fmt::Display for Location {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if let Some(id) = &self.xml_id {
                    write!(f, "<{}#{}> at {}", self.element, id, self.path)
                } else {
                    write!(f, "<{}> at {}", self.element, self.path)
                }
            }
        }

        /// Validation error with location information.
        #[derive(Debug, Clone)]
        pub enum ValidationError {
            /// A Schematron constraint was violated.
            ConstraintViolation {
                location: Location,
                constraint: String,
                message: String,
            },
            /// An attribute value does not match its pattern.
            PatternMismatch {
                location: Location,
                attribute: String,
                value: String,
                pattern: String,
            },
            /// An attribute value is out of range.
            RangeViolation {
                location: Location,
                attribute: String,
                value: String,
                min: String,
                max: String,
            },
            /// A required attribute is missing.
            MissingRequired {
                location: Location,
                attribute: String,
            },
            /// A reference does not resolve to any element.
            UnresolvedReference {
                location: Location,
                attribute: String,
                reference: String,
            },
        }

        impl fmt::Display for ValidationError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    ValidationError::ConstraintViolation { location, constraint, message } => {
                        write!(f, "[{}] Constraint '{}' violated: {}", location, constraint, message)
                    }
                    ValidationError::PatternMismatch { location, attribute, value, pattern } => {
                        write!(f, "[{}] Attribute @{}: value '{}' does not match pattern /{}/",
                            location, attribute, value, pattern)
                    }
                    ValidationError::RangeViolation { location, attribute, value, min, max } => {
                        write!(f, "[{}] Attribute @{}: value {} out of range [{}, {}]",
                            location, attribute, value, min, max)
                    }
                    ValidationError::MissingRequired { location, attribute } => {
                        write!(f, "[{}] Required attribute @{} is missing", location, attribute)
                    }
                    ValidationError::UnresolvedReference { location, attribute, reference } => {
                        write!(f, "[{}] Reference @{}='{}' does not resolve to any element",
                            location, attribute, reference)
                    }
                }
            }
        }

        impl std::error::Error for ValidationError {}

        /// Validation context that tracks location in the document tree.
        pub struct ValidationContext {
            path_stack: Vec<String>,
            errors: Vec<ValidationError>,
        }

        impl Default for ValidationContext {
            fn default() -> Self {
                Self::new()
            }
        }

        impl ValidationContext {
            /// Create a new validation context.
            pub fn new() -> Self {
                Self {
                    path_stack: Vec::new(),
                    errors: Vec::new(),
                }
            }

            /// Enter a child element.
            pub fn enter(&mut self, element: &str, index: usize) {
                self.path_stack.push(format!("{}[{}]", element, index));
            }

            /// Exit the current element.
            pub fn exit(&mut self) {
                self.path_stack.pop();
            }

            /// Get the current location.
            pub fn location(&self, element: &str, xml_id: Option<&str>) -> Location {
                Location {
                    path: self.path_stack.join("/"),
                    element: element.to_string(),
                    xml_id: xml_id.map(String::from),
                }
            }

            /// Add a validation error.
            pub fn add_error(&mut self, error: ValidationError) {
                self.errors.push(error);
            }

            /// Add a constraint violation error.
            pub fn add_constraint_violation(
                &mut self,
                element: &str,
                xml_id: Option<&str>,
                constraint: &str,
                message: &str,
            ) {
                self.errors.push(ValidationError::ConstraintViolation {
                    location: self.location(element, xml_id),
                    constraint: constraint.to_string(),
                    message: message.to_string(),
                });
            }

            /// Add a pattern mismatch error.
            pub fn add_pattern_mismatch(
                &mut self,
                element: &str,
                xml_id: Option<&str>,
                attribute: &str,
                value: &str,
                pattern: &str,
            ) {
                self.errors.push(ValidationError::PatternMismatch {
                    location: self.location(element, xml_id),
                    attribute: attribute.to_string(),
                    value: value.to_string(),
                    pattern: pattern.to_string(),
                });
            }

            /// Finish validation and return the result.
            pub fn finish(self) -> ValidationResult {
                if self.errors.is_empty() {
                    Ok(())
                } else {
                    Err(self.errors)
                }
            }

            /// Check if any errors have been recorded.
            pub fn has_errors(&self) -> bool {
                !self.errors.is_empty()
            }

            /// Get the current error count.
            pub fn error_count(&self) -> usize {
                self.errors.len()
            }
        }

        /// Result of validation.
        pub type ValidationResult = Result<(), Vec<ValidationError>>;

        /// Trait for types that can be validated.
        pub trait Validate {
            /// Validate this item, adding any errors to the context.
            fn validate_with_context(&self, ctx: &mut ValidationContext);

            /// Convenience method for standalone validation.
            fn validate(&self) -> ValidationResult {
                let mut ctx = ValidationContext::new();
                self.validate_with_context(&mut ctx);
                ctx.finish()
            }
        }
    };

    write_tokens_to_file(&tokens, &path)?;
    println!("  Generated: validation.rs");

    Ok(())
}

// ============================================================================
// lib.rs
// ============================================================================

fn generate_mod_rs(_defs: &OddDefinitions, output: &Path) -> Result<()> {
    let tokens = quote! {
        //! Generated types from MEI ODD specification.
        //!
        //! This module contains Rust types that map 1:1 to MEI constructs.
        //!
        //! DO NOT EDIT - regenerate with: cargo run -p mei-codegen

        pub mod data;
        pub mod att;
        pub mod model;
        pub mod pattern_entities;
        pub mod elements;
        pub mod validation;

        pub use data::*;
        pub use elements::*;
        pub use validation::{Validate, ValidationContext, ValidationError, ValidationResult};

        use serde::{Deserialize, Deserializer, Serialize, Serializer};
        use std::fmt;
        use std::str::FromStr;

        /// Wrapper for space-separated list values in MEI attributes.
        ///
        /// MEI uses space-separated lists for some attributes (e.g., bezier coordinates).
        /// This wrapper handles serialization/deserialization of such values.
        #[derive(Debug, Clone, PartialEq, Default)]
        pub struct SpaceSeparated<T>(pub Vec<T>);

        impl<T> SpaceSeparated<T> {
            pub fn new(items: Vec<T>) -> Self {
                Self(items)
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }

        impl<T: fmt::Display> Serialize for SpaceSeparated<T> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let s: std::string::String = self.0.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
                serializer.serialize_str(&s)
            }
        }

        impl<'de, T> Deserialize<'de> for SpaceSeparated<T>
        where
            T: FromStr,
            T::Err: fmt::Display,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = std::string::String::deserialize(deserializer)?;
                let items: Result<Vec<T>, _> = s
                    .split_whitespace()
                    .map(|part| part.parse::<T>().map_err(serde::de::Error::custom))
                    .collect();
                Ok(SpaceSeparated(items?))
            }
        }
    };

    write_tokens_to_file(&tokens, &output.join("mod.rs"))?;
    println!("  Generated: mod.rs");

    Ok(())
}

// ============================================================================
// Helpers
// ============================================================================

/// Convert MEI identifier to Rust type name.
/// e.g., "data.DURATION.cmn" -> "DataDurationCmn"
/// e.g., "att.duration.log" -> "AttDurationLog"
/// e.g., "note" -> "Note"
fn mei_ident_to_type(ident: &str) -> Ident {
    let name = ident
        // Split by both '.' and '_' to handle identifiers like data.MIDIVALUE_pan
        .split(|c| c == '.' || c == '_')
        .map(|part| {
            if part.chars().all(|c| c.is_uppercase() || c.is_numeric()) {
                // Convert UPPERCASE to Titlecase
                let mut chars = part.chars();
                match chars.next() {
                    Some(first) => first
                        .to_uppercase()
                        .chain(chars.flat_map(|c| c.to_lowercase()))
                        .collect(),
                    None => String::new(),
                }
            } else {
                part.to_upper_camel_case()
            }
        })
        .collect::<String>();

    format_ident!("{}", name)
}

/// Convert MEI value identifier to Rust enum variant.
/// e.g., "1" -> "N1" (prefix with N for numeric)
/// e.g., "6.0-dev" -> "N6_0Dev"
/// e.g., "long" -> "Long"
fn mei_value_to_variant(ident: &str) -> Ident {
    // First, sanitize by replacing non-alphanumeric with underscores for proper camel casing
    let sanitized = ident.replace(['.', '-', ':', '+', '/', '*', '#', '@'], "_");

    let name = if ident.is_empty() {
        "Empty".to_string()
    } else if ident.chars().next().map_or(false, |c| c.is_numeric()) {
        // Numeric prefix - use sanitized version with N prefix
        format!("N{}", sanitized.to_upper_camel_case())
    } else {
        let camel = sanitized.to_upper_camel_case();
        // to_upper_camel_case can return empty for non-alphanumeric inputs
        if camel.is_empty() {
            format!(
                "V{}",
                ident
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            )
        } else {
            camel
        }
    };

    // Handle reserved keywords
    let name = match name.as_str() {
        "Self" => "Self_".to_string(),
        "Type" => "Type_".to_string(),
        "Mod" => "Mod_".to_string(),
        "" => "Unknown".to_string(), // Final fallback
        _ => name,
    };

    format_ident!("{}", name)
}

/// Map XML Schema datatype to Rust type.
fn rng_data_to_rust(type_name: &str) -> &'static str {
    match type_name {
        "string" => "String",
        "token" => "String",
        "NMTOKEN" => "String",
        "NMTOKENS" => "String",
        "ID" => "String",
        "IDREF" => "String",
        "IDREFS" => "String",
        "decimal" => "f64",
        "integer" => "i64",
        "nonNegativeInteger" => "u64",
        "positiveInteger" => "u64",
        "boolean" => "bool",
        "anyURI" => "String",
        "language" => "String",
        "NCName" => "String",
        "Name" => "String",
        "QName" => "String",
        "date" => "String",
        "dateTime" => "String",
        "time" => "String",
        "duration" => "String",
        "base64Binary" => "String",
        "hexBinary" => "String",
        _ => "String",
    }
}

/// Rust keywords that need special handling.
const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

/// Create an identifier, escaping Rust keywords with r# prefix.
fn make_safe_ident(name: &str) -> Ident {
    if RUST_KEYWORDS.contains(&name) {
        format_ident!("r#{}", name)
    } else {
        format_ident!("{}", name)
    }
}

/// Escape keyword for use in file/module names by appending underscore.
fn escape_keyword_filename(name: &str) -> String {
    if RUST_KEYWORDS.contains(&name) {
        format!("{}_", name)
    } else {
        name.to_string()
    }
}

// ============================================================================
// MEI Attribute Trait Impls (ExtractAttributes / CollectAttributes)
// ============================================================================

/// Generate ExtractAttributes and CollectAttributes impls for all attribute
/// classes and write them into the tusk-mei crate source tree.
pub fn generate_mei_attr_impls(defs: &OddDefinitions, mei_crate_path: &Path) -> Result<()> {
    // Generate ExtractAttributes impls
    let extract_tokens = generate_extract_attributes_impls(defs);
    let extract_path = mei_crate_path.join("deserializer/impls/generated_att_impls.rs");
    write_tokens_to_file(&extract_tokens, &extract_path)?;

    // Generate CollectAttributes impls
    let collect_tokens = generate_collect_attributes_impls(defs);
    let collect_path = mei_crate_path.join("serializer/impls/generated_att_impls.rs");
    write_tokens_to_file(&collect_tokens, &collect_path)?;

    let count = defs.att_classes.len();
    println!(
        "  Generated: {} ExtractAttributes + {} CollectAttributes impls",
        count, count
    );

    Ok(())
}

/// Generate ExtractAttributes impls for all attribute classes.
fn generate_extract_attributes_impls(defs: &OddDefinitions) -> TokenStream {
    let mut impls = Vec::new();

    for ac in defs.att_classes.values() {
        impls.push(generate_extract_attributes_impl(ac, defs));
    }

    quote! {
        //! Auto-generated ExtractAttributes impls for all MEI attribute classes.
        //!
        //! DO NOT EDIT - regenerate with:
        //!   cargo run -p mei-codegen -- -i specs/mei/modules -o crates/core/model/src/generated --mei-crate crates/formats/mei/src

        use super::super::{AttributeMap, DeserializeResult, ExtractAttributes};
        #[allow(unused_imports)]
        use super::from_attr_string;
        use tusk_model::att::*;

        #(#impls)*
    }
}

/// Generate CollectAttributes impls for all attribute classes.
fn generate_collect_attributes_impls(defs: &OddDefinitions) -> TokenStream {
    let mut impls = Vec::new();

    for ac in defs.att_classes.values() {
        impls.push(generate_collect_attributes_impl(ac, defs));
    }

    quote! {
        //! Auto-generated CollectAttributes impls for all MEI attribute classes.
        //!
        //! DO NOT EDIT - regenerate with:
        //!   cargo run -p mei-codegen -- -i specs/mei/modules -o crates/core/model/src/generated --mei-crate crates/formats/mei/src

        use super::super::CollectAttributes;
        #[allow(unused_imports)]
        use super::{to_attr_string, serialize_vec_serde};
        use tusk_model::att::*;

        #(#impls)*
    }
}

/// Generate a single ExtractAttributes impl for one attribute class.
fn generate_extract_attributes_impl(ac: &AttClass, defs: &OddDefinitions) -> TokenStream {
    let name = mei_ident_to_type(&ac.ident);
    let all_attrs = defs.collect_attributes(&ac.ident);

    let extractions: Vec<TokenStream> = all_attrs
        .iter()
        .map(|attr| {
            let xml_name = &attr.ident;
            let field_name =
                make_safe_ident(&attr.ident.replace(['.', '-', ':'], "_").to_snake_case());
            let is_unbounded = attr.max_occurs.as_deref() == Some("unbounded");

            match (&attr.datatype, is_unbounded) {
                // Vec<String> — datatype=None + unbounded
                (None, true) => {
                    quote! { extract_attr!(attrs, #xml_name, vec_string self.#field_name); }
                }
                // Option<String> — datatype=None
                (None, false) => {
                    quote! { extract_attr!(attrs, #xml_name, string self.#field_name); }
                }
                // Vec<T> — Ref(known) + unbounded
                (Some(AttributeDataType::Ref(ref_name)), true)
                    if defs.data_types.contains_key(ref_name) =>
                {
                    quote! { extract_attr!(attrs, #xml_name, vec self.#field_name); }
                }
                // Vec<String> — Ref(unknown) + unbounded
                (Some(AttributeDataType::Ref(_)), true) => {
                    quote! { extract_attr!(attrs, #xml_name, vec_string self.#field_name); }
                }
                // Option<T> — Ref(known)
                (Some(AttributeDataType::Ref(ref_name)), false)
                    if defs.data_types.contains_key(ref_name) =>
                {
                    quote! { extract_attr!(attrs, #xml_name, self.#field_name); }
                }
                // Option<String> — Ref(unknown)
                (Some(AttributeDataType::Ref(_)), false) => {
                    quote! { extract_attr!(attrs, #xml_name, string self.#field_name); }
                }
                // Vec<T> — Primitive + unbounded
                (Some(AttributeDataType::Primitive { type_name, .. }), true) => {
                    if is_string_primitive(type_name) {
                        quote! { extract_attr!(attrs, #xml_name, vec_string self.#field_name); }
                    } else {
                        quote! { extract_attr!(attrs, #xml_name, vec self.#field_name); }
                    }
                }
                // Option<T> — Primitive
                (Some(AttributeDataType::Primitive { type_name, .. }), false) => {
                    if is_string_primitive(type_name) {
                        quote! { extract_attr!(attrs, #xml_name, string self.#field_name); }
                    } else {
                        quote! { extract_attr!(attrs, #xml_name, self.#field_name); }
                    }
                }
                // Vec<T> — InlineValList + unbounded
                (Some(AttributeDataType::InlineValList(_)), true) => {
                    quote! { extract_attr!(attrs, #xml_name, vec self.#field_name); }
                }
                // Option<T> — InlineValList
                (Some(AttributeDataType::InlineValList(_)), false) => {
                    quote! { extract_attr!(attrs, #xml_name, self.#field_name); }
                }
                // Option<SpaceSeparated<T>> — List
                (Some(AttributeDataType::List { .. }), _) => {
                    quote! { extract_attr!(attrs, #xml_name, space_separated self.#field_name); }
                }
            }
        })
        .collect();

    quote! {
        impl ExtractAttributes for #name {
            fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
                #(#extractions)*
                Ok(())
            }
        }
    }
}

/// Generate a single CollectAttributes impl for one attribute class.
fn generate_collect_attributes_impl(ac: &AttClass, defs: &OddDefinitions) -> TokenStream {
    let name = mei_ident_to_type(&ac.ident);
    let all_attrs = defs.collect_attributes(&ac.ident);

    let collections: Vec<TokenStream> = all_attrs
        .iter()
        .map(|attr| {
            let xml_name = &attr.ident;
            let field_name =
                make_safe_ident(&attr.ident.replace(['.', '-', ':'], "_").to_snake_case());
            let is_unbounded = attr.max_occurs.as_deref() == Some("unbounded");

            match (&attr.datatype, is_unbounded) {
                // Vec<String> — datatype=None + unbounded
                (None, true) => {
                    quote! { push_attr!(attrs, #xml_name, vec self.#field_name); }
                }
                // Option<String> — datatype=None
                (None, false) => {
                    quote! { push_attr!(attrs, #xml_name, clone self.#field_name); }
                }
                // Vec<T> — any type + unbounded
                (Some(_), true) => {
                    quote! { push_attr!(attrs, #xml_name, vec self.#field_name); }
                }
                // Option<T> — Ref(unknown) → clone
                (Some(AttributeDataType::Ref(ref_name)), false)
                    if !defs.data_types.contains_key(ref_name) =>
                {
                    quote! { push_attr!(attrs, #xml_name, clone self.#field_name); }
                }
                // Option<T> — Primitive(String type) → clone
                (Some(AttributeDataType::Primitive { type_name, .. }), false)
                    if is_string_primitive(type_name) =>
                {
                    quote! { push_attr!(attrs, #xml_name, clone self.#field_name); }
                }
                // Option<T> — Ref(known), Primitive(non-string), InlineValList, List → default
                (Some(_), false) => {
                    quote! { push_attr!(attrs, #xml_name, self.#field_name); }
                }
            }
        })
        .collect();

    if collections.is_empty() {
        quote! {
            impl CollectAttributes for #name {
                fn collect_attributes(&self) -> Vec<(&'static str, String)> {
                    Vec::new()
                }
            }
        }
    } else {
        quote! {
            impl CollectAttributes for #name {
                fn collect_attributes(&self) -> Vec<(&'static str, String)> {
                    let mut attrs = Vec::new();
                    #(#collections)*
                    attrs
                }
            }
        }
    }
}

/// Check if an XML Schema primitive type maps to String in Rust.
fn is_string_primitive(type_name: &str) -> bool {
    matches!(rng_data_to_rust(type_name), "String")
}

/// Write token stream to file with formatting.
fn write_tokens_to_file(tokens: &TokenStream, path: &Path) -> Result<()> {
    let code = tokens.to_string();

    // Try to format with prettyplease, fall back to raw output
    let formatted = match syn::parse_file(&code) {
        Ok(syntax_tree) => prettyplease::unparse(&syntax_tree),
        Err(_) => code,
    };

    fs::write(path, formatted)?;
    Ok(())
}
