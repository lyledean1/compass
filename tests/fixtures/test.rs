// Test Rust file with intentional issues for testing

fn main() {
    let result = risky_operation();
    result.unwrap(); // Should trigger unwrap rule
}

fn risky_operation() -> Result<i32, String> {
    Ok(42)
}

// TODO: Fix this later
fn large_function() {
    let x = 1;
    let y = 2;
    let z = 3;
    if x > 0 {
        println!("x is positive");
        if y > 0 {
            println!("y is positive");
            if z > 0 {
                println!("deeply nested"); // Should trigger deep nesting
            }
        }
    }
}
