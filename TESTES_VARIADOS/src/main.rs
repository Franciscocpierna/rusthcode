use std::env;
fn funcionar(){
  let key = "HOME";
  match env::var_os(key) {
    Some(val) => println!("{key}: {val:?}"),
    None => println!("{key} is not defined in the environment.")
  }
}

fn main() {
    funcionar();
}