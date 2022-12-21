use mockall::mock;


#[cfg(not(test))]
pub type Sender<T> = std::sync::mpsc::Sender<T>;

#[cfg(not(test))]
pub use std::sync::mpsc::{SendError};

pub use std::sync::mpsc::{Receiver, channel};

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct SendError<T>{phantom_data: core::marker::PhantomData<T>}

#[cfg(test)]
pub type Sender<T> = MockSender<T>;

mock! {
    pub Sender<T>{
        pub fn send(&self, t: T) -> Result<(), SendError<T>>;
    }

    impl <T> Clone for Sender<T>{
        fn clone(&self) -> Self;
    }
}