fn main() {
    let mut x = 5;
    { //Introduce a new scope
        let y = &mut x;
        *y = 10;
    }
    let z = &x;
    println!("{}", z);
}