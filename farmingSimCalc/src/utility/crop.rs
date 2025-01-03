


fn allowed_crops_to_use() -> Vec<String> {
    let mut allowed_crops: Vec<String> = Vec::new();

    allowed_crops.push(Crops::Oats.to_string());
    allowed_crops.push(Crops::Wheat.to_string());
    allowed_crops.push(Crops::Canola.to_string());

    return allowed_crops;
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

fn ready_for_harvest_check(fields: &HashMap<String, Field>, field_name: &String, crops: &HashMap<String, Crop>) -> bool {
    let crop_on_field = fields.get(field_name).unwrap().crop.clone();

    if let Some(crop) = crops.get(&crop_on_field) {
        if fields.get(field_name).unwrap().time_since_planted >= crop.time_to_harvest {
            return true;
        }
    }
    false
}