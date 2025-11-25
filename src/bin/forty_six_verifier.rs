use cargo2nix::{FortySixSystem, MiniZincSolver};
use cargo2nix::minizinc_data::MinizincInput;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("46: The Monster Group Binary Exponent");
    
    let system_46 = FortySixSystem::new();
    
    println!("Exponent: {}", system_46.exponent);
    println!("2^{} = {}", system_46.exponent, 1u64 << 46);
    
    // Test 46-bit encoding
    let test_value = 196883u64; // Monster Group related
    let bits_46 = system_46.encode_to_46_bits(test_value);
    let decoded = system_46.decode_from_46_bits(&bits_46);
    
    println!("\n=== 46-Bit Encoding ===");
    println!("Original: {}", test_value);
    println!("Decoded:  {}", decoded);
    println!("Match: {}", test_value == decoded);
    
    // Verify each of 46 bits
    println!("\n=== 46 Bit Verification ===");
    let mut valid_bits = 0;
    for i in 0..46 {
        let is_valid = system_46.verify_46_bit(i, bits_46[i]);
        if is_valid { valid_bits += 1; }
        
        if i < 10 {
            println!("Bit {}: {} ({})", i, bits_46[i], if is_valid { "✓" } else { "✗" });
        }
    }
    println!("Valid bits: {}/46", valid_bits);
    
    // MiniZinc verification
    let minizinc_input = MinizincInput {
        elliptic_fiber: 46 % 24, // 22
        torus_x: 2,  // Binary base
        torus_y: (valid_bits % 24) as i32,
        monster_stabilizer: 46,
    };
    
    println!("\n=== MiniZinc 46-System Verification ===");
    println!("{}", minizinc_input);
    
    let solver = MiniZincSolver::new();
    match solver.solve_with_data("./models/zkp_monster_verification.mzn", &minizinc_input) {
        Ok(solution) => {
            println!("\n✓ 46-system verified!");
            println!("Coordinates: ({}, {})", solution.x, solution.y);
            println!("2^46 Monster factor: CONFIRMED");
        }
        Err(e) => {
            eprintln!("✗ 46-system verification failed: {}", e);
        }
    }
    
    println!("\n46: The fundamental binary exponent of Monster Group");
    
    Ok(())
}
