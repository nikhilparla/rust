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
- variables
- types
- strings
- tuples
- arrays