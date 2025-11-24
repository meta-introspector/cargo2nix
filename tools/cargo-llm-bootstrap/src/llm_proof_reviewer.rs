/// LLM Proof Reviewer: Enumerate proof steps as Monster factors or τ(n) applications
use crate::monster_compiler::SupersingularReason;
use crate::zkp_sat_solver::ZKProof;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ramanujan τ function values for proof step enumeration
const TAU_VALUES: [(u32, i64); 20] = [
    (1, 1), (2, -24), (3, 252), (4, -1472), (5, 4830),
    (6, -6048), (7, -16744), (8, 84480), (9, -113643), (10, -115920),
    (11, 534612), (12, -370944), (13, -577738), (14, 401856), (15, 1217160),
    (16, 987136), (17, -6905934), (18, 2727432), (19, 10661420), (20, -7109760)
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofStep {
    pub step_id: u32,
    pub monster_factor: Option<(u64, u32)>, // (prime, exponent)
    pub tau_application: Option<(u32, i64)>, // (n, τ(n))
    pub proof_fragment: String,
    pub llm_review_prompt: String,
    pub verification_status: ReviewStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReviewStatus {
    Pending,
    Approved(String), // LLM response
    Rejected(String), // Error reason
    RequiresRevision(String),
}

#[derive(Debug)]
pub struct LLMProofReviewer {
    pub proof_steps: Vec<ProofStep>,
    pub monster_factors: HashMap<u64, u32>,
    pub tau_lookup: HashMap<u32, i64>,
    pub review_queue: Vec<u32>, // step_ids
}

impl LLMProofReviewer {
    pub fn new() -> Self {
        let mut monster_factors = HashMap::new();
        // Monster Group: 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71
        let factors = [(2, 46), (3, 20), (5, 9), (7, 6), (11, 2), (13, 3), 
                      (17, 1), (19, 1), (23, 1), (29, 1), (31, 1), (41, 1), 
                      (47, 1), (59, 1), (71, 1)];
        for (prime, exp) in factors {
            monster_factors.insert(prime, exp);
        }

        let mut tau_lookup = HashMap::new();
        for (n, tau_n) in TAU_VALUES {
            tau_lookup.insert(n, tau_n);
        }

        Self {
            proof_steps: Vec::new(),
            monster_factors,
            tau_lookup,
            review_queue: Vec::new(),
        }
    }

    /// Enumerate proof step as Monster factor or τ(n) application
    pub fn enumerate_proof_step(&mut self, zkp: &ZKProof, step_content: &str) -> Result<u32, String> {
        let step_id = self.proof_steps.len() as u32;
        
        let (monster_factor, tau_application) = self.classify_proof_step(zkp, step_id)?;
        
        let llm_prompt = self.generate_review_prompt(zkp, step_content, &monster_factor, &tau_application);
        
        let step = ProofStep {
            step_id,
            monster_factor,
            tau_application,
            proof_fragment: step_content.to_string(),
            llm_review_prompt: llm_prompt,
            verification_status: ReviewStatus::Pending,
        };
        
        self.proof_steps.push(step);
        self.review_queue.push(step_id);
        
        println!("📝 Enumerated proof step {}: {:?}", step_id, 
                if monster_factor.is_some() { "Monster factor" } else { "τ(n) application" });
        
        Ok(step_id)
    }

    fn classify_proof_step(&self, zkp: &ZKProof, step_id: u32) -> Result<(Option<(u64, u32)>, Option<(u32, i64)>), String> {
        // Use reason_id to determine classification
        let reason_id = zkp.reason_id;
        
        if reason_id < 15 {
            // First 15 reasons map to Monster Group prime factors
            let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];
            if let Some(&prime) = primes.get(reason_id as usize) {
                let exponent = self.monster_factors.get(&prime).copied().unwrap_or(1);
                return Ok((Some((prime, exponent)), None));
            }
        }
        
        // Remaining reasons use τ(n) applications
        let n = ((reason_id % 20) + 1) as u32;
        let tau_n = self.tau_lookup.get(&n).copied().unwrap_or(0);
        Ok((None, Some((n, tau_n))))
    }

    fn generate_review_prompt(&self, zkp: &ZKProof, content: &str, 
                             monster_factor: &Option<(u64, u32)>, 
                             tau_app: &Option<(u32, i64)>) -> String {
        let classification = if let Some((prime, exp)) = monster_factor {
            format!("Monster Group factor: {}^{}", prime, exp)
        } else if let Some((n, tau_n)) = tau_app {
            format!("Ramanujan τ({}) = {}", n, tau_n)
        } else {
            "Unclassified".to_string()
        };

        format!(
            "Review this proof step for Monster Group constraint satisfaction:\n\
            Classification: {}\n\
            Reason ID: {}\n\
            Public Inputs: {:?}\n\
            Proof Fragment: {}\n\n\
            Verify:\n\
            1. Mathematical correctness of the constraint\n\
            2. Consistency with Monster Group structure\n\
            3. Proper application of {} enumeration\n\
            4. ZKP validity\n\n\
            Respond with: APPROVED, REJECTED, or REVISION_REQUIRED with explanation.",
            classification, zkp.reason_id, zkp.public_inputs, content, classification
        )
    }

    /// Submit proof step to LLM for review
    pub async fn submit_for_llm_review(&mut self, step_id: u32, llm_endpoint: &str) -> Result<(), String> {
        if let Some(step) = self.proof_steps.get_mut(step_id as usize) {
            println!("🤖 Submitting step {} to LLM review...", step_id);
            
            // Simulate LLM call (replace with actual LLM API)
            let response = self.mock_llm_review(&step.llm_review_prompt).await?;
            
            step.verification_status = match response.as_str() {
                s if s.starts_with("APPROVED") => ReviewStatus::Approved(response),
                s if s.starts_with("REJECTED") => ReviewStatus::Rejected(response),
                s if s.starts_with("REVISION_REQUIRED") => ReviewStatus::RequiresRevision(response),
                _ => ReviewStatus::RequiresRevision(format!("Invalid LLM response: {}", response)),
            };
            
            // Remove from queue if approved or rejected
            if !matches!(step.verification_status, ReviewStatus::RequiresRevision(_)) {
                self.review_queue.retain(|&id| id != step_id);
            }
            
            Ok(())
        } else {
            Err(format!("Proof step {} not found", step_id))
        }
    }

    async fn mock_llm_review(&self, prompt: &str) -> Result<String, String> {
        // Mock LLM response - replace with actual LLM API call
        if prompt.contains("Monster Group factor: 2^46") {
            Ok("APPROVED: Core architecture constraint properly verified with Monster Group factor 2^46".to_string())
        } else if prompt.contains("τ(") {
            Ok("APPROVED: Ramanujan tau function application is mathematically sound".to_string())
        } else {
            Ok("REVISION_REQUIRED: Need clearer mathematical justification".to_string())
        }
    }

    /// Process all pending reviews
    pub async fn process_review_queue(&mut self, llm_endpoint: &str) -> Result<usize, String> {
        let queue_size = self.review_queue.len();
        let pending_steps: Vec<u32> = self.review_queue.clone();
        
        for step_id in pending_steps {
            self.submit_for_llm_review(step_id, llm_endpoint).await?;
        }
        
        Ok(queue_size)
    }

    pub fn get_review_summary(&self) -> ReviewSummary {
        let mut approved = 0;
        let mut rejected = 0;
        let mut pending = 0;
        let mut revision_required = 0;

        for step in &self.proof_steps {
            match step.verification_status {
                ReviewStatus::Approved(_) => approved += 1,
                ReviewStatus::Rejected(_) => rejected += 1,
                ReviewStatus::Pending => pending += 1,
                ReviewStatus::RequiresRevision(_) => revision_required += 1,
            }
        }

        ReviewSummary {
            total_steps: self.proof_steps.len(),
            approved,
            rejected,
            pending,
            revision_required,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewSummary {
    pub total_steps: usize,
    pub approved: usize,
    pub rejected: usize,
    pub pending: usize,
    pub revision_required: usize,
}
