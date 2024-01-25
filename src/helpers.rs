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
pub fn largest_i32(v_ref: &Vec<i32>) -> i32 {
    let mut largest = v_ref[0];
     for &numb in v_ref {
        // dbg!(numb);
        if numb > largest {
            largest = numb;
        }
    }
    largest
}
pub fn largest_char(c_ref: &Vec<char>) -> char {
    let mut largest_char = c_ref[0];
    for &char in c_ref {
        if char > largest_char {
            largest_char = char;
        }
    }
    largest_char
}

pub fn largest<T: std::cmp::PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];

    for val in list {
        if val > largest {
            largest = val;
        }
    }
    largest
}