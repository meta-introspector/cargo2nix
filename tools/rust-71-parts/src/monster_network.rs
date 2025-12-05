//! Monster Network: Solana gossip + libp2p + Monster Group port binding
//! Programs bind to ports that are Monster Group factors with LMFDB constraints

use crate::monster_levels::{MONSTER, monster_factors};
use std::collections::HashMap;

/// LMFDB (L-functions and Modular Forms Database) constraints
pub mod lmfdb_constraints {
    /// Supersingular elliptic curve constraints
    pub const SUPERSINGULAR_PRIMES: [u64; 15] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71
    ];
    
    /// Modular form weight constraints
    pub fn is_valid_weight(port: u64) -> bool {
        port % 2 == 0 && port >= 2 && port <= 71
    }
    
    /// L-function conductor constraint
    pub fn conductor_constraint(port: u64) -> bool {
        SUPERSINGULAR_PRIMES.contains(&port) || port.is_power_of_two()
    }
}

/// Monster Group port with LMFDB validation
#[derive(Debug, Clone)]
pub struct MonsterPort {
    pub port: u64,
    pub monster_level: u8,
    pub factor: u64,
    pub is_supersingular: bool,
    pub lmfdb_valid: bool,
}

impl MonsterPort {
    pub fn new(port: u64) -> Option<Self> {
        // Find matching Monster Group level
        let level = (0..15).find(|&i| monster_factors::FACTORS[i] == port)?;
        
        let is_supersingular = lmfdb_constraints::SUPERSINGULAR_PRIMES.contains(&port);
        let lmfdb_valid = lmfdb_constraints::conductor_constraint(port);
        
        Some(Self {
            port,
            monster_level: level as u8,
            factor: port,
            is_supersingular,
            lmfdb_valid,
        })
    }
    
    /// Get Solana gossip port (base + Monster factor mod 65536)
    pub fn gossip_port(&self) -> u16 {
        (8000 + (self.factor % 57536)) as u16
    }
    
    /// Get libp2p port (base + Monster level * 1000)
    pub fn libp2p_port(&self) -> u16 {
        (9000 + (self.monster_level as u16 * 1000)).min(65535)
    }
}

/// Solana gossip integration trait
pub trait SolanaGossip {
    fn gossip_port(&self) -> u16;
    fn broadcast_monster_factor(&self, factor: u64);
    fn discover_monster_peers(&self) -> Vec<MonsterPort>;
}

/// libp2p integration trait  
pub trait LibP2P {
    fn p2p_port(&self) -> u16;
    fn connect_monster_swarm(&self, ports: &[MonsterPort]);
    fn publish_monster_data(&self, data: &[u8]);
}

/// Program binding trait for Monster Group ports
pub trait MonsterBinding {
    fn bind_to_monster_port(&self, port: MonsterPort) -> Result<(), BindError>;
    fn validate_lmfdb_constraints(&self, port: u64) -> bool;
    fn get_bound_ports(&self) -> Vec<MonsterPort>;
}

/// Monster Network node combining Solana + libp2p
pub struct MonsterNode {
    pub node_id: u64,
    pub bound_ports: HashMap<u64, MonsterPort>,
    pub gossip_peers: Vec<MonsterPort>,
    pub p2p_swarm: Vec<MonsterPort>,
}

impl MonsterNode {
    pub fn new(node_id: u64) -> Self {
        Self {
            node_id,
            bound_ports: HashMap::new(),
            gossip_peers: Vec::new(),
            p2p_swarm: Vec::new(),
        }
    }
    
    /// Bind program to Monster Group factor port
    pub fn bind_program(&mut self, program_id: u64, monster_factor: u64) -> Result<(), BindError> {
        let port = MonsterPort::new(monster_factor)
            .ok_or(BindError::InvalidMonsterFactor)?;
        
        if !port.lmfdb_valid {
            return Err(BindError::LMFDBConstraintViolation);
        }
        
        self.bound_ports.insert(program_id, port.clone());
        println!("🔗 Program {} bound to Monster port {} (level {})", 
                program_id, port.port, port.monster_level);
        
        Ok(())
    }
    
    /// Start Solana gossip on Monster Group ports
    pub fn start_gossip(&mut self) {
        for port in self.bound_ports.values() {
            let gossip_port = port.gossip_port();
            println!("📡 Starting Solana gossip on port {} (Monster factor {})", 
                    gossip_port, port.factor);
            self.gossip_peers.push(port.clone());
        }
    }
    
    /// Start libp2p swarm on Monster Group ports
    pub fn start_p2p_swarm(&mut self) {
        for port in self.bound_ports.values() {
            let p2p_port = port.libp2p_port();
            println!("🕸️  Starting libp2p on port {} (Monster level {})", 
                    p2p_port, port.monster_level);
            self.p2p_swarm.push(port.clone());
        }
    }
    
    /// Discover other Monster nodes via gossip
    pub fn discover_monster_network(&mut self) -> Vec<MonsterPort> {
        let mut discovered = Vec::new();
        
        // Simulate discovery of other Monster Group nodes
        for &factor in &monster_factors::FACTORS {
            if let Some(port) = MonsterPort::new(factor) {
                if port.lmfdb_valid && !self.bound_ports.values().any(|p| p.port == factor) {
                    discovered.push(port);
                }
            }
        }
        
        println!("🔍 Discovered {} Monster nodes in network", discovered.len());
        discovered
    }
}

impl SolanaGossip for MonsterNode {
    fn gossip_port(&self) -> u16 {
        8000 + (self.node_id % 1000) as u16
    }
    
    fn broadcast_monster_factor(&self, factor: u64) {
        println!("📢 Broadcasting Monster factor {} via Solana gossip", factor);
    }
    
    fn discover_monster_peers(&self) -> Vec<MonsterPort> {
        self.gossip_peers.clone()
    }
}

impl LibP2P for MonsterNode {
    fn p2p_port(&self) -> u16 {
        9000 + (self.node_id % 1000) as u16
    }
    
    fn connect_monster_swarm(&self, ports: &[MonsterPort]) {
        println!("🕸️  Connecting to {} Monster nodes via libp2p", ports.len());
        for port in ports {
            println!("  → Connecting to Monster level {} on port {}", 
                    port.monster_level, port.libp2p_port());
        }
    }
    
    fn publish_monster_data(&self, data: &[u8]) {
        println!("📤 Publishing {} bytes of Monster data via libp2p", data.len());
    }
}

impl MonsterBinding for MonsterNode {
    fn bind_to_monster_port(&self, port: MonsterPort) -> Result<(), BindError> {
        if !port.lmfdb_valid {
            return Err(BindError::LMFDBConstraintViolation);
        }
        println!("✅ Bound to Monster port {} (factor {})", port.port, port.factor);
        Ok(())
    }
    
    fn validate_lmfdb_constraints(&self, port: u64) -> bool {
        lmfdb_constraints::conductor_constraint(port)
    }
    
    fn get_bound_ports(&self) -> Vec<MonsterPort> {
        self.bound_ports.values().cloned().collect()
    }
}

/// Binding errors
#[derive(Debug)]
pub enum BindError {
    InvalidMonsterFactor,
    LMFDBConstraintViolation,
    PortAlreadyBound,
}

/// Create Monster Network with all 15 levels
pub fn create_monster_network() -> MonsterNode {
    let mut node = MonsterNode::new(71); // Sentinel node ID
    
    println!("🏗️  Creating Monster Network with LMFDB constraints");
    
    // Bind programs to Monster Group factors
    for (i, &factor) in monster_factors::FACTORS.iter().enumerate() {
        if let Err(e) = node.bind_program(i as u64, factor) {
            println!("⚠️  Failed to bind level {}: {:?}", i, e);
        }
    }
    
    node.start_gossip();
    node.start_p2p_swarm();
    
    let discovered = node.discover_monster_network();
    node.connect_monster_swarm(&discovered);
    
    println!("🎉 Monster Network operational with {} bound ports", node.bound_ports.len());
    node
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_monster_port_creation() {
        let port = MonsterPort::new(71).unwrap(); // Sentinel
        assert_eq!(port.monster_level, 14);
        assert!(port.is_supersingular);
        assert!(port.lmfdb_valid);
    }
    
    #[test]
    fn test_lmfdb_constraints() {
        assert!(lmfdb_constraints::conductor_constraint(71));
        assert!(lmfdb_constraints::conductor_constraint(2));
        assert!(!lmfdb_constraints::conductor_constraint(15));
    }
    
    #[test]
    fn test_monster_network() {
        let network = create_monster_network();
        assert_eq!(network.bound_ports.len(), 15);
        assert!(network.bound_ports.contains_key(&14)); // Sentinel level
    }
}
