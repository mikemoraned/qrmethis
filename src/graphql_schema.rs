use juniper::{EmptyMutation, FieldResult, RootNode};
use url::Url;

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A message, represented as a QR code")]
struct QrCodeMessage {
    message: String,
    img_url: Url
}

pub struct Model {
    base_url: Url,
}

impl Model {
    pub fn new(base_url: Url) -> Model {
        Model { base_url }
    }

    pub fn url_for(&self, message: String) -> Url {
        let path = format!("/{}", message);

        let options = Url::options();
        options.base_url(Some(&self.base_url)).parse(&path).unwrap()
    }
}

impl juniper::Context for Model {}

pub struct QueryRoot;

juniper::graphql_object!(QueryRoot: Model |&self| {

    description: "Root object",

    field qr_code(&executor, message: String) -> FieldResult<QrCodeMessage> {
        let model = executor.context();
        Ok(QrCodeMessage {
            message: message.clone(),
            img_url: model.url_for(message.clone())
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Model>>;