mod arquivo;
use arquivo::{caminho_arquivo, criar, ler, existe};

fn main() {
    if Ok(()) == existe(&r"C:\rust\s1159_criando_arquivos\hcode.txt"){
         println!("Arquivo existe");
    }else{
        println!("Arquivo não existe");
    }
    let caminho = caminho_arquivo().unwrap();
    criar(&caminho,&"hcode.txt");
    ler(&r"C:\rust\s1159_criando_arquivos\hcode.txt");

  
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