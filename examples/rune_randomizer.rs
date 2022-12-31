use lcu_rs::client::{get_current_page, get_styles, put_page, LCUCredentials};
use rand::{thread_rng, Rng};

fn main() {
    let credentials =
        LCUCredentials::from_fs().expect("Failed to get credentials. Is the client open?");
    let styles = get_styles(&credentials).expect("Failed to get styles");
    let mut current_page = get_current_page(&credentials).expect("Failed to get current page.");
    current_page.name = "Randomizer".to_string();

    let mut rng = thread_rng();

    let style = &styles[rng.gen_range(0..styles.len())];
    current_page.primary_style_id = style.id;

    let sub_style_id = style.allowed_sub_styles[rng.gen_range(0..style.allowed_sub_styles.len())];
    current_page.sub_style_id = sub_style_id;

    current_page.selected_perk_ids.clear();

    let keystone_slot_perks = &style
        .slots
        .iter()
        .find(|slot| &slot.slot_type == "kKeyStone")
        .unwrap()
        .perks;
    current_page
        .selected_perk_ids
        .push(keystone_slot_perks[rng.gen_range(0..keystone_slot_perks.len())]);

    for slot in style
        .slots
        .iter()
        .filter(|slot| &slot.slot_type == "kMixedRegularSplashable")
    {
        current_page
            .selected_perk_ids
            .push(slot.perks[rng.gen_range(0..slot.perks.len())]);
    }

    let sub_style = styles
        .iter()
        .find(|style| style.id == sub_style_id)
        .unwrap();
    let mut sub_slots = sub_style
        .slots
        .iter()
        .filter(|slot| &slot.slot_type == "kMixedRegularSplashable")
        .cloned()
        .collect::<Vec<_>>();
    for _ in 0..2 {
        let slot_index = rng.gen_range(0..sub_slots.len());
        let sub_slot = sub_slots.remove(slot_index);
        current_page
            .selected_perk_ids
            .push(sub_slot.perks[rng.gen_range(0..sub_slot.perks.len())]);
    }

    for slot in sub_style
        .slots
        .iter()
        .filter(|slot| &slot.slot_type == "kStatMod")
    {
        current_page
            .selected_perk_ids
            .push(slot.perks[rng.gen_range(0..slot.perks.len())]);
    }

    put_page(&credentials, &current_page, current_page.id)
        .expect("Failed to update the specified page");
}
