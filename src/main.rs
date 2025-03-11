use std::io;

fn calcular_iva(neto: f64, iva: f64) -> f64 {
    if neto < 0.0 {
        println!("El valor no puede ser negativo");
        std::process::exit(0);
    } else if neto == 0.0 {
        println!("El valor no puede ser cero");
        std::process::exit(0);
    } else {
        return neto * iva;  
    }
}

fn main() {
    let mut input = String::new();
    println!("Introduce el valor neto: ");

    let _ = io::stdin().read_line(&mut input);
    let neto: f64 = input.trim().parse().expect("Ingresa un número válido");
   
    let iva = 0.19;   
    let iva_p = calcular_iva(neto, iva);
    let bruto = neto + iva_p;
    
    println!("Neto: $ {}, IVA: $ {}, Bruto: $ {}", neto, iva_p, bruto);
    
let conceptos = vec!["Neto, IVA, Bruto"];

for concepto in conceptos.clone().into_iter() {
    match concepto {
        "Neto" => println!("Existen distintos conceptos en tributaria"),
        _ => println!("Por ejemplo {}", concepto)
    }
}

println!("Conceptos: {:?}", conceptos);
}   