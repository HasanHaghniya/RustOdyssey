// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: u8,
}

impl Customer {
    fn new(age: u8) -> Result<Self, String> {
        if age >= 21 {
            Ok(Self { age })
        } else {
            Err("You can not purchase".to_owned())
        }
    }
}

fn main() {
    let anna = Customer::new(19);
    let ashley = Customer::new(21);

    match anna {
        Ok(anna) => println!("You are an adult! with {} years old", anna.age),
        Err(e) => println!("{e}"),
    }

    match ashley {
        Ok(ashley) => println!("You are an adult {} years old", ashley.age),
        Err(e) => println!("{e}"),
    }
}
