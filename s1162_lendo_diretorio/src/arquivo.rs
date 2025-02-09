//use std::fs;
use std::{env, fs};

pub fn caminho_arquivo() -> Option<String> {
  if let Some(caminho_home) = env::var_os("HOME"){
      Some(caminho_home.into_string().unwrap())
  }else{
      None
  }

}

pub fn ler_diretorio(caminho: &str) -> Result<(),std::io::Error>{
  let itens = fs::read_dir(caminho)?;
  for item in  itens{
     let item = item?;
     let item_caminho = item.path();

    if item_caminho.is_dir(){
      println!("Diretorio {}", item_caminho.display());
    }else{
      println!("Arquivo {}", item_caminho.display());
    }
  }
  Ok(())

}
