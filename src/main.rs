use std::collections::HashMap;

use lambda_http::{http::StatusCode, run, service_fn, Body, Error, Request, Response};

/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // For now a Hashmap with the path and the url to redirect too.
    // TODO: Create a json or yaml or something file that is statically embedded at compile time
    // and build the map from that for doing the lookups... We never expect there to be enough to
    // require a proper external data source

    let url_lookup = HashMap::from([
        ("zoom","https://us05web.zoom.us/j/83526723885?pwd=TUtWL2wyTnBoMitFa2RuYnRicUZhQT09"),
        ("discord","https://discord.com/invite/kqhzghtcds"),
        ("recordings","https://tokyo-python.notion.site/Tokyo-Python-Recordings-d45b4e6484164ca7aae02c7142d4d92f")

    ]);

    //TODO: Create a 404 Page and statically embed it in the executable.

    // Extract the path from the request url, and trim the first char '/' from it
    let path: String = event.uri().path().chars().skip(1).collect();

    // Lookup the url, and present a redirect or a 404 if not found

    let resp = match url_lookup.get(path.as_str()) {
        Some(&url) => Response::builder()
            .status(StatusCode::TEMPORARY_REDIRECT)
            .header("Location", url)
            .body("".into())
            .map_err(Box::new)?,
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("The request URL was not found.".into())
            .map_err(Box::new)?,
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
