# Vanes: CI testing for embedded systems. 

Program for executing continuous integration test targeting embedded boards.
This program uses GPIO to connect to the testing board. Then runs tests which set board inputs (via GPIO) and monitors
its outputs.
 
## Installation
 
  * Cross compiling: https://github.com/japaric/rust-cross

 
## Commands
 
 ```bash
cargo build --target=armv7-unknown-linux-gnueabihf
```