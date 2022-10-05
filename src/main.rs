use std::io::{self, Read};

fn main() {
    println!("Enter your weight in lbs:");

    // input string is mutably borrowed here 
    // then its value is changed by the read_line function
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    // any function that returns a Result can be 'unwrapped'
    // Results contain an Ok or an Err
    // if an Ok, unwrap() yields the returned value
    // if an Err, unwrap() halts execution of the program
    let earth_weight: f32 = input.trim().parse().unwrap();

    let mars_weight = calculate_weight_on_mars(earth_weight);
    println!("Weight on Mars: {:?}lb", mars_weight);
}

fn calculate_weight_on_mars(earth_weight: f32) -> f32 {
    (earth_weight/9.81) * 3.711
}
