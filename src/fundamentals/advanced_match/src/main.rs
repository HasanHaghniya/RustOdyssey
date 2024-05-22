pub enum Ticket {
    Backstage(String, f64),
    Vip(String, f64),
    Standard(f64),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage("Vladimir".to_owned(), 50.0),
        Ticket::Vip("Anna".to_owned(), 40.0),
        Ticket::Standard(25.0),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(name, price) => {
                println!("Backstage ticket for {} and the price: {}", name, price)
            }
            Ticket::Vip(name, price) => {
                println!("Standard ticket for {} and the price: {}", name, price)
            }
            Ticket::Standard(price) => println!("Standard ticket and the price: {}", price),
        }
    }
}
