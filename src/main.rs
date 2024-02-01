fn main() {
    let x= 5;
    {
        let x=6;
        println!("the value of x is {}",x);
    }
    println!("the value of x is {}",x);
}
