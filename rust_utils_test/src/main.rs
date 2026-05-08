mod utils;
use utils::cache::test_cache;
use utils::io::test_io;
use utils::ta::sma;

fn main() {
    println!("Testing Utils module imports...");
    
    // Test basic functions
    let rounded = utils::x_round(3.1415926, 4);
    println!("x_round(3.1415926, 4) = {}", rounded);
    
    // Test cache functionality
    match test_cache() {
        Ok(_) => println!("Cache test passed!"),
        Err(e) => println!("Cache test failed: {}", e),
    }
    
    // Test IO functionality
    match test_io() {
        Ok(_) => println!("IO test passed!"),
        Err(e) => println!("IO test failed: {}", e),
    }
    
    // Test TA functionality
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let sma_result = sma(&data, 3);
    println!("SMA result: {:?}", sma_result);
    
    println!("All tests completed successfully!");
}