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
    crop: String,
    time_since_planted: f32,
}

#[derive(Debug)]
struct Crop {
    name: String,
    time_to_harvest: f32,
    yield_per_ha: f32,
    multi_harvest: bool,
    can_be_planted: Vec<Months>,
}