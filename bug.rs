fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; // This is the error, multiple mutable references
    *y = 10;
    *z = 12; //Error
}