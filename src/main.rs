fn main() {
    let x = 1;
    let item = (x != 0) && ((x & (x - 1)) == 0);
    println!("{item}");
}


