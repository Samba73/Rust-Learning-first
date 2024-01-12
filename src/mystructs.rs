#[allow(dead_code)]
pub mod myenums;

use myenums::State;

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
    pub fn new
    (first_name: String, last_name: String, mobile: String, 
        email: String, state: myenums::State) -> Self 
        {
            Self { first_name, last_name, mobile, email, state }
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