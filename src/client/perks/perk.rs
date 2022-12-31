use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Perk {
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    pub id: u16,
    #[serde(rename = "longDesc")]
    pub long_desc: String,
    pub name: String,
    #[serde(rename = "recommendationDescriptor")]
    pub recommendation_descriptor: String,
    #[serde(rename = "shortDesc")]
    pub short_desc: String,
    #[serde(rename = "slotType")]
    pub slot_type: String,
    #[serde(rename = "styleId")]
    pub style_id: i32,
    #[serde(rename = "styleIdName")]
    pub style_id_name: String,
    pub tooltip: String,
}
