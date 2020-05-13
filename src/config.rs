#[derive(Clone)]
pub struct Config {
    pub cert_path: String,
    pub wallet_path: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            cert_path: String::from("./cert.json"),
            wallet_path: String::from("./wallet.json"),
        }
    }
}
