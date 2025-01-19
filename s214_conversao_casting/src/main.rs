fn type_of<T>(_: T)-> &'static str{
    std::any::type_name::<T>()
}



fn main() {
    let inteiro: i32 = 10;
    let int_para_float: f32=  inteiro as f32;
    let float = 2.5;
    let float_to_int = float as i32;
    
    println!("inteiro {}, {}", inteiro, type_of(inteiro));

    println!("inteiro {}, {}", inteiro,type_of(float));
    println!("int_float {}, {}", int_para_float,type_of(inteiro) );
    
    println!("float {}, {}", float, type_of(float_to_int));
    println!("float_int {}, {}", float_to_int, type_of(int_para_float));

    let int_to_string: String = inteiro.to_string(); 
    println!("inteiro {}, {}", inteiro,type_of(&int_to_string));

    let string: &str = "42";
    let string_to_int = string.parse::<i64>().unwrap(); 
    println!("inteiro {}, string_to_int {}", inteiro,type_of(&string_to_int));

}
