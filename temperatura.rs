use std::{
    io::{
        self,
        Write,
    },
    process,
};

fn convert(farenheit: f64) -> f64 {
    5.0 * (farenheit - 32.0) / 9.0
}

fn main(){
    println!("Escriba la temperatura en grados Farenheit que desea convertir a grados Celsius: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let farenheit = input.trim().parse::<f64>().unwrap_or_else(|_|{
        drop(input);
        process::exit(1)
    });
    println!("{} grados Farenheit son {} grados Celsius", farenheit, convert(farenheit));

}