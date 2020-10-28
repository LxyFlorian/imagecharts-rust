use std::collections::HashMap;
use std::cell::RefCell;

pub struct ImageCharts {
    pub query: RefCell<HashMap<String, String>>
}

impl ImageCharts {
    pub fn create() -> Self {
        ImageCharts { query: RefCell::new(HashMap::new()) }
    }

    fn clone(self, key: String, value: String) -> Self{
        self.query.borrow_mut().insert(key.to_string(), value.to_string());
        self
    }
    
    pub fn cht(self, cht: String) -> Self {
        Self::clone(self, "cht".to_string(), cht.to_string())
    }
    
    pub fn chs(self, chs: String) -> Self {
        Self::clone(self, "chs".to_string(), chs.to_string())
    }

    pub fn to_uri(self) -> String {
        let mut final_list = Vec::new();
        //for each key/value in queries, add key=value in final vector
        for (key, value) in self.query.into_inner(){
            final_list.push(key.to_string() + "=" + &value.to_string());
        }
        //create the URI -> TODO : Use a library to create a clean uri
        final_list.join("&")
    }
}
