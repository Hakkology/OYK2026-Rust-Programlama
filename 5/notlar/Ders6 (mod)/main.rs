mod modules;

use modules::product::{Category, Intersect, Product};

fn main() {
    let product1 = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
    let product2 = Product::new(2, String::from("T-Shirt"), 20.0, Category::Clothing);
    let product3 = Product::new(3, String::from("Book"), 10.0, Category::Books);

    let set1: Vec<&Product> = vec![&product1, &product2];
    let set2: Vec<&Product> = vec![&product2, &product3];
    let intersection = set1.intersect(set2);
    println!("The intersection is: {:?}", intersection);
}
