#![deny(clippy::all)]

const MY_AGE: u8 = 22;

fn main() {
    //Inmutable variables
    let name: &str = "Foo";
    // name = "Matsi"

    let last_name = "12321";

    println!("Your name is {}", last_name);

    //Mutable variable
    let mut score = 13;
    score = 30;

    print!("Your score is {}", score);

    //Variables types cannot be changed
    let mut opinion = "this is my oponion";
    // opinion = 13;

    //Integer types
    let mut age_1: u8 = 30;
    let mut age_2: u8 = 30u8;
    let mut age_3: i128 = -121312312312312312;
    let mut money: i128 = 62_000_000;
    let red = 0xFA;
    let rgb = 0xFF000;

    //Floating variables
    let distance_in_m = 5.5;
    let distance_in_km = 5.5f32;

    //Operations
    let d1 = 5.9;
    let d2 = 9.2;
    let d3 = 1.9;
    let total = d1 + d2 + d3;

    //Tuples
    let personal_data = (300, "Bag");
    let (cost, product) = personal_data;
    let cost = personal_data.0;
}

fn variable_shadowing() {
    let name = "Lio";
    let name = "Messi";
    let name = 1.2302;
    {
        let name = name.to_string();
    }
}
