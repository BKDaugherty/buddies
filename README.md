# Buddies Backend

The goal of this project is to create a good way of tracking interactions we have with our pals.

## Development Environment Setup
If you don't already have Rust and Cargo, install it through rustup [by following these instructions](https://www.rust-lang.org/tools/install).

## Development
To figure out how to boot up the server, run `cargo run -- --help` in the root of the project.

## Code Structure

Here's a quick rundown of the structure of the application. I pulled most of the boilerplate from my [notes application which you can see here](https://github.com/BKDaugherty/notes)

### Entrypoint
`src/main.rs` is the entry point of the application. There you'll find the parsing of command line arguments, us deciding which `Storage` we will use, initialization of the logger, request_handler, and webserver.

The majority of the logic however is inside `src/lib`.

I know it looks like a lot of boilerplate, but it provides a good abstraction (I think) in
case we want to swap anything out.

The `src/lib/mod.rs` file defines the modules inside the lib. mod.rs is the file that main looks at when it checks what's inside the lib. You can read about Rust [modules here](https://doc.rust-lang.org/reference/items/modules.html#module-source-filenames) if you're curious but its not too important. There are 4 modules we care about (at the time of this writing at least), and I'll describe them below.

### mod service

The service is where the majority of our application logic should go. The service defines a trait, (Rust's word for interface) that looks something like this.

```
pub trait BuddiesService: Send + Sync + Clone + 'static {
    ... A bunch of methods
}
```

As well as a struct `RequestHandler`
```
#[derive(Clone)]
pub struct RequestHandler<S> {
    pub storage: S,
}

impl<S: BuddiesStore> RequestHandler<S> {
    pub fn new(storage: S) -> RequestHandler<S> {
        RequestHandler { storage }
    }
}

impl<S: BuddiesStore> BuddiesService for RequestHandler<S> {
     ... A bunch more methods
}
```

A RequestHandler implements the BuddiesService interface. This RequestHandler does exactly what you'd think it does. It handles requests. Notice that the `RequestHandler` is generic over the type `S`. Then, the implementations for that structure note that the generic `S` has one trait bound: `S` must implement the `BuddiesStore` trait. In this way, the RequestHandler can communicate with whatever storage we've supplied it with.

## mod storage

Inside the `src/lib/storage` directory, you'll find the trait mentioned above in `traits.rs`. This trait (while currently empty) provides an interface for the RequestHandler to communicate to the database without knowing whether that database is sql, psql, or an in-memory solution.

```
pub trait BuddiesStore: Send + Sync + Clone + 'static {}
```

There's also some boilerplate for a memory version of BuddiesStore in `src/lib/storage/memory.rs`that's currently attached to the RequestHandler in `src/main.rs`. This boilerplate shows that the memory storage implements BuddiesStore, and thus can be supplied to the RequestHandler.

```
#[derive(Clone)]
pub struct MemoryBuddiesStore {}

impl MemoryBuddiesStore {
    pub fn new() -> Self {
	Self {

	}
    }
}

impl BuddiesStore for MemoryBuddiesStore {}
```


## mod routes

`src/lib/routes.rs` is where we connect our `BuddiesService` to the web framework I chose to use ([warp](https://docs.rs/warp/0.3.1/warp/)). Note that this file is where we keep logic that happens before the request has been accepted by the server. It also handles any web shit we need to do. For each endpoint, we write a function, and then slam it all together using warps `or` `Filter`s.

Here's an example.

```
// Function that will translate whatever we accept from the web into something
// our BuddiesService can handle
async fn get_buddies<S: BuddiesStore>(
    user_id: Uuid,
    handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Convert what we get from the Warp Framework into something
    // our service will accept
    let request = GetBuddiesRequest {
      user_id,
    };

    // Call the handler to get the information we need
    let buddies = handler.get_buddies(request);

    // Wrap that back into something warp will understand
    // For now though, just say it's not implemented.
    Ok(warp::reply::with_status(
        "Unimplemented".to_string(),
        http::StatusCode::NOT_IMPLEMENTED,
    ))
}

// Function that builds our routes for us, and
// creates the final web server
pub fn build_warp_routes<S: BuddiesStore>(
    handler: RequestHandler<S>,
) -> BoxedFilter<(impl Reply,)> {
    // Handle annoying web stuff incorrectly
    // because i don't understand security but will soon
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "PUT", "POST"]);

    // Add a filter that will supply our handler to each endpoint function
    let handler_filter = warp::any().map(move || handler.clone());

    ... (more stuff)

    // Function that creates the route
    // /buddies/<uuid of user>
    let get_buddies = warp::get()
        .and(warp::path("buddies"))
        .and(warp::path::param::<Uuid>())
        .and(handler_filter.clone())
        .and_then(get_buddies);

    // Join all of our routes together as one `Filter`
    let routes = get_buddies
	.or(other_routes_go_here)
	... (more routes)
    routes
}
```

While drawing a hard line  between the `routes` and `service` might feel a little silly, I think it's super worth, as honestly, I'm not sure how I feel about `warp` yet as a framework, and there are many others to try. By separating this logic, it makes it easy to potentially swap onto another framework.

It also makes it a lot easier to write tests, as you can just instantiate a RequestHandler and go to town!

## mod types

This is where we'll define all of our sweet sweet types used throughout the application. Today they are all empty, and really only exist so the boilerplate can compile.


