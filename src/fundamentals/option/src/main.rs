struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let anna = Student {
        name: "Anna".to_owned(),
        locker: Some(20),
    };

    match anna.locker {
        Some(locker) => println!("Student locker assignment: {}", locker),
        None => println!("No locker assigment"),
    }
}
