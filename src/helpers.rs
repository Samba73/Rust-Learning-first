#[allow(unused_must_use)]
use std::io;
// use crate::mystructs::ClassicCar;
use crate::mystructs::{myenums::Transmission, traits::{MakeSound,Vehicle}, MotorCar, MotorCycle, NewCar, Message, LimitTracker};

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

pub fn display_sound(animal: &impl MakeSound){
    println!("The animal make {}", animal.make_sound());
}
// the fn below is static dispatch (trait bound) where the type that implements the trait
// is known at compile time
// pub fn get_fuel(name: &String, vehicle: &impl Vehicle)
//     {
//         println!("The fuel left in {} after initial consumption is {}", name, vehicle.calculate_fuel_left());
//     }
// the fn below is static dispatch (trait bound) where the type that implements the trait
// is known at compile time    
// pub fn get_fuel<T: Vehicle>(name: &String, vehicle: &T)
//     {
//         println!("The fuel left in {} after initial consumption is {}", name, vehicle.calculate_fuel_left());
//     }
// pub fn get_fuel<T>(name: &String, vehicle: &T)
//     where T: Vehicle
//     {
//         println!("The fuel left in {} after initial consumption is {}", name, vehicle.calculate_fuel_left());
//     }
// the fn below is dynamic dispatch (trait object) where the type that implements the trait
// is known at run time
pub fn get_fuel(name: &str, vehicle: &dyn Vehicle)
    {
        println!("The fuel left in {} after initial consumption is {}", name, vehicle.calculate_fuel_left());
    }
// pub fn get_fuel<T: Vehicle>(vehicle1: &T, vehicle2: &T)
//     {
//         println!("The fuel left (mult)after initial consumption is {}", vehicle1.calculate_fuel_left());
//         vehicle2.display_fuel_left();
//     }
// pub fn get_fuel(vehicle1: &Vehicle, vehicle2: &Vehicle)
//     {
//         println!("The fuel left (mult)after initial consumption is {}", vehicle1.calculate_fuel_left());
//         vehicle2.display_fuel_left();
//     }
// following fn uses dynamic dispatch (trait object), the type that implements the trait is known
// at run time and also dynamically created instance, here trait is return type
pub fn get_vehicle(whl: i32) -> Box<dyn Vehicle> {
    match whl {
        2 => Box::new(MotorCycle{fuel_reading: 75, fuel_used: 25}),
        4 => Box::new(MotorCar{fuel_reading:125,fuel_used:35}),
        _ => panic!("Unknown vehicle type"),
    }
}

pub fn take_input(user_input: &mut String) -> String {
    io::stdin()
        .read_line(user_input).expect("Failed to read input");
    user_input.to_string()
         
}

pub fn car_factory(
    color:          String,
    transmission:   Transmission,
    brand:          String,
    convertible:    bool,
    car_type:       String
) -> NewCar {
    NewCar {
        color,
        transmission,
        brand,
        convertible,
        car_type
    }
}

pub fn vector_merge_sort(v: &[i32]) -> Vec<i32> {
    let length = v.len();

    if length < 2 {
        return Vec::from(v);
    } 

    let mid = length /2 ;

    let left_v = &v[0..mid];
    let right_v = &v[mid..length];

    let mut v1 = vector_merge_sort(&left_v);
    let mut v2 = vector_merge_sort(&right_v);

    let return_vector = vector_merge(&mut v1, &mut v2);

    return_vector

}

fn vector_merge(v1: &mut Vec<i32>, v2: &mut Vec<i32>) -> Vec<i32> {

    let mut result = Vec::new();

    while !v1.is_empty() && !v2.is_empty() {
        if v1[0] <= v2[0] {
            result.push(v1.remove(0));
        } else {
            result.push(v2.remove(0));
        }
    }
    result.extend_from_slice(v1);
    result.extend_from_slice(v2);

    result
}

pub fn messages_sent() {
    let msg = Message::new();

    let mut limit_tracker = LimitTracker::new(&msg,  100);

    limit_tracker.set_value(90);

    assert_eq!(msg.sent_messages.borrow().len(), 1);

    println!("The messages are {:#?}", msg);

}

pub fn add_one(x: i32) -> i32 {
    x + 1
}
pub fn add_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
