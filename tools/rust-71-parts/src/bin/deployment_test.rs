//! Test complete deployment pipeline: Commit → Build → Run → Proof → Commit

use rust_71_parts::service_deployment::test_deployment_pipeline;

fn main() {
    println!("🧪 MONSTER GROUP SERVICE DEPLOYMENT TEST");
    println!("========================================");
    println!("Testing complete cycle: Commit → Build → Run → Proof → Commit\n");
    
    match test_deployment_pipeline() {
        Ok(()) => {
            println!("\n🎉 ALL DEPLOYMENT TESTS PASSED!");
            println!("✅ Complete pipeline operational");
            println!("🔐 ZKP proofs generated and verified");
            println!("📝 Git commits with service proofs");
            println!("🚀 Services running with Monster Group alignment");
            
            std::process::exit(0);
        }
        Err(e) => {
            println!("\n❌ DEPLOYMENT TEST FAILED: {}", e);
            println!("🔧 Check git configuration and permissions");
            
            std::process::exit(1);
        }
    }
}
