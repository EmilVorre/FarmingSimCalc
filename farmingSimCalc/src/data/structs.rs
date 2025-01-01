use std::collections::HashMap;

#[derive(Debug)]
struct Recipe {
    inputs: HashMap<String, u32>,
    outputs: HashMap<String, u32>,
    producer: String,
}

#[derive(Debug)]
struct Field {
    name: String,
    size: f32,
    crop: String, // Renamed from 'content' to 'crop'
}