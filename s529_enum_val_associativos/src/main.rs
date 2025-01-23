enum Coordenada{
   DoisD(i32, i32),
   TresD(i32, i32, i32), 


}


fn main() {
    //println!("Hello, world!");
    let ponto2d = Coordenada::DoisD(5, 10);
    let ponto3d = Coordenada::TresD(3, 8, 15);

    match ponto2d{
        Coordenada::DoisD(x, y) => println!("coordenada 2d {}, {}", x,y),
        Coordenada::TresD(_, _,_) => println!("coordenad 3d"),
    }
    match ponto3d{
        Coordenada::DoisD(_, _) => println!("coordenada 2d"),
        Coordenada::TresD(x, y, z) => println!("coordenad 3d {}, {}, {}", x, y, z),
    }

}

