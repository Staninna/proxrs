use dotenv::dotenv;

const ENV_PREFIX: &str = "PROXRS_";

pub struct Config {
    pub port: u16,
    pub addr: [u8; 4],
}

pub fn config() -> Config {
    // Load environment variables from .env file
    dotenv().ok().expect("Failed to load .env file");

    // Load and parse the port
    let port = std::env::var(format!("{}PORT", ENV_PREFIX))
        .expect("Failed to load port from environment variables")
        .parse::<u16>()
        .expect("Failed to parse port from environment variables");

    // Load and parse the address
    let addr = std::env::var(format!("{}ADDRESS", ENV_PREFIX))
        .expect("Failed to load address from environment variables")
        .split('.')
        .map(|x| {
            x.parse::<u8>()
                .expect("Failed to parse address from environment variables")
        })
        .collect::<Vec<u8>>();

    // Create and return the config
    Config {
        port,
        addr: [addr[0], addr[1], addr[2], addr[3]],
    }
}
