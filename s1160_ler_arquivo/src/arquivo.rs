use std::{env, fs::File};
use std::io::prelude::*;

use crate::arquivo; //importa tudo io
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
    Ok(mut arquivo) => {
        let conteudo ="hcode treinamentos ola";
        match arquivo.write_all(conteudo.as_bytes()){
            Ok(_) => {
                print!("Arquivo foi criado com sucesso no caminho {}",caminho_completo);
            },
            Err(e) => {
                println!("Erro ao criar Arquivo  {}",e);
            }
        }
        
    },
    Err(e) => {
        println!("Erro ao criar Arquivo {}" ,e);
    }
   }
}

pub fn ler(caminho_completo: &str){
   match File::open(&caminho_completo){
    Ok(mut arquivo) =>{
        let mut conteudo = String::new();
        arquivo.read_to_string(&mut conteudo).unwrap();
        println!();
        println!("Arquivo aberto {}",conteudo);
        println!();
    },
    Err(e) =>{
        println!("Erro ao abrir o aruivo {}",e);
    }
   };


}