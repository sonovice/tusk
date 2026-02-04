//! MusicXML 4.0 dynamics types.

use serde::{Deserialize, Serialize};

/// Dynamic marking container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dynamics {
    /// Dynamic values (ppp, pp, p, mp, mf, f, ff, fff, etc.)
    #[serde(rename = "$value")]
    pub values: Vec<DynamicsValue>,
}

/// Individual dynamic marking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DynamicsValue {
    /// Pianississimo (ppp)
    Ppp,
    /// Pianissimo (pp)
    Pp,
    /// Piano (p)
    P,
    /// Mezzo-piano (mp)
    Mp,
    /// Mezzo-forte (mf)
    Mf,
    /// Forte (f)
    F,
    /// Fortissimo (ff)
    Ff,
    /// Fortississimo (fff)
    Fff,
    /// Forte-piano (fp)
    Fp,
    /// Sforzando (sf)
    Sf,
    /// Sforzando-forte (sfz)
    Sfz,
    /// Sforzando-piano (sfp)
    Sfp,
    /// Sforzando-pianissimo (sfpp)
    Sfpp,
    /// Sforzando-fortissimo (sffz)
    Sffz,
    /// Sforzando-forte-piano (sfzp) - MusicXML 4.0
    Sfzp,
    /// Rinforzando (rf)
    Rf,
    /// Rinforzando-forte (rfz)
    Rfz,
    /// Fortepiano (fz)
    Fz,
    /// Niente (n)
    N,
    /// Pianissississimo (pppp) - very rare
    Pppp,
    /// Fortissississimo (ffff) - very rare
    Ffff,
    /// Pianississississimo (ppppp) - very rare
    Ppppp,
    /// Fortississississimo (fffff) - very rare
    Fffff,
    /// Pianissississississimo (pppppp) - very rare
    Pppppp,
    /// Fortississississimo (ffffff) - very rare
    Ffffff,
    /// Other dynamics not in the standard list
    #[serde(rename = "other-dynamics")]
    OtherDynamics(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamics_values() {
        let dynamics = Dynamics {
            values: vec![DynamicsValue::Mf],
        };
        assert_eq!(dynamics.values.len(), 1);
    }

    #[test]
    fn test_dynamics_multiple_values() {
        let dynamics = Dynamics {
            values: vec![DynamicsValue::Sf, DynamicsValue::P],
        };
        assert_eq!(dynamics.values.len(), 2);
    }

    #[test]
    fn test_dynamics_other() {
        let dynamics = Dynamics {
            values: vec![DynamicsValue::OtherDynamics("sfffz".to_string())],
        };
        if let DynamicsValue::OtherDynamics(s) = &dynamics.values[0] {
            assert_eq!(s, "sfffz");
        } else {
            panic!("Expected OtherDynamics");
        }
    }
}
