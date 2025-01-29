use std::fs::File;
use std::io::{self,Read};
pub fn main() {
   // println!("Hello, world!");
   let resultado = ler_arquivo(r"C:\rust\s739_result_type\src\main2.rs");
   match resultado {
    Ok(conteudo) =>{
        println!("Conteúdo do Arquivo {}", conteudo);
    },
    Err(erro) => {
        println!("erro ao ler o Arquivo {}", erro);
    }
   }
}

fn ler_arquivo(caminho: &str) -> Result<String, io::Error>{

    let mut arquivo = File::open(caminho)?;
    let mut conteudo= String::new();
    arquivo.read_to_string(&mut conteudo); 
    Ok(conteudo)
    
}