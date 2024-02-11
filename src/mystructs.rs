#[allow(dead_code)]
#[allow(unused_variables)]
pub mod myenums;
pub mod traits;

use myenums::{State, Transmission};
use traits::{MakeSound, Vehicle, Summary, Messenger};

use std::cell::RefCell;
use std::rc::{Rc, Weak};

// use self::traits::Vehicle;
// use self::traits::MakeSound;

pub struct ClassicCar {
    pub make: String,
    models: Vec<(&'static str, i32)>,
}

impl ClassicCar {

    pub fn new(make: String, models: Vec<(& 'static str, i32)>) -> Self {
        Self {
            make,
            models,
        }
    }

    pub fn get_smart_car<F>(&self, f:F) 
        where 
            F: Fn(&Vec<(&'static str, i32)>)
            {
                f(&self.models);
    }
}
pub struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height
        }
    }

    pub fn area(&self) -> i32 {
        self.width * self.height
    }
}

pub struct Person {
    first_name: String,
    last_name: String,
    mobile: String,
    email: String,
    state: State,
}

impl Person {
    fn new
    (first_name: String, last_name: String, mobile: String, 
        email: String, state: State) -> Self 
        {
            Self { first_name, last_name, mobile, email, state }
    }

    pub fn create_person
        (first_name: String, last_name: String, mobile: String, email: String, state: State) 
        -> Person
        {
            let p = Person::new(first_name,last_name,mobile,email,state);
            return p;
        }

    pub fn print_person(&self) 
     {
        println!("Details are new Person added:
                    {}, {}
                    {}
                    {}
                    {}", self.first_name, self.last_name,self.mobile,
                                    self.email,
                                    match self.state{
                                        State::MH => "Maharastra".to_string(),
                                        State::TN => "Tamil Nadu".to_string(),
                                        State::WB => "West Bengal".to_string(),
                                        State::GJ => "Gujarat".to_string()
                                    });
    }

}

#[allow(dead_code)]
pub struct Rect<T, U> {
    pub width: T,
    pub height: U,
}

impl <T, U> Rect<T, U> 
    where 
    for <'a> &'a T: std::ops::Mul<&'a U, Output = T>
    {
    pub fn area(&self) -> T

    {
        &self.width * &self.height
    }
}

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>{
    pub fn get_x(&self) -> &T {
        &self.x
    }


}

impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub struct AnotherPoint<X1, Y1> {
    pub x: X1,
    pub y: Y1,
}

impl<X1, Y1> AnotherPoint<X1, Y1> {
    pub fn mix_up<X2, Y2>(self, other: AnotherPoint<X2, Y2>) -> AnotherPoint<X1, Y2> {
        AnotherPoint {
            x: self.x,
            y: other.y,
        }
    }
}

pub struct Dog;
impl MakeSound for Dog {
    fn make_sound(&self) -> String {
        String::from("Woof Woof")
    }
}

pub struct Cat;
impl MakeSound for Cat {
    fn make_sound(&self)-> String {
        String::from("Meow Meow")
    }
}

pub struct Bird;
impl MakeSound for Bird {
    fn make_sound(&self)-> String {
        String::from("Chirp Chirp")
    }
}

pub struct MotorCar {
    pub fuel_reading: i32,
    pub fuel_used: i32,
}
impl Vehicle for MotorCar {
    fn calculate_fuel_left(&self) -> i32 {
        &self.fuel_reading - &self.fuel_used
    }

    // fn display_fuel_left(&self) {
    //     println!("The fuel left is {}", &self.calculate_fuel_left());
    // }
}

pub struct MotorCycle {
    pub fuel_reading: i32,
    pub fuel_used: i32,
}
impl Vehicle for MotorCycle {
    fn calculate_fuel_left(&self) -> i32 {
        &self.fuel_reading - &self.fuel_used - 5
    }
    fn display_fuel_left(&self) {
        println!("The fuel left in my motorcycle is {}", &self.calculate_fuel_left());
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct NewCar {
    pub color:          String,
    pub car_type:       String,
    pub transmission:   Transmission,
    pub convertible:    bool,
    pub brand:          String
}

#[derive(Debug)]
pub struct LimitTracker<'a, T: Messenger> {
    pub message: &'a T,
    pub value: usize,
    pub max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(message: &'a T, max: usize) -> Self {
        LimitTracker { message, value: 0, max,}
    }      

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.message.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.message
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.message
                .send("Warning: You've used up over 75% of your quota!");
        }
    }     

}    

#[derive(Debug)]
pub struct Message { 
    pub sent_messages: RefCell<Vec<String>>,
}    

impl Message {
    pub fn new() -> Message {
        Message { sent_messages: RefCell::new(vec![]), }
    }

}    

impl Messenger for Message {
     fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
     }    
}

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}