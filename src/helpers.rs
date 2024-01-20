use std::io;
// use crate::mystructs::ClassicCar;
pub fn driving_age() -> bool {
    let age_to_drive = 16u8;

    println!("Enter your age to determine the driving access");

    let mut user_age = String::new();
    io::stdin().read_line(&mut user_age).expect("Unable to read input");
    let age = user_age.trim().parse::<u8>().unwrap();
    if age >= age_to_drive {
        true
    } else {
        false
    }
}
