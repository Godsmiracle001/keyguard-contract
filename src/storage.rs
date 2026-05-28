use soroban_sdk::{Address, BytesN, Env};
use crate::types::{DataKey, KeyRecord};

pub const DAY_IN_LEDGERS: u32 = 17_280;
pub const THIRTY_DAYS_IN_LEDGERS: u32 = 518_400;

pub fn set_key_record(env: &Env, owner: Address, key_id: BytesN<32>, record: KeyRecord) {
    let key = DataKey::Record(owner, key_id);
    env.storage().instance().set(&key, &record);
    env.storage().instance().extend_ttl(DAY_IN_LEDGERS, THIRTY_DAYS_IN_LEDGERS);
}

pub fn get_key_record(env: &Env, owner: Address, key_id: BytesN<32>) -> Option<KeyRecord> {
    let key = DataKey::Record(owner, key_id);
    if let Some(record) = env.storage().instance().get::<_, KeyRecord>(&key) {
        env.storage().instance().extend_ttl(DAY_IN_LEDGERS, THIRTY_DAYS_IN_LEDGERS);
        Some(record)
    } else {
        None
    }
}

pub fn has_key_record(env: &Env, owner: Address, key_id: BytesN<32>) -> bool {
    let key = DataKey::Record(owner, key_id);
    env.storage().instance().has(&key)
}

pub fn remove_key_record(env: &Env, owner: Address, key_id: BytesN<32>) {
    let key = DataKey::Record(owner, key_id);
    env.storage().instance().remove(&key);
}
