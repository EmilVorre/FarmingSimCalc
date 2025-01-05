use std::hash::Hash;



fn load_recipes() -> HashMap<String, Vec<Recipe>> {
    let mut recipes: HashMap<String, Vec<Recipe>> = HashMap::new();

    recipes.insert(
        Products::Flour.to_string(),
        vec![
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Barley.to_string(), 30)])
                ],
                outputs: HashMap::from([(Products::Flour.to_string(), 22)]),
                cycles_per_month: 480,
                producer: Buildings::Mill.to_string(),
            },
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Oats.to_string(), 15)])
                ],
                outputs: HashMap::from([(Products::Flour.to_string(), 15)]),
                cycles_per_month: 1200,
                producer: Buildings::Mill.to_string(),
            },
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Sorghum.to_string(), 15)])
                ],
                outputs: HashMap::from([(Products::Flour.to_string(), 13)]),
                cycles_per_month: 1200,
                producer: Buildings::Mill.to_string(),
            },
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Wheat.to_string(), 5)])
                ],
                outputs: HashMap::from([(Products::Flour.to_string(), 4)]),
                cycles_per_month: 2520,
                producer: Buildings::Mill.to_string(),
            },
        ],
    );
    recipes.insert(
        Products::FlourRice.to_string(),
        vec![
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Rice.to_string(), 9)])
                ],
                outputs: HashMap::from([(Products::FlourRice.to_string(), 15)]),
                cycles_per_month: 720,
                producer: Buildings::Mill.to_string(),
            },
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::DryRice.to_string(), 15)])
                ],
                outputs: HashMap::from([(Products::FlourRice.to_string(), 13)]),
                cycles_per_month: 720,
                producer: Buildings::Mill.to_string(),
            },
        ],
    );
    recipes.insert(
        Products::Bread.to_string(),
        vec![
            Recipe {
                inputs: vec![
                    HashMap::from([(Products::Flour.to_string(), 90)])
                ],
                outputs: HashMap::from([(Products::Bread.to_string(), 45)]),
                cycles_per_month: 120,
                producer: Buildings::Bakery.to_string(),
            },
            Recipe {
                inputs: vec![HashMap::from([(Products::FlourRice.to_string(), 68)])],
                outputs: HashMap::from([(Products::Bread.to_string(), 45)]),
                cycles_per_month: 48,
                producer: Buildings::Bakery.to_string(),
            },
        ],
    );
    recipes.insert( 
        Products::Cake.to_string(),
        vec![
            Recipe {
                inputs: vec![
                    HashMap::from([(Products::Flour.to_string(), 8)]),
                    HashMap::from([(Products::Sugar.to_string(), 8)]),
                    HashMap::from([(Products::MilkBottled.to_string(), 8)]),
                    HashMap::from([(Products::Eggs.to_string(), 8)]),
                    HashMap::from([(Products::Butter.to_string(), 8)]),
                    HashMap::from([(Products::Strawberries.to_string(), 8)]),
                ],
                outputs: HashMap::from([(Products::Cake.to_string(), 21)]),
                cycles_per_month: 48,
                producer: Buildings::Bakery.to_string(),
            },
            Recipe {
                inputs: vec![
                    HashMap::from([(Products::FlourRice.to_string(), 6)]),
                    HashMap::from([(Products::Sugar.to_string(), 8)]),
                    HashMap::from([(Products::MilkBottled.to_string(), 8)]),
                    HashMap::from([(Products::Eggs.to_string(), 8)]),
                    HashMap::from([(Products::Butter.to_string(), 8)]),
                    HashMap::from([(Products::Strawberries.to_string(), 8)]),
                ],
                outputs: HashMap::from([(Products::Cake.to_string(), 21)]),
                cycles_per_month: 48,
                producer: Buildings::Bakery.to_string(),
            },
        ],
    );
    recipes.insert(
        Products::CanolaOil.to_string(),
        vec![
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Canola.to_string(), 20)])
                ],
                outputs: HashMap::from([(Products::CanolaOil.to_string(), 10)]),
                cycles_per_month: 480,
                producer: Buildings::OilMill.to_string(),
            },
        ],
    );
    recipes.insert(
        Products::OliveOil.to_string(),
        vec![
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Olives.to_string(), 25)])
                ],
                outputs: HashMap::from([(Products::OliveOil.to_string(), 10)]),
                cycles_per_month: 240,
                producer: Buildings::OilMill.to_string(),
            },
        ],
    );
    recipes.insert(
        Products::RiceOil.to_string(),
        vec![
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::DryRice.to_string(), 25)])
                ],
                outputs: HashMap::from([(Products::RiceOil.to_string(), 10)]),
                cycles_per_month: 480,
                producer: Buildings::OilMill.to_string(),
            },
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Rice.to_string(), 12)])
                ],
                outputs: HashMap::from([(Products::RiceOil.to_string(), 10)]),
                cycles_per_month: 480,
                producer: Buildings::OilMill.to_string(),
            },
        ],
    );
    recipes.insert(
        Products::SunflowerOil.to_string(),
        vec![
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Sunflower.to_string(), 20)])
                ],
                outputs: HashMap::from([(Products::SunflowerOil.to_string(), 10)]),
                cycles_per_month: 480,
                producer: Buildings::OilMill.to_string(),
            },
        ],
    );
    recipes.insert(
        Products::PotatoChips.to_string(),
        vec![
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Potatoes.to_string(), 100)]),
                    HashMap::from([(Products::SunflowerOil.to_string(), 5)]),
                ],
                outputs: HashMap::from([(Products::PotatoChips.to_string(), 30)]),
                cycles_per_month: 240,
                producer: Buildings::PotatoPlant.to_string(),
            },
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Potatoes.to_string(), 100)]),
                    HashMap::from([(Products::CanolaOil.to_string(), 5)]),
                ],
                outputs: HashMap::from([(Products::PotatoChips.to_string(), 30)]),
                cycles_per_month: 240,
                producer: Buildings::PotatoPlant.to_string(),
            },
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Potatoes.to_string(), 100)]),
                    HashMap::from([(Products::OliveOil.to_string(), 4)]),
                ],
                outputs: HashMap::from([(Products::PotatoChips.to_string(), 30)]),
                cycles_per_month: 240,
                producer: Buildings::PotatoPlant.to_string(),
            },
        ],
    );
    recipes.insert(
        Products::CarrotSoup.to_string(),
        vec![
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Carrot.to_string(), 300)])
                ],
                outputs: HashMap::from([(Products::CarrotSoup.to_string(), 75)]),
                cycles_per_month: 144,
                producer: Buildings::SoupFactory.to_string(),
            },
        ],
    );
    recipes.insert(
        Products::ParsnipSoup.to_string(),
        vec![
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Parsnip.to_string(), 300)])
                ],
                outputs: HashMap::from([(Products::ParsnipSoup.to_string(), 75)]),
                cycles_per_month: 144,
                producer: Buildings::SoupFactory.to_string(),
            },
        ],
    );
    recipes.insert(
        Products::PotatoSoup.to_string(),
        vec![
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Potatoes.to_string(), 300)])
                ],
                outputs: HashMap::from([(Products::PotatoSoup.to_string(), 75)]),
                cycles_per_month: 144,
                producer: Buildings::SoupFactory.to_string(),
            },
        ],
    );
    recipes.insert(
        Products::RedBeetSoup.to_string(),
        vec![
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::RedBeet.to_string(), 300)])
                ],
                outputs: HashMap::from([(Products::RedBeetSoup.to_string(), 75)]),
                cycles_per_month: 144,
                producer: Buildings::SoupFactory.to_string(),
            },
        ],
    );
    recipes.insert(
        Products::TripleSoup.to_string(),
        vec![
            Recipe {
                inputs: vec![
                    HashMap::from([(Crops::Carrot.to_string(), 100)]),
                    HashMap::from([(Crops::Parsnip.to_string(), 100)]),
                    HashMap::from([(Crops::RedBeet.to_string(), 100)]),
                ],
                outputs: HashMap::from([(Products::TripleSoup.to_string(), 75)]),
                cycles_per_month: 120,
                producer: Buildings::SoupFactory.to_string(),
            },
        ],
    );

    return recipes;
}


