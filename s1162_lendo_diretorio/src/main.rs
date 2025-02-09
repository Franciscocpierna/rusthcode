
mod arquivo;
use arquivo::{ler_diretorio,caminho_arquivo};
fn main() {
    let caminho = caminho_arquivo().unwrap();
    //criar(&caminho,&"hcode.txt");
    //ler(&r"C:\rust\s1159_criando_arquivos\hcode.txt");
    match  ler_diretorio(&caminho){
        Ok(_) => println!("leiura ok"),
        Err(_) => println!("Falha na leitura")
    }
}
 
        
 