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
#[derive(Debug)]
pub enum ListRef{
    Link(i32, RefCell<Rc<ListRef>>),
    Nil,
}

impl ListRef {
    pub fn tail(&self) -> Option<&RefCell<Rc<ListRef>>> {
        match self {
            ListRef::Link(_, item) => Some(item),
            ListRef::Nil => None,
        }
    }    

}