use rand::Rng;
use std::collections::HashMap;

fn main() {
    // 1. Generar 10 000 000 números aleatorios entre 0-36 y 00 (representado como 37)
    let mut rng = rand::thread_rng();
    let mut numeros = Vec::new();
    
    for _ in 0..10000000 {
        let num = rng.gen_range(0..=37); // 0-36 + 00 (37)
        numeros.push(num);
    }

    // 2. Contar la frecuencia de cada número
    let mut frecuencia = HashMap::new();
    
    for &num in &numeros {
        *frecuencia.entry(num).or_insert(0) += 1;
    }

    // Encontrar el número menos frecuente
    let (num_menos_frecuente, conteo) = frecuencia.iter()
        .min_by_key(|&(_, count)| count)
        .unwrap();

    // Mostrar resultados
    println!("De 10 000 000 números generados:");
    
    // Convertir 37 a "00" para mostrar
    let num_str = if *num_menos_frecuente == 37 {
        "00".to_string()
    } else {
        num_menos_frecuente.to_string()
    };
    
    println!("El número menos frecuente fue: {} (apareció {} veces)", num_str, conteo);
}