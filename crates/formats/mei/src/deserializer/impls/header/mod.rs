//! Deserializer implementations for MEI header elements.
//!
//! This module is split into submodules organized by element category:
//! - `mei_head`: Main header container (MeiHead, FileDesc, TitleStmt, SourceDesc)
//! - `pub_stmt`: Publication statement elements (PubStmt, Publisher, Availability, etc.)
//! - `bibl`: Bibliographic elements (Bibl, BiblStruct, BiblScope, Imprint, Locus)
//! - `encoding_desc`: Encoding description elements (EncodingDesc, AppInfo, EditorialDecl, etc.)
//! - `agents`: Agent/contributor elements (Creator, Editor, RespStmt, PersName, CorpName, etc.)
//! - `address`: Address-related elements (Address, AddrLine, GeogName, etc.)
//! - `misc`: Shared mixed-content elements (P, Ptr, Ref, Annot)
//! - `phys_desc`: Physical description elements (Dimensions, Height, Width, etc.)
//! - `layout_hand`: Layout, hand, and script elements (LayoutDesc, Layout, Hand, etc.)

mod address;
mod agents;
mod bibl;
mod deco_binding;
mod encoding_desc;
mod layout_hand;
mod mei_head;
mod misc;
mod misc_header;
mod names;
pub(crate) mod phys_desc;
mod provenance;
mod pub_stmt;
mod recording;
mod relations;

// Re-export all public parse functions
pub(crate) use address::{
    parse_addr_line_from_event, parse_address_from_event, parse_bloc_from_event,
    parse_country_from_event, parse_district_from_event, parse_geog_feat_from_event,
    parse_geog_name_from_event, parse_post_box_from_event, parse_post_code_from_event,
    parse_region_from_event, parse_settlement_from_event, parse_street_from_event,
};

pub(crate) use agents::{
    parse_contributor_from_event, parse_corp_name_from_event, parse_creator_from_event,
    parse_deprecated_creator_from_event, parse_editor_from_event, parse_funder_from_event,
    parse_name_from_event, parse_pers_name_from_event, parse_resp_from_event,
    parse_resp_stmt_from_event, parse_sponsor_from_event,
};

pub(crate) use bibl::{
    parse_bibl_from_event, parse_bibl_scope_from_event, parse_bibl_struct_from_event,
    parse_imprint_from_event, parse_locus_from_event, parse_locus_grp_from_event,
};

pub(crate) use encoding_desc::{
    parse_alt_id_from_event, parse_app_info_from_event, parse_application_from_event,
    parse_att_usage_from_event, parse_cat_rel_from_event, parse_category_from_event,
    parse_class_decls_from_event, parse_correction_from_event, parse_desc_from_event,
    parse_domains_decl_from_event, parse_editorial_decl_from_event, parse_encoding_desc_from_event,
    parse_interpretation_from_event, parse_namespace_from_event, parse_normalization_from_event,
    parse_project_desc_from_event, parse_sampling_decl_from_event, parse_segmentation_from_event,
    parse_std_vals_from_event, parse_tag_usage_from_event, parse_tags_decl_from_event,
    parse_taxonomy_from_event,
};

pub(crate) use mei_head::{
    parse_file_desc_from_event, parse_head_from_event, parse_source_desc_from_event,
    parse_source_from_event, parse_title_from_event, parse_title_part_from_event,
    parse_title_stmt_from_event,
};

pub(crate) use misc::{
    parse_annot_from_event, parse_p_from_event, parse_ptr_from_event, parse_ref_from_event,
};

pub(crate) use names::{
    parse_add_name_from_event, parse_fam_name_from_event, parse_fore_name_from_event,
    parse_gen_name_from_event, parse_name_link_from_event, parse_period_name_from_event,
    parse_style_name_from_event,
};

pub(crate) use pub_stmt::{
    parse_access_restrict_from_event, parse_availability_from_event, parse_date_from_event,
    parse_distributor_from_event, parse_identifier_from_event, parse_price_from_event,
    parse_pub_place_from_event, parse_pub_stmt_from_event, parse_publisher_from_event,
    parse_sys_req_from_event, parse_unpub_from_event, parse_use_restrict_from_event,
};

pub(crate) use phys_desc::{
    parse_collation_from_event, parse_condition_from_event, parse_depth_from_event,
    parse_dim_from_event, parse_dimensions_from_event, parse_foliation_from_event,
    parse_height_from_event, parse_support_desc_from_event, parse_support_from_event,
    parse_width_from_event,
};

pub(crate) use layout_hand::{
    parse_col_layout_from_event, parse_hand_from_event, parse_hand_list_from_event,
    parse_layout_desc_from_event, parse_layout_from_event, parse_script_desc_from_event,
    parse_script_note_from_event,
};

pub(crate) use deco_binding::{
    parse_binding_desc_from_event, parse_binding_from_event, parse_deco_desc_from_event,
    parse_deco_note_from_event, parse_seal_desc_from_event, parse_seal_from_event,
};

pub(crate) use provenance::{
    parse_acc_mat_from_event, parse_acquisition_from_event, parse_add_desc_from_event,
    parse_exhib_hist_from_event, parse_provenance_from_event, parse_type_desc_from_event,
    parse_type_note_from_event, parse_watermark_desc_from_event, parse_watermark_from_event,
    parse_watermark_list_from_event,
};

pub(crate) use recording::{
    parse_capture_mode_from_event, parse_carrier_form_from_event, parse_file_char_from_event,
    parse_other_char_from_event, parse_perf_duration_from_event, parse_performance_from_event,
    parse_playing_speed_from_event, parse_recording_from_event, parse_score_format_from_event,
    parse_sound_chan_from_event, parse_track_config_from_event,
};

pub(crate) use relations::{
    parse_component_list_from_event, parse_item_from_event, parse_item_list_from_event,
    parse_phys_loc_from_event, parse_related_item_from_event, parse_relation_from_event,
    parse_relation_list_from_event, parse_repository_from_event,
};

pub(crate) use misc_header::{
    parse_audience_from_event, parse_genre_from_event, parse_heraldry_from_event,
    parse_inscription_from_event, parse_pg_desc_from_event, parse_recipient_from_event,
    parse_sec_folio_from_event, parse_spec_repro_from_event, parse_text_lang_from_event,
    parse_treat_hist_from_event, parse_treat_sched_from_event,
};

#[cfg(test)]
mod tests;
