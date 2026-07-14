use soroban_sdk::{contracttype, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyRegistered {
    pub owner: soroban_sdk::Address,
    pub key_id: soroban_sdk::BytesN<32>,
    pub label: soroban_sdk::String,
    pub registered_at: u64,
}

pub fn emit_key_registered(env: &Env, event: KeyRegistered) {
    let topic = Symbol::new(env, "key_registered");
    env.events().publish((topic,), event);
}
