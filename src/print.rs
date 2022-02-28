pub fn run() {
    // Print to console
    println!("Hello from the print rs file");

    println!("{0} is from {1}, and {0} likes to {2}", "Nils", "New Jersey", "code");

    println!("{name} likes to play {activity}", name = "John", activity = "Juggling");

    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    println!("{:?}", (12, true, "hello"));

    println!("10 + 10 = {}", 10 + 10);

    println!("{}", "Jersey");
}
