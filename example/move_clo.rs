fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |k| k == x;
    //move vec will give ownership to clousure
    //println!("can't use x here : {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
