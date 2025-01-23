fn main() {
    //println!("Hello, world!");
    deu_match()
}


fn deu_match(){
   let estacao_atual = "primavera";
    match estacao_atual{
        "primavera" => {
            println!("é primavera");
        } 
        "verão" => {
            println!("é verão");
        } 
        "outono" => {
            println!("é outono");
        } 
        "inverno" => {
            println!("é inverno");
        } 

       _=> {
             println!("nenhuma das opções");
        }
    }
}