use std::collections::HashMap;

pub struct ImageCharts {
    pub host: String,
    pub protocol: String,
    pub port: i16,
    pub pathname: String,
    pub timeout: i16,
    pub query: HashMap<String, String>
}

impl ImageCharts {
    pub fn get() -> Self {
        Self {
            host: "image-charts.com:".to_string(),
            protocol: "https://".to_string(),
            port: 443,
            pathname: "/chart".to_string(),
            timeout: 5000,
            query: HashMap::new()
        }
    }
}
