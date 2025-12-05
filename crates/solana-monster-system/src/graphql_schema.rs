use async_graphql::{Object, Schema, SimpleObject, ID, Context};
use serde::{Deserialize, Serialize};
use crate::solana_rocksdb_storage::SolanaRocksStorage;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct RustType {
    pub id: ID,
    pub name: String,
    pub kind: TypeKind,
    pub monster_factor: u64,
    pub dependencies: Vec<ID>,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct RustFunction {
    pub id: ID,
    pub name: String,
    pub input_types: Vec<ID>,
    pub output_type: ID,
    pub monster_factor: u64,
}

#[derive(async_graphql::Enum, Serialize, Deserialize)]
pub enum TypeKind {
    Struct,
    Enum,
    Trait,
    Function,
    Impl,
}

pub struct Query;

#[Object]
impl Query {
    async fn rust_type(&self, ctx: &Context<'_>, id: ID) -> async_graphql::Result<Option<RustType>> {
        let storage = ctx.data::<SolanaRocksStorage>()?;
        Ok(storage.get_chunk(&id.to_string()).ok().flatten().and_then(|data| 
            bincode::deserialize(&data).ok()
        ))
    }

    async fn rust_function(&self, ctx: &Context<'_>, id: ID) -> async_graphql::Result<Option<RustFunction>> {
        let storage = ctx.data::<SolanaRocksStorage>()?;
        Ok(storage.get_chunk(&id.to_string()).ok().flatten().and_then(|data| 
            bincode::deserialize(&data).ok()
        ))
    }

    async fn types_by_factor(&self, ctx: &Context<'_>, factor: u64) -> async_graphql::Result<Vec<RustType>> {
        let storage = ctx.data::<SolanaRocksStorage>()?;
        let chunks = storage.get_chunks_by_factor(factor)?;
        Ok(chunks.into_iter().filter_map(|data| 
            bincode::deserialize(&data).ok()
        ).collect())
    }

    async fn all_types(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<RustType>> {
        let storage = ctx.data::<SolanaRocksStorage>()?;
        let chunks = storage.get_all_chunks()?;
        Ok(chunks.into_iter().filter_map(|data| 
            bincode::deserialize(&data).ok()
        ).collect())
    }
}

pub type GraphQLSchema = Schema<Query, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub fn create_schema(storage: SolanaRocksStorage) -> GraphQLSchema {
    Schema::build(Query, async_graphql::EmptyMutation, async_graphql::EmptySubscription)
        .data(storage)
        .finish()
}
