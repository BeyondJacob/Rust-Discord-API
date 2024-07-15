use std::env;

/// Loads the environment variables from a `.env` file.
pub fn load_dotenv() {
    dotenv::dotenv().ok();
}

/// Retrieves the value of an environment variable.
///
/// # Arguments
///
/// * `key` - The key of the environment variable.
///
/// # Returns
///
/// The value of the environment variable.
pub fn get_env_var(key: &str) -> String {
    env::var(key).expect(&format!("Expected environment variable {}", key))
}
