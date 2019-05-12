use rocket;
use juniper_rocket;

use crate::graphql_schema;

#[get("/")]
pub fn graphiql() -> rocket::response::content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
pub fn get_graphql_handler(
    context: rocket::State<graphql_schema::Model>,
    request: juniper_rocket::GraphQLRequest,
    schema: rocket::State<graphql_schema::Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    context: rocket::State<graphql_schema::Model>,
    request: juniper_rocket::GraphQLRequest,
    schema: rocket::State<graphql_schema::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}