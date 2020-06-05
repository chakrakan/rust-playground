## Ensuring Reproducible Builds with the Cargo.lock File

Cargo has a mechanism that ensures you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only the versions of the dependencies you specified until you indicate otherwise. For example, what happens if next week version 0.5.6 of the rand crate comes out and contains an important bug fix but also contains a regression that will break your code?

The answer to this problem is the Cargo.lock file, which was created the first time you ran cargo build and is now in your guessing_game directory. When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at 0.5.5 until you explicitly upgrade, thanks to the Cargo.lock file.

When you do want to update a crate, Cargo provides another command, `cargo update`, which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml. If that works, Cargo will write those versions to the Cargo.lock file.

The next time you run cargo build, Cargo will update the registry of crates available and reevaluate your rand requirements according to the new version you have specified.

The parse method on strings parses a string into some kind of number. Because this method can parse a variety of number types, we need to tell Rust the exact number type we want by using let guess: u32. The colon (:) after guess tells Rust we’ll annotate the variable’s type.