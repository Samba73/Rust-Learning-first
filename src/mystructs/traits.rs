pub trait MakeSound {
    fn make_sound(&self)->String;
}

pub trait Vehicle {
    fn calculate_fuel_left(&self) -> i32;
    fn display_fuel_left(&self){
        println!("The fuel left is {}", &self.calculate_fuel_left());
    }

}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Messenger {
    fn send(&self, message: &str);
}