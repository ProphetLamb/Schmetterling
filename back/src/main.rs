pub mod markup;

use std::collections::HashMap;

use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldError, GraphQLEnum, RootNode,
};
use markup::Markup;
use warp::{http::Response, Filter};

#[derive(Clone, Debug)]
struct Context;

impl juniper::Context for Context {}

struct Section {
    id: i32,
    order: i32,
    title: String,
    content: Markup,
}

#[graphql_object(context = Context)]
impl Section {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn order(&self) -> i32 {
        self.order
    }
}

struct Document {
    id: i32,
    order: i32,
    header: Section,
    section: HashMap<i32, Section>,
}

struct Project {
    id: i32,
    order: i32,
    header: Section,
    documents: HashMap<i32, Document>,
}

#[tokio::main]
async fn main() {}
