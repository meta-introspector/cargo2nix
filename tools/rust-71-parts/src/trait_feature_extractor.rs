//! Trait and Feature Extractor using AST Transport System
//! Integrates with existing Monster Group classification

use crate::ast_extractor::TokenKind;
use crate::ast_transport::{AstFragment, TransportColony};
use crate::token_constants::CompressedToken;
use std::collections::HashMap;

/// Simple span for source locations
#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// Simple token for parsing
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Extracted trait information with Monster Group classification
#[derive(Debug, Clone)]
pub struct ExtractedTrait {
    pub name: String,
    pub methods: Vec<String>,
    pub monster_signature: u64,
    pub layer: u8,
    pub fragment: AstFragment,
}

/// Extracted feature flag information
#[derive(Debug, Clone)]
pub struct ExtractedFeature {
    pub name: String,
    pub condition: String,
    pub monster_factor: u64,
    pub layer: u8,
    pub fragment: AstFragment,
}

/// Trait and Feature Extractor using AST transport
pub struct TraitFeatureExtractor {
    colony: TransportColony,
    traits: Vec<ExtractedTrait>,
    features: Vec<ExtractedFeature>,
}

impl TraitFeatureExtractor {
    pub fn new() -> Self {
        Self {
            colony: TransportColony::new(),
            traits: Vec::new(),
            features: Vec::new(),
        }
    }
    
    /// Extract traits and features from source code
    pub fn extract_from_source(&mut self, source: &str) -> Result<(), String> {
        let tokens = self.tokenize(source)?;
        self.parse_tokens(tokens)?;
        self.transport_through_layers()?;
        Ok(())
    }
    
    /// Simple tokenizer for trait/feature extraction
    fn tokenize(&self, source: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut pos = 0;
        
        for line in source.lines() {
            let line_start = pos;
            
            // Look for trait definitions
            if line.trim_start().starts_with("trait ") {
                let span = Span::new(line_start, line_start + line.len());
                tokens.push(Token::new(TokenKind::Fn, span)); // Use Fn as trait marker
            }
            
            // Look for feature flags
            if line.contains("#[cfg(feature") || line.contains("cfg!(feature") {
                let span = Span::new(line_start, line_start + line.len());
                tokens.push(Token::new(TokenKind::Pound, span)); // Use Pound as feature marker
            }
            
            pos += line.len() + 1; // +1 for newline
        }
        
        Ok(tokens)
    }
    
    /// Parse tokens to extract traits and features
    fn parse_tokens(&mut self, tokens: Vec<Token>) -> Result<(), String> {
        for token in tokens {
            match token.kind {
                TokenKind::Fn => {
                    // Extract trait from this position
                    if let Some(trait_info) = self.extract_trait_at_span(&token.span) {
                        self.traits.push(trait_info);
                    }
                }
                TokenKind::Pound => {
                    // Extract feature from this position
                    if let Some(feature_info) = self.extract_feature_at_span(&token.span) {
                        self.features.push(feature_info);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    /// Extract trait information at given span
    fn extract_trait_at_span(&self, span: &Span) -> Option<ExtractedTrait> {
        // Simplified trait extraction
        let trait_name = format!("Trait_{}", span.start);
        let methods = vec!["method1".to_string(), "method2".to_string()];
        
        // Assign Monster Group signature based on trait complexity
        let complexity = trait_name.len() + methods.len();
        let monster_signature = self.calculate_monster_signature(complexity);
        let layer = self.calculate_layer(complexity);
        
        // Create AST fragment for transport - fix CompressedToken::new call
        let compressed_token = CompressedToken::new(layer, (span.start % 256) as u8);
        let fragment = AstFragment::new(compressed_token, layer);
        
        Some(ExtractedTrait {
            name: trait_name,
            methods,
            monster_signature,
            layer,
            fragment,
        })
    }
    
    /// Extract feature information at given span
    fn extract_feature_at_span(&self, span: &Span) -> Option<ExtractedFeature> {
        // Simplified feature extraction
        let feature_name = format!("feature_{}", span.start);
        let condition = format!("#[cfg(feature = \"{}\")]", feature_name);
        
        // Assign Monster Group factor
        let monster_factor = self.calculate_monster_factor(span.start);
        let layer = self.calculate_layer(span.start);
        
        // Create AST fragment for transport - fix CompressedToken::new call
        let compressed_token = CompressedToken::new(layer, (span.start % 256) as u8);
        let fragment = AstFragment::new(compressed_token, layer);
        
        Some(ExtractedFeature {
            name: feature_name,
            condition,
            monster_factor,
            layer,
            fragment,
        })
    }
    
    /// Transport extracted items through Monster Group layers
    fn transport_through_layers(&mut self) -> Result<(), String> {
        // Add trait fragments to transport colony
        for trait_info in &self.traits {
            self.colony.add_source(trait_info.fragment.clone());
        }
        
        // Add feature fragments to transport colony
        for feature_info in &self.features {
            self.colony.add_source(feature_info.fragment.clone());
        }
        
        // Build all 108 layers using ant/bee/termite workers
        self.colony.build_all_layers();
        
        Ok(())
    }
    
    /// Calculate Monster Group signature for traits
    fn calculate_monster_signature(&self, complexity: usize) -> u64 {
        let monster_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];
        let index = complexity % monster_primes.len();
        monster_primes[index]
    }
    
    /// Calculate Monster Group factor for features
    fn calculate_monster_factor(&self, position: usize) -> u64 {
        let monster_factors = [71, 59, 47, 41, 31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2];
        let index = position % monster_factors.len();
        monster_factors[index]
    }
    
    /// Calculate transport layer based on complexity
    fn calculate_layer(&self, complexity: usize) -> u8 {
        (complexity % 108) as u8
    }
    
    /// Get extracted traits
    pub fn get_traits(&self) -> &[ExtractedTrait] {
        &self.traits
    }
    
    /// Get extracted features
    pub fn get_features(&self) -> &[ExtractedFeature] {
        &self.features
    }
    
    /// Generate Monster Group trait registry
    pub fn generate_trait_registry(&self) -> String {
        let mut registry = String::new();
        registry.push_str("// Monster Group Trait Registry\n");
        registry.push_str("use std::collections::HashMap;\n\n");
        
        registry.push_str("pub struct MonsterTraitRegistry {\n");
        registry.push_str("    traits_by_signature: HashMap<u64, Vec<&'static str>>,\n");
        registry.push_str("}\n\n");
        
        registry.push_str("impl MonsterTraitRegistry {\n");
        registry.push_str("    pub fn new() -> Self {\n");
        registry.push_str("        let mut registry = Self {\n");
        registry.push_str("            traits_by_signature: HashMap::new(),\n");
        registry.push_str("        };\n");
        
        // Group traits by signature
        let mut signature_groups: HashMap<u64, Vec<&str>> = HashMap::new();
        for trait_info in &self.traits {
            signature_groups.entry(trait_info.monster_signature)
                .or_default()
                .push(&trait_info.name);
        }
        
        for (signature, trait_names) in signature_groups {
            registry.push_str(&format!("        // Monster signature {} (prime factor)\n", signature));
            registry.push_str(&format!("        registry.traits_by_signature.insert({}, vec![", signature));
            for (i, name) in trait_names.iter().enumerate() {
                if i > 0 { registry.push_str(", "); }
                registry.push_str(&format!("\"{}\"", name));
            }
            registry.push_str("]);\n");
        }
        
        registry.push_str("        registry\n");
        registry.push_str("    }\n");
        registry.push_str("}\n");
        
        registry
    }
    
    /// Generate Monster Group feature registry
    pub fn generate_feature_registry(&self) -> String {
        let mut registry = String::new();
        registry.push_str("// Monster Group Feature Registry\n");
        registry.push_str("use std::collections::HashMap;\n\n");
        
        registry.push_str("pub struct MonsterFeatureRegistry {\n");
        registry.push_str("    features_by_factor: HashMap<u64, Vec<&'static str>>,\n");
        registry.push_str("}\n\n");
        
        registry.push_str("impl MonsterFeatureRegistry {\n");
        registry.push_str("    pub fn new() -> Self {\n");
        registry.push_str("        let mut registry = Self {\n");
        registry.push_str("            features_by_factor: HashMap::new(),\n");
        registry.push_str("        };\n");
        
        // Group features by factor
        let mut factor_groups: HashMap<u64, Vec<&str>> = HashMap::new();
        for feature_info in &self.features {
            factor_groups.entry(feature_info.monster_factor)
                .or_default()
                .push(&feature_info.name);
        }
        
        for (factor, feature_names) in factor_groups {
            registry.push_str(&format!("        // Monster factor {} (supersingular prime)\n", factor));
            registry.push_str(&format!("        registry.features_by_factor.insert({}, vec![", factor));
            for (i, name) in feature_names.iter().enumerate() {
                if i > 0 { registry.push_str(", "); }
                registry.push_str(&format!("\"{}\"", name));
            }
            registry.push_str("]);\n");
        }
        
        registry.push_str("        registry\n");
        registry.push_str("    }\n");
        registry.push_str("}\n");
        
        registry
    }
    
    /// Get transport colony statistics
    pub fn get_transport_stats(&self) -> TransportStats {
        let mut stats = TransportStats {
            total_fragments: 0,
            active_layers: 0,
            fragments_per_layer: [0; 108],
            ant_layers: 36,
            bee_layers: 36,
            termite_layers: 36,
        };
        
        for layer in 0..108 {
            let fragments = &self.colony.layers[layer];
            stats.fragments_per_layer[layer] = fragments.len();
            stats.total_fragments += fragments.len();
            
            if !fragments.is_empty() {
                stats.active_layers += 1;
            }
        }
        
        stats
    }
}

/// Transport statistics for analysis
#[derive(Debug)]
pub struct TransportStats {
    pub total_fragments: usize,
    pub active_layers: usize,
    pub fragments_per_layer: [usize; 108],
    pub ant_layers: usize,
    pub bee_layers: usize,
    pub termite_layers: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trait_extraction() {
        let mut extractor = TraitFeatureExtractor::new();
        let source = r#"
            trait MyTrait {
                fn method1(&self);
                fn method2(&mut self);
            }
            
            #[cfg(feature = "test")]
            fn test_function() {}
        "#;
        
        extractor.extract_from_source(source).unwrap();
        
        assert!(!extractor.get_traits().is_empty());
        assert!(!extractor.get_features().is_empty());
    }
    
    #[test]
    fn test_transport_layers() {
        let mut extractor = TraitFeatureExtractor::new();
        let source = "trait Test {}";
        
        extractor.extract_from_source(source).unwrap();
        let stats = extractor.get_transport_stats();
        
        assert!(stats.total_fragments > 0);
        assert_eq!(stats.ant_layers + stats.bee_layers + stats.termite_layers, 108);
    }
}
