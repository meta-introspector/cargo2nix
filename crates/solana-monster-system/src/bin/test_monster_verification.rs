use solana_monster_system::monster_ffi::{
    verify_hecke_eigenvalue, verify_modular_constraint, verify_monster_element,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Monster Group FFI verification...");

    // Test Monster Group order constraint
    println!("Testing Monster Group elements:");
    assert!(verify_monster_element(0));
    assert!(verify_monster_element(196882));
    assert!(!verify_monster_element(196883));
    println!("✓ Monster Group order verification passed");

    // Test Hecke eigenvalues
    println!("Testing Hecke eigenvalues:");
    assert!(verify_hecke_eigenvalue(196883));
    assert!(verify_hecke_eigenvalue(-5472));
    assert!(!verify_hecke_eigenvalue(0));
    assert!(!verify_hecke_eigenvalue(1));
    println!("✓ Hecke eigenvalue verification passed");

    // Test modular constraint (Ramanujan τ)
    println!("Testing modular constraints:");
    let valid_elements = vec![24, 48, 72]; // sum = 144, 144 % 24 = 0
    assert!(verify_modular_constraint(&valid_elements));

    let invalid_elements = vec![1, 2, 3]; // sum = 6, 6 % 24 != 0
    assert!(!verify_modular_constraint(&invalid_elements));
    println!("✓ Modular constraint verification passed");

    println!("All Monster Group FFI tests passed!");
    Ok(())
}
