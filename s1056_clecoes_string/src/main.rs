fn main() {
    string();
}

fn string(){
  //let texto = String::new();
  let mut texto = String::from("Hcode");
  texto.push_str(" treinamentos");
  texto.push('c');
  texto.split_off(3);
  println!("Texto = {}", texto);
}