enum Fruta{
    Maca,
    Banana,
    Morango,
    Acai


}


fn main() {
    //println!("Hello, world!");
    enumeracao(Fruta::Morango);
    enumeracao(Fruta::Maca);
    enumeracao(Fruta::Banana);
    enumeracao(Fruta::Acai);
}

fn  enumeracao(fruta: Fruta ){
     match fruta{
        Fruta::Maca => println!("É uma maçã"),
        Fruta::Banana => println!("É uma banana"),
        Fruta::Morango => println!("É um morango"),
        Fruta::Acai => println!("É  Acai"),

     }
}