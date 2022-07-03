use std::{
    io::{
        self,
        Write,
    },
    process,
};
fn main(){
    // Obtener base
    println!("Ingrese la base del triangulo");
    io::stdout().flush().unwrap();
    let mut put = String::new();
    io::stdin().read_line(&mut put).unwrap();
    let base = put.trim().parse::<i32>().unwrap_or_else(|_| {
        drop(put);
        process::exit(1)
    });

    // Obtener altura
    println!("Ingrese la altura del triangulo");
    io::stdout().flush().unwrap();
    put = String::new();
    io::stdin().read_line(&mut put).unwrap();
    let alt = put.trim().parse::<i32>().unwrap_or_else(|_| {
        drop(put);
        process::exit(1);
    });
    // Area de triangulo
    let area = (base * alt) / 2;
    println!("El area del triangulo es: {}", area);
    
}