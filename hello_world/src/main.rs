fn main(){
    let mut x: i32 = 4;
    x = sumar(x);
    println!("El numero es {}", x);
}
fn sumar(x: i32) -> !{
    panic!("Esto es");
    x
}
