fn main() {
    let mars_weight = calculate_weight_on_mars(175.0);
    println!("Weight on Mars: {:?}lb", mars_weight);
}

fn calculate_weight_on_mars(earth_weight: f32) -> f32 {
    (earth_weight/9.81) * 3.711
}
