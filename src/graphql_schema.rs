use juniper::{EmptyMutation, FieldResult, RootNode};

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
}

pub struct Model {
    
}

impl Model {
    pub fn new() -> Model {
        Model {
            
        }
    }
}

impl juniper::Context for Model {}

pub struct QueryRoot;

juniper::graphql_object!(QueryRoot: Model |&self| {

    description: "Root object",

    field human() -> FieldResult<Human> {
        let human = Human {
            id: "123".into(),
            name: "Chesney Hawkes".into()
        };

        Ok(human)
    }
});

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Model>>;