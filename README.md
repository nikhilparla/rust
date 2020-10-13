### Intro
- System programming language
- System language
	- C, C++, RUST
### Appliciation programming language
- JAVA, C#, JAVAScript
### Memory Management
- Doesnt have garbage collection, no need of managing memory
- C and C++ you need to handle memory yourself
### Package manager
- Rust - Cargo
- pip for python
- composer for php
### Installation
For WSL \
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` \
- Proceeding with default installation 
- restart terminal 
- it installs a few utilities 
`rustup --version` \
`rustc --version` \
`cargo --version` 
### Running
#### Hello world  
`vim hello.rs`  
`rustc hello.rs`  
`./hello`  
#### Practical applications 
- you use cargo to setup project for you instead of indicidual files  
`mkdir cargo_hello`  
`cargo init`  
- created a few files and folders for us  
- also created a main.rs file for us with a hello world print command  
`cargo run` 	- builds and runs the project  
`cargo build` 	- only build  
`cargo build --release`	- build for production with optimizations
### Topics
- print formattings
	- basic formatting of prints
- variables
- types
- strings
	- primitive strings and heap allocated String data structure
- tuples
	- similar to struct but more functions to operate on
- arrays
	- primitive arrays
- Pointers and reference
- structs
	- very important in Rust. Check the file, has lot of comments
	- like classes in python, php. Can have functions to act on structs

There are other topics like functions, enums, loops etc but ignored them since some of them are implemented in the pointers topic and structures topic.\
Lots of comments in the files, so they can be used as reference.\