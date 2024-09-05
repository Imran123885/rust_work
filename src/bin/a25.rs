fn main() {
    print_perimeter(Square { side_length: 50.0 });
    print_perimeter(Triangle { side_length_1: 50.0, side_length_2: 43.0, side_length_3: 20.5 });
}

trait Perimiter {
    fn calculate_perimeter(&self);
}

struct Square {
    side_length: f64,
}

struct Triangle {
    side_length_1: f64,
    side_length_2: f64,
    side_length_3: f64,
}

impl Perimiter for Square {
    fn calculate_perimeter(&self) {
        let perimeter = self.side_length * 4.0;
        println!("Perimeter of square: {perimeter:?}");
    }
}

impl Perimiter for Triangle {
    fn calculate_perimeter(&self) {
        let perimeter = self.side_length_1 + self.side_length_2 + self.side_length_3;
        println!("Perimeter of square: {perimeter:?}");
    }
}

fn print_perimeter(shape: impl Perimiter) {
    shape.calculate_perimeter();
}