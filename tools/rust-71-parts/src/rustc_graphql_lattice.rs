//! Each rustc datatype as a GraphQL lattice point

use juniper::{GraphQLObject, GraphQLEnum, FieldResult, ID};
use crate::monster_levels::MONSTER;

// Core rustc lattice points as GraphQL types

#[derive(GraphQLObject)]
pub struct TokenKind {
    pub id: ID,
    pub name: String,
    pub monster_factor: i32,
    pub layer: i32,
}

#[derive(GraphQLObject)]
pub struct Span {
    pub id: ID,
    pub start: i32,
    pub end: i32,
    pub monster_factor: i32,
}

#[derive(GraphQLObject)]
pub struct Expr {
    pub id: ID,
    pub kind: String,
    pub span: Span,
    pub monster_factor: i32,
    pub layer: i32,
}

#[derive(GraphQLObject)]
pub struct Stmt {
    pub id: ID,
    pub kind: String,
    pub monster_factor: i32,
    pub layer: i32,
}

#[derive(GraphQLObject)]
pub struct Item {
    pub id: ID,
    pub name: String,
    pub kind: String,
    pub monster_factor: i32,
    pub layer: i32,
}

#[derive(GraphQLObject)]
pub struct Ty {
    pub id: ID,
    pub kind: String,
    pub monster_factor: i32,
    pub layer: i32,
}

#[derive(GraphQLObject)]
pub struct TraitDef {
    pub id: ID,
    pub name: String,
    pub methods: Vec<String>,
    pub monster_factor: i32,
    pub layer: i32,
}

#[derive(GraphQLObject)]
pub struct ImplBlock {
    pub id: ID,
    pub trait_name: Option<String>,
    pub type_name: String,
    pub monster_factor: i32,
    pub layer: i32,
}

#[derive(GraphQLEnum)]
pub enum LatticePointType {
    Token,
    Expr,
    Stmt,
    Item,
    Ty,
    Trait,
    Impl,
}

#[derive(GraphQLObject)]
pub struct LatticePoint {
    pub id: ID,
    pub point_type: LatticePointType,
    pub monster_factor: i32,
    pub layer: i32,
    pub connections: Vec<ID>,
}

pub struct Context {
    pub lattice_points: std::collections::HashMap<String, LatticePoint>,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    fn lattice_point(context: &Context, id: ID) -> FieldResult<Option<LatticePoint>> {
        Ok(context.lattice_points.get(&id.to_string()).cloned())
    }
    
    fn lattice_points_by_type(context: &Context, point_type: LatticePointType) -> FieldResult<Vec<LatticePoint>> {
        let points = context.lattice_points
            .values()
            .filter(|p| p.point_type == point_type)
            .cloned()
            .collect();
        Ok(points)
    }
    
    fn lattice_points_by_factor(context: &Context, factor: i32) -> FieldResult<Vec<LatticePoint>> {
        let points = context.lattice_points
            .values()
            .filter(|p| p.monster_factor == factor)
            .cloned()
            .collect();
        Ok(points)
    }
    
    fn lattice_points_by_layer(context: &Context, layer: i32) -> FieldResult<Vec<LatticePoint>> {
        let points = context.lattice_points
            .values()
            .filter(|p| p.layer == layer)
            .cloned()
            .collect();
        Ok(points)
    }
    
    fn connected_points(context: &Context, id: ID) -> FieldResult<Vec<LatticePoint>> {
        if let Some(point) = context.lattice_points.get(&id.to_string()) {
            let connected = point.connections
                .iter()
                .filter_map(|conn_id| context.lattice_points.get(&conn_id.to_string()))
                .cloned()
                .collect();
            Ok(connected)
        } else {
            Ok(vec![])
        }
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    fn add_lattice_point(
        context: &Context, 
        point_type: LatticePointType,
        monster_factor: i32,
        layer: i32
    ) -> FieldResult<LatticePoint> {
        let id = ID::new(format!("{}_{}_{}_{}", 
            format!("{:?}", point_type).to_lowercase(),
            monster_factor, 
            layer,
            context.lattice_points.len()
        ));
        
        let point = LatticePoint {
            id: id.clone(),
            point_type,
            monster_factor,
            layer,
            connections: vec![],
        };
        
        Ok(point)
    }
}

pub type Schema = RootNode<'static, Query, Mutation, juniper::EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, juniper::EmptySubscription::new())
}

pub fn create_context() -> Context {
    let mut lattice_points = std::collections::HashMap::new();
    
    // Add sample rustc lattice points
    let token_point = LatticePoint {
        id: ID::new("token_1".to_string()),
        point_type: LatticePointType::Token,
        monster_factor: 71,
        layer: 0,
        connections: vec![ID::new("expr_1".to_string())],
    };
    
    let expr_point = LatticePoint {
        id: ID::new("expr_1".to_string()),
        point_type: LatticePointType::Expr,
        monster_factor: 59,
        layer: 1,
        connections: vec![ID::new("stmt_1".to_string())],
    };
    
    lattice_points.insert("token_1".to_string(), token_point);
    lattice_points.insert("expr_1".to_string(), expr_point);
    
    Context { lattice_points }
}
