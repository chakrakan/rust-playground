fn main() {
    println!("Hello, world!");

    another_function(12);

    // creating a variable and assigning value to it with let keyword is a statement
    // functions itself are statements as well
    let y = 77;
    // statments don't return values
    let z = {
        let x = 2;
        // call func five here within expression
        x + 1 + five()
    };

    println!("Val of z {}, val of y {}", z, y);

    // functions return values to code that calls them
    // we don't name return values but we delcare their type after arrow
}

// func defs start with fn with param and its type
fn another_function(x: i32) {
    println!("Another function with param {}", x);
}

// func with return
// in Rust, return val of func is synonymous with val of final expression in block of func body
fn five() -> i32 {
    5
}

// NOTE: adding semicolons to the expressions will cause errors cause they will be interpreted as statements.
            
