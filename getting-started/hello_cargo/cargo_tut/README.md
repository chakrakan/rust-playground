`cargo check` to build without producing an executable. Much faster

`cargo build` to build with executable and `cargo run` to run the executable 

Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code.

Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. 

The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run cargo build --release and benchmark with the executable in target/release.

With simple projects, Cargo doesn’t provide a lot of value over just using rustc, but it will prove its worth as your programs become more intricate. With complex projects composed of multiple crates, it’s much easier to let Cargo coordinate the build.