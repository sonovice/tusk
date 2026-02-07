//! MEI validation support (generated from ODD).
//!
//! Provides opt-in validation for MEI documents. Validation is NOT performed
//! during deserialization - call `validate()` explicitly after loading.
//!
#![doc = "/// \n/// # Sample constraints (showing 20 of 241):\n/// - `extremis_disallows_gestural_pitch` (context: `mei:note[@extremis]`)\n/// - `check_barmethod` (context: `@bar.method[parent::*[matches(local-name(), '(staffDef|measure)')]]`)\n/// - `mensuration_conflicting_attributes` (context: `mei:mensur[@divisio]`)\n/// - `check_altsymTarget` (context: `@altsym`)\n/// - `check_altsymTarget` (context: `@altsym`)\n/// - `check_altsymTarget` (context: `@altsym`)\n/// - `check_dataTarget` (context: `@data`)\n/// - `check_dataTarget` (context: `@data`)\n/// - `meiVersion.onlyRoot` (context: `/mei:*//*`)\n/// - `check_defTarget_layer` (context: `mei:layer/@def`)\n/// - `check_defTarget_layer` (context: `mei:layer/@def`)\n/// - `checkComponentType` (context: `mei:*[@comptype]`)\n/// - `check_whenTarget` (context: `@when`)\n/// - `check_whenTarget` (context: `@when`)\n/// - `check_duplex_quality` (context: `(mei:note|mei:space)[@dur.quality='duplex']`)\n/// - `check_maiorminor_quality` (context: `(mei:note|mei:space)[@dur.quality='maior' or @dur.quality='minor']`)\n/// - `check_sourceTarget` (context: `@source`)\n/// - `check_sourceTarget` (context: `@source`)\n/// - `check_handTarget` (context: `@hand`)\n/// - `check_handTarget` (context: `@hand`)"]
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
    ConstraintViolation { location: Location, constraint: String, message: String },
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
    MissingRequired { location: Location, attribute: String },
    /// A reference does not resolve to any element.
    UnresolvedReference { location: Location, attribute: String, reference: String },
}
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::ConstraintViolation { location, constraint, message } => {
                write!(
                    f, "[{}] Constraint '{}' violated: {}", location, constraint, message
                )
            }
            ValidationError::PatternMismatch { location, attribute, value, pattern } => {
                write!(
                    f, "[{}] Attribute @{}: value '{}' does not match pattern /{}/",
                    location, attribute, value, pattern
                )
            }
            ValidationError::RangeViolation { location, attribute, value, min, max } => {
                write!(
                    f, "[{}] Attribute @{}: value {} out of range [{}, {}]", location,
                    attribute, value, min, max
                )
            }
            ValidationError::MissingRequired { location, attribute } => {
                write!(f, "[{}] Required attribute @{} is missing", location, attribute)
            }
            ValidationError::UnresolvedReference { location, attribute, reference } => {
                write!(
                    f, "[{}] Reference @{}='{}' does not resolve to any element",
                    location, attribute, reference
                )
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
        self.errors
            .push(ValidationError::ConstraintViolation {
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
        self.errors
            .push(ValidationError::PatternMismatch {
                location: self.location(element, xml_id),
                attribute: attribute.to_string(),
                value: value.to_string(),
                pattern: pattern.to_string(),
            });
    }
    /// Finish validation and return the result.
    pub fn finish(self) -> ValidationResult {
        if self.errors.is_empty() { Ok(()) } else { Err(self.errors) }
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
