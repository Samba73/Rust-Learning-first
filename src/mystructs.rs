#[allow(dead_code)]
pub mod myenums;

use myenums::State;

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
        email: String, state: myenums::State) -> Self 
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