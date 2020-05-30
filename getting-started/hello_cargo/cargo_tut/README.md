`cargo check` to build without producing an executable. Much faster

`cargo build` to build with executable and `cargo run` to run the executable 

Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code.

Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.