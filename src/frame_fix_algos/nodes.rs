use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug, PartialEq, Eq)]
pub struct StateStep {
    pub sum_hd: usize,
    pub state: String,
    pub next: Option<Rc<RefCell<StateStep>>>,
}


impl StateStep{
    pub fn new(sum_hd: usize, state: String) -> Self {
        Self {
            sum_hd,
            state,
            next: None
        }
    }
}
