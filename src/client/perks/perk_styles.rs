use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct PerkStyle {
    #[serde(rename = "allowedSubStyles")]
    pub allowed_sub_styles: Vec<u16>,
    #[serde(rename = "assetMap")]
    pub asset_map: HashMap<String, String>,
    #[serde(rename = "defaultPageName")]
    pub default_page_name: String,
    #[serde(rename = "defaultPerks")]
    pub default_perks: Vec<u16>,
    #[serde(rename = "defaultSubStyle")]
    pub default_sub_style: u16,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    pub id: u16,
    #[serde(rename = "idName")]
    pub id_name: String,
    pub name: String,
    pub slots: Vec<PerkStyleSlot>,
    #[serde(rename = "subStyleBonus")]
    pub sub_style_bonus: Vec<PerkSubStyleBonus>,
    pub tooltip: String,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct PerkStyleSlot {
    pub perks: Vec<u16>,
    #[serde(rename = "slotLabel")]
    pub slot_label: String,
    #[serde(rename = "type")]
    pub slot_type: String,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct PerkSubStyleBonus {
    #[serde(rename = "perkId")]
    pub perk_id: u16,
    #[serde(rename = "styleId")]
    pub style_id: u16,
}
