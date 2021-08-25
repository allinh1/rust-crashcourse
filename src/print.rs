pub fn run() {
    // Print to console
    println!("Hello print.rs file");

    // Basic Formatting
    println!("{} is from {}", "Allen", "Sydney");

    // Positional Arguments
    println!(
        "{1} is from {0} and likes to {2}",
        "Allen", "Sydney", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    );
    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
