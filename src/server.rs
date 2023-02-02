async fn graphql(
  schema: Arc<Schema>,
  ctx: Arc<Context>,
  req: GraphQLRequest,
) -> Result<impl warp::Reply, Infallible> {
  let res = req.execute(&schema, &ctx);
  let json = serde_json::to_string(&res).expect("Invalid JSON response");
  Ok(json)
}

async fn start() {
  let schema = Arc::new(Schema::new(Query, Mutation));
  // Create a warp filter for the schema
  let schema = warp::any().map(move || Arc::clone(&schema));

  let ctx = Arc::new(Context { data: String::from("Here's your test data") });
  // Create a warp filter for the context
  let ctx = warp::any().map(move || Arc::clone(&ctx));

  let graphql_route = warp::post()
      .and(warp::path!("graphql"))
      .and(schema.clone())
      .and(ctx.clone())
      .and(warp::body::json())
      .and_then(graphql);

  let graphiql_route = warp::get()
      .and(warp::path!("graphiql"))
      .map(|| warp::reply::html(graphiql_source("graphql")));

  let routes = graphql_route.or(graphiql_route);

  warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}