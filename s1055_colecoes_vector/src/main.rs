fn main() {
    // Array memoria stack tamanho fixo
    let lista: [u8;5] = [1, 2, 3, 4, 5];
    println!("valor na posicao {:?}", lista[0]);
    // vec memria heap
    let mut numeros: Vec<u8> = Vec::new();
    numeros.push(1);
    numeros.push(2);
    numeros.push(3);
    numeros.push(4);
    numeros.push(5);
    println!("valor na posicao {:?}", numeros[0]);
    for n in numeros{
        println!("valor na posicao {:?}", n);
    }

}
