use std::io; //use io from the standard libraries

fn main() {
    loop{
        println!("\n\nPlease input a corresponding number: ");
        println!("0: Quit\n1: Run Test\n2: Guessing game\n3: Fizzbuzz (1-1000)\n");

        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("Failed to read input"); //sets reference to selection to inputted value
        match selection.trim().parse::<u8>() { //matches selection (with removed whitespaces and made into u8 int) with below
            Ok(0) => break, //quits
            Ok(1) => testing(), //runs testing
            Ok(2) => guessgame(),
            Ok(3) => fizzbuzz(1, 1000), //may want to multi-thread //passed as params
            Ok(_) => continue, //if other val re-does loop
            Err(_) => continue, //same for err
        };
    }
}

fn testing() {
    println!("Hello, world!");
    let i32num: i16 = -255; //creates new var as signed 16-bit int
    let unsignedarch: usize = 4_000_000_000; //creates new unsigned architecture-max int //can use _'s as separator
    let float8: f32 = 3.5; //creates new single-precision float
    let tru: bool = true; //creates new boolean with value true
    let aint = 5; //creates new int (compiler can often determine type at compile-time
    let mut mutable = 8; //variables are not mutable by default (adding mut changes that)
    mutable = mutable + 1;
    println!("i32num is {} and unsignedarch is {}", i32num, unsignedarch);
    print!("A float is {}", float8); //as you can see println adds a newline after the printed string
    println!("a bool is {}", tru);
    println!("aint is {}", aint); //print's and println's act in a similar way to how they do in java
    println!("a mutable var was 8, but now is: {}", mutable);
}

fn guessgame() { //new function called guessgame (from the tutorial)
    //code
}

fn fizzbuzz(f: i32, t: i32) { //hybrid of two main fizzbuzz approaches (single threaded) //inputs signed 32 bit ints
    println!("\n\n");
    for i in f..=t { //for i in from (inc) to to (inc)
        if i % 3 != 0 && i % 5 != 0 { //most common case (53 in 100) //put at top
            println!("{}", i);
            continue;
        }
        //skips allocating space on heap for most common case
        let mut res = String::new(); //creates string
        //may be slower due to strings being on the heap, not stack
        if i % 3 == 0 {
            res = String::from("fizz");
        }
        if i % 5 == 0 {
            res = res + "buzz";
        }
        println!("{}", res); //prints the result
    }
}
