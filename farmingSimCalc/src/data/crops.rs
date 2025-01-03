


fn load_crops() -> HashMap<String, Crop> {
    let mut crops: HashMap<String, Crop> = HashMap::new();

    // Spring crops
    crops.insert(
        "Oat".to_string(),
        Crop {
            name: Crops::Oats.to_string(),
            time_to_harvest: 3.0,
            yield_per_ha: 3.0,
            multi_harvest: false,
            can_be_planted: vec![Months::March, Months::April],
        },
    );
    crops.insert(
        "Corn".to_string(),
        Crop {
            name: Crops::Corn.to_string(),
            time_to_harvest: 6.0,
            yield_per_ha: 3.0,
            multi_harvest: false,
            can_be_planted: vec![Months::April, Months::May],
        },
    );
    crops.insert(
        "Sunflower".to_string(),
        Crop {
            name: Crops::Sunflower.to_string(),
            time_to_harvest: 7.0,
            yield_per_ha: 3.0,
            multi_harvest: false,
            can_be_planted: vec![Months::March, Months::April],
        },
    );

    // Summer crops


    // Fall crops
    crops.insert(
        "Wheat".to_string(),
        Crop {
            name: Crops::Wheat.to_string(),
            time_to_harvest: 10.0,
            yield_per_ha: 3.0,
            multi_harvest: false,
            can_be_planted: vec![Months::September, Months::October],
        },
    );
    crops.insert(
        "Barley".to_string(),
        Crop {
            name: Crops::Barley.to_string(),
            time_to_harvest: 9.0,
            yield_per_ha: 3.0,
            multi_harvest: false,
            can_be_planted: vec![Months::September, Months::October],
        },
    );
    crops.insert(
        "Canola".to_string(),
        Crop {
            name: Crops::Canola.to_string(),
            time_to_harvest: 11.0,
            yield_per_ha: 3.0,
            multi_harvest: false,
            can_be_planted: vec![Months::August, Months::September],
        },
    );

    // Winter crops
    crops.insert(
        "Cotton".to_string(),
        Crop {
            name: Crops::Cotton.to_string(),
            time_to_harvest: 8.0,
            yield_per_ha: 3.0,
            multi_harvest: false,
            can_be_planted: vec![Months::February, Months::March],
        },
    );


    
    return crops;
}