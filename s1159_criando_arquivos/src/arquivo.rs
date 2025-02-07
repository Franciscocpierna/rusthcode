use std::{env, fs::File};
use std::io::prelude::*; //importa tudo io
//use std::io::Write;
//C:\rust\s1159_criando_arquivos
pub fn caminho_arquivo() -> Option<String> {
    if let Some(caminho_home) = env::var_os("HOME"){
        Some(caminho_home.into_string().unwrap())
    }else{
        None
    }

}

pub fn criar(caminho: &str, nome_arquivo: &str){
    println!("criar o arquivo caminho em {} ", caminho);
    println!("nome do arquivo  {} ", nome_arquivo);
    let caminho_completo =format!(r"{}\{}", caminho, nome_arquivo);
   match File::create(&caminho_completo){
    Ok(_) => {
        print!("Arquivo foi criado com sucesso no caminho {}",caminho_completo);
    },
    Err(e) => {
        print!("Erro ao criar Arquivo  {}",e);
    }
   }
}