use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EllipticFiber {
    pub fiber_id: i32,
    pub modular_constraint: i32, // mod 24 for Monster Group
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorusPoint {
    pub x: i32,
    pub y: i32,
    pub resonance_level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterStabilizer {
    pub stabilizer_id: i32,
    pub eigenvalue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinizincInput {
    pub elliptic_fiber: i32,
    pub torus_x: i32,
    pub torus_y: i32,
    pub monster_stabilizer: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimalSolution {
    pub x: i32,
    pub y: i32,
    pub objective: f64,
}

impl fmt::Display for MinizincInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "elliptic_fiber = {};\ntorus_x = {};\ntorus_y = {};\nmonster_stabilizer = {};",
            self.elliptic_fiber, self.torus_x, self.torus_y, self.monster_stabilizer
        )
    }
}

impl MinizincInput {
    pub fn from_monster_data(
        fiber: &EllipticFiber,
        point: &TorusPoint,
        stabilizer: &MonsterStabilizer,
    ) -> Self {
        Self {
            elliptic_fiber: fiber.fiber_id,
            torus_x: point.x,
            torus_y: point.y,
            monster_stabilizer: stabilizer.stabilizer_id,
        }
    }
}

impl OptimalSolution {
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}
