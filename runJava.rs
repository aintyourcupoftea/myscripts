// run_java.rs
use std::env;
use std::process::Command;
use std::fs;

fn main() {
    // Check if a filename is provided
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <JavaFileName>", args[0]);
        std::process::exit(1);
    }

    // Get the filename without the extension
    let filename = &args[1];
    let class_name = filename.trim_end_matches(".java");

    // Compile the Java file
    let compile_status = Command::new("javac")
        .arg(filename)
        .status()
        .expect("Failed to execute javac");

    // Check if compilation was successful
    if compile_status.success() {
        // Run the compiled class
        let _ = Command::new("java")
            .arg(class_name)
            .status()
            .expect("Failed to execute java");

        // Delete the .class file
        let _ = fs::remove_file(format!("{}.class", class_name));
    } else {
        eprintln!("Compilation failed.");
    }
}