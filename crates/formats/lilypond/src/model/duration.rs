//! Duration representation for LilyPond AST.
//!
//! Mirrors the `steno_duration`, `dots`, and `multipliers` productions in
//! the grammar.

/// A LilyPond duration value.
///
/// Durations are expressed as a base note value (power of 2), optional dots,
/// and optional multiplier(s).
///
/// Examples:
/// - `4` → base 4 (quarter note), 0 dots
/// - `2.` → base 2 (half note), 1 dot
/// - `8..` → base 8 (eighth note), 2 dots
/// - `1*3/4` → base 1 (whole), multiplier 3/4
#[derive(Debug, Clone, PartialEq)]
pub struct Duration {
    /// Base note value as the denominator: 1=whole, 2=half, 4=quarter,
    /// 8=eighth, 16=sixteenth, etc. Also supports `\\breve` (0), `\\longa` (-1),
    /// `\\maxima` (-2) represented as 0 internally for now.
    pub base: u32,
    /// Number of augmentation dots.
    pub dots: u8,
    /// Optional duration multipliers, applied in order.
    /// Each is (numerator, denominator). E.g. `*3` → (3,1), `*3/4` → (3,4).
    pub multipliers: Vec<(u32, u32)>,
}

impl Duration {
    /// Check if the base value is a valid LilyPond duration.
    ///
    /// Valid values are powers of 2: 1, 2, 4, 8, 16, 32, 64, 128.
    pub fn is_valid_base(base: u32) -> bool {
        base > 0 && base.is_power_of_two() && base <= 128
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_bases() {
        for base in [1, 2, 4, 8, 16, 32, 64, 128] {
            assert!(Duration::is_valid_base(base), "base {base} should be valid");
        }
    }

    #[test]
    fn invalid_bases() {
        for base in [0, 3, 5, 6, 7, 256] {
            assert!(
                !Duration::is_valid_base(base),
                "base {base} should be invalid"
            );
        }
    }
}
