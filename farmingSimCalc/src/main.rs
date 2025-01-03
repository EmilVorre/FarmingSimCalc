include!("../src/data/structs.rs");
include!("../src/data/enums.rs");
include!("../src/data/fields.rs");
include!("../src/data/recipes.rs");
include!("../src/data/crops.rs");

fn main() {

    let recipes = load_recipes();

    let fields = load_fields();

    let crops = load_crops();

    

    println!("Hello, world!");
}
