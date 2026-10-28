//! Offline names: the only identity Scoria has.
use super::*;

#[test]
fn a_name_is_three_to_sixteen_letters_digits_or_underscore() {
    assert!(valid_offline_name("Builder_1"));
    assert!(valid_offline_name("abc"));
    assert!(!valid_offline_name("ab"));
    assert!(!valid_offline_name("a".repeat(17).as_str()));
    assert!(!valid_offline_name("two words"));
    assert!(valid_offline_name("éclair"));
    assert!(valid_offline_name("Игрок"));
    assert!(!valid_offline_name("Иг рок"));
}

#[test]
fn a_russian_name_is_kept_once_in_any_case() {
    let mut accounts = Accounts::default();
    accounts.play_as(Player::Offline("Игрок".into())).unwrap();
    accounts.play_as(Player::Offline("игрок".into())).unwrap();
    assert_eq!(accounts.offline, ["Игрок"]);
    assert_eq!(accounts.player(), Player::Offline("Игрок".into()));
}

#[test]
fn names_are_kept_once_in_any_case_and_removed_cleanly() {
    let mut accounts = Accounts::default();
    accounts
        .play_as(Player::Offline("Builder_1".into()))
        .unwrap();
    accounts
        .play_as(Player::Offline("builder_1".into()))
        .unwrap();
    assert_eq!(accounts.offline, ["Builder_1"]);
    assert_eq!(accounts.player(), Player::Offline("Builder_1".into()));
    accounts.play_as(Player::GameName).unwrap();
    assert_eq!(accounts.player(), Player::GameName);
    accounts.remove_offline("BUILDER_1").unwrap();
    assert!(accounts.offline.is_empty());
    assert!(accounts.remove_offline("Builder_1").is_err());
}

#[test]
fn accounts_json_from_the_microsoft_era_still_loads_offline_names() {
    let old =
        r#"{"accounts":[],"selected":null,"offline":["Builder_1"],"offline_selected":"Builder_1"}"#;
    let accounts: Accounts = serde_json::from_str(old).unwrap();
    assert_eq!(accounts.player(), Player::Offline("Builder_1".into()));
    assert_eq!(accounts.name_arg(), Some("Builder_1"));
}

#[test]
fn the_keychain_store_round_trips_in_memory() {
    let store = MemoryStore::default();
    assert_eq!(store.get("k").unwrap(), None);
    store.set("k", "v").unwrap();
    assert_eq!(store.get("k").unwrap().as_deref(), Some("v"));
    store.delete("k").unwrap();
    assert_eq!(store.get("k").unwrap(), None);
    assert!(!store.persistent());
}
