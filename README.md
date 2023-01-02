# RustLearning
This is the repository for learning Rust
# Installation
Bold Mac OS or Linux
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
$ xcode-select –install
Windows
To acquire the build tools, you’ll need to install Visual Studio 2022. When asked which workloads to install, include:
“Desktop Development with C++”
The Windows 10 or 11 SDK
The English language pack component, along with any other language pack of your choosing
Install rust using https://www.rust-lang.org/tools/install

### Check the version
rustc –version
# update Rust
rustup update
# uninstall Rust
rustup self uninstall

### Rust Hello world
Create project directory
Open main.rs file in the choice of your editor 
    fn main() {
         println!("Hello, world!");
      }

##On mac Os or Linux

$ rustc main.rs 
$ ./main 

Hello, world! �
On Windows

$ rustc main.rs 
$ .\main.exe 

Hello, world! �
About
Learning Rust

