fn main() {
    let number = 3;

    if number !=0  /*can't only variable*/{
        println!("condition was true");
    }
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2"); // previous condition will execute ,so this one won't
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };//if else value need to be same type

    println!("The value of if/else number is: {}", number);
    let mut counter = 0;

    let result = loop { // similiar while(1)
        counter += 1;

        if counter == 10 {
            break counter * 2; //break jump out of loop then result = counter * 2
        }
    };
    println!("The result of loop is {}", result);
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("while LIFTOFF!!!");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value of while array is: {}", a[index]);

        index += 1;
    }

    for element in a.iter() { //smart one,will auto iter the whole array and won't out of index range
        println!("the value of for iter is: {}", element);
    }
    for number in (1..4).rev() {  //1~3 actually
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    
}
