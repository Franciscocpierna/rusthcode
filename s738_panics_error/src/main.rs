
fn main() {
   // println!("Hello, world!");
  //let valor1 = funcao_com_panic(0);
   //println!("funcao_com_panic {}", valor1);
   let resultado = std::panic::catch_unwind(||{
      let a = funcao_com_panic(0);
      Ok::<i32, &str>(a)
   });
   match resultado{
     Ok(valor) =>  {
        println!("Resultado usando Result {}", valor.unwrap());
     },
     Err(_) => {
        println!("A função resultou um panic ");
     }
   }
}


fn funcao_com_panic(valor: i32) -> i32{
    if valor == 0{
        panic!("valor não pode ser zero");
    } 
    valor
      

}