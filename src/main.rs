use oop_the_rust_way::AverageCollection;

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
    println!("{collection:?}")
}
