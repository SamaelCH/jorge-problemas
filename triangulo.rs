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
    let mut in_base = String::new();
    io::stdin().read_line(&mut in_base).unwrap();
    let base = in_base.trim().parse::<i32>().unwrap_or_else(|_| {
        drop(in_base);
        process::exit(1)
    });

    // Obtener altura
    println!("Ingrese la altura del triangulo");
    io::stdout().flush().unwrap();
    let mut in_alt = String::new();
    io::stdin().read_line(&mut in_alt).unwrap();
    let alt = in_alt.trim().parse::<i32>().unwrap_or_else(|_| {
        drop(in_alt);
        process::exit(1);
    });
    // Area de triangulo
    let area = (base * alt) / 2;
    println!("El area del triangulo es: {}", area);
    
}