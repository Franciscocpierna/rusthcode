
pub struct Retangulo{
   altura: u32,
   largura: u32,
}

impl Retangulo{
  fn calcular_area(&self) -> u32{
     self.largura * self.altura
  }
  //construtor de retangulo  
  fn new(largura: u32, altura: u32) -> Self{
    Self{largura, altura}

  }
}

fn main() {
    let retangulo = Retangulo{
        largura: 10,
        altura: 20 
    };

    let retangulo2 = Retangulo{
        largura: 10,
        altura: 10 
    };
    // esse exemplo usando o construtor
    let retangulo3 = Retangulo::new(30,10);
    let area = retangulo.calcular_area();
    let area1 = retangulo2.calcular_area();
    let area2 = retangulo3.calcular_area();
    println!("Area do retângulo é {}", area);
    println!("Area do retângulo2 é {}", area1);
    println!("Area do retângulo2 é {}", area2);
}


