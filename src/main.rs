mod config;
use config::CONFIG;

fn main() {
    println!("Hello, world from {}", CONFIG.tenant_id);
}
