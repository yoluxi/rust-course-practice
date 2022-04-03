fn main() {
    trans();
}

fn trans() {
    let a: i32 = 10;
    let b: u16 = 100;

    if a < b.into() {
        println!("10 < 100")
    }
}
