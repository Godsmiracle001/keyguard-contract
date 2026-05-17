use soroban_sdk::{contracttype, Address, BytesN, String};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataKey {
    Record(Address, BytesN<32>),
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyRecord {
    pub owner: Address,
    pub key_id: BytesN<32>,
    pub label: String,
    pub registered_at: u64,
}
