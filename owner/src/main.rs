fn main() {
    let mut s = String::from("hello");//from will requests the memory it needs

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
    /*let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);*/
    //above will error ,when function end,it will call drop twice to the same memory allocation.
    //will cause memory  corruption
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    //this will work because clone deeply copy the heap data of the String
    takes_ownership(s2);
    let x = 5;                      // x comes into scope
    //println!("s1 = {}, s2 = {}", s1, s2); error beacuse s2 will be drop after ownership function finish
    makes_copy(x);
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    //send s1 references will not give ownership,so calculate_length won't drop s1
    println!("The length of '{}' is {}.", s1, len);
    let mut s = String::from("hello");

    change(&mut s);
    //we can change s only when s is mutable and &s is mutable too

    /*let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);*/
    
    //this will fail beacuse  only one mutable reference to a particular piece of data in a particular scope
    
    println!("The length of '{}' is {}.", first_word(&s), first_word(&s).len());
    let slice = &s[..];
    println!("The length of '{}' is {}.", slice, slice.len());
}
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {// i :index item:value
        if item == b' ' || item ==b','{ //ASCII byte literal transfer ' '
            return &s[..i];
        }
    }

    &s[..]
}
