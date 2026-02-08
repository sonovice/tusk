//! Generate Rust code from the MusicXML XSD schema AST.

use anyhow::Result;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::ast::*;

/// Map XSD type name to Rust type (for primitives and common types).
fn xsd_type_to_rust(s: &str) -> String {
    match s {
        "xs:string" | "xs:token" | "xs:normalizedString" => "String".to_string(),
        "xs:decimal" | "xs:float" | "xs:double" => "f64".to_string(),
        "xs:integer" | "xs:nonNegativeInteger" | "xs:positiveInteger" | "xs:int" | "xs:long" => "i64".to_string(),
        "xs:boolean" => "bool".to_string(),
        "xs:anyURI" => "String".to_string(),
        _ => {
            // MusicXML type like "yes-no", "left-center-right" -> enum or alias
            let name = s.trim_start_matches("xs:");
            if name.contains('-') || name.chars().next().map(|c| c.is_lowercase()).unwrap_or(false) {
                type_name_to_rust(name)
            } else {
                type_name_to_rust(name)
            }
        }
    }
}

fn type_name_to_rust(name: &str) -> String {
    if name.is_empty() {
        return "String".to_string();
    }
    let mut s = name.to_string();
    if s.starts_with("xs:") {
        s = s[3..].to_string();
    }
    s.split('-')
        .map(|p| {
            let mut c = p.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(c).collect(),
            }
        })
        .collect::<String>()
}

/// Generate all Rust files into output directory.
pub fn generate(schema: &Schema, output: &Path) -> Result<()> {
    fs::create_dir_all(output)?;
    generate_data(schema, &output.join("data.rs"))?;
    generate_mod(schema, &output.join("mod.rs"))?;
    println!("  Generated: {}", output.join("data.rs").display());
    println!("  Generated: {}", output.join("mod.rs").display());
    Ok(())
}

fn generate_data(schema: &Schema, path: &Path) -> Result<()> {
    let mut out = String::from(
        "//! MusicXML data types (generated from XSD). DO NOT EDIT.\n\n\
         use serde::{Deserialize, Serialize};\n\n",
    );

    let mut seen = HashSet::new();
    for (name, st) in &schema.simple_types {
        let rust_name = type_name_to_rust(name);
        if seen.contains(&rust_name) {
            continue;
        }
        seen.insert(rust_name.clone());
        match st {
            SimpleType::Enum { base: _, values } => {
                out.push_str(&format!(
                    "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]\n\
                     #[serde(rename_all = \"kebab-case\")]\npub enum {} {{\n",
                    rust_name
                ));
                for v in values {
                    let variant = type_name_to_rust(v);
                    out.push_str(&format!("    {},\n", variant));
                }
                out.push_str("}\n\n");
            }
            SimpleType::Alias { base, .. } => {
                let rust_ty = xsd_type_to_rust(base);
                if rust_ty == "String" || base.starts_with("xs:") {
                    continue;
                }
                out.push_str(&format!("pub type {} = {};\n\n", rust_name, rust_ty));
            }
        }
    }

    fs::write(path, out)?;
    Ok(())
}

fn generate_mod(_schema: &Schema, path: &Path) -> Result<()> {
    let mut out = String::from(
        "//! MusicXML model (generated from XSD). DO NOT EDIT.\n\npub mod data;\n\n",
    );
    out.push_str("// Re-export data types\npub use data::*;\n");
    fs::write(path, out)?;
    Ok(())
}
