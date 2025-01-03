

fn load_products() -> HashMap<String, Product> {
    let mut products: HashMap<String, Product> = HashMap::new();
    // Raw products
    products.insert(
        "Oat".to_string(),
        Product {
            name: Crops::Oats.to_string(),
            price: 1165.0,
            month_to_sell: vec![Months::January],
        },
    );
    products.insert(
        "Wheat".to_string(),
        Product {
            name: Crops::Wheat.to_string(),
            price: 731.0,
            month_to_sell: vec![Months::December],
        },
    );
    products.insert(
        "Canola".to_string(),
        Product {
            name: Crops::Canola.to_string(),
            price: 272.0,
            month_to_sell: vec![Months::January],
        },
    );


    // First step products
    products.insert(
        "Flour".to_string(),
        Product {
            name: Products::Flour.to_string(),
            price: 1.0,
            month_to_sell: vec![Months::January, Months::February, Months::March, Months::April, Months::May, Months::June, Months::July, Months::August, Months::September, Months::October, Months::November, Months::December],
        },
    );

    // Second step products
    products.insert(
        "Bread".to_string(),
        Product {
            name: Products::Bread.to_string(),
            price: 2.0,
            month_to_sell: vec![Months::January, Months::February, Months::March, Months::April, Months::May, Months::June, Months::July, Months::August, Months::September, Months::October, Months::November, Months::December],
        },
    );
    

    return products;
}