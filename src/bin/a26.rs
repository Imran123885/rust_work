fn main() {
    let truck = Vehicle::new( Truck {}, Green {});
    let car = Vehicle::new( Car {}, Red {});
}

trait Body {} // Marker traits
trait Color {}

struct Vehicle<T: Body, U: Color> {
    body: T,
    color: U
}
impl<T, U> Vehicle<T, U>
where 
    T: Body,
    U: Color,
{
    fn new(body: T, color: U) -> Self {
        Self {
            body,
            color,
        }
    }
}

struct Car;
impl Body for Car {}
struct Truck;
impl Body for Truck {}

struct Red;
impl Color for Red {}
struct Green;
impl Color for Green {}
