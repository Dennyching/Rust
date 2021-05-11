fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    let mut s = String::from("lo");
    s.push('l');
    println!("{}",s);
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been mov
    println!("{}",s3);
    let s1 = String::from("नमस्ते");
    //some special char will use multi bytes to store so it might be problem
    //let h = s1[0]; rust don't support string index use down solution
    let s = &s1[0..3];//use 3 bytes to store
    println!("{}",s);
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}
