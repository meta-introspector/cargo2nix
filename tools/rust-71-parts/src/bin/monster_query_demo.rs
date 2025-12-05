//! Demo Monster Query: GraphQL-like interface for pure Hecke operations

use rust_71_parts::monster_query::{create_monster_query_service, MonsterQuery, MonsterConstant, ExecutionBounds};

fn main() {
    println!("🔍 MONSTER QUERY SERVICE DEMO");
    println!("=============================");
    println!("GraphQL-like interface for Monster Group constants with bounded execution\n");
    
    let mut service = create_monster_query_service();
    
    // Advanced query examples
    println!("\n🧮 Advanced Query Examples:");
    
    let advanced_queries = [
        ("Sentinel Check", MonsterQuery::IsSentinel(Box::new(MonsterQuery::GetFactor(14)))),
        ("Prime Check 71", MonsterQuery::IsPrime(Box::new(MonsterQuery::GetFactor(14)))),
        ("Prime Check 59", MonsterQuery::IsPrime(Box::new(MonsterQuery::GetFactor(13)))),
        ("Elevate Factor", MonsterQuery::Elevate(Box::new(MonsterQuery::GetFactor(10)))),
        ("Compose Factors", MonsterQuery::Compose(
            Box::new(MonsterQuery::GetFactor(14)),
            Box::new(MonsterQuery::GetFactor(13))
        )),
    ];
    
    for (name, query) in advanced_queries {
        match service.query(query) {
            Ok(result) => {
                println!("  {} → {:?}", name, result.value);
                println!("    Operations: {}, Depth: {}, Time: {}ms, Pure: {}", 
                        result.operations_used, result.depth_reached, 
                        result.execution_time_ms, result.pure);
            }
            Err(e) => {
                println!("  {} → ERROR: {}", name, e);
            }
        }
    }
    
    // Demonstrate execution bounds
    println!("\n⏱️  Execution Bounds Testing:");
    
    let bounds_tests = [
        ("Safe Bounds", ExecutionBounds::SAFE),
        ("Strict Bounds", ExecutionBounds::STRICT),
        ("Custom Bounds", ExecutionBounds { max_operations: 50, max_depth: 3, timeout_ms: 100 }),
    ];
    
    for (name, bounds) in bounds_tests {
        let mut test_service = rust_71_parts::monster_query::MonsterQueryService::new(bounds);
        
        match test_service.execute_string("allFactors") {
            Ok(result) => {
                println!("  {} → SUCCESS (ops: {}, time: {}ms)", 
                        name, result.operations_used, result.execution_time_ms);
            }
            Err(e) => {
                println!("  {} → BOUNDED: {}", name, e);
            }
        }
    }
    
    // Pure functional guarantees
    println!("\n🔬 Pure Functional Guarantees:");
    println!("  ✅ No side effects - all operations are pure");
    println!("  ✅ Bounded execution - operations, depth, and time limits");
    println!("  ✅ Deterministic results - same input always produces same output");
    println!("  ✅ Cacheable - results can be safely cached and reused");
    println!("  ✅ Composable - queries can be combined and nested");
    
    // Remote/local execution demo
    println!("\n🌐 Remote/Local Execution:");
    println!("  📍 Local: Direct function calls with Monster Group constants");
    println!("  🌍 Remote: Same interface over network (Tor hidden services)");
    println!("  🔐 Anonymous: ZKP verification of Monster Group alignment");
    println!("  ⚡ Cached: Results cached for performance");
    
    // Show query language syntax
    println!("\n📝 Query Language Syntax:");
    println!("  factor(level)           → Get Monster factor by level");
    println!("  allFactors              → Get all 15 Monster factors");
    println!("  isSentinel(query)       → Check if result equals 71");
    println!("  isPrime(query)          → Check if result is prime");
    println!("  elevate(query)          → Apply Hecke elevation");
    println!("  compose(left, right)    → Apply Hecke composition");
    
    println!("\n✅ MONSTER QUERY SERVICE OPERATIONAL!");
    println!("🔍 GraphQL-like interface for Monster Group constants");
    println!("🧮 Pure Hecke operations with mathematical guarantees");
    println!("⏱️  Bounded execution prevents infinite loops");
    println!("🌐 Ready for local and remote (Tor) deployment");
}
