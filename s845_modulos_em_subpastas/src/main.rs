use operacoes_matematicas::matematica::{somar,subtrair};
mod operacoes_matematicas;

fn main() {
    //let soma= somar(7, 8);
    //let subtrae = subtrair(8, 7);
    println!("a soma é {}", somar(7, 8));
    println!("a subtração é {}", subtrair(8, 7));
}