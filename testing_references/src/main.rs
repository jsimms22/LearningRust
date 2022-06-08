fn main() {
    let s: i32 = 5;

    println!("printing 's': {}",s);
    println!("printing '&s': {}",&s);

    let s_ptr = &s;

    println!("printing 's': {}", s_ptr);
    println!("printing '&s': {}", &s_ptr);
}
