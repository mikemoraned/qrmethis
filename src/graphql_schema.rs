use juniper::{EmptyMutation, FieldResult, RootNode};

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A message, represented as a QR code")]
struct QrCodeMessage {
    message: String
}

pub struct Model {}

impl Model {
    pub fn new() -> Model {
        Model {}
    }
}

impl juniper::Context for Model {}

pub struct QueryRoot;

juniper::graphql_object!(QueryRoot: Model |&self| {

    description: "Root object",

    field qr_code(message: String) -> FieldResult<QrCodeMessage> {
        Ok(QrCodeMessage {
            message
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Model>>;