
# Intro
I wanted to develop on my own a mid-sized Rust project in order to improve my skills in Rust. I got the idea of making a game because it combines UI development and algorithmic logic in a multithreaded environment. I've not used any mutex or semaphore in the game. The backend and the frontend communicates exclusively through queues and the backend is typically an Active Object. I guess this is a legacy I inherited from the extensive use of the QP Active Objects framework suite. 
# Application structure
The `backend` module contains all of business logic of the application. I features a board co
# Unit tests
The only unit tests of this project are the one made for backend::Head.rs with the Mockall framework.  
It's the first time I used this framework.  
I think you can develop powerful tests with it but they are really painful to make and to debug.  
To run theses tests: 
``` cargo test  -- --test-threads=1```
