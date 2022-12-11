pub trait Calculator {
    fn new(a: u8, b: u8) -> Self;

    fn subtract(&self) -> u8;

    fn add(object: CasioCalculator) -> u8 {
        object.x + object.y
    }
}

/* Inheritance as a Type System and as Code Sharing
* Inheritance is a mechanism whereby an object can inherit
* elements from another object’s definition, thus gaining the
* parent object’s data and behavior without you having to
* define them again.

? If a language must have inheritance to be an object-oriented
? language, then Rust is not one. There is no way to define a struct
? that inherits the parent struct’s fields and method implementations
? without using a macro.
* */

#[derive(Debug, Clone, Copy)]
pub struct CasioCalculator {
    x: u8,
    y: u8,
}

impl Calculator for CasioCalculator {
    fn new(a: u8, b: u8) -> Self {
        CasioCalculator { x: a, y: b }
    }

    fn subtract(&self) -> u8 {
        self.x - self.y
    }
}
