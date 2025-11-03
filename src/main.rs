use std::io::{self, Write};


#[derive(Debug, Clone, Copy)]
enum Op {
    Suma,
    Resta,
    Mult,
    Div,
}

//Parsea una línea del tipo "a op b" y devuelve (a, op, b).
fn parse_expresion(linea: &str) -> Result<(f64, Op, f64), String> {
    // Normalizamos espacios
    let linea = linea.trim();

    let tokens: Vec<&str> = linea.split_whitespace().collect();

    let (a_str, op_str, b_str) = if tokens.len() == 3 {
        (tokens[0], tokens[1], tokens[2])
    } else {
        // Si no hay espacios, buscamos el operador en la cadena
        // en el orden: + - * / (cuidando el signo negativo del primer número)
        let mut idx_op: Option<usize> = None;
        for (i, ch) in linea.char_indices() {
            if (ch == '+' || ch == '*' || ch == '/') ||
               (ch == '-' && i != 0) // '-' como operador, no como signo inicial
            {
                idx_op = Some(i);
                break;
            }
        }
        if let Some(i) = idx_op {
            let (lhs, rest) = linea.split_at(i);
            let (op, rhs) = rest.split_at(1);
            (lhs.trim(), op.trim(), rhs.trim())
        } else {
            return Err("No se pudo detectar un operador. Usa formato como: 3 + 4".into());
        }
    };

    let a: f64 = a_str.parse().map_err(|_| format!("Número inválido: '{a_str}'"))?;
    let b: f64 = b_str.parse().map_err(|_| format!("Número inválido: '{b_str}'"))?;

    let op = match op_str {
        "+" => Op::Suma,
        "-" => Op::Resta,
        "*" | "x" | "X" => Op::Mult,
        "/" | "÷" => Op::Div,
        _ => return Err(format!("Operador no soportado: '{op_str}'. Usa + - * /")),
    };

    Ok((a, op, b))
}

//Aplica la operación y devuelve el resultado o un error (p. ej., división por cero).
fn calcular(a: f64, op: Op, b: f64) -> Result<f64, String> {
    match op {
        Op::Suma => Ok(a + b),
        Op::Resta => Ok(a - b),
        Op::Mult => Ok(a * b),
        Op::Div => {
            if b == 0.0 {
                Err("Error: división por cero.".into())
            } else {
                Ok(a / b)
            }
        }
    }
}

fn main() {
    println!("Calculadora Rust");
    println!("Operadores: +  -  *  /   |  Escribe 'salir' para terminar.\n");

    loop {
        print!("> ");
        
        io::stdout().flush().unwrap();

        //Leemos la línea
        let mut linea = String::new();
        match io::stdin().read_line(&mut linea) {
            Ok(0) => {
                //EOF (Ctrl+D / Ctrl+Z), salimos
                println!("\nAdiós");
                break;
            }
            Ok(_) => {
                let entrada = linea.trim();
                if entrada.eq_ignore_ascii_case("salir") {
                    println!("Adiós");
                    break;
                }
                if entrada.is_empty() {
                    continue;
                }

                match parse_expresion(entrada) {
                    Ok((a, op, b)) => match calcular(a, op, b) {
                        Ok(res) => println!("= {}", res),
                        Err(e) => eprintln!("{e}"),
                    },
                    Err(e) => eprintln!("Entrada inválida: {e}"),
                }
            }
            Err(e) => {
                eprintln!("Error leyendo entrada: {e}");
                break;
            }
        }
    }
}
