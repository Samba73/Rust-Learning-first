use mystructs::{Rectangle,Person};
//use helpers::driving_age;
use crate::mystructs::myenums::State;

pub mod helpers;
pub mod mystructs;
fn main() {
    let rect = Rectangle::new(10,20);

    println!("The area of rectangle rect is {}", rect.area());

    let p: Person = Person::create_person("Samba".to_string(),"Krishnamurthy".to_string(),
    "9999999".to_string(),"samba@gmail.com".to_string(),State::TN);

    p.print_person();

    //println!("Driving Eligibility is {}",driving_age());

    let clsr2 = |x: i32, y| x + y;
    let clsr = clsr2;
    println!("The sum of 2 numbers are {}", clsr(2,5));
    //println!("Concatenate strings {}",add2("Hello","World"));

    let x = 10;
    let y = 5;
    let clsr1 = || x * y;
  
    let clsr = clsr1;
    //let y = 5;

    println!("The product of 2 numbers {} and {} is {}", x, y, clsr());

}