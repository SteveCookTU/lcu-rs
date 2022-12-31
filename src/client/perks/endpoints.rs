use crate::client::{LCUCredentials, Perk, PerkInventory, PerkPage, PerkStyle, CLIENT};
use crate::{
    declare_del_endpoint, declare_get_endpoint, declare_post_endpoint, declare_put_endpoint,
};
use reqwest::{Method, Url};
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

declare_get_endpoint!(get_current_page, "lol-perks/v1/currentpage", PerkPage);

declare_get_endpoint!(get_pages, "lol-perks/v1/pages", Vec<PerkPage>);

declare_get_endpoint!(get_page, "lol-perks/v1/pages/{}", PerkPage, id: u64);

declare_get_endpoint!(
    get_customization_limits,
    "lol-perks/v1/customizationlimits",
    text
);

declare_get_endpoint!(get_inventory, "lol-perks/v1/inventory", PerkInventory);

declare_get_endpoint!(get_perks, "lol-perks/v1/perks", Vec<Perk>);

declare_get_endpoint!(
    get_disabled_perks_ids,
    "lol-perks/v1/perks/disabled",
    Vec<u16>
);

declare_get_endpoint!(
    get_updated_perk_ids,
    "lol-perks/v1/perks/gameplay-updated",
    Vec<u16>
);

declare_get_endpoint!(get_styles, "lol-perks/v1/styles", Vec<PerkStyle>);

declare_del_endpoint!(delete_pages, "lol-perks/v1/pages", HashMap<String, String>);

declare_del_endpoint!(delete_page, "lol-perks/v1/pages/{}", HashMap<String, String>, id: u64);

declare_put_endpoint!(put_current_page, "lol-perks/v1/currentpage", u64, HashMap<String, String>);

declare_put_endpoint!(put_page, "lol-perks/v1/pages/{}", PerkPage, HashMap<String, String>, id: u64);

declare_post_endpoint!(post_page, "lol-perks/v1/pages", PerkPage, PerkPage);
