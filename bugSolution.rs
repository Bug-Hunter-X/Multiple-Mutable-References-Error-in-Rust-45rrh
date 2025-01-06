fn main() {
    let mut x = 5;
    { //Creating a new scope for y
        let y = &mut x; 
        *y = 10; 
    }
    let z = &mut x; 
    *z = 12;
    println!("x = {}", x);
}