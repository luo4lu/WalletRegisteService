use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls};

#[derive(Clone)]
pub struct ConfigPath {
    pub cert_path: String,
}

impl Default for ConfigPath {
    fn default() -> ConfigPath {
        ConfigPath {
            cert_path: String::from("./cert.json"),
        }
    }
}

pub fn get_db() -> Pool {
    //配置连接数据库
    let mut cfg = Config::new();
    cfg.host("localhost"); //数据库地址
    cfg.user("postgres"); //数据库用户名称
    cfg.password("postgres"); //数据库密码
    cfg.dbname("walletregistesystem"); //数据库名称
    let mgr = Manager::new(cfg, NoTls);
    Pool::new(mgr, 8)
}
