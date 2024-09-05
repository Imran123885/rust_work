fn main() {
    print_adv_memory_concepts();

    let data = Entry { id: 3 };
    let data_ptr: Box<Entry> = Box::new(data);
    let data_stack = *data_ptr;
}

fn print_adv_memory_concepts() {
    println!("Stack: \n");
    println!("Limited Space | Data Spaced Sequentially");
    println!("All VARIABLES stored on stack | NOT all DATA");
    println!("Very fast to work with | offsets to access");
    println!("OFFSETTING: Adding or subtracting to get to another memory address");
    println!("Heap: \n");
    println!("Data placed Algorithmically | Slower than stack, addresses need to be CALCULATED");
    println!("Unlimited space | (RAM/disk limits do apply)");
    println!("Use pointers | pointers: fixed size | usize data type");
    println!("Vectors and HashMaps are stored on the heap");
    println!(" - All dynamically sized collections are stored on the heap");
}

struct Entry {
    id: i32,
}