
mod arquivo;
use arquivo::{criar,caminho_arquivo};

fn main() {
   
    let caminho = caminho_arquivo().unwrap();
    criar(&caminho,&"hcode.txt");


  
    } 
  
/*
  let caminho = caminho_arquivo();
    match caminho {
    
      Some(cami) => { 
        criar(&cami);
        println!("Caminho encontrado")
    }
      None =>{
        println!("Caminho não encontrado");
      }
    };

*/