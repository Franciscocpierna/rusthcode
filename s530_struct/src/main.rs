
pub struct Pessoa{
    codigo: String,
    nome: String,
    idade: i8,
    altura: f32,
}



fn main() {
    let pessoa = Pessoa{codigo:String::from("0005"),
    nome:String::from("Maria"), 
    idade: 20,
    altura: 1.75};

    println!("o Codigo {}", pessoa.codigo);
    println!("Nome {}", pessoa.nome);
    println!("idade {}", pessoa.idade); 
    println!("altura {}", pessoa.altura);

}

