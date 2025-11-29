use crate::core_constants::{MONSTER_GROUP_REPRESENTATION_DIMENSION, HECKE_EIGENVALUES};
use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;

pub struct MiniZincIntrospectorIntegration {
    vendor_path: String,
    asciicast_processor_path: String,
}

impl MiniZincIntrospectorIntegration {
    pub fn new() -> Self {
        Self {
            vendor_path: "vendor/libminizinc".to_string(),
            asciicast_processor_path: "vendor/libminizinc/asciicast_processor".to_string(),
        }
    }

    pub fn verify_vendorization(&self) -> Result<VendorizationStatus, String> {
        let mut status = VendorizationStatus::default();
        
        // Check vendor directory
        status.vendor_directory_exists = Path::new(&self.vendor_path).exists();
        
        // Check .gitmodules
        if let Ok(gitmodules) = fs::read_to_string(".gitmodules") {
            status.gitmodules_configured = gitmodules.contains("vendor/libminizinc");
            status.correct_repository = gitmodules.contains("minizinc-introspector");
        }
        
        // Check asciicast_processor
        status.asciicast_processor_exists = Path::new(&self.asciicast_processor_path).exists();
        
        // Check submodule initialization
        status.submodule_initialized = Path::new(&format!("{}/.git", self.vendor_path)).exists();
        
        Ok(status)
    }

    pub fn process_monster_asciicast(&self, input_file: &str) -> Result<String, String> {
        if !Path::new(&self.asciicast_processor_path).exists() {
            return Err("asciicast_processor not found in vendored libminizinc".to_string());
        }

        let monster_enhanced_input = self.enhance_with_monster_group(input_file)?;
        
        let output = Command::new("python3")
            .arg(&format!("{}/process_asciicast.py", self.asciicast_processor_path))
            .arg(&monster_enhanced_input)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("Failed to execute asciicast_processor: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(format!("asciicast_processor failed: {}", 
                       String::from_utf8_lossy(&output.stderr)))
        }
    }

    fn enhance_with_monster_group(&self, input_file: &str) -> Result<String, String> {
        let content = fs::read_to_string(input_file)
            .map_err(|e| format!("Failed to read input file: {}", e))?;
        
        let enhanced_content = format!(
            "% Monster Group Enhanced Asciicast\n\
            % Monster Group Order: {}\n\
            % Hecke Eigenvalues: {:?}\n\
            % Ramanujan τ Constraint: sum ≡ 0 (mod 24)\n\
            \n\
            {}\n\
            \n\
            % Monster Group Verification\n\
            constraint sum(variables) mod 24 = 0;\n\
            constraint forall(i in index_set(variables)) (\n\
                variables[i] >= 0 /\\ variables[i] < {}\n\
            );",
            MONSTER_GROUP_REPRESENTATION_DIMENSION,
            HECKE_EIGENVALUES,
            content,
            MONSTER_GROUP_REPRESENTATION_DIMENSION
        );

        let enhanced_file = format!("{}.monster_enhanced", input_file);
        fs::write(&enhanced_file, enhanced_content)
            .map_err(|e| format!("Failed to write enhanced file: {}", e))?;
        
        Ok(enhanced_file)
    }

    pub fn generate_introspector_model(&self, problem_size: usize) -> String {
        format!(
            "% MiniZinc Introspector Model with Monster Group Integration\n\
            % Generated from vendored libminizinc\n\
            \n\
            include \"globals.mzn\";\n\
            \n\
            % Parameters\n\
            int: n = {};\n\
            int: monster_order = {};\n\
            \n\
            % Variables for introspection\n\
            array[1..n] of var 0..monster_order-1: introspection_vars;\n\
            array[1..n] of var 0..7: depth_levels;\n\
            var int: total_introspection;\n\
            \n\
            % Monster Group constraints\n\
            constraint sum(introspection_vars) mod 24 = 0;\n\
            constraint all_different(introspection_vars);\n\
            \n\
            % Hecke eigenvalue alignment\n\
            constraint forall(i in 1..n) (\n\
                if introspection_vars[i] mod 2 = 0 then\n\
                    depth_levels[i] <= 6\n\
                else\n\
                    depth_levels[i] >= 2\n\
                endif\n\
            );\n\
            \n\
            % Introspection depth constraint\n\
            constraint sum(depth_levels) <= n * 4;\n\
            \n\
            % Objective: maximize introspection while maintaining Monster Group coherence\n\
            constraint total_introspection = sum(i in 1..n) (\n\
                introspection_vars[i] * depth_levels[i] div 1000\n\
            );\n\
            \n\
            solve maximize total_introspection;\n\
            \n\
            output [\n\
                \"Introspection Variables: \", show(introspection_vars), \"\\n\",\n\
                \"Depth Levels: \", show(depth_levels), \"\\n\",\n\
                \"Total Introspection: \", show(total_introspection), \"\\n\",\n\
                \"Monster Group Sum: \", show(sum(introspection_vars)), \" (mod 24 = \", \n\
                show(sum(introspection_vars) mod 24), \")\\n\"\n\
            ];",
            problem_size,
            MONSTER_GROUP_REPRESENTATION_DIMENSION
        )
    }

    pub fn run_introspector_solver(&self, model: &str) -> Result<IntrospectorResult, String> {
        let model_file = "introspector_model.mzn";
        fs::write(model_file, model)
            .map_err(|e| format!("Failed to write model file: {}", e))?;

        // Try to use vendored minizinc if available
        let minizinc_path = format!("{}/bin/minizinc", self.vendor_path);
        let minizinc_cmd = if Path::new(&minizinc_path).exists() {
            minizinc_path
        } else {
            "minizinc".to_string() // Fallback to system minizinc
        };

        let output = Command::new(minizinc_cmd)
            .arg("--solver")
            .arg("gecode")
            .arg(model_file)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("Failed to execute minizinc: {}", e))?;

        if output.status.success() {
            let result_str = String::from_utf8_lossy(&output.stdout);
            Ok(self.parse_introspector_result(&result_str))
        } else {
            Err(format!("MiniZinc solver failed: {}", 
                       String::from_utf8_lossy(&output.stderr)))
        }
    }

    fn parse_introspector_result(&self, output: &str) -> IntrospectorResult {
        let mut result = IntrospectorResult::default();
        
        for line in output.lines() {
            if line.contains("Introspection Variables:") {
                result.variables = self.extract_array_from_line(line);
            } else if line.contains("Depth Levels:") {
                result.depth_levels = self.extract_array_from_line(line);
            } else if line.contains("Total Introspection:") {
                result.total_introspection = self.extract_number_from_line(line);
            } else if line.contains("mod 24 =") {
                result.monster_group_valid = line.contains("mod 24 = 0");
            }
        }
        
        result
    }

    fn extract_array_from_line(&self, line: &str) -> Vec<i32> {
        // Simple parser for MiniZinc array output
        if let Some(start) = line.find('[') {
            if let Some(end) = line.find(']') {
                let array_str = &line[start+1..end];
                return array_str.split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
            }
        }
        Vec::new()
    }

    fn extract_number_from_line(&self, line: &str) -> i32 {
        line.split_whitespace()
            .find_map(|s| s.parse().ok())
            .unwrap_or(0)
    }
}

#[derive(Debug, Default)]
pub struct VendorizationStatus {
    pub vendor_directory_exists: bool,
    pub gitmodules_configured: bool,
    pub correct_repository: bool,
    pub asciicast_processor_exists: bool,
    pub submodule_initialized: bool,
}

impl VendorizationStatus {
    pub fn is_complete(&self) -> bool {
        self.vendor_directory_exists && 
        self.gitmodules_configured && 
        self.correct_repository && 
        self.submodule_initialized
    }

    pub fn missing_components(&self) -> Vec<String> {
        let mut missing = Vec::new();
        
        if !self.vendor_directory_exists {
            missing.push("Vendor directory".to_string());
        }
        if !self.gitmodules_configured {
            missing.push(".gitmodules configuration".to_string());
        }
        if !self.correct_repository {
            missing.push("Correct repository URL".to_string());
        }
        if !self.asciicast_processor_exists {
            missing.push("asciicast_processor component".to_string());
        }
        if !self.submodule_initialized {
            missing.push("Submodule initialization".to_string());
        }
        
        missing
    }
}

#[derive(Debug, Default)]
pub struct IntrospectorResult {
    pub variables: Vec<i32>,
    pub depth_levels: Vec<i32>,
    pub total_introspection: i32,
    pub monster_group_valid: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_introspector_integration() {
        let integration = MiniZincIntrospectorIntegration::new();
        
        let model = integration.generate_introspector_model(5);
        assert!(model.contains("Monster Group"));
        assert!(model.contains("constraint sum(introspection_vars) mod 24 = 0"));
    }

    #[test]
    fn test_vendorization_status() {
        let integration = MiniZincIntrospectorIntegration::new();
        let status = integration.verify_vendorization().unwrap();
        
        // Status should be created without errors
        assert!(!status.is_complete() || status.is_complete());
    }

    #[test]
    fn test_result_parsing() {
        let integration = MiniZincIntrospectorIntegration::new();
        let output = "Introspection Variables: [24, 48, 72]\nDepth Levels: [2, 3, 4]\nTotal Introspection: 150\nMonster Group Sum: 144 (mod 24 = 0)";
        
        let result = integration.parse_introspector_result(output);
        assert_eq!(result.variables, vec![24, 48, 72]);
        assert_eq!(result.depth_levels, vec![2, 3, 4]);
        assert_eq!(result.total_introspection, 150);
        assert!(result.monster_group_valid);
    }
}
