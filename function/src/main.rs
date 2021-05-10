fn main() {
    let  x = 5;

    let y = {
        x + 1
    };
    println!("The value of y is: {}", y);
    another_function(5, 6);
    let x = five();

    println!("The value of x is: {}", x);
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1 //don't add ; rust will deal this as a statement not return value
}
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
