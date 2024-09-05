fn main() {
    run_trait(Vase {});
    run_trait(Cat {});
    run_trait(Hiroshima {});
}

trait Fall {
    fn hit_ground(&self);
}

struct Vase;
impl Fall for Vase {
    fn hit_ground(&self) {
        println!("Loud noise");
    }
}

struct Cat;
impl Fall for Cat {
    fn hit_ground(&self) {
        println!("It survived");
    }
}

struct Hiroshima;
impl Fall for Hiroshima {
    fn hit_ground(&self) {
        println!("Very Loud Noise");
    }
}

fn run_trait(message: impl Fall) {
    message.hit_ground();
}