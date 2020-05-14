#[derive(Clone)]
pub struct Config {
    pub cert_path: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            cert_path: String::from("./cert.json"),
        }
    }
}
