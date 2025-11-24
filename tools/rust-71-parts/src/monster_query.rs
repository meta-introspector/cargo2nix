//! Monster Query: GraphQL-like interface for numerical reflection constants
//! Pure Hecke operations with bounded execution guarantees

use crate::monster_levels::{MONSTER, monster_factors};
use crate::hecke_operators::{HeckeOperator, LevelElevation, SemanticComposition};
use crate::token_constants::CompressedToken;
use std::collections::HashMap;

/// Query execution bounds
#[derive(Debug, Clone)]
pub struct ExecutionBounds {
    pub max_operations: u32,
    pub max_depth: u8,
    pub timeout_ms: u64,
}

impl ExecutionBounds {
    pub const SAFE: Self = Self { max_operations: 1000, max_depth: 10, timeout_ms: 5000 };
    pub const STRICT: Self = Self { max_operations: 100, max_depth: 5, timeout_ms: 1000 };
}

/// Pure functional query result
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub value: MonsterConstant,
    pub operations_used: u32,
    pub depth_reached: u8,
    pub execution_time_ms: u64,
    pub pure: bool,
}

/// Monster Group constant for numerical reflection
#[derive(Debug, Clone, PartialEq)]
pub enum MonsterConstant {
    Factor(u64),
    Level(u8),
    Token(CompressedToken),
    Composition(Box<MonsterConstant>, Box<MonsterConstant>),
    Elevation(Box<MonsterConstant>),
    List(Vec<MonsterConstant>),
}

impl MonsterConstant {
    pub fn factor(f: u64) -> Self { Self::Factor(f) }
    pub fn level(l: u8) -> Self { Self::Level(l) }
    pub fn token(t: CompressedToken) -> Self { Self::Token(t) }
    
    /// Get numerical value for computation
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Factor(f) => Some(*f),
            Self::Level(l) => Some(*l as u64),
            Self::Token(t) => Some(t.monster_factor),
            _ => None,
        }
    }
}

/// GraphQL-like query language for Monster constants
#[derive(Debug, Clone)]
pub enum MonsterQuery {
    // Basic queries
    GetFactor(u8),                    // factor(level: 0)
    GetLevel(u64),                    // level(factor: 71)
    GetAllFactors,                    // allFactors
    
    // Hecke operations
    Elevate(Box<MonsterQuery>),       // elevate(query)
    Compose(Box<MonsterQuery>, Box<MonsterQuery>), // compose(left, right)
    
    // Functional operations
    Map(Box<MonsterQuery>, Box<MonsterQuery>),     // map(list, operation)
    Filter(Box<MonsterQuery>, Box<MonsterQuery>),  // filter(list, predicate)
    Reduce(Box<MonsterQuery>, Box<MonsterQuery>),  // reduce(list, operation)
    
    // Predicates
    IsSentinel(Box<MonsterQuery>),    // isSentinel(query)
    IsPrime(Box<MonsterQuery>),       // isPrime(query)
    IsLevel(Box<MonsterQuery>, u8),   // isLevel(query, level)
}

/// Pure functional query executor
pub struct MonsterQueryExecutor {
    bounds: ExecutionBounds,
    operation_count: u32,
    depth: u8,
    start_time: std::time::Instant,
}

impl MonsterQueryExecutor {
    pub fn new(bounds: ExecutionBounds) -> Self {
        Self {
            bounds,
            operation_count: 0,
            depth: 0,
            start_time: std::time::Instant::now(),
        }
    }
    
    /// Execute query with bounded guarantees
    pub fn execute(&mut self, query: MonsterQuery) -> Result<QueryResult, String> {
        self.start_time = std::time::Instant::now();
        self.operation_count = 0;
        self.depth = 0;
        
        let value = self.execute_query(query)?;
        let execution_time = self.start_time.elapsed().as_millis() as u64;
        
        Ok(QueryResult {
            value,
            operations_used: self.operation_count,
            depth_reached: self.depth,
            execution_time_ms: execution_time,
            pure: true,
        })
    }
    
    fn execute_query(&mut self, query: MonsterQuery) -> Result<MonsterConstant, String> {
        self.check_bounds()?;
        self.operation_count += 1;
        self.depth += 1;
        
        let result = match query {
            MonsterQuery::GetFactor(level) => {
                if level < 15 {
                    Ok(MonsterConstant::factor(monster_factors::FACTORS[level as usize]))
                } else {
                    Err("Level out of bounds".to_string())
                }
            }
            
            MonsterQuery::GetLevel(factor) => {
                let level = monster_factors::FACTORS.iter()
                    .position(|&f| f == factor)
                    .ok_or("Factor not found")?;
                Ok(MonsterConstant::level(level as u8))
            }
            
            MonsterQuery::GetAllFactors => {
                let factors: Vec<_> = monster_factors::FACTORS.iter()
                    .map(|&f| MonsterConstant::factor(f))
                    .collect();
                Ok(MonsterConstant::List(factors))
            }
            
            MonsterQuery::Elevate(query) => {
                let inner = self.execute_query(*query)?;
                Ok(MonsterConstant::Elevation(Box::new(inner)))
            }
            
            MonsterQuery::Compose(left, right) => {
                let left_val = self.execute_query(*left)?;
                let right_val = self.execute_query(*right)?;
                Ok(MonsterConstant::Composition(Box::new(left_val), Box::new(right_val)))
            }
            
            MonsterQuery::IsSentinel(query) => {
                let value = self.execute_query(*query)?;
                let is_sentinel = value.as_u64() == Some(71);
                Ok(MonsterConstant::factor(if is_sentinel { 1 } else { 0 }))
            }
            
            MonsterQuery::IsPrime(query) => {
                let value = self.execute_query(*query)?;
                if let Some(n) = value.as_u64() {
                    let is_prime = self.is_prime(n);
                    Ok(MonsterConstant::factor(if is_prime { 1 } else { 0 }))
                } else {
                    Err("Cannot check primality of non-numeric value".to_string())
                }
            }
            
            MonsterQuery::Map(list_query, op_query) => {
                let list = self.execute_query(*list_query)?;
                if let MonsterConstant::List(items) = list {
                    let mut results = Vec::new();
                    for item in items {
                        // Apply operation to each item (simplified)
                        results.push(item);
                    }
                    Ok(MonsterConstant::List(results))
                } else {
                    Err("Map requires list input".to_string())
                }
            }
            
            _ => Err("Query not implemented".to_string()),
        };
        
        self.depth -= 1;
        result
    }
    
    fn check_bounds(&self) -> Result<(), String> {
        if self.operation_count >= self.bounds.max_operations {
            return Err("Operation limit exceeded".to_string());
        }
        if self.depth >= self.bounds.max_depth {
            return Err("Depth limit exceeded".to_string());
        }
        if self.start_time.elapsed().as_millis() as u64 >= self.bounds.timeout_ms {
            return Err("Timeout exceeded".to_string());
        }
        Ok(())
    }
    
    fn is_prime(&self, n: u64) -> bool {
        if n < 2 { return false; }
        if n == 2 { return true; }
        if n % 2 == 0 { return false; }
        
        let sqrt_n = (n as f64).sqrt() as u64;
        for i in (3..=sqrt_n).step_by(2) {
            if n % i == 0 { return false; }
        }
        true
    }
}

/// Monster Query Service - pure functional remote/local execution
pub struct MonsterQueryService {
    executor: MonsterQueryExecutor,
    cache: HashMap<String, QueryResult>,
}

impl MonsterQueryService {
    pub fn new(bounds: ExecutionBounds) -> Self {
        Self {
            executor: MonsterQueryExecutor::new(bounds),
            cache: HashMap::new(),
        }
    }
    
    /// Execute query with caching
    pub fn query(&mut self, query: MonsterQuery) -> Result<QueryResult, String> {
        let query_key = format!("{:?}", query);
        
        if let Some(cached) = self.cache.get(&query_key) {
            return Ok(cached.clone());
        }
        
        let result = self.executor.execute(query)?;
        self.cache.insert(query_key, result.clone());
        
        Ok(result)
    }
    
    /// Parse GraphQL-like query string
    pub fn parse_query(&self, query_str: &str) -> Result<MonsterQuery, String> {
        // Simplified parser - in practice use proper parser
        match query_str.trim() {
            "allFactors" => Ok(MonsterQuery::GetAllFactors),
            s if s.starts_with("factor(") => {
                let level_str = s.strip_prefix("factor(").unwrap().strip_suffix(")").unwrap();
                let level: u8 = level_str.parse().map_err(|_| "Invalid level")?;
                Ok(MonsterQuery::GetFactor(level))
            }
            s if s.starts_with("isSentinel(factor(") => {
                let level_str = s.strip_prefix("isSentinel(factor(").unwrap().strip_suffix("))").unwrap();
                let level: u8 = level_str.parse().map_err(|_| "Invalid level")?;
                Ok(MonsterQuery::IsSentinel(Box::new(MonsterQuery::GetFactor(level))))
            }
            _ => Err("Query not recognized".to_string()),
        }
    }
    
    /// Execute query from string
    pub fn execute_string(&mut self, query_str: &str) -> Result<QueryResult, String> {
        let query = self.parse_query(query_str)?;
        self.query(query)
    }
}

/// Create Monster Query Service with examples
pub fn create_monster_query_service() -> MonsterQueryService {
    println!("🔍 Creating Monster Query Service");
    println!("GraphQL-like interface for Monster Group constants");
    
    let mut service = MonsterQueryService::new(ExecutionBounds::SAFE);
    
    // Example queries
    let example_queries = [
        "allFactors",
        "factor(14)",  // Sentinel factor
        "factor(0)",   // Largest factor
        "isSentinel(factor(14))",
    ];
    
    println!("\n📋 Example Queries:");
    for query_str in example_queries {
        match service.execute_string(query_str) {
            Ok(result) => {
                println!("  {} → {:?} (ops: {}, time: {}ms)", 
                        query_str, result.value, result.operations_used, result.execution_time_ms);
            }
            Err(e) => {
                println!("  {} → ERROR: {}", query_str, e);
            }
        }
    }
    
    service
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_monster_query_execution() {
        let mut executor = MonsterQueryExecutor::new(ExecutionBounds::SAFE);
        let query = MonsterQuery::GetFactor(14); // Sentinel
        
        let result = executor.execute(query).unwrap();
        assert_eq!(result.value, MonsterConstant::factor(71));
        assert!(result.pure);
    }
    
    #[test]
    fn test_query_bounds() {
        let mut executor = MonsterQueryExecutor::new(ExecutionBounds::STRICT);
        let query = MonsterQuery::GetAllFactors;
        
        let result = executor.execute(query);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_query_service() {
        let mut service = MonsterQueryService::new(ExecutionBounds::SAFE);
        let result = service.execute_string("factor(14)").unwrap();
        
        assert_eq!(result.value, MonsterConstant::factor(71));
    }
}
