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

    if_divisible(34);

    // functions return values to code that calls them
    // we don't name return values but we delcare their type after arrow

    // returning values from loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result is {}", result);

    // while loop to eval condition eliminates redundant if else with loop

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let sample: [i32; 5] = [1, 3, 4, 6, 8];
    println!("Sum of a collection is {}", loop_collection(sample));

    // lastly, how to use a regular for loop ..= (inclusive range), .. exclusive range
    for num in 1..=5 {
        println!("{}", num);
    }
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

// control flow
fn if_divisible(number: u32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if in a let statement
    let random_num = if true { 3 } else { 9 };
    println!("{}", random_num);
}

// fn to iterate over a collection with for loop and provide sum
fn loop_collection(arr: [i32; 5]) -> i32 {
    let mut sum = 0;
    for ele in arr.iter() {
        sum += ele;
    }
    return sum;
}
