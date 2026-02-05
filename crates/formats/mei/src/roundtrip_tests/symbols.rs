//! Round-trip serialization tests for symbol-related MEI elements.
//!
//! Tests for SymbolTable, SymbolDef, SymName, SymProp, PropName, PropValue, Mapping elements.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// SymbolTable Tests
// ============================================================================

#[test]
fn symbol_table_roundtrip_empty() {
    use tusk_model::elements::SymbolTable;

    let original = SymbolTable::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = SymbolTable::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn symbol_table_roundtrip_with_xml_id() {
    use tusk_model::elements::SymbolTable;

    let mut original = SymbolTable::default();
    original.common.xml_id = Some("symtable-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = SymbolTable::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("symtable-1".to_string()));
}

#[test]
fn symbol_table_roundtrip_with_symbol_def() {
    use tusk_model::elements::{SymbolDef, SymbolTable, SymbolTableChild};

    let mut original = SymbolTable::default();
    original.common.xml_id = Some("symtable-1".to_string());

    let mut symbol_def = SymbolDef::default();
    symbol_def.common.xml_id = Some("symdef-1".to_string());
    original
        .children
        .push(SymbolTableChild::SymbolDef(Box::new(symbol_def)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = SymbolTable::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SymbolTableChild::SymbolDef(sd) => {
            assert_eq!(sd.common.xml_id, Some("symdef-1".to_string()));
        }
    }
}

// ============================================================================
// SymbolDef Tests
// ============================================================================

#[test]
fn symbol_def_roundtrip_empty() {
    use tusk_model::elements::SymbolDef;

    let original = SymbolDef::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = SymbolDef::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn symbol_def_roundtrip_with_attributes() {
    use tusk_model::elements::SymbolDef;

    let mut original = SymbolDef::default();
    original.common.xml_id = Some("symdef-1".to_string());
    original.coordinated.ulx = Some(10);
    original.coordinated.uly = Some(20);
    original.coordinated.lrx = Some(100);
    original.coordinated.lry = Some(80);
    original.data_selecting.select = Some("/mei:mei/mei:music".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = SymbolDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("symdef-1".to_string()));
    assert_eq!(parsed.coordinated.ulx, Some(10));
    assert_eq!(parsed.coordinated.uly, Some(20));
    assert_eq!(parsed.coordinated.lrx, Some(100));
    assert_eq!(parsed.coordinated.lry, Some(80));
    assert_eq!(
        parsed.data_selecting.select,
        Some("/mei:mei/mei:music".to_string())
    );
}

#[test]
fn symbol_def_roundtrip_with_sym_name() {
    use tusk_model::elements::{SymName, SymNameChild, SymbolDef, SymbolDefChild};

    let mut original = SymbolDef::default();
    original.common.xml_id = Some("symdef-1".to_string());

    let mut sym_name = SymName::default();
    sym_name.common.xml_id = Some("symname-1".to_string());
    sym_name
        .children
        .push(SymNameChild::Text("My Symbol".to_string()));
    original
        .children
        .push(SymbolDefChild::SymName(Box::new(sym_name)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = SymbolDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SymbolDefChild::SymName(sn) => {
            assert_eq!(sn.common.xml_id, Some("symname-1".to_string()));
            assert_eq!(sn.children.len(), 1);
            match &sn.children[0] {
                SymNameChild::Text(t) => assert_eq!(t, "My Symbol"),
            }
        }
        _ => panic!("Expected SymName child"),
    }
}

#[test]
fn symbol_def_roundtrip_with_sym_prop() {
    use tusk_model::elements::{
        PropName, PropNameChild, PropValue, PropValueChild, SymProp, SymPropChild, SymbolDef,
        SymbolDefChild,
    };

    let mut original = SymbolDef::default();
    original.common.xml_id = Some("symdef-1".to_string());

    let mut sym_prop = SymProp::default();
    sym_prop.common.xml_id = Some("symprop-1".to_string());

    let mut prop_name = PropName::default();
    prop_name
        .children
        .push(PropNameChild::Text("width".to_string()));
    sym_prop
        .children
        .push(SymPropChild::PropName(Box::new(prop_name)));

    let mut prop_value = PropValue::default();
    prop_value
        .children
        .push(PropValueChild::Text("100".to_string()));
    sym_prop
        .children
        .push(SymPropChild::PropValue(Box::new(prop_value)));

    original
        .children
        .push(SymbolDefChild::SymProp(Box::new(sym_prop)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = SymbolDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SymbolDefChild::SymProp(sp) => {
            assert_eq!(sp.common.xml_id, Some("symprop-1".to_string()));
            assert_eq!(sp.children.len(), 2);
        }
        _ => panic!("Expected SymProp child"),
    }
}

// ============================================================================
// SymName Tests
// ============================================================================

#[test]
fn sym_name_roundtrip_empty() {
    use tusk_model::elements::SymName;

    let original = SymName::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = SymName::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn sym_name_roundtrip_with_text() {
    use tusk_model::elements::{SymName, SymNameChild};

    let mut original = SymName::default();
    original.common.xml_id = Some("symname-1".to_string());
    original
        .children
        .push(SymNameChild::Text("MUSICAL SYMBOL CODA".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = SymName::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("symname-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SymNameChild::Text(t) => assert_eq!(t, "MUSICAL SYMBOL CODA"),
    }
}

// ============================================================================
// SymProp Tests
// ============================================================================

#[test]
fn sym_prop_roundtrip_empty() {
    use tusk_model::elements::SymProp;

    let original = SymProp::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = SymProp::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn sym_prop_roundtrip_with_children() {
    use tusk_model::elements::{
        PropName, PropNameChild, PropValue, PropValueChild, SymProp, SymPropChild,
    };

    let mut original = SymProp::default();
    original.common.xml_id = Some("symprop-1".to_string());

    let mut prop_name = PropName::default();
    prop_name
        .children
        .push(PropNameChild::Text("height".to_string()));
    original
        .children
        .push(SymPropChild::PropName(Box::new(prop_name)));

    let mut prop_value = PropValue::default();
    prop_value
        .children
        .push(PropValueChild::Text("50".to_string()));
    original
        .children
        .push(SymPropChild::PropValue(Box::new(prop_value)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = SymProp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("symprop-1".to_string()));
    assert_eq!(parsed.children.len(), 2);

    match &parsed.children[0] {
        SymPropChild::PropName(pn) => {
            assert_eq!(pn.children.len(), 1);
            match &pn.children[0] {
                PropNameChild::Text(t) => assert_eq!(t, "height"),
            }
        }
        _ => panic!("Expected PropName child"),
    }

    match &parsed.children[1] {
        SymPropChild::PropValue(pv) => {
            assert_eq!(pv.children.len(), 1);
            match &pv.children[0] {
                PropValueChild::Text(t) => assert_eq!(t, "50"),
            }
        }
        _ => panic!("Expected PropValue child"),
    }
}

// ============================================================================
// PropName Tests
// ============================================================================

#[test]
fn prop_name_roundtrip_empty() {
    use tusk_model::elements::PropName;

    let original = PropName::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = PropName::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn prop_name_roundtrip_with_type() {
    use tusk_model::elements::{PropName, PropNameChild};

    let mut original = PropName::default();
    original.basic.xml_id = Some("propname-1".to_string());
    original.r#type = Some("dimension".to_string());
    original
        .children
        .push(PropNameChild::Text("width".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = PropName::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("propname-1".to_string()));
    assert_eq!(parsed.r#type, Some("dimension".to_string()));
    assert_eq!(parsed.children.len(), 1);
}

// ============================================================================
// PropValue Tests
// ============================================================================

#[test]
fn prop_value_roundtrip_empty() {
    use tusk_model::elements::PropValue;

    let original = PropValue::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = PropValue::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn prop_value_roundtrip_with_text() {
    use tusk_model::elements::{PropValue, PropValueChild};

    let mut original = PropValue::default();
    original.common.xml_id = Some("propvalue-1".to_string());
    original
        .children
        .push(PropValueChild::Text("42".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = PropValue::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("propvalue-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        PropValueChild::Text(t) => assert_eq!(t, "42"),
    }
}

// ============================================================================
// Mapping Tests
// ============================================================================

#[test]
fn mapping_roundtrip_empty() {
    use tusk_model::elements::Mapping;

    let original = Mapping::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mapping::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn mapping_roundtrip_with_text() {
    use tusk_model::elements::{Mapping, MappingChild};

    let mut original = Mapping::default();
    original.common.xml_id = Some("mapping-1".to_string());
    original
        .children
        .push(MappingChild::Text("U+1D10D".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mapping::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("mapping-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        MappingChild::Text(t) => assert_eq!(t, "U+1D10D"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn mapping_roundtrip_with_symbol() {
    use tusk_model::elements::{Mapping, MappingChild, Symbol};

    let mut original = Mapping::default();
    original.common.xml_id = Some("mapping-1".to_string());

    let mut symbol = Symbol::default();
    symbol.common.xml_id = Some("symbol-1".to_string());
    original
        .children
        .push(MappingChild::Symbol(Box::new(symbol)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mapping::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("mapping-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        MappingChild::Symbol(s) => {
            assert_eq!(s.common.xml_id, Some("symbol-1".to_string()));
        }
        _ => panic!("Expected Symbol child"),
    }
}

// ============================================================================
// Complex nested structure tests
// ============================================================================

#[test]
fn symbol_table_roundtrip_complex_nested() {
    use tusk_model::elements::{
        PropName, PropNameChild, PropValue, PropValueChild, SymName, SymNameChild, SymProp,
        SymPropChild, SymbolDef, SymbolDefChild, SymbolTable, SymbolTableChild,
    };

    let mut original = SymbolTable::default();
    original.common.xml_id = Some("symtable-1".to_string());

    // Create symbol definition with name and property
    let mut symbol_def = SymbolDef::default();
    symbol_def.common.xml_id = Some("symdef-coda".to_string());

    // Add symbol name
    let mut sym_name = SymName::default();
    sym_name
        .children
        .push(SymNameChild::Text("CODA".to_string()));
    symbol_def
        .children
        .push(SymbolDefChild::SymName(Box::new(sym_name)));

    // Add symbol property (width)
    let mut sym_prop = SymProp::default();
    let mut prop_name = PropName::default();
    prop_name
        .children
        .push(PropNameChild::Text("width".to_string()));
    sym_prop
        .children
        .push(SymPropChild::PropName(Box::new(prop_name)));
    let mut prop_value = PropValue::default();
    prop_value
        .children
        .push(PropValueChild::Text("200".to_string()));
    sym_prop
        .children
        .push(SymPropChild::PropValue(Box::new(prop_value)));
    symbol_def
        .children
        .push(SymbolDefChild::SymProp(Box::new(sym_prop)));

    original
        .children
        .push(SymbolTableChild::SymbolDef(Box::new(symbol_def)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = SymbolTable::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("symtable-1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        SymbolTableChild::SymbolDef(sd) => {
            assert_eq!(sd.common.xml_id, Some("symdef-coda".to_string()));
            assert_eq!(sd.children.len(), 2);

            // Verify symbol name
            match &sd.children[0] {
                SymbolDefChild::SymName(sn) => {
                    assert_eq!(sn.children.len(), 1);
                    match &sn.children[0] {
                        SymNameChild::Text(t) => assert_eq!(t, "CODA"),
                    }
                }
                _ => panic!("Expected SymName as first child"),
            }

            // Verify symbol property
            match &sd.children[1] {
                SymbolDefChild::SymProp(sp) => {
                    assert_eq!(sp.children.len(), 2);
                }
                _ => panic!("Expected SymProp as second child"),
            }
        }
    }
}
