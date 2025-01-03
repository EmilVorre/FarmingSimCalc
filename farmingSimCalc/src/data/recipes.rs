

fn load_recipes() -> HashMap<String, Recipe> {
    let mut recipes: HashMap<String, Recipe> = HashMap::new();

    recipes.insert(
            "Flour".to_string(),
            Recipe {
                inputs: HashMap::from([(Crops::Wheat.to_string(), 100)]),
                outputs: HashMap::from([(Products::Flour.to_string(), 40)]),
                producer: "Mill".to_string(),
            },
    );
    recipes.insert(
          "Bread".to_string(),
          Recipe {
                inputs: HashMap::from([(Products::Flour.to_string(), 60)]),
                outputs: HashMap::from([(Products::Bread.to_string(), 20)]),
                producer: "Bakery".to_string(),
            },
    );

    return recipes;
}


