
# Intro
I wanted to develop on my own a mid-sized Rust project in order to improve my skills in Rust.
I got the idea of making a game because it combines UI development and algorithmic logic in a multithreaded environment.
I've not used any mutex or semaphore in the game. The backend and the frontend communicates exclusively through queues and the backend is typically an Active Object.
I guess this is a legacy I inherited from the extensive use of the QP Active Objects framework suite. 

# Application structure
The `backend` module contains all of business logic of the application. It features 3 major components:
- the head module. A head is an autonomous component of the application capable of moving on the map and change the nature of its tiles it go through.
- the map module. It holds the data of the map you sent as argument and provides an interface for allowing editing and reading of its tiles by the heads. 
- the backend module itself. It has many crucial roles.
  - its states of victory or defeat conditions
  - it translates and forward the events coming from the frontend to the map and heads.
  - It is also responsible of managing the birth, death, and execution cycles of the heads. It controls their access to the map through mutable reference.

# Unit tests
The only unit tests of this project are the one made for backend::Head.rs with the Mockall framework.  
It's the first time I used this framework.  
I think you can develop powerful tests with it but they are really painful to make and to debug.  
To run theses tests: 
``` cargo test  -- --test-threads=1```
