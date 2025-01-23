fn main() {
    //println!("Hello, world!");
    loop_infinito();
}


fn loop_infinito(){
   let  mut contador = 0;
   
   loop{
    println!("Contador igual {}", contador);
    contador +=1;
    if contador == 5{
        break;
    }
   } 

}
