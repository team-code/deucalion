fn main() {
    // Fake-main technique. This allows the main() function to return an int (i32) status code.
    std::process::exit(fake_main());
}

fn fake_main() -> i32 {
    println!("Hello from fakemain.");
    return 0;
}
