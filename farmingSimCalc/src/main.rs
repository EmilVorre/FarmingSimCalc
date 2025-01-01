include!("../src/data/structs.rs");
include!("../src/data/enums.rs");
include!("../src/data/fields.rs");
include!("../src/data/recipes.rs");

fn main() {

    let recipes = load_recipes();

    let fields = load_fields();

    println!("Hello, world!");
}
