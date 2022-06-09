use ps::PrimeSieveSingle;

pub mod ps;

fn main() {
    let mut a = PrimeSieveSingle::new();
    a.calc();

    println!("Hello, world!");
}
