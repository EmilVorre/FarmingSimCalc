

fn load_products() -> HashMap<String, Product> {
    let mut products: HashMap<String, Product> = HashMap::new();
    
    // Raw products
    products.insert(
        Crops::Wheat.to_string(),
        Product {
            name: Crops::Wheat.to_string(),
            price: 734.0,
            month_to_sell: vec![Months::January],
        },
    );
    products.insert(
        Crops::Barley.to_string(),
        Product {
            name: Crops::Barley.to_string(),
            price: 682.0,
            month_to_sell: vec![Months::January],
        },
    );
    products.insert(
        Crops::Oats.to_string(),
        Product {
            name: Crops::Oats.to_string(),
            price: 1159.0,
            month_to_sell: vec![Months::January],
        },
    );
    products.insert(
        Crops::Canola.to_string(),
        Product {
            name: Crops::Canola.to_string(),
            price: 1314.0,
            month_to_sell: vec![Months::December],
        },
    );
    products.insert(
        Crops::Sorghum.to_string(),
        Product {
            name: Crops::Sorghum.to_string(),
            price: 945.0,
            month_to_sell: vec![Months::January],
        },
    );
    products.insert(
        Crops::Sunflower.to_string(),
        Product {
            name: Crops::Sunflower.to_string(),
            price: 1454.0,
            month_to_sell: vec![Months::March],
        },
    );
    products.insert(
        Crops::Soybeans.to_string(),
        Product {
            name: Crops::Soybeans.to_string(),
            price: 2227.0,
            month_to_sell: vec![Months::July],
        },
    );
    products.insert(
        Crops::Corn.to_string(),
        Product {
            name: Crops::Corn.to_string(),
            price: 909.0,
            month_to_sell: vec![Months::January],
        },
    );
    products.insert(
        Crops::Potatoes.to_string(),
        Product {
            name: Crops::Potatoes.to_string(),
            price: 459.0,
            month_to_sell: vec![Months::December],
        },
    );
    products.insert(
        Crops::SugarBeet.to_string(),
        Product {
            name: Crops::SugarBeet.to_string(),
            price: 356.0,
            month_to_sell: vec![Months::January],
        },
    );
    products.insert(
        Crops::Carrot.to_string(),
        Product {
            name: Crops::Carrot.to_string(),
            price: 274.0,
            month_to_sell: vec![Months::January],
        },
    );
    products.insert(
        Crops::Parsnip.to_string(),
        Product {
            name: Crops::Parsnip.to_string(),
            price: 272.0,
            month_to_sell: vec![Months::January],
        },
    );
    products.insert(
        Crops::Cotton.to_string(),
        Product {
            name: Crops::Cotton.to_string(),
            price: 2502.0,
            month_to_sell: vec![Months::March],
        },
    );
    products.insert(
        Crops::DryRice.to_string(),
        Product {
            name: Crops::DryRice.to_string(),
            price: 1000.0,
            month_to_sell: vec![Months::July, Months::August, Months::September, Months::October],
        },
    );
    products.insert(
        Crops::Rice.to_string(),
        Product {
            name: Crops::Rice.to_string(),
            price: 2079.0,
            month_to_sell: vec![Months::July, Months::August, Months::September, Months::October],
        },
    );
    products.insert(
        Crops::GreenBean.to_string(),
        Product {
            name: Crops::GreenBean.to_string(),
            price: 1361.0,
            month_to_sell: vec![Months::August, Months::September, Months::October, Months::November],
        },
    );
    products.insert(
        Crops::Peas.to_string(),
        Product {
            name: Crops::Peas.to_string(),
            price: 2060.0,
            month_to_sell: vec![Months::December],
        },
    );
    products.insert(
        Crops::Spinach.to_string(),
        Product {
            name: Crops::Spinach.to_string(),
            price: 416.0,
            month_to_sell: vec![Months::August, Months::September, Months::October, Months::November],
        },
    );


    // First step products
    products.insert(
        Products::Flour.to_string(),
        Product {
            name: Products::Flour.to_string(),
            price: 1.0,
            month_to_sell: vec![Months::January, Months::February, Months::March, Months::April, Months::May, Months::June, Months::July, Months::August, Months::September, Months::October, Months::November, Months::December],
        },
    );

    // Second step products
    products.insert(
        Products::Bread.to_string(),
        Product {
            name: Products::Bread.to_string(),
            price: 2.0,
            month_to_sell: vec![Months::January, Months::February, Months::March, Months::April, Months::May, Months::June, Months::July, Months::August, Months::September, Months::October, Months::November, Months::December],
        },
    );
    

    return products;
}