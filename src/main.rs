use oop_the_rust_way::encapsulation::AverageCollection;
use oop_the_rust_way::inheritance::*;
use oop_the_rust_way::polymorphism::{Button, Screen, SelectBox};

// use oop_the_rust_way::polymorphism::

fn main() {
    /* Rust is object-oriented: structs and enums have data,
     * and impl blocks provide methods on structs and enums.
     * Even though structs and enums with methods aren’t called
     * objects, they provide the same functionality, according
     * to the Gang of Four’s definition of objects.
     * */

    let mut collection = AverageCollection::new(vec![]);
    collection.add(2);
    collection.add(3);
    collection.add(10);
    println!("{collection:?}");

    let calculator = CasioCalculator::new(4, 2);
    let add = CasioCalculator::add(calculator);

    println!("subtract: {}", calculator.subtract());
    println!("add: {}", add);

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
