// Store ANY data within a structure
// AKA "generic constraints"
// Useful when making data collections of your own

fn main() {
    let airline = Ticket { location: AirlineSeat::BusinessClass };
    let concert = Ticket { location: ConcertSeat::FrontRow };

    print_seat(airline);
    print_seat(concert);
}

trait Seat {
    fn show(&self);
}

struct Ticket<T: Seat> {
    location: T,
}

enum AirlineSeat {
    FirstClass,
    BusinessClass,
    Economy,
}
impl Seat for AirlineSeat {
    fn show(&self) {
        match self {
            Self::FirstClass => println!("This is a first class seat"),
            Self::BusinessClass => println!("This is a business class seat"),
            Self::Economy => println!("This is a economy seat"),
        }
    }
}

enum ConcertSeat {
    FrontRow,
    MidSection(u32),
    Back(u32),
}

impl Seat for ConcertSeat {
    fn show(&self) {
        match self {
            Self::FrontRow => println!("This is a front row seat"),
            Self::MidSection(value) => println!("This is a middle section seat | {value:?}"),
            Self::Back(value) => println!("This is a back row seat | {value:?}"),
        }
    }
}

fn print_seat<T: Seat>(place: Ticket<T>) {
    place.location.show();
}