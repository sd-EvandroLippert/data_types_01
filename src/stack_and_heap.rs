#![allow(dead_code)]
use std::mem;

struct Point
{
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}
pub fn stack_and_heap(){
    let p1 = origin(); // Valor será armazenado no stack
    let p2 = Box::new(origin()); // endereço do heap será armazenado no stack

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));
    
    // Para salvar o valor de p2 inteiro no stack, usamos *
    let p3 = *p2;
    println!("p3 takes up {} bytes", mem::size_of_val(&p3));
}