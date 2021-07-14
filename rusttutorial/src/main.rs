use std::io; //use io from the standard libraries
use colored::*; //use for terminal colors
use std::cmp::Ordering; //use for checking order
use rand::Rng; //use random numbers

fn main() {
    loop {
        println!("\n\nPlease input a corresponding number: ");
        println!("0: Quit\n1: Guessing game\n2: Fizzbuzz (1-1000)\n3: Simple test\n4: Ownership & structs testing\n");

        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("Failed to read input"); //sets reference to selection to inputted value
        match selection.trim().parse::<u8>() { //matches selection (with removed whitespaces and made into u8 int) with below
            Ok(0) => break, //quits
            Ok(1) => guessgame(),
            Ok(2) => fizzbuzz(1, 1000), //may want to multi-thread //passed as params
            Ok(3) => simpletesting(), //runs testing //mainly chapter 3
            Ok(4) => ownershipandstructstesting(),
            Ok(_) => continue, //if other val re-does loop
            Err(_) => continue, //same for err
        };
    }
}

fn simpletesting() {
    println!("Hello, world!");
    let i32num: i16 = -255; //creates new var as signed 16-bit int
    let unsignedarch: usize = 4_000_000_000; //creates new unsigned architecture-max int //can use _'s as separator
    let float8: f32 = 3.5; //creates new single-precision float
    let tru: bool = true; //creates new boolean with value true
    let aint = 5; //creates new int (compiler can often determine type at compile-time
    let strlit = "I am a rustation"; //creates a new string literal (on stack instead of heap)
    let mut mutable = 8; //variables are not mutable by default (adding mut changes that)
    mutable = mutable + 1;
    println!("i32num is {} and unsignedarch is {}", i32num, unsignedarch);
    print!("A float is {}", float8); //as you can see println adds a newline after the printed string
    println!("a bool is {}", tru);
    println!("aint is {}", aint); //print's and println's act in a similar way to how they do in java
    println!("stringlit is: {}", strlit);
    println!("a mutable var was 8, but now is: {}", mutable);
    //add slice type (delete once done)
}

fn ownershipandstructstesting() {
    println!("To implement")
    //implement
}

fn guessgame() { //new function called guessgame (from the tutorial)
    println!("\nGuess a number from 1-25 (inclusive):\n");
    let num = rand::thread_rng().gen_range(1..26);
    let mut tries = 0;
    loop {
        let mut guess = String::new();
        tries = tries + 1;
        io::stdin().read_line(&mut guess).expect("Failed to read input"); //same as in selection loop
        let guess: u8 = match guess.trim().parse() { //shadows the var guess
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&num) { //match statement like above
            Ordering::Less => println!("{}", "Higher!".yellow().italic()),
            Ordering::Greater => println!("{}", "Lower!".cyan().italic()),
            Ordering::Equal => {
                println!("{} with {} tries.", "Nice, you win".green(), tries.to_string().blue().bold().italic()); //adds color (needs string)
                break;
            },
        };
    }
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




//INT OVERFLOW:
//Relying on integer overflow’s wrapping behavior is considered an error.
//
//To explicitly handle the possibility of overflow, you can use these families of methods that the standard library provides on primitive numeric types:
//
//    Wrap in all modes with the wrapping_* methods, such as wrapping_add
//    Return the None value if there is overflow with the checked_* methods
//    Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
//    Saturate at the value's minimum or maximum values with saturating_* methods

