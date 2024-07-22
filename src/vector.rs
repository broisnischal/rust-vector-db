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

    fn cosine_similarity(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
        let dot_product: f64 = vec1.iter().zip(vec2).map(|(a, b)| a * b).sum();
        let magnitude1: f64 = vec1.iter().map(|v| v * v).sum::<f64>().sqrt();
        let magnitude2: f64 = vec2.iter().map(|v| v * v).sum::<f64>().sqrt();

        if magnitude1 == 0.0 || magnitude2 == 0.0 {
            0.0
        } else {
            dot_product / (magnitude1 * magnitude2)
        }
    }

    // Recommender System
    pub fn recommend(&self, query_vector: &Vec<f64>, top_n: usize) -> Vec<(String, f64)> {
        let mut similarities: Vec<(String, f64)> = self
            .data
            .iter()
            .map(|(key, vector)| {
                let similarity = Self::cosine_similarity(query_vector, vector);
                (key.clone(), similarity)
            })
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.into_iter().take(top_n).collect()
    }
}
