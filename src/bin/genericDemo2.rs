fn main() {
    let immu = Passenger {};
    let jaani = Pilot {};
    let hai = Cargo {};

    process_item(immu);
    process_item(jaani);
    process_item(hai);
}

trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("Checked in as Pilot");
    }
    fn process(&self) {
        println!("Pilot has entered the cockpit");
    }
}
struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("Checked in as Passenger");
    }
    fn process(&self) {
        println!("Passenger has boarded the plane");
    }
}
struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("cargo checked in");
    }
    fn process(&self) {
        println!("Cargo moved to storage");
    }
}

fn process_item<T: CheckIn>(item: T) {
    item.check_in();
    item.process();
}