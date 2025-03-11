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
   
let opciones = 2;
let conceptos =  vec!["Neto","IVA","Bruto"];
let definicion = vec!["El valor Neto es el monto base antes del IVA.","El IVA es el impuesto agregado al valor neto.", "El valor Bruto es el total después de aplicar el IVA."];

match opciones {
    1 => {
        let mut input = String::new();
        println!("Introduce el valor neto: ");
        let _ = io::stdin().read_line(&mut input);
        let neto: f64 = input.trim().parse().expect("Ingresa un número válido");
        
        let iva = 0.19;   
        let iva_p = calcular_iva(neto, iva);
        let bruto = neto + iva_p;
        
        println!("Neto: $ {}, IVA: $ {}, Bruto: $ {}", neto, iva_p, bruto);
    }
    2 => {
        for (concepto, definicion) in conceptos.iter().zip(definicion.iter()) {
            println!("- {:?}: {}", concepto, definicion);
        }
    }
    _ => {
        println!("Ingrese una opción válida");
    }
}

}
