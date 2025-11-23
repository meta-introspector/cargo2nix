use std::fmt;
use serde::{Serialize, Deserialize};

// Placeholder for LMFDB ID. This would eventually be a more structured type
// representing a specific entry or concept from LMFDB.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LmfdbId(pub String);

impl fmt::Display for LmfdbId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a composite semantic ID for a declaration, encoding various
/// metrics and its association with mathematical objects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticId {
    pub unique_idx: usize, // A simple unique index for internal tracking
    pub weight: f64,       // A calculated metric of complexity/importance
    pub depth: usize,      // Nesting level in the AST or dependency graph
    pub lmfdb_id: Option<LmfdbId>, // Associated LMFDB identifier
    pub ast_types_count: usize, // Number of unique AST types used
    pub expressions_count: usize, // Number of expressions within the declaration
    // Add more fields for prime factors of numerical constants, etc., later
}

impl SemanticId {
    pub fn new(unique_idx: usize) -> Self {
        SemanticId {
            unique_idx,
            weight: 0.0,
            depth: 0,
            lmfdb_id: None,
            ast_types_count: 0,
            expressions_count: 0,
        }
    }

    /// Converts the SemanticId into a vector of numerical features suitable for an EigenMatrix.
    pub fn to_numerical_vector(&self) -> Vec<f64> {
        vec![
            self.unique_idx as f64,
            self.weight,
            self.depth as f64,
            // Placeholder for LMFDB ID numerical representation
            // For now, if present, assign 1.0, else 0.0. This needs refinement.
            if self.lmfdb_id.is_some() { 1.0 } else { 0.0 },
            self.ast_types_count as f64,
            self.expressions_count as f64,
        ]
    }
}

impl fmt::Display for SemanticId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID:{}-W:{:.2}-D:{}-LMFDB:{}-Types:{}-Exprs:{}",
            self.unique_idx,
            self.weight,
            self.depth,
            self.lmfdb_id.as_ref().map_or("None".to_string(), |id| id.to_string()),
            self.ast_types_count,
            self.expressions_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lmfdb_id_new() {
        let lmfdb_id = LmfdbId("1.2.3.4".to_string());
        assert_eq!(lmfdb_id.0, "1.2.3.4");
    }

    #[test]
    fn test_semantic_id_new() {
        let sem_id = SemanticId::new(1);
        assert_eq!(sem_id.unique_idx, 1);
        assert_eq!(sem_id.weight, 0.0);
        assert_eq!(sem_id.depth, 0);
        assert!(sem_id.lmfdb_id.is_none());
        assert_eq!(sem_id.ast_types_count, 0);
        assert_eq!(sem_id.expressions_count, 0);
    }

    #[test]
    fn test_semantic_id_to_numerical_vector() {
        let sem_id = SemanticId {
            unique_idx: 10,
            weight: 5.5,
            depth: 3,
            lmfdb_id: Some(LmfdbId("test".to_string())),
            ast_types_count: 2,
            expressions_count: 7,
        };
        let vec = sem_id.to_numerical_vector();
        assert_eq!(vec, vec![10.0, 5.5, 3.0, 1.0, 2.0, 7.0]);

        let sem_id_no_lmfdb = SemanticId::new(1);
        let vec_no_lmfdb = sem_id_no_lmfdb.to_numerical_vector();
        assert_eq!(vec_no_lmfdb, vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_semantic_id_display() {
        let sem_id = SemanticId {
            unique_idx: 1,
            weight: 1.234,
            depth: 2,
            lmfdb_id: Some(LmfdbId("LMFDB_X".to_string())),
            ast_types_count: 5,
            expressions_count: 10,
        };
        let expected = "ID:1-W:1.23-D:2-LMFDB:LMFDB_X-Types:5-Exprs:10";
        assert_eq!(format!("{}", sem_id), expected);

        let sem_id_no_lmfdb = SemanticId::new(2);
        let expected_no_lmfdb = "ID:2-W:0.00-D:0-LMFDB:None-Types:0-Exprs:0";
        assert_eq!(format!("{}", sem_id_no_lmfdb), expected_no_lmfdb);
    }
}
