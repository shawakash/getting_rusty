use std::env;

pub fn server_port() -> u16 {
    match env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap_or(8080),
        Err(_) => 8080,
    }
}

pub fn binance_api_key() -> String {
    match env::var("BINANCE_API_KEY") {
        Ok(api_key) => api_key,
        Err(_) => String::from(""),
    }
}

pub fn binance_api_sercet() -> String {
    match env::var("BINANCE_API_SECRET") {
        Ok(api_secret) => api_secret,
        Err(_) => String::from(""),
    }
}
