use mockall::mock;
use std::sync::mpsc::{SendError};

#[cfg(not(test))]
pub type Sender<T> = std::sync::mpsc::Sender<T>;

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