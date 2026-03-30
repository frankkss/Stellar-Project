use soroban_sdk::{contract, contractimpl, Env, Symbol, Bytes};

#[contract]
pub struct FileShareContract;

#[contractimpl]
impl FileShareContract {
    pub fn upload_file(env: Env, file_id: Symbol, owner: Symbol) {
        env.storage().set(&file_id, &owner);
    }

    pub fn grant_access(env: Env, file_id: Symbol, user: Symbol) {
        let key = Symbol::from_str(&format!("{}_access_{}", file_id.to_string(), user.to_string()));
        env.storage().set(&key, &true);
    }

    pub fn check_access(env: Env, file_id: Symbol, user: Symbol) -> bool {
        let key = Symbol::from_str(&format!("{}_access_{}", file_id.to_string(), user.to_string()));
        env.storage().get::<bool>(&key).unwrap_or(false)
    }

    pub fn track_access(env: Env, file_id: Symbol, user: Symbol) {
        let log_key = Symbol::from_str(&format!("{}_last_access", file_id.to_string()));
        let log_value = Bytes::from_slice(&env, user.to_string().as_bytes());
        env.storage().set(&log_key, &log_value);
    }

    pub fn last_access(env: Env, file_id: Symbol) -> Option<Bytes> {
        let log_key = Symbol::from_str(&format!("{}_last_access", file_id.to_string()));
        env.storage().get::<Bytes>(&log_key)
    }
}
