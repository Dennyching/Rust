use std::io; //include standard library(std) io 
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);//1~100
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();//mutable  //string::new() will allocate new string
        
        io::stdin()
            .read_line(&mut guess)//need to be mutable so read_line can read unfix string
            //& means reference
            .expect("Failed to read line");//if readline return err, call expect
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //parse() return Ok value that contains the resulting number when success.
        //return Err when there is an error
        println!("You guessed: {}", guess);
        //shadow, allow to convert value to another type
        //trim() will eliminate any whitespace and \n at the beginning and end
        //parse() parse string to particular type.guess: u32 so it change to u32  
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
               println!("You win!");
               break;
           }
        }
    }
    
}