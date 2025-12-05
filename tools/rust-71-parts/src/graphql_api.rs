//! Simple GraphQL API using Juniper

use crate::declaration_splitter::{MonsterDeclarationSplitter, DeclarationType};
use juniper::{GraphQLObject, FieldResult, RootNode, EmptySubscription};

#[derive(GraphQLObject)]
pub struct TraitNode {
    pub name: String,
    pub monster_factor: i32,
    pub transport_layer: i32,
    pub declaration_type: String,
}

#[derive(GraphQLObject)]
pub struct PipelineState {
    pub total_declarations: i32,
    pub traits: i32,
    pub structs: i32,
    pub functions: i32,
}

pub struct Context {
    pub splitter: MonsterDeclarationSplitter,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    fn pipeline_state(context: &Context) -> FieldResult<PipelineState> {
        Ok(PipelineState {
            total_declarations: context.splitter.declarations.len() as i32,
            traits: context.splitter.get_declarations_by_type(DeclarationType::Trait).len() as i32,
            structs: context.splitter.get_declarations_by_type(DeclarationType::Struct).len() as i32,
            functions: context.splitter.get_declarations_by_type(DeclarationType::Function).len() as i32,
        })
    }

    fn traits_by_factor(context: &Context, factor: i32) -> FieldResult<Vec<TraitNode>> {
        let traits = context.splitter.declarations
            .iter()
            .filter(|d| d.monster_factor == factor as u64 && d.declaration_type == DeclarationType::Trait)
            .map(|d| TraitNode {
                name: d.name.clone(),
                monster_factor: d.monster_factor as i32,
                transport_layer: d.transport_layer as i32,
                declaration_type: format!("{:?}", d.declaration_type),
            })
            .collect();
        Ok(traits)
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    fn hello() -> FieldResult<String> {
        Ok("Hello from Monster Group API!".to_string())
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
