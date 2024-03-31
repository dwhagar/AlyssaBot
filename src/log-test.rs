mod log; // Bring the 'log' module into scope

fn main() {
    // Test each verbosity level
    for verbosity_level in 0..=3 {
        println!("--- Testing verbosity level: {} ---", verbosity_level);

        let mut logger = log::Log::new(verbosity_level);

        // What should be seen at this level:
        match verbosity_level {
            0 => println!("Expect: Only essential messages"),
            1 => println!("Expect: Essential and error messages"),
            2 => println!("Expect: Essential, error, and warning messages"),
            3 => println!("Expect: All messages (essential, error, warning, debug)"),
            _ => println!("Unexpected verbosity level"),
        }

        // Log messages at each priority level
        logger.essential("Essential message (always shown)");
        logger.error("Error message");
        logger.warn("Warning message");
        logger.debug("Debug message");

        println!(""); // Add a separator between tests
    }
}