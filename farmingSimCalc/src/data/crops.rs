


fn load_crops() -> HashMap<String, Crop> {
    let mut crops: HashMap<String, Crop> = HashMap::new();

    crops.insert(
        "Oat".to_string(),
        Crop {
            name: Crops::Oats.to_string(),
            time_to_harvest: 3.0,
            yield_per_ha: 3.0,
            multi_harvest: false,
        },
    );
    
    return crops;
}