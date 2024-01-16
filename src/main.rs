use mystructs::{Rectangle,Person};
//use helpers::driving_age;
use crate::mystructs::myenums::State;


pub mod helpers;
pub mod mystructs;
/////
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // user_preference.unwrap_or_else(self.most_stocked());
        // user_preference.unwrap_or_else(Inventory::most_stocked(&self))
        user_preference.unwrap_or_else(||self.most_stocked())
        // if let Some(shirt) = user_preference {
        //     shirt
        // } else {
        //     self.most_stocked()
        // }
        // match user_preference {
        //     Some(shirt) => shirt,
        //     None => {
        //         self.most_stocked()
        //     }
        // }
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
////
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


    let x = 10;
    let y = 5;
    let clsr1 = || x * y;
  
    let clsr = clsr1;
    let y = 7;

    println!("The product of 2 numbers {} and {} is {}", x, y, clsr());

    let cls = |x,y| x + y;

    println!("The result is {}", cls(1,1));
    // println!("Here the result is {}", cls("Hello".to_string()," world"));    

    let str1 = "hello".to_string();
    let clsr = |x| println!("{}", x);
    clsr("world");
    clsr("world2");

    let mut str = "hello".to_string();
    let mut clsr = |x| str.push_str(x);
    clsr(" world");
    clsr(" world2");
    clsr(" world3");
    println!("fnmut {}",str);

    

   let mut y = String::new();     
   let clsr = ||y = str1;
   clsr(); 
  
   println!("{}",y);
   

    //////////////
    //
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );



}
