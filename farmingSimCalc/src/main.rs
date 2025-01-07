include!("../src/data/structs.rs");
include!("../src/data/enums.rs");
include!("../src/data/fields.rs");
include!("../src/data/recipes.rs");
include!("../src/data/crops.rs");
include!("../src/data/products.rs");

include!("../src/utility/field.rs");
include!("../src/utility/crop.rs");
include!("../src/utility/general.rs");
include!("../src/utility/ISAAC.rs");

include!("../src/ui/app.rs");

fn main() {

    let recipes = load_recipes();

    let fields = load_fields();

    let crops = load_crops();

    let products = load_products();

    let owned_fields: HashMap<String, Field> = HashMap::new();

    /*
    move_field_to_owned_fields(&mut fields, &"Field 1".to_string(), &mut owned_fields);
    move_field_to_owned_fields(&mut fields, &"Field 2".to_string(), &mut owned_fields);

    plant_crop(&mut owned_fields, "Field 1".to_string(), Crops::Oats.to_string());

    progress_time(&mut owned_fields, 3.0);

    print_fields(&owned_fields, &crops);
    */
    
    app(recipes, fields, crops, products, owned_fields);
}
