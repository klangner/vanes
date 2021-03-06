# Vanes: CI testing for embedded systems. 

Program for executing continuous integration test targeting embedded boards.
This program uses GPIO to connect to the testing board. Then runs tests which set board inputs (via GPIO) and monitors
its outputs.
 
## Installation
 
  * Cross compiling: https://github.com/japaric/rust-cross
  
  
## Features
  
  * Define system as collection of states, and transitions between states (State Machine).
  * Check state of:
    * Digit pin
  * Execute action
    * Set value on digital pin

 
## Commands
 
 ```bash
cargo build --target=armv7-unknown-linux-gnueabihf
```

## Useful links

  * [BeagleBone GPIO mapping](http://beagleboard.org/Support/bone101/#headers)