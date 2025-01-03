


fn progress_time(fields: &mut HashMap<String, Field>, months_passed: f32) {
    for (_, field) in fields {
        if field.crop != Crops::Empty.to_string() {
            field.time_since_planted += months_passed;
        }
    }
}