fn main() {
    let d: i32 = 5; 
    let resultado_incremento = incrementa(d);
    println!("o resultado original {} e o novo é {} ", d, resultado_incremento);
}

fn incrementa(mut a: i32)->i32{
    a+=1;
    a
}