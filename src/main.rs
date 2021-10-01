#[allow(dead_code)]
#[allow(unused_variables)]

mod stack_and_heap;

use std::mem;


fn main() {

    fundamental_data_types();
    operators();
    scope_and_shadowing();
    stack_and_heap::stack_and_heap();
}

fn fundamental_data_types(){
    // Data types
    

    // Integers
    // u = unsigned só maiores ou igual a 0
    // i = signed negativos e positivos
    // u8, u16, u32, u64, i8, i16, ...


    // Strings
    // char

    // Float
    // f32, f64
    // Todos eles são signed


    // Integers
    // u = unsigned, 8 = 8 bits, unsigned variables just accept positive integers 
    // 0 - 255
    let a: u8 = 255;
    println!("{}", a);

    // Criar uma variável mutável
    // Devemos adicionar o mut antes da variávle
    // i = signed; 8 = 8 bits, variáveis signed aceitam valores negativos e positivos
    // -128 - 127
    let mut b: i8 = -55;
    println!("b = {} before", b);
    b = 100;
    println!("b = {} after", b);
    
    // Verificar o espaço que um número ocupa na memória
    let mut c = 123456789;
    println!("c = {}, and it takes up {} bytes", c, mem::size_of_val(&c));

    c = 10;
    println!("c = {}, and it takes up {} bytes", c, mem::size_of_val(&c));

    // Char
    // Só uma letra delimitada por aspas simples
    let d: char = 'x';
    println!("{} is a char, its size = {} bytes", d, mem::size_of_val(&d));

    // Float
    let e: f32 = 2.1;
    println!("e = {}, its size = {} bytes", e, mem::size_of_val(&e));

    let f: f64 = 2.145687;
    println!("f = {}, its size = {} bytes", f, mem::size_of_val(&f));
}

fn operators(){
    // arithmetic
    // Em rust existem os mesmos operadores que python
    // + / * - %
    //
    // As diferenças estão a cargo das operações de potenciação

    let a = (2 + 5 * 3) / 7;
    let a_cubed = i32::pow(a, 3);
    println!("{} na terceira poitência é {}", a, a_cubed);


    //Bitwise operators
    // | = OR; & = AND; ^ = XOR; ! = NOR;
    let c = 1 | 2; // 01 OR 10 = 11 == 3
    println!("1|2 = {}", c);


    // Logical operators
    // <; >; <=; >=; ==  
    let three_less_than_five = 3 < 5;
    println!("3 is less than five: {}", three_less_than_five);
}

fn scope_and_shadowing() {
    let a = 123;

    {
        //Novo scope -> parte delimitada pelos {}
        let b = 456;
        // b só existirá dentro desse scope, 
        // após esse trecho de código ser executado, b será apagado da memória
        println!("inside this scope b is equal {}", b);
        
        // Entretanto, a existe dentro desse scope, pois ela é do scope "pai"
        println!("a = {}", a);

        // Porém é possível escondê-la redeclarando uma variável a dentro desse scope
        let a = 7;
        println!("new a = {}", a)

    }
    // Fora do antigo scope a voltará a ter o valor inicial, declarado no inicio da função
    println!("a in the parent scope = {}", a)
}