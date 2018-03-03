use juniper::FieldResult;
use juniper;
use rocket;
use juniper_rocket;
use rocket::response::content;
use rocket::State;

struct Query;

struct Context;

graphql_object!(Query : Context |&self| {
    field hello(&executor) -> FieldResult<String> {
        Ok("meow".to_string())
    }
});

struct Mutation;

graphql_object!(Mutation : Context |&self| {
    field say_hello(&executor) -> FieldResult<String> {
        Ok("hi".to_string())
    }
});

type Schema = juniper::RootNode<'static, Query, Mutation>;


#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

pub fn start() {
    rocket::ignite()
        .manage(Context)
        .manage(Schema::new(
            Query,
            Mutation,
        ))
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        ).launch();
}