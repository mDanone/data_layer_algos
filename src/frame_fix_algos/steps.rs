#[derive(Debug, PartialEq, Eq)]
pub struct StateStep {
    pub sum_hd: usize,
    pub state: String,
}


impl StateStep{
    pub fn new(sum_hd: usize, state: String) -> Self {
        Self {
            sum_hd,
            state,
        }
    }
}
