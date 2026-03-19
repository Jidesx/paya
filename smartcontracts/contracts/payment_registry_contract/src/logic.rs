use soroban_sdk::Env;

use crate::storage;

pub fn ensure_initialized(env: &Env) {
    if storage::get_version(env) == 0 {
        storage::set_version(env, 1);
    }
}
