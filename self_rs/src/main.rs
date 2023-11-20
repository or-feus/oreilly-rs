fn main() {
    println!("Hello, world!");
}


struct work {
    pc: String,
    number_of_pc: i32,
    position: i32,
}

trait Working {

    type Output;
    fn read(&mut self) -> &mut Self;
    fn write(&self);

}

impl Working for work {
}
