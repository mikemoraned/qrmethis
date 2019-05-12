use juniper::{EmptyMutation, FieldResult, RootNode};

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
}

pub struct Context;

impl juniper::Context for Context {}

pub struct Query;

juniper::graphql_object!(Query: Context |&self| {

    field human() -> FieldResult<Human> {
        let human = Human {
            id: "123".into(),
            name: "Chesney Hawkes".into()
        };

        Ok(human)
    }
});

pub type Schema = RootNode<'static, Query, EmptyMutation<Query>>;