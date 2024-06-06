trait Perimeter {
    fn calculate(&self) -> i32;
}

struct Square {
    width: i32,
}

impl Perimeter for Square {
    fn calculate(&self) -> i32 {
        self.width * 4
    }
}

struct Triangle {
    width_a: i32,
    width_b: i32,
    width_c: i32,
}
impl Perimeter for Triangle {
    fn calculate(&self) -> i32 {
        self.width_a + self.width_b + self.width_c
    }
}

fn print_perimeter(shape: impl Perimeter) {
    println!("shape perimeter is: {}", shape.calculate())
}

fn main() {
    print_perimeter(Square { width: 4 });
    print_perimeter(Triangle {
        width_a: 4,
        width_b: 3,
        width_c: 5,
    })
}
