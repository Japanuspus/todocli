mod config;
use config::CONFIG; //from config.rs generated by build.rs

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Tokens {
    access_token: String,
    refresh_token: String,
}

impl ::std::default::Default for Tokens {
    fn default() -> Self { Self { access_token: "invalid".into(), refresh_token: "invalid".into() } }
}

fn main() {
    let mut tokens: Tokens = confy::load(&CONFIG.app_name).expect("Failed to load tokens");
    println!("Hello, world from {} with token: {}", CONFIG.tenant_id, tokens.access_token);

    tokens.access_token = format!("{}{}", tokens.access_token, "x");
    confy::store(&CONFIG.app_name, &tokens).expect("Failed to store tokens");
}
