use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize, Debug)]
pub struct PerkInventory {
    #[serde(rename = "canAddCustomPage")]
    pub can_add_custom_page: bool,
    #[serde(rename = "customPageCount")]
    pub custom_page_count: u32,
    #[serde(rename = "isCustomPageCreationUnlocked")]
    pub is_custom_page_creation_unlocked: bool,
    #[serde(rename = "ownedPageCount")]
    pub owned_page_count: u32,
}
