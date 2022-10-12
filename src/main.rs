use std::io;

fn main() {
    println!("Enter Your weight (Kg)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();


    //casting string to f32
    let weight: f32 = input.trim().parse().unwrap();

    //debug macro
    // dbg!(weight);
    let weight_mars = calculate_weight_mars(weight);
    println!("Weight on Mars : {}kg", weight_mars );
}



fn calculate_weight_mars(weight: f32) -> f32{
    return (weight/9.81) * 3.711;
}