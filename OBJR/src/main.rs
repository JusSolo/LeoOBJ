use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} archivo.obj", args[0]);
        std::process::exit(1);
    }

    let file = fs::File::open(&args[1])?;
    let reader = io::BufReader::new(file);

    let mut vertices = Vec::new();
    let mut normales = Vec::new();
    let mut texturas = Vec::new();
    let mut materiales = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let Some(tag) = parts.next() {
            match tag {
                "v" => {
                    if vertices.len() < 5 {
                        vertices.push(parts.collect::<Vec<&str>>().join(" "))
                    }
                }
                "vn" => {
                    if normales.len() < 5 {
                        normales.push(parts.collect::<Vec<&str>>().join(" "))
                    }
                }
                "vt" => {
                    if texturas.len() < 5 {
                        texturas.push(parts.collect::<Vec<&str>>().join(" "))
                    }
                }
                "f" => {
                    if materiales.len() < 5 {
                        materiales.push(parts.collect::<Vec<&str>>().join(" "))
                    }
                }
                _ => {}
            }
        }
        if vertices.len() >= 5
            && normales.len() >= 5
            && texturas.len() >= 5
            && materiales.len() >= 5
        {
            // Ya tenemos 5 de cada uno, no hace falta seguir leyendo
            break;
        }
    }

    println!("--- Primeros elementos del OBJ ---");
    println!("\nVertices (v):");
    for (i, v) in vertices.iter().enumerate() {
        println!("  {}: {}", i + 1, v);
    }

    println!("\nNormales (vn):");
    for (i, n) in normales.iter().enumerate() {
        println!("  {}: {}", i + 1, n);
    }

    println!("\nTexturas (vt):");
    for (i, t) in texturas.iter().enumerate() {
        println!("  {}: {}", i + 1, t);
    }
    println!("\nMateriales (f):");
    for (i, t) in materiales.iter().enumerate() {
        println!("  {}: {}", i + 1, t);
    }

    Ok(())
}
