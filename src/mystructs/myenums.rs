use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code)]
pub enum State {
    MH,
    TN,
    WB,
    GJ,
}
#[derive(PartialEq, Debug)]
pub enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}
#[derive(Debug)]
pub enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}