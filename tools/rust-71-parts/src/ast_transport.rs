//! AST Transport System: 108-layer construction using Hecke operations
//! Ant/Bee/Termite colony behavior for source→target AST transport

use crate::monster_levels::{MONSTER, MonsterLevel};
use crate::hecke_operators::{HeckeOperator, LevelElevation, SemanticComposition};
use crate::token_constants::CompressedToken;

/// AST fragment for transport
#[derive(Debug, Clone)]
pub struct AstFragment {
    pub data: CompressedToken,
    pub layer: u8,
    pub monster_factor: u64,
}

impl AstFragment {
    pub fn new(data: CompressedToken, layer: u8) -> Self {
        let level = MONSTER.level(layer.min(14));
        Self {
            data,
            layer,
            monster_factor: level.factor,
        }
    }
}

/// Transport worker types (like ant castes)
#[derive(Debug, Clone, Copy)]
pub enum WorkerType {
    Ant,      // Simple transport (layers 0-35)
    Bee,      // Complex transport (layers 36-71) 
    Termite,  // Advanced transport (layers 72-107)
}

impl WorkerType {
    pub fn for_layer(layer: u8) -> Self {
        match layer {
            0..=35 => WorkerType::Ant,
            36..=71 => WorkerType::Bee,
            _ => WorkerType::Termite,
        }
    }
}

/// Transport worker with Hecke operations
pub struct TransportWorker {
    pub worker_type: WorkerType,
    pub layer: u8,
    pub capacity: u8,
}

impl TransportWorker {
    pub fn new(layer: u8) -> Self {
        let worker_type = WorkerType::for_layer(layer);
        let capacity = match worker_type {
            WorkerType::Ant => 1,     // Single fragment
            WorkerType::Bee => 3,     // Multiple fragments
            WorkerType::Termite => 7, // Complex structures
        };
        
        Self { worker_type, layer, capacity }
    }
    
    /// Transport fragment using Hecke elevation
    pub fn transport(&self, fragment: AstFragment) -> AstFragment {
        let elevated_data = LevelElevation::apply(fragment.data);
        AstFragment::new(elevated_data, self.layer)
    }
    
    /// Combine fragments using Hecke composition
    pub fn combine(&self, left: AstFragment, right: AstFragment) -> AstFragment {
        let composed_data = SemanticComposition::apply((left.data, right.data));
        AstFragment::new(composed_data, self.layer)
    }
}

/// Colony of transport workers for 108-layer construction
pub struct TransportColony {
    workers: Vec<TransportWorker>,
    pub layers: [Vec<AstFragment>; 108],
}

impl TransportColony {
    pub fn new() -> Self {
        let mut workers = Vec::new();
        for layer in 0..108 {
            workers.push(TransportWorker::new(layer));
        }
        
        Self {
            workers,
            layers: std::array::from_fn(|_| Vec::new()),
        }
    }
    
    /// Add source fragment to layer 0
    pub fn add_source(&mut self, fragment: AstFragment) {
        self.layers[0].push(fragment);
    }
    
    /// Transport fragments up one layer
    pub fn transport_layer(&mut self, from_layer: usize) {
        if from_layer >= 107 { return; }
        
        let worker = &self.workers[from_layer + 1];
        let fragments = self.layers[from_layer].clone();
        
        for fragment in fragments {
            let transported = worker.transport(fragment);
            self.layers[from_layer + 1].push(transported);
        }
    }
    
    /// Build all 108 layers from source
    pub fn build_all_layers(&mut self) {
        for layer in 0..107 {
            self.transport_layer(layer);
            
            // Combine fragments at current layer (termite behavior)
            if layer >= 72 {
                self.combine_layer(layer + 1);
            }
        }
    }
    
    /// Combine fragments within a layer
    fn combine_layer(&mut self, layer: usize) {
        if layer >= 108 { return; }
        
        let worker = &self.workers[layer];
        let fragments = self.layers[layer].clone();
        
        if fragments.len() >= 2 {
            let mut combined = Vec::new();
            for chunk in fragments.chunks(2) {
                if chunk.len() == 2 {
                    let result = worker.combine(chunk[0].clone(), chunk[1].clone());
                    combined.push(result);
                } else {
                    combined.push(chunk[0].clone());
                }
            }
            self.layers[layer] = combined;
        }
    }
    
    /// Get final result from layer 107
    pub fn get_result(&self) -> Option<&AstFragment> {
        self.layers[107].first()
    }
    
    /// Get colony statistics
    pub fn stats(&self) -> ColonyStats {
        let mut ant_count = 0;
        let mut bee_count = 0;
        let mut termite_count = 0;
        let mut total_fragments = 0;
        
        for (i, layer) in self.layers.iter().enumerate() {
            total_fragments += layer.len();
            match WorkerType::for_layer(i as u8) {
                WorkerType::Ant => ant_count += layer.len(),
                WorkerType::Bee => bee_count += layer.len(),
                WorkerType::Termite => termite_count += layer.len(),
            }
        }
        
        ColonyStats {
            ant_fragments: ant_count,
            bee_fragments: bee_count,
            termite_fragments: termite_count,
            total_fragments,
            layers_built: self.layers.iter().filter(|l| !l.is_empty()).count(),
        }
    }
}

/// Colony statistics
#[derive(Debug)]
pub struct ColonyStats {
    pub ant_fragments: usize,
    pub bee_fragments: usize,
    pub termite_fragments: usize,
    pub total_fragments: usize,
    pub layers_built: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_constants::TokenConstants;
    
    #[test]
    fn test_worker_types() {
        assert!(matches!(WorkerType::for_layer(10), WorkerType::Ant));
        assert!(matches!(WorkerType::for_layer(50), WorkerType::Bee));
        assert!(matches!(WorkerType::for_layer(90), WorkerType::Termite));
    }
    
    #[test]
    fn test_transport_worker() {
        let worker = TransportWorker::new(5);
        let fragment = AstFragment::new(TokenConstants::PLUS, 0);
        let transported = worker.transport(fragment);
        
        assert_eq!(transported.layer, 5);
        assert_eq!(transported.data.level, 3); // Elevated from 2 to 3
    }
    
    #[test]
    fn test_colony_construction() {
        let mut colony = TransportColony::new();
        
        // Add source fragments
        colony.add_source(AstFragment::new(TokenConstants::FN, 0));
        colony.add_source(AstFragment::new(TokenConstants::PLUS, 0));
        
        // Build first few layers
        for layer in 0..5 {
            colony.transport_layer(layer);
        }
        
        let stats = colony.stats();
        assert!(stats.layers_built >= 5);
        assert!(stats.total_fragments > 0);
    }
}
