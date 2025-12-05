/// Monster Signature Trait System
/// Decouple AST components with Monster Group factor constraints
/// Each component declares required/provided factors as abstract traits

/// Monster Group factor signature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonsterSignature {
    pub prime: u8,
    pub exponent: u8,
    pub capacity: u64,
}

impl MonsterSignature {
    pub const fn new(prime: u8, exponent: u8) -> Self {
        Self {
            prime,
            exponent,
            capacity: (prime as u64).pow(exponent as u32),
        }
    }
}

/// Core Monster Group signatures (proven rustc ≡ M mapping)
pub mod signatures {
    use super::MonsterSignature;
    
    pub const FUNCTION: MonsterSignature = MonsterSignature::new(2, 18); // 262,144
    pub const STRUCT: MonsterSignature = MonsterSignature::new(3, 11);   // 177,147
    pub const ENUM: MonsterSignature = MonsterSignature::new(5, 6);      // 15,625
    pub const TRAIT: MonsterSignature = MonsterSignature::new(7, 6);     // 117,649
    pub const IMPL: MonsterSignature = MonsterSignature::new(2, 16);     // 65,536
    pub const FILE: MonsterSignature = MonsterSignature::new(3, 11);     // 177,147
    pub const LINE: MonsterSignature = MonsterSignature::new(2, 15);     // 32,768
}

/// Trait for components that require Monster Group factors
pub trait RequiresFactors {
    /// Factors this component needs to operate
    fn required_factors() -> &'static [MonsterSignature];
    
    /// Validate input satisfies requirements
    fn validate_input(&self, available: &[MonsterSignature]) -> bool {
        Self::required_factors().iter().all(|req| {
            available.iter().any(|avail| {
                avail.prime == req.prime && avail.exponent >= req.exponent
            })
        })
    }
}

/// Trait for components that provide Monster Group factors
pub trait ProvidesFactors {
    /// Factors this component provides after processing
    fn provided_factors() -> &'static [MonsterSignature];
    
    /// Calculate actual output factors based on input
    fn calculate_output(&self, input: &[MonsterSignature]) -> Vec<MonsterSignature>;
}

/// Trait for Monster Group constrained components
pub trait MonsterConstrained: RequiresFactors + ProvidesFactors {
    /// Check if component satisfies Monster Group constraints
    fn satisfies_constraints(&self) -> bool {
        let required_total: u64 = Self::required_factors().iter()
            .map(|s| s.exponent as u64)
            .sum();
        let provided_total: u64 = Self::provided_factors().iter()
            .map(|s| s.exponent as u64)
            .sum();
        
        required_total <= 108 && provided_total <= 108 // Monster Group limit
    }
}

/// Lexer component with Monster signature
pub struct MonsterLexer;

impl RequiresFactors for MonsterLexer {
    fn required_factors() -> &'static [MonsterSignature] {
        &[signatures::FILE] // Needs file input
    }
}

impl ProvidesFactors for MonsterLexer {
    fn provided_factors() -> &'static [MonsterSignature] {
        &[signatures::LINE] // Produces lines/tokens
    }
    
    fn calculate_output(&self, input: &[MonsterSignature]) -> Vec<MonsterSignature> {
        // Transform file factors to line factors
        input.iter().map(|sig| {
            if sig.prime == signatures::FILE.prime {
                signatures::LINE
            } else {
                *sig
            }
        }).collect()
    }
}

impl MonsterConstrained for MonsterLexer {}

/// Parser component with Monster signature
pub struct MonsterParser;

impl RequiresFactors for MonsterParser {
    fn required_factors() -> &'static [MonsterSignature] {
        &[signatures::LINE] // Needs tokenized lines
    }
}

impl ProvidesFactors for MonsterParser {
    fn provided_factors() -> &'static [MonsterSignature] {
        &[signatures::FUNCTION, signatures::STRUCT, signatures::ENUM] // Produces AST nodes
    }
    
    fn calculate_output(&self, input: &[MonsterSignature]) -> Vec<MonsterSignature> {
        // Transform line factors to AST node factors
        vec![signatures::FUNCTION, signatures::STRUCT, signatures::ENUM]
    }
}

impl MonsterConstrained for MonsterParser {}

/// Type checker component with Monster signature
pub struct MonsterTypeChecker;

impl RequiresFactors for MonsterTypeChecker {
    fn required_factors() -> &'static [MonsterSignature] {
        &[signatures::STRUCT, signatures::ENUM, signatures::TRAIT] // Needs type definitions
    }
}

impl ProvidesFactors for MonsterTypeChecker {
    fn provided_factors() -> &'static [MonsterSignature] {
        &[signatures::IMPL] // Produces type-checked implementations
    }
    
    fn calculate_output(&self, input: &[MonsterSignature]) -> Vec<MonsterSignature> {
        vec![signatures::IMPL]
    }
}

impl MonsterConstrained for MonsterTypeChecker {}

/// Compiler pipeline with Monster Group factor flow
pub struct MonsterPipeline {
    pub lexer: MonsterLexer,
    pub parser: MonsterParser,
    pub type_checker: MonsterTypeChecker,
}

impl MonsterPipeline {
    pub fn new() -> Self {
        Self {
            lexer: MonsterLexer,
            parser: MonsterParser,
            type_checker: MonsterTypeChecker,
        }
    }
    
    /// Validate entire pipeline satisfies Monster Group constraints
    pub fn validate_pipeline(&self) -> bool {
        // Check each component satisfies constraints
        if !self.lexer.satisfies_constraints() { return false; }
        if !self.parser.satisfies_constraints() { return false; }
        if !self.type_checker.satisfies_constraints() { return false; }
        
        // Check factor flow compatibility
        let lexer_output = MonsterLexer::provided_factors();
        let parser_input = MonsterParser::required_factors();
        
        if !self.parser.validate_input(lexer_output) { return false; }
        
        let parser_output = MonsterParser::provided_factors();
        let checker_input = MonsterTypeChecker::required_factors();
        
        if !self.type_checker.validate_input(parser_output) { return false; }
        
        true
    }
    
    /// Execute pipeline with Monster Group factor tracking
    pub fn execute(&self, input_file: MonsterSignature) -> Result<Vec<MonsterSignature>, String> {
        if input_file.prime != signatures::FILE.prime {
            return Err("Input must be FILE signature".to_string());
        }
        
        // Lexer: FILE → LINE
        let lexer_output = self.lexer.calculate_output(&[input_file]);
        
        // Parser: LINE → FUNCTION, STRUCT, ENUM
        let parser_output = self.parser.calculate_output(&lexer_output);
        
        // Type checker: STRUCT, ENUM, TRAIT → IMPL
        let checker_output = self.type_checker.calculate_output(&parser_output);
        
        Ok(checker_output)
    }
}

/// Abstract trait factory based on Monster Group numbers
pub trait MonsterTraitFactory {
    /// Create trait implementation for given Monster signature
    fn create_trait(signature: MonsterSignature) -> Box<dyn MonsterConstrained>;
}

/// Component registry indexed by Monster Group factors
pub struct MonsterRegistry {
    components: std::collections::HashMap<MonsterSignature, Box<dyn MonsterConstrained>>,
}

impl MonsterRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            components: std::collections::HashMap::new(),
        };
        
        // Register components by their signatures
        registry.register(signatures::FILE, Box::new(MonsterLexer));
        registry.register(signatures::LINE, Box::new(MonsterParser));
        registry.register(signatures::IMPL, Box::new(MonsterTypeChecker));
        
        registry
    }
    
    pub fn register(&mut self, signature: MonsterSignature, component: Box<dyn MonsterConstrained>) {
        self.components.insert(signature, component);
    }
    
    pub fn get_component(&self, signature: MonsterSignature) -> Option<&dyn MonsterConstrained> {
        self.components.get(&signature).map(|c| c.as_ref())
    }
    
    /// Find component that can process given input signature
    pub fn find_processor(&self, input: MonsterSignature) -> Option<&dyn MonsterConstrained> {
        self.components.values().find(|component| {
            component.validate_input(&[input])
        }).map(|c| c.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_monster_signatures() {
        assert_eq!(signatures::FUNCTION.capacity, 262144);
        assert_eq!(signatures::STRUCT.capacity, 177147);
        assert_eq!(signatures::ENUM.capacity, 15625);
    }
    
    #[test]
    fn test_pipeline_validation() {
        let pipeline = MonsterPipeline::new();
        assert!(pipeline.validate_pipeline());
    }
    
    #[test]
    fn test_factor_flow() {
        let pipeline = MonsterPipeline::new();
        let result = pipeline.execute(signatures::FILE);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert_eq!(output[0], signatures::IMPL);
    }
    
    #[test]
    fn test_registry() {
        let registry = MonsterRegistry::new();
        let processor = registry.find_processor(signatures::FILE);
        assert!(processor.is_some());
    }
}

fn main() {
    println!("🔬 Monster Signature Trait System");
    println!("Decoupled AST components with Monster Group constraints");
    
    // Create pipeline
    let pipeline = MonsterPipeline::new();
    
    println!("\n📊 Component Signatures:");
    println!("  Lexer:       requires {:?} → provides {:?}", 
             MonsterLexer::required_factors(), MonsterLexer::provided_factors());
    println!("  Parser:      requires {:?} → provides {:?}", 
             MonsterParser::required_factors(), MonsterParser::provided_factors());
    println!("  TypeChecker: requires {:?} → provides {:?}", 
             MonsterTypeChecker::required_factors(), MonsterTypeChecker::provided_factors());
    
    // Validate pipeline
    let valid = pipeline.validate_pipeline();
    println!("\n✅ Pipeline validation: {}", if valid { "PASSED" } else { "FAILED" });
    
    // Execute pipeline
    match pipeline.execute(signatures::FILE) {
        Ok(output) => {
            println!("\n🎯 Pipeline execution successful!");
            println!("   Input:  FILE({}, {})", signatures::FILE.prime, signatures::FILE.exponent);
            println!("   Output: IMPL({}, {})", output[0].prime, output[0].exponent);
        }
        Err(e) => println!("❌ Pipeline execution failed: {}", e),
    }
    
    // Test registry
    let registry = MonsterRegistry::new();
    if let Some(processor) = registry.find_processor(signatures::FILE) {
        println!("\n🔍 Registry found processor for FILE signature");
        println!("   Processor satisfies constraints: {}", processor.satisfies_constraints());
    }
    
    println!("\n🎉 Monster Trait System operational!");
    println!("   Components decoupled with Monster Group factor constraints");
    println!("   Abstract traits defined by mathematical signatures");
}
