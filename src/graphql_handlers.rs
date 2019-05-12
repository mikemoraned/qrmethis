use rocket::response::content;
use rocket::State;

use crate::graphql_schema::*;

#[rocket::get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    context: State<Query>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, 
                    &context)
}