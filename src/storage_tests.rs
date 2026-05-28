use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, String};
use crate::storage::{get_key_record, has_key_record, remove_key_record, set_key_record};
use crate::types::KeyRecord;

#[test]
fn test_storage_crud() {
    let env = Env::default();
    let owner = Address::generate(&env);
    let key_id = BytesN::from_array(&env, &[7u8; 32]);
    let label = String::from_slice(&env, "Main Recovery Key");
    let registered_at = 1672531199;

    let record = KeyRecord {
        owner: owner.clone(),
        key_id: key_id.clone(),
        label: label.clone(),
        registered_at,
    };

    // Verify initial state
    assert!(!has_key_record(&env, owner.clone(), key_id.clone()));
    assert_eq!(get_key_record(&env, owner.clone(), key_id.clone()), None);

    // Set record
    set_key_record(&env, owner.clone(), key_id.clone(), record.clone());

    // Verify record exists and matches exactly
    assert!(has_key_record(&env, owner.clone(), key_id.clone()));
    let fetched = get_key_record(&env, owner.clone(), key_id.clone()).expect("Record should exist");
    assert_eq!(fetched, record);
    assert_eq!(fetched.owner, owner);
    assert_eq!(fetched.key_id, key_id);
    assert_eq!(fetched.label, label);
    assert_eq!(fetched.registered_at, registered_at);

    // Remove record
    remove_key_record(&env, owner.clone(), key_id.clone());

    // Verify removed state
    assert!(!has_key_record(&env, owner.clone(), key_id.clone()));
    assert_eq!(get_key_record(&env, owner, key_id), None);
}
