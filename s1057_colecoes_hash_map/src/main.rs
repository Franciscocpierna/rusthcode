use std::collections::HashMap;
fn main() {

    hashmap();
    
}

fn hashmap(){
    let mut mapa: HashMap<String, &str> = HashMap::new();
    mapa.insert("nome".to_string(), "Hcode" );
    mapa.insert("url".to_string(), "https//hcode.com.br");

    match mapa.get(&"url".to_string()){
        Some(vl) => {
            println!("{}", vl);
        }    
        None => {
            println!("valor invalido");
        }    
    }
    
    println!("{:?}", mapa);
    
}