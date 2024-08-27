// algo.rs
pub struct AlgoConfig {
    pub algorithm: &'static str,
    pub pool_url: &'static str,
    pub user: &'static str,
    pub log_file: &'static str,
}

pub fn get_ergo_config() -> AlgoConfig {
    AlgoConfig {
        algorithm: "AUTOLYKOS2",
        pool_url: "stratum+tcp://de.ergo.herominers.com:1180",
        user: "9hwFm6uwUHT4vJUDg7KX8ucBtnPn817cEJrQ5qby292B9uQGWnN.MyWorker",
        log_file: "logs/miner.log",
    }
}

// Add more configurations here for different algorithms