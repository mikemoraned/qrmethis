use juniper::{FieldResult, EmptyMutation, RootNode};

#[derive(juniper::GraphQLObject)]
#[graphql(description="A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
}

struct Context;

impl juniper::Context for Context {}

struct Query;

juniper::graphql_object!(Query: Context |&self| {

    field human() -> FieldResult<Human> {
        let human = Human {
            id: "123",
            name: "Chesney Hawkes"
        };

        Ok(human)
    }
});

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Query>>;