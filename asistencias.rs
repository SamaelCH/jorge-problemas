use std::io::Write;
fn porcentaje_de_faltas(
    asistencias_max: f32,
    asistencias: f32
    ) -> f32 {
        asistencias / asistencias_max * 100.0
}
fn main() {
    let asistencias_max = 34.0;
    let mut asistencias;
    println!("Ingrese el numero de faltas: ");

    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let faltas = input.trim().parse::<f32>().unwrap_or_else(|_|{
        drop(input);
        std::process::exit(1);
    });
    asistencias = asistencias_max - faltas;
    println!("{}", porcentaje_de_faltas(asistencias_max, asistencias));
}
