fn main() {
    // variables and mutability

    let x = 5;
    let mut y = 8;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    // cannot assign below cuz var is not mutable (by default vars are not mutable)
    // x = 6;
    // but we can reassign y cuz 'mut' makes it mutable
    y = 6;
    println!("The value of y is: {}", y);

    // diff between defaul vars and constants
    // 1) you can't use mut with constants, it's implied that you can't change them
    const MAX_POINTS: u32 = 100_000;
    // 2.) declare with 'const' instead of let
    // 3.) can be declared in any scope, including global scope
    // 4.) constants may be set only to a constant expression, not the result of a function call or 
    // any other value that could only be computed at runtime

    // concept of shadowing
    // declaring a new var with the same name but new var shadows previous var
    let x = 18;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);

    // Shadowing is different from marking a variable as mut, because we’ll get a 
    // compile-time error if we accidentally try to reassign to this variable 
    // without using the let keyword. By using let, we can perform a few 
    // transformations on a value but have the variable be immutable after those 
    // transformations have been completed.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    // Data Types
    
    // scalar types represent a single value, 4 primary scalar types
    // integers, floats, bool, char
    let a: i8 = 5; // or u8 (unsigned)
    let b: i16 = 1200; // u16
    let c: i32 = 120_000_000; // u32
    let d: i64 = 120_000_000_000; // u64
    let e: i128 = 120_000_000_000_000_000; // u128
    println!("{}, {}, {}, {}, {}", a, b, c, d, e);

    // unsigned means no negative numbers
    // signed nums are stored in twos complement
    // isize and usize types depend on the computer you are running on 
    // Rust defaults to i32, and its generally fastest even on 64bit systems

    // if you overflow when using a particular type, RUst does not include checks when you
    // compile using --release tag and wrapping occurs (wrap around back to 0, or min value of type)

    // for floats, default type is f64 because it's capable of more precision and roughly same speed as f32
    let xyz = 2.0; // default f64
    let f: f32 = 2.0; // explicit f32

    // Numeric operations

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // boolean is 1 byte in size
    let t = true;

    let f: bool = false; // with explicit type annotation

    // Char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value, 
    // which means it can represent a lot more than just ASCII. Accented letters; 
    // Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are
    // all valid char values in Rust. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF
    // inclusive. 


    // Compound types
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    // Tuple
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // pattern matching to destructure tuple values and output them
    let wow = (1000, 2.8, 1);
    let (m, n, o) = wow;
    println!("Val of n is {}", n);
    // you can also access a tuple var using 'tup.[index]' notation
    println!("OH my ${}", wow.0);

    // Array
    // every element must have same type
    // have fixed length like tuples
    // data is allocated on the stack, rather than the heap
    // a vector is more flexible than array where it is allowed to grow and shrink in size, but we'll go there later

    let arr = [1, 2, 3, 4, 5];
    // more explicitly [type; size]
    let arrTypeDef: [i32; 5] = [2, 4, 6, 8, 10];
    println!("{}", arr[2]);
    // if your index is out of range you'll get a runtime error and exit successfully
    // Rust is safe <3
    // other low lvl languages, if you specify incorrect index you might get invalid memory access
}
