use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct PerkPage {
    #[serde(rename = "autoModifiedSelections")]
    pub auto_modified_selections: Vec<u32>,
    pub current: bool,
    pub id: u64,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeletable")]
    pub is_deletable: bool,
    #[serde(rename = "isEditable")]
    pub is_editable: bool,
    #[serde(rename = "isTemporary")]
    pub is_temporary: bool,
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    #[serde(rename = "lastModified")]
    pub last_modified: u64,
    pub name: String,
    pub order: u8,
    #[serde(rename = "primaryStyleId")]
    pub primary_style_id: u16,
    #[serde(rename = "selectedPerkIds")]
    pub selected_perk_ids: Vec<u16>,
    #[serde(rename = "subStyleId")]
    pub sub_style_id: u16,
}
