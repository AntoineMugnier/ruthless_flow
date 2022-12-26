


pub use std::sync::mpsc::{Receiver};

#[cfg(test)]
pub use test::*;

#[cfg(not(test))]
pub use production::*;

#[cfg(test)]
mod test{
use mockall::mock;

#[derive(Debug, Clone)]
pub struct SendError<T>{phantom_data: core::marker::PhantomData<T>}

pub type Sender<T> = MockSender<T>;

mock! {
    pub Sender<T>{
        pub fn send(&self, t: T) -> Result<(), SendError<T>>;
    }
        
    impl <T> Clone for Sender<T>{
        fn clone(&self) -> Self;
    }
}
}

#[cfg(not(test))]
mod production{
    pub use std::sync::mpsc::{SendError, Sender, channel};
}



