
mod matematica{
    pub fn somar(a: i32, b: i32) -> i32{
        a+b
    }
    pub fn subtrair(a: i32, b: i32) -> i32{
        a-b
    }
}
fn main() {
    //let soma= somar(7, 8);
    //let subtrae = subtrair(8, 7);
    println!("a soma é {}", matematica::somar(7, 8));
    println!("a subtração é {}", matematica::subtrair(8, 7));
}
