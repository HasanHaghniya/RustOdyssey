#[derive(Debug)]
enum ServicePriority {
    High,
    Standard,
}

trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

fn print_priorities<T: Priority + std::fmt::Debug>(priority: T) {
    println!(
        "priorities for {:?}: {:?}",
        priority,
        priority.get_priority()
    )
}

#[derive(Debug)]
struct ImportantGuest;
impl Priority for ImportantGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High
    }
}

#[derive(Debug)]
struct Guest;
impl Priority for Guest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}

fn main() {
    print_priorities(ImportantGuest);
    print_priorities(Guest);
}
