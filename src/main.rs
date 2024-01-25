use mystructs::{Rectangle,Person, ClassicCar};
//use helpers::driving_age;
use crate::{helpers::{largest_i32, largest, largest_char}, mystructs::myenums::State};
//use crate::iterators;

pub mod helpers;
pub mod mystructs;
pub mod iterators;
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
fn add_five<F>(x: i32, mut f:F)
where
    F: FnMut(i32)  {
        f(x);
    }
fn main() {

    let ford_model = vec![("Thunderbird", 1960),
                                            ("Cobra", 1966),
                                            ("GT", 1967),
                                            ("Mustang Grande", 1969)];

    let fords = ClassicCar::new(String::from("Ford"),ford_model);

    fords.get_smart_car(|x| {
        let res: Vec<_> = x.into_iter().filter(|m|m.1>1960).collect();
        println!("The filter models for {} are {:#?}", fords.make, res);
    });

    iterators::iterator();


    let mut num1 = 6;
    //let num2 = 5;
    add_five(5, |x| {num1 += x; println!("closure as parameter to fn {}", num1)});
    add_five(4, |x| {num1 += x; println!("closure as parameter to fn {}", num1)});

    let mut c1 = |x| { num1 += x; println!("closure same as parameter to fn {}", num1)};
    c1(5);
    c1(4);


    let rect = Rectangle::new(10,20);

    println!("The area of rectangle rect is {}", rect.area());

    let p: Person = Person::create_person("Samba".to_string(),"Krishnamurthy".to_string(),
    "9999999".to_string(),"samba@gmail.com".to_string(),State::TN);

    p.print_person();

    //println!("Driving Eligibility is {}",driving_age());

    let add = |x: i32, y| {
        println!("x: {} y: {}", x, y);
        x + y
    };
    let clsr = add(2,5);
    let print_clsr = |x| println!("The result is {}",(clsr + x));
    print_clsr(94);
    // println!("The sum of 2 numbers are {}", clsr(2,5));
    let another_clsr = clsr * 2;
    println!("Another clsr is {}", another_clsr);

    let x = 10;
    let y = 5;
    let clsr1 = || x * y;
    println!("The closure read from its env {}", clsr1());
    let clsr = clsr1;
    let y = 7;

    // following are how values are captured by closures (reference / move)
    // Fn trait
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // Fnmut trait
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    //Fnonce trait
    let str = String::from("hello");
    let mut x = String::new();
    let clsronce = || x = str;
    clsronce();
    // println!("Original str value is {}", str);
    println!("New value of x is {}",x);




    println!("The product of 2 numbers {} and {} is {}", x, y, clsr());

    let cls = |x,y| x + y;

    // println!("The result is {}", cls(1,1));
    println!("Here the result is {}", cls("Hello".to_string()," world"));    

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

// the following 2 calls are functions to find largest i32    
let v = vec![23,34,112,45,67];
let result = largest_i32(&v);
println!("The largest value in vector {:#?} is {}", v, result);

let v1 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
let result = largest_i32(&v1);
println!("The largest value in vector {:#?} is {}", v1, result);

// the following call is to function to find largest char
let v_char = vec!['s', 'p', 'z', 'g'];
let result = largest_char(&v_char);
println!("The largest char in vector {:#?} is {}", v_char, result);

let v_i32 = vec![23,34,112,45,67];
let result = largest(&v_i32);
println!("The largest value in vector {:#?} is {}", v_i32, result);

let v_char = vec!['s', 'p', 'y', 'g'];
let result = largest(&v_char);
println!("The largest char in vector {:#?} is {}", v_char, result);


}
