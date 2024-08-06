use std::io;

fn main() {
println!(" Bienvenido a la calculadora simple en Rust");

    loop{
        // solicitar al usuario que ingrese dos numeros
        println!("Por favor, ingresa el primer numero");
        let num1 = leer_numero();

        println!("Por favor, ingresa el segundo numero");
        let num2 = leer_numero();

        // Solicitar al usuario que elija la operacion a realizar
        println!("Selecciona una operacion a realizar (+(sumar), -(restar), *(Multiplicar), /(dividir)) o 'q'(quit) para salir:");
        let mut operacion = String::new();

        io::stdin().read_line(&mut operacion).expect("Error al leer la entrada");

        let operacion = operacion.trim();

        //Realizar la operacion seleccionada
        match operacion {
            "+" => println!("Resultado: {}", num1 + num2),
            "-" => println!("Resultado: {}", num1 - num2),
            "*" => println!("Resultado: {}", num1 * num2),
            "/" => {
                if num2 != 0.0{
                    println!("Resultado: {}",num1/num2 )
                }else { println!("Error: Division por cero no permitida") }
            },
            "q"=> {
                println!("Saliendo de la calculadora");
                break;
            },
            _=>{println!("Operacion no valida, por favor intenta de nuevo")}
        }
    }
}

//Funcion que realiza los calculos
fn leer_numero() -> f64 {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer la entrada");
    /*
    match entrada.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada no válida, por favor ingresa un número.");
            leer_numero()
        }
    }
    */
    // corregido
    entrada.trim().parse::<f64>().unwrap_or_else(|_| {
        println!("Entrada no válida, por favor ingresa un número.");
        leer_numero()
    })
}