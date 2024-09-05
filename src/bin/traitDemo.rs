fn main() {
    hello(Person {});
    hello(Dog {});
}

trait Noise {
    fn make_noise(&self);
}

fn hello(noisy: impl Noise) {
    noisy.make_noise();
}

struct Person;
impl Noise for Person {
    fn make_noise(&self) {
        println!("Hello World");
    }
}

struct Dog;
impl Noise for Dog {
    fn make_noise(&self) {
        println!("Woof");
    }
}