

fn add_field(fields: &mut HashMap<String, Field>, name: String, size: f32) {
    fields.insert(
        name.clone(),
        Field {
            name: name,
            size: size,
            crop: Crops::Empty.to_string(),
            time_since_planted: 0.0,
        },
    );
}

fn delete_field(fields: &mut HashMap<String, Field>, name: String) {
    fields.remove(&name);
}

fn print_fields(fields: &HashMap<String, Field>, crops: &HashMap<String, Crop>) {
    for (name, field) in fields {
        println!("{}: {} ha, {}", name, field.size, field.crop);
        println!("Time since planted: {}", field.time_since_planted);

        if ready_for_harvest_check(&fields, &field.name, &crops) {
            println!("{}: Ready for harvest", "Field 1");
        } else {
            println!("{}: Not ready for harvest", "Field 1");
        }

        println!("----------------------");
    }
}

fn move_field_to_owned_fields(fields: &mut HashMap<String, Field>, field_name: &String, owned_fields: &mut HashMap<String, Field>) {
    if let Some(field) = fields.remove(field_name) {
        owned_fields.insert(field_name.clone(), field);
    }
}