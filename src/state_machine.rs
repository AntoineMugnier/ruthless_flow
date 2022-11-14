//use crate::Signals;

pub trait StateMachine {}

impl dyn StateMachine {
    pub fn init() {}
    pub fn dispatch() {}
}
