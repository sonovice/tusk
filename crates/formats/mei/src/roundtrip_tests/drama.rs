//! Round-trip serialization tests for drama-related MEI elements.
//!
//! Tests for Sp, Speaker, StageDir, Role, RoleName elements.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// Sp Tests
// ============================================================================

#[test]
fn sp_roundtrip_empty() {
    use tusk_model::elements::Sp;

    let original = Sp::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Sp::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn sp_roundtrip_with_xml_id() {
    use tusk_model::elements::Sp;

    let mut original = Sp::default();
    original.common.xml_id = Some("sp-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Sp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sp-1".to_string()));
}

#[test]
fn sp_roundtrip_with_speaker() {
    use tusk_model::elements::{Sp, SpChild, Speaker, SpeakerChild};

    let mut original = Sp::default();
    original.common.xml_id = Some("sp-1".to_string());

    let mut speaker = Speaker::default();
    speaker.common.xml_id = Some("speaker-1".to_string());
    speaker
        .children
        .push(SpeakerChild::Text("Hamlet".to_string()));
    original.children.push(SpChild::Speaker(Box::new(speaker)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Sp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SpChild::Speaker(s) => {
            assert_eq!(s.common.xml_id, Some("speaker-1".to_string()));
            assert_eq!(s.children.len(), 1);
            match &s.children[0] {
                SpeakerChild::Text(t) => assert_eq!(t, "Hamlet"),
                _ => panic!("Expected Text child"),
            }
        }
        _ => panic!("Expected Speaker child"),
    }
}

#[test]
fn sp_roundtrip_with_stage_dir() {
    use tusk_model::elements::{Sp, SpChild, StageDir, StageDirChild};

    let mut original = Sp::default();
    original.common.xml_id = Some("sp-1".to_string());

    let mut stage_dir = StageDir::default();
    stage_dir.common.xml_id = Some("stageDir-1".to_string());
    stage_dir
        .children
        .push(StageDirChild::Text("Exit stage left".to_string()));
    original
        .children
        .push(SpChild::StageDir(Box::new(stage_dir)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Sp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SpChild::StageDir(sd) => {
            assert_eq!(sd.common.xml_id, Some("stageDir-1".to_string()));
            assert_eq!(sd.children.len(), 1);
            match &sd.children[0] {
                StageDirChild::Text(t) => assert_eq!(t, "Exit stage left"),
                _ => panic!("Expected Text child"),
            }
        }
        _ => panic!("Expected StageDir child"),
    }
}

// ============================================================================
// Speaker Tests
// ============================================================================

#[test]
fn speaker_roundtrip_empty() {
    use tusk_model::elements::Speaker;

    let original = Speaker::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Speaker::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn speaker_roundtrip_with_text() {
    use tusk_model::elements::{Speaker, SpeakerChild};

    let mut original = Speaker::default();
    original.common.xml_id = Some("speaker-1".to_string());
    original
        .children
        .push(SpeakerChild::Text("Macbeth".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Speaker::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("speaker-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SpeakerChild::Text(t) => assert_eq!(t, "Macbeth"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn speaker_roundtrip_with_rend() {
    use tusk_model::elements::{Rend, Speaker, SpeakerChild};

    let mut original = Speaker::default();
    original.common.xml_id = Some("speaker-1".to_string());

    let mut rend = Rend::default();
    rend.common.xml_id = Some("rend-1".to_string());
    original.children.push(SpeakerChild::Rend(Box::new(rend)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Speaker::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SpeakerChild::Rend(r) => {
            assert_eq!(r.common.xml_id, Some("rend-1".to_string()));
        }
        _ => panic!("Expected Rend child"),
    }
}

// ============================================================================
// StageDir Tests
// ============================================================================

#[test]
fn stage_dir_roundtrip_empty() {
    use tusk_model::elements::StageDir;

    let original = StageDir::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = StageDir::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn stage_dir_roundtrip_with_text() {
    use tusk_model::elements::{StageDir, StageDirChild};

    let mut original = StageDir::default();
    original.common.xml_id = Some("stageDir-1".to_string());
    original
        .children
        .push(StageDirChild::Text("Enter Ghost".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = StageDir::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("stageDir-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StageDirChild::Text(t) => assert_eq!(t, "Enter Ghost"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn stage_dir_roundtrip_with_attributes() {
    use tusk_model::data::{DataStaffrel, DataStaffrelBasic};
    use tusk_model::elements::StageDir;

    let mut original = StageDir::default();
    original.common.xml_id = Some("stageDir-1".to_string());
    original.stage_dir_vis.place = Some(DataStaffrel::DataStaffrelBasic(DataStaffrelBasic::Above));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = StageDir::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("stageDir-1".to_string()));
    assert_eq!(
        parsed.stage_dir_vis.place,
        Some(DataStaffrel::DataStaffrelBasic(DataStaffrelBasic::Above))
    );
}

// ============================================================================
// Role Tests
// ============================================================================

#[test]
fn role_roundtrip_empty() {
    use tusk_model::elements::Role;

    let original = Role::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Role::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn role_roundtrip_with_text() {
    use tusk_model::elements::{Role, RoleChild};

    let mut original = Role::default();
    original.common.xml_id = Some("role-1".to_string());
    original
        .children
        .push(RoleChild::Text("Ophelia".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Role::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("role-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        RoleChild::Text(t) => assert_eq!(t, "Ophelia"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn role_roundtrip_with_rend() {
    use tusk_model::elements::{Rend, Role, RoleChild};

    let mut original = Role::default();
    original.common.xml_id = Some("role-1".to_string());

    let mut rend = Rend::default();
    rend.common.xml_id = Some("rend-1".to_string());
    original.children.push(RoleChild::Rend(Box::new(rend)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Role::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        RoleChild::Rend(r) => {
            assert_eq!(r.common.xml_id, Some("rend-1".to_string()));
        }
        _ => panic!("Expected Rend child"),
    }
}

// ============================================================================
// RoleName Tests
// ============================================================================

#[test]
fn role_name_roundtrip_empty() {
    use tusk_model::elements::RoleName;

    let original = RoleName::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = RoleName::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn role_name_roundtrip_with_text() {
    use tusk_model::elements::{RoleName, RoleNameChild};

    let mut original = RoleName::default();
    original.common.xml_id = Some("roleName-1".to_string());
    original
        .children
        .push(RoleNameChild::Text("Duke of Cornwall".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = RoleName::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("roleName-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        RoleNameChild::Text(t) => assert_eq!(t, "Duke of Cornwall"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn role_name_roundtrip_with_name_attributes() {
    use tusk_model::elements::RoleName;

    let mut original = RoleName::default();
    original.common.xml_id = Some("roleName-1".to_string());
    original.name.auth = Some("local".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = RoleName::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("roleName-1".to_string()));
    assert_eq!(parsed.name.auth, Some("local".to_string()));
}

// ============================================================================
// Complex nested tests
// ============================================================================

#[test]
fn sp_roundtrip_complex_dialogue() {
    use tusk_model::elements::{
        L, LChild, Sp, SpChild, Speaker, SpeakerChild, StageDir, StageDirChild,
    };

    let mut original = Sp::default();
    original.common.xml_id = Some("sp-1".to_string());

    // Add speaker
    let mut speaker = Speaker::default();
    speaker.common.xml_id = Some("speaker-1".to_string());
    speaker
        .children
        .push(SpeakerChild::Text("Hamlet".to_string()));
    original.children.push(SpChild::Speaker(Box::new(speaker)));

    // Add stage direction
    let mut stage_dir = StageDir::default();
    stage_dir.common.xml_id = Some("stageDir-1".to_string());
    stage_dir
        .children
        .push(StageDirChild::Text("Aside".to_string()));
    original
        .children
        .push(SpChild::StageDir(Box::new(stage_dir)));

    // Add line
    let mut l = L::default();
    l.common.xml_id = Some("l-1".to_string());
    l.children
        .push(LChild::Text("To be, or not to be".to_string()));
    original.children.push(SpChild::L(Box::new(l)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Sp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sp-1".to_string()));
    assert_eq!(parsed.children.len(), 3);

    // Verify speaker
    match &parsed.children[0] {
        SpChild::Speaker(s) => {
            assert_eq!(s.common.xml_id, Some("speaker-1".to_string()));
        }
        _ => panic!("Expected Speaker as first child"),
    }

    // Verify stage direction
    match &parsed.children[1] {
        SpChild::StageDir(sd) => {
            assert_eq!(sd.common.xml_id, Some("stageDir-1".to_string()));
        }
        _ => panic!("Expected StageDir as second child"),
    }

    // Verify line
    match &parsed.children[2] {
        SpChild::L(l) => {
            assert_eq!(l.common.xml_id, Some("l-1".to_string()));
        }
        _ => panic!("Expected L as third child"),
    }
}
