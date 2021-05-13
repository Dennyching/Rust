#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
}
fn main(){
    let v1: Vec<i32> = vec![1, 2, 3];

    let coll : Vec<_>= v1.iter().map(|x| x+2 ).collect();
    println!("{:?}",coll);

}