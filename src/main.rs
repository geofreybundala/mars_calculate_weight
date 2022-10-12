fn main() {
    let weight_mars = calculate_weight_mars(100.0);
    println!("Weight on Mars : {}kg", weight_mars );
}



fn calculate_weight_mars(weight: f32) -> f32{
    return (weight/9.81) * 3.711;
}
