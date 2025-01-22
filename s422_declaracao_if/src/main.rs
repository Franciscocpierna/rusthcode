fn main() {
   let resultado = declaracao_if();
   println!(" a condição é {}", resultado);
}

fn declaracao_if()-> &'static str{
    let condicao = true;

    let resultado = if condicao{
        "condição é verdadeira " 
    }else{
        "condição é falsa " 
          
    };
    //println!(" a condição é {}", resultado)
    resultado 
}
