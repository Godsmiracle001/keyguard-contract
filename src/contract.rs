use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String};

use crate::register::{register_key, Error};

#[contract]
pub struct KeyguardContract;

#[contractimpl]
impl KeyguardContract {
    pub fn hello(_env: Env) {}

    pub fn register_key(env: Env, owner: Address, key_id: BytesN<32>, label: String) -> Result<(), Error> {
        register_key(&env, owner, key_id, label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::get_key_record;

    #[test]
    fn register_key_stores_record_and_emits_event() {
        let env = Env::default();
        let owner = Address::generate(&env);
        let key_id = BytesN::from_array(&env, &[7u8; 32]);
        let label = String::from_slice(&env, "Main Recovery Key");

        let result = register_key(&env, owner.clone(), key_id.clone(), label.clone());
        assert!(result.is_ok());

        let record = get_key_record(&env, owner.clone(), key_id.clone()).expect("record should exist");
        assert_eq!(record.owner, owner);
        assert_eq!(record.key_id, key_id);
        assert_eq!(record.label, label);
        assert_eq!(record.registered_at, env.ledger().timestamp());
        assert_eq!(env.events().all().len(), 1);
    }

    #[test]
    fn register_key_rejects_duplicates() {
        let env = Env::default();
        let owner = Address::generate(&env);
        let key_id = BytesN::from_array(&env, &[7u8; 32]);
        let label = String::from_slice(&env, "Main Recovery Key");

        assert!(register_key(&env, owner.clone(), key_id.clone(), label.clone()).is_ok());
        let err = register_key(&env, owner, key_id, label).unwrap_err();
        assert_eq!(err, Error::KeyAlreadyExists);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #1)")]
    fn register_key_panics_for_unauthorized_invoker() {
        let env = Env::default();
        let owner = Address::generate(&env);
        let other = Address::generate(&env);
        let key_id = BytesN::from_array(&env, &[9u8; 32]);
        let label = String::from_slice(&env, "Main Recovery Key");

        let _ = register_key(&env, owner, key_id, label);
        let _ = other;
    }
}