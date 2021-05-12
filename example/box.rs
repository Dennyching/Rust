//智慧指標是一種會自動在建立時分配一塊記憶體，並在變數消失時自動釋放空間的容器，主要就只有 Box 與 Rc 
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
  }
//can't use List because List has to decide size when comlie,but box don't
fn main(){
    let x = Box::new(42);
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    
    println!("{:?}",list);
    use std::rc::Rc;

    let a = Rc::new(42);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);
    println!("{:?}",a);//won't release after call print
    println!("{:?}",b);
}