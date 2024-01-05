use std::io::{self, Write};

pub mod mutex;
pub mod region_critica;
pub mod dormir_despertar;

fn main() {
    let mut opt: i8 = 0;

    while opt != 5 {
        println!("---------------------------");
        println!("SIMULADOR");
        println!("---------------------------");
        println!("Elija un a opción: ");
        println!("1: Regiones criticas");
        println!("2: Exclusion mutua");
        println!("3: Dormir y despertar");
        println!("4: ");
        println!("5: Salir");
        println!("---------------------------");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Error no se reconoce lo escrito...");

        match input.trim().parse::<i8>() {
            Ok(value) => {
                opt = value;

                match opt {
                    1 => {
                        println!("---------------------------");
                        point_one();
                    }

                    2 => {
                        println!("---------------------------");
                        point_two();
                    }

                    3 => {
                        println!("---------------------------");
                        point_three();
                    }

                    _ => {
                        if opt == 5 {
                            println!("Gracias");
                        } else {
                            println!("Opción no valida");
                        }
                    }
                }
            }
            Err(error) => {
                println!("No se reconoce el digito ingresado: {}", error);
            }
        }
    }
}

fn point_one() {
    let hilo_type_one = obtener_numero("Ingrese cuántos hilos de tipo 1 desea");
    let hilo_type_two = obtener_numero("Ingrese cuántos hilos de tipo 2 desea");

    if let (Some(value1), Some(value2)) = (hilo_type_one, hilo_type_two) {
        println!("---------------------------");
        region_critica::iniciar(value1, value2);
    }
}

fn point_two() {
    let hilo_type_one = obtener_numero("Ingrese cuántos hilos de tipo 1 desea");
    let hilo_type_two = obtener_numero("Ingrese cuántos hilos de tipo 2 desea");

    if let (Some(value1), Some(value2)) = (hilo_type_one, hilo_type_two) {
        println!("---------------------------");
        mutex::iniciar(value1, value2);
    }
}

fn point_three(){
    let tiempo_dormir = obtener_numero("Digite cuantos segundos quiere que duerma el hilo");

    if let Some(sleep) = tiempo_dormir {
        dormir_despertar::iniciar(sleep);
    }
}

fn obtener_numero(mensaje: &str) -> Option<i32> {
    let mut input = String::new();

    print!("{}: ", mensaje);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok()?;

    input.trim().parse().ok()
}
