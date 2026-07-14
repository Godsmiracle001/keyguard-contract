use soroban_sdk::{contracterror, Address, BytesN, Env, String};

use crate::events::{emit_key_registered, KeyRegistered};
use crate::storage::{get_key_record, set_key_record};
use crate::types::KeyRecord;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    NotAuthorized = 1,
    KeyAlreadyExists = 2,
    InvalidKeyId = 3,
}

pub fn register_key(env: &Env, owner: Address, key_id: BytesN<32>, label: String) -> Result<(), Error> {
    let invoker = env.invoker();
    if invoker != owner {
        env.panic_with_error(Error::NotAuthorized);
    }

    if key_id.as_slice().iter().all(|b| *b == 0) {
        return Err(Error::InvalidKeyId);
    }

    if get_key_record(env, owner.clone(), key_id.clone()).is_some() {
        return Err(Error::KeyAlreadyExists);
    }

    let record = KeyRecord {
        owner: owner.clone(),
        key_id: key_id.clone(),
        label: label.clone(),
        registered_at: env.ledger().timestamp(),
    };

    set_key_record(env, owner.clone(), key_id.clone(), record.clone());

    emit_key_registered(
        env,
        KeyRegistered {
            owner: owner.clone(),
            key_id: key_id.clone(),
            label,
            registered_at: record.registered_at,
        },
    );

    Ok(())
}
