

fn add_field(fields: &mut HashMap<String, Field>, name: String, size: f64) {
    fields.insert(
        name.clone(),
        Field {
            name: name,
            size: size,
            crop: Crops::Empty.to_string(),
        },
    );
}

fn delete_field(fields: &mut HashMap<String, Field>, name: String) {
    fields.remove(&name);
}

fn plant_crop(fields: &mut HashMap<String, Field>, field_name: String, crop_name: String) {
    if let Some(field) = fields.get_mut(&field_name) {
        field.crop = crop_name;
    }
}

fn harvest_crop(fields: &mut HashMap<String, Field>, field_name: String) {
    if let Some(field) = fields.get_mut(&field_name) {
        field.crop = Crops::Empty.to_string();
    }
}

fn print_fields(fields: &HashMap<String, Field>) {
    for (name, field) in fields {
        println!("{}: {} ha, {}", name, field.size, field.crop);
    }
}


fn move_field(fields: &mut HashMap<String, Field>, field_name: String, owned_fields: &mut HashMap<String, Field>) {
    if let Some(field) = fields.remove(&field_name) {
        owned_fields.insert(field_name, field);
    }
}

