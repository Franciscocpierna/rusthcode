

fn main() -> (){
   nome_da_funcao();
   let a = com_return();
   let b = sem_return();
   println!("o valor de a com return {}", a);
   println!("o valor de b sem return  {}", b);  
}

fn nome_da_funcao(){
    println!("Hello, world!");
}

fn com_return()-> i32 {
    return 3;
}    

fn sem_return() -> i32{
     3
}    
