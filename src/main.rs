#[derive(Debug)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    // Tipos básicos
    let entero: i32 = 42;
    let flotante: f64 = 3.14;
    let booleano: bool = true;
    let caracter: char = 'a';
    let tupla: (i32, f64, char) = (1, 2.5, 'a');
    let array: [i32; 3] = [1, 2, 3];

    println!("Hello, world!");
    println!("valor es: {}", entero);
    println!("flotante: {}", flotante);
    println!("booleano: {}", booleano);
    println!("caracter: {}", caracter);
    println!("La tupla es: {:?}", tupla);
    println!("Tupla por elementos: ({}, {}, {})", tupla.0, tupla.1, tupla.2);
    println!("array: {:?}", array);

    // Mutabilidad
    let mut x = 5;
    x = 10;
    println!("x (mut): {}", x);

    let mut y = 10;
    y = 20;
    println!("y (mut): {}", y);

    // Shadowing (nuevo binding con mismo nombre)
    let x = x + 1; // x = 11
    println!("x tras shadowing numérico: {}", x);
    let x = "Hola"; // ahora x es &str
    println!("x tras shadowing a texto: {}", x);

    // Especificación de tipo
    let z: i8 = 42;
    println!("z = {}", z);

    // Constantes
    const PI: f64 = 3.1416;
    println!("PI = {}", PI);

    // Struct + Debug y acceso a campos (evita warning de dead_code)
    let p = Punto { x: 10, y: 20 };
    println!("Punto: {:?}", p);
    println!("Punto.x = {}, Punto.y = {}", p.x, p.y);
}

