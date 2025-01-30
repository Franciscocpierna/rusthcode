
pub fn main() {
   let a: f64=10.0;
   let resultado = dividir(100 as f64, a);
   match resultado{
    Some(resultado1) => {
        println!("O resultado da divisão é {}", resultado1)
    }
    None =>{
        println!("Não foi possível fazer a divisão");
    } 
   }

}

fn dividir(dividendo: f64, divisor: f64) -> Option<f64>{
     if divisor == 0.0{
       None
     }else{
       Some(dividendo / divisor) 
     }
}