

fn load_recipes() -> HashMap<String, Vec<Recipe>> {
    let mut recipes: HashMap<String, Vec<Recipe>> = HashMap::new();

    recipes.insert(
        "Flour".to_string(),
        vec![
            Recipe {
                inputs: HashMap::from([(Crops::Wheat.to_string(), 100)]),
                outputs: HashMap::from([(Products::Flour.to_string(), 40)]),
                producer: Buildings::Mill.to_string(),
            },
            Recipe {
                inputs: HashMap::from([(Crops::Barley.to_string(), 100)]),
                outputs: HashMap::from([(Products::Flour.to_string(), 40)]),
                producer: Buildings::Mill.to_string(),
            },
        ],
    );
    recipes.insert(
        "Bread".to_string(),
        vec![
            Recipe {
                inputs: HashMap::from([(Products::Flour.to_string(), 60)]),
                outputs: HashMap::from([(Products::Bread.to_string(), 20)]),
                producer: Buildings::Bakery.to_string(),
            },
        ],
    );

    return recipes;
}


