


fn load_crops() -> HashMap<String, Crop> {
    let mut crops: HashMap<String, Crop> = HashMap::new();

    // Spring crops
    crops.insert(
        Crops::Oats.to_string(),
        Crop {
            name: Crops::Oats.to_string(),
            time_to_harvest: 3.0,
            yield_per_ha: 11370.49,
            multi_harvest: false,
            can_be_planted: vec![Months::March, Months::April],
        },
    );
    crops.insert(
        Crops::Corn.to_string(),
        Crop {
            name: Crops::Corn.to_string(),
            time_to_harvest: 6.0,
            yield_per_ha: 18349.18,
            multi_harvest: false,
            can_be_planted: vec![Months::April, Months::May],
        },
    );
    crops.insert(
        Crops::Sunflower.to_string(),
        Crop {
            name: Crops::Sunflower.to_string(),
            time_to_harvest: 7.0,
            yield_per_ha: 10611.48,
            multi_harvest: false,
            can_be_planted: vec![Months::March, Months::April],
        },
    );
    crops.insert(
        Crops::Soybeans.to_string(),
        Crop {
            name: Crops::Soybeans.to_string(),
            time_to_harvest: 6.0,
            yield_per_ha: 8977.05,
            multi_harvest: false,
            can_be_planted: vec![Months::April, Months::May],
        },
    );
    crops.insert(
        Crops::Potatoes.to_string(),
        Crop {
            name: Crops::Potatoes.to_string(),
            time_to_harvest: 5.0,
            yield_per_ha: 85409.84,
            multi_harvest: false,
            can_be_planted: vec![Months::March, Months::April],
        },
    );
    crops.insert(
        Crops::Rice.to_string(),
        Crop {
            name: Crops::Rice.to_string(),
            time_to_harvest: 4.0,
            yield_per_ha: 12701.64,
            multi_harvest: false,
            can_be_planted: vec![Months::April, Months::May],
        },
    );
    crops.insert(
        Crops::DryRice.to_string(),
        Crop {
            name: Crops::DryRice.to_string(),
            time_to_harvest: 5.0,
            yield_per_ha: 16601.64,
            multi_harvest: false,
            can_be_planted: vec![Months::April],
        },
    );
    crops.insert(
        Crops::SugarBeet.to_string(),
        Crop {
            name: Crops::SugarBeet.to_string(),
            time_to_harvest: 7.0,
            yield_per_ha: 115286.89,
            multi_harvest: false,
            can_be_planted: vec![Months::March, Months::April],
        },
    );
    crops.insert(
        Crops::Sorghum.to_string(),
        Crop {
            name: Crops::Sorghum.to_string(),
            time_to_harvest: 4.0,
            yield_per_ha: 16357.38,
            multi_harvest: false,
            can_be_planted: vec![Months::April, Months::May],
        },
    );
    crops.insert(
        Crops::RedBeet.to_string(),
        Crop {
            name: Crops::RedBeet.to_string(),
            time_to_harvest: 4.0,
            yield_per_ha: 95908.20,
            multi_harvest: false,
            can_be_planted: vec![Months::April, Months::May, Months::June],
        },
    );
    crops.insert(
        Crops::Carrot.to_string(),
        Crop {
            name: Crops::Carrot.to_string(),
            time_to_harvest: 4.0,
            yield_per_ha: 107254.10,
            multi_harvest: false,
            can_be_planted: vec![Months::April, Months::May, Months::June],
        },
    );
    crops.insert(
        Crops::Parsnip.to_string(),
        Crop {
            name: Crops::Parsnip.to_string(),
            time_to_harvest: 4.0,
            yield_per_ha: 99680.33,
            multi_harvest: false,
            can_be_planted: vec![Months::April, Months::May, Months::June],
        },
    );
    crops.insert(
        Crops::GreenBean.to_string(),
        Crop {
            name: Crops::GreenBean.to_string(),
            time_to_harvest: 4.0,
            yield_per_ha: 13913.11,
            multi_harvest: false,
            can_be_planted: vec![Months::April, Months::May, Months::June],
        },
    );
    crops.insert(
        Crops::Peas.to_string(),
        Crop {
            name: Crops::Peas.to_string(),
            time_to_harvest: 4.0,
            yield_per_ha: 9573.77,
            multi_harvest: false,
            can_be_planted: vec![Months::March, Months::April],
        },
    );
    crops.insert(
        Crops::Spinach.to_string(),
        Crop {
            name: Crops::Spinach.to_string(),
            time_to_harvest: 4.0,
            yield_per_ha: 91573.77,
            multi_harvest: true,
            can_be_planted: vec![Months::March, Months::April, Months::May],
        },
    );

    // Summer crops


    // Fall crops
    crops.insert(
        Crops::Wheat.to_string(),
        Crop {
            name: Crops::Wheat.to_string(),
            time_to_harvest: 10.0,
            yield_per_ha: 17754.10,
            multi_harvest: false,
            can_be_planted: vec![Months::September, Months::October],
        },
    );
    crops.insert(
        Crops::Barley.to_string(),
        Crop {
            name: Crops::Barley.to_string(),
            time_to_harvest: 9.0,
            yield_per_ha: 19149.18,
            multi_harvest: false,
            can_be_planted: vec![Months::September, Months::October],
        },
    );
    crops.insert(
        Crops::Canola.to_string(),
        Crop {
            name: Crops::Canola.to_string(),
            time_to_harvest: 11.0,
            yield_per_ha: 11778.69,
            multi_harvest: false,
            can_be_planted: vec![Months::August, Months::September],
        },
    );

    // Winter crops
    crops.insert(
        Crops::Cotton.to_string(),
        Crop {
            name: Crops::Cotton.to_string(),
            time_to_harvest: 8.0,
            yield_per_ha: 9913.11,
            multi_harvest: false,
            can_be_planted: vec![Months::February, Months::March],
        },
    );


    
    return crops;
}