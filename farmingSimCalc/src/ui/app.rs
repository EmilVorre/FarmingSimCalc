use std::io;


fn app(recipes: HashMap<String, Vec<Recipe>>, fields: HashMap<String, Field>, crops: HashMap<String, Crop>, products: HashMap<String, Product>, mut owned_fields: HashMap<String, Field>) {
    welcome();

    main_terminal(recipes, fields, crops, products, owned_fields);


}

fn welcome() {
    println!("Welcome to Farming Simulator Calculator!");
    println!("This is a tool to help you manage your farm in the game Farming Simulator 25.");
    println!("Let's get started!");
    println!("Press any key to continue...");
    
    wait_for_input();
    clear_screen();
}

fn wait_for_input() {
    let _ = io::stdin().read_line(&mut String::new());
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main_terminal(recipes: HashMap<String, Vec<Recipe>>, fields: HashMap<String, Field>, crops: HashMap<String, Crop>, products: HashMap<String, Product>, mut owned_fields: HashMap<String, Field>) {
    loop {
        println!("What would you like to do?");
        println!("1. Buy a field");
        println!("2. Actions for this month");
        println!("3. See fields");
        println!("4. Progress time");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        match choice {
            1 => plant_crop_terminal(),
            2 => harvest_crop_terminal(),
            3 => print_fields(&owned_fields, &crops),
            4 => progress_time(&mut owned_fields, 1.0),
            5 => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn plant_crop_terminal() {
    println!("Plant crop");
}

fn harvest_crop_terminal() {
    println!("Harvest crop");
}