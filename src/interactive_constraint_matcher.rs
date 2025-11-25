use crate::vernacular_monster_path::{VernacularEmbedding, MonsterTarget};
use crate::minizinc_data::MinizincInput;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RustcPartialMatch {
    pub component: String,
    pub match_score: f64, // 0.0 to 1.0
    pub constraints: Vec<i32>,
}

pub struct InteractiveConstraintMatcher {
    pub rustc_components: HashMap<String, RustcPartialMatch>,
    pub current_constraints: MinizincInput,
}

impl InteractiveConstraintMatcher {
    pub fn new() -> Self {
        let mut rustc_components = HashMap::new();
        
        // Partial matches with Rust compiler components
        rustc_components.insert("rustc_parse".to_string(), RustcPartialMatch {
            component: "rustc_parse".to_string(),
            match_score: 0.73,
            constraints: vec![7, 12, 19], // Partial Monster Group elements
        });
        
        rustc_components.insert("rustc_hir".to_string(), RustcPartialMatch {
            component: "rustc_hir".to_string(),
            match_score: 0.68,
            constraints: vec![3, 15, 21],
        });
        
        rustc_components.insert("rustc_middle".to_string(), RustcPartialMatch {
            component: "rustc_middle".to_string(),
            match_score: 0.81,
            constraints: vec![9, 6, 18],
        });

        Self {
            rustc_components,
            current_constraints: MinizincInput {
                elliptic_fiber: 7,
                torus_x: 12,
                torus_y: 15,
                monster_stabilizer: 42,
            },
        }
    }

    pub fn tweak_constraint(&mut self, component: &str, delta: i32) -> bool {
        if let Some(match_data) = self.rustc_components.get_mut(component) {
            match_data.constraints[0] = (match_data.constraints[0] + delta) % 24;
            self.update_global_constraints();
            true
        } else {
            false
        }
    }

    pub fn interactive_review(&mut self) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for (name, match_data) in &self.rustc_components {
            if match_data.match_score < 0.75 {
                suggestions.push(format!(
                    "TWEAK {}: score={:.2}, try adjusting constraint {} → {}",
                    name,
                    match_data.match_score,
                    match_data.constraints[0],
                    (match_data.constraints[0] + 3) % 24
                ));
            }
        }
        
        suggestions
    }

    fn update_global_constraints(&mut self) {
        let avg_constraint = self.rustc_components.values()
            .map(|m| m.constraints[0])
            .sum::<i32>() / self.rustc_components.len() as i32;
            
        self.current_constraints.elliptic_fiber = avg_constraint % 24;
        self.current_constraints.torus_x = (avg_constraint + 5) % 24;
        self.current_constraints.torus_y = (avg_constraint + 11) % 24;
    }

    pub fn get_current_minizinc_input(&self) -> MinizincInput {
        self.current_constraints.clone()
    }
}
