use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();

    stock.insert("Chairs", 5);
    stock.insert("Beds", 3);
    stock.insert("Tables", 2);
    stock.insert("Couches", 0);

    let mut total_items = 0;

    for (item, quantity) in stock.iter() {
        total_items = total_items + quantity;

        if quantity == &0 {
            println!("{}: out of stock", item);
        } else {
            println!("we have {} {}", quantity, item);
        }
    }

    match total_items {
        0 => println!("total count: out of stock"),
        (count) => println!("total item count: {}", count),
    }
}
