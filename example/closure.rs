fn main(){
    //let f = |x| x * 2;
    let f = |x: i32| -> i32 { x * 2 };
    let n = 3;
    //can get n value
    let f = |x| x * n;
    //n ==mut so n can be change
    let mut n = 0;
    let mut counter = || {
        n += 1;
        n
    };
    println!("{}", counter());
    println!("{}", counter());
    println!("{}", counter());
    println!("{}", f(10));
    let v = vec![1, 2, 3];
    let is_equal_v = move |a| v == a ;
    println!("{}", is_equal_v(vec![1, 2, 3]));
    println!("{}", is_equal_v(vec![4, 5, 6]));
    // 這邊無法使用 v 因為move 把所有權給了 is_equal_v
    fn call_closure<F: Fn(i32) -> i32>(work: F) {
        println!("{}", work(10));
      }
     // 接受閉包的函式
    call_closure(|x| x * 2);
}