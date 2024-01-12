use mystructs::{Rectangle,Person};

use crate::mystructs::myenums::State;


mod mystructs;
fn main() {
    let rect = Rectangle::new(10,20);

    println!("The area of rectangle rect is {}", rect.area());

    let p = Person::new
    ("Samba".to_string(),"Krishnamurthy".to_string(),
    "99999".to_string(),"samba@gmail.com".to_string(),State::TN);

    p.print_person();
}