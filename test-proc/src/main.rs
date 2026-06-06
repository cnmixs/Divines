use std::process::Command;

fn main() {
    // Test 1: Command::status()
    println!("=== Test 1: Command::status() ===");
    match Command::new("cmd").arg("/c").arg("echo hello").status() {
        Ok(status) => println!("SUCCESS: {:?}", status),
        Err(e) => println!("ERROR: {:?} kind={:?}", e, e.kind()),
    }

    // Test 2: Command::output() with cmd
    println!("=== Test 2: Command::output() with cmd ===");
    match Command::new("cmd").arg("/c").arg("echo hello").output() {
        Ok(output) => println!("SUCCESS: {}", String::from_utf8_lossy(&output.stdout)),
        Err(e) => println!("ERROR: {:?} kind={:?}", e, e.kind()),
    }

    // Test 3: Command::spawn() and wait
    println!("=== Test 3: Command::spawn() ===");
    match Command::new("cmd").arg("/c").arg("echo hello").spawn() {
        Ok(mut child) => {
            match child.wait() {
                Ok(status) => println!("SUCCESS: {:?}", status),
                Err(e) => println!("wait ERROR: {:?}", e),
            }
        }
        Err(e) => println!("spawn ERROR: {:?} kind={:?}", e, e.kind()),
    }
}