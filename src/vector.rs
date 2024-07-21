use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VectorDB {
    data: HashMap<String, Vec<f64>>,
}

impl VectorDB {
    pub fn new() -> Self {
        VectorDB {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, vector: Vec<f64>) {
        self.data.insert(key, vector);
    }

    pub fn get(&self, key: &str) -> Option<&Vec<f64>> {
        self.data.get(key)
    }

    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn update(&mut self, key: &str, vector: Vec<f64>) -> bool {
        if self.data.contains_key(key) {
            self.data.insert(key.to_string(), vector);
            true
        } else {
            false
        }
    }
}
