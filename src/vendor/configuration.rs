use std::sync::Mutex;
use std::collections::HashMap;

pub struct Configuration {
    storage: Mutex<HashMap<String,String>>,
}
impl Configuration {
    #[allow(dead_code)]
    pub fn set(&self, key : String, value: String) {
        self.storage.lock().unwrap().insert(key,value);
    }

    pub fn get(&self, key : &str) -> Option<String> {
        self.storage.lock().unwrap().get(key).cloned()
    }

    pub fn new() -> Self {
        let mut map = HashMap::new();
        dotenv::dotenv().ok();

        dotenv::vars().for_each(|(key, value)| {
            map.insert(key,value);
        });

        Configuration{ storage: Mutex::new(map)}
    }
}

impl Clone for Configuration {
    fn clone(&self) -> Self {
        Configuration{ storage: Mutex::new(self.storage.lock().unwrap().clone()) }
    }
}