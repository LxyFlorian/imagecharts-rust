pub struct UriProperties {
    pub host: String,
    pub protocol: String,
    pub port: i16,
    pub pathname: String,
    pub timeout: i16,
}

impl UriProperties {
    pub fn get() -> Self {
        Self {
            host: "image-charts.com:".to_string(),
            protocol: "https://".to_string(),
            port: 443,
            pathname: "/chart".to_string(),
            timeout: 5000,
            
        }
    }
}