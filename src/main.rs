use poem::{
    endpoint::{EmbeddedFilesEndpoint, EmbeddedFileEndpoint},
    EndpointExt,
    error::NotFoundError,
    http::StatusCode,
    IntoResponse,
    listener::TcpListener,
    Route,
    Server,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "frontend/dist"]
pub struct FrontendAssets;

async fn not_found_handler(_: NotFoundError) -> impl IntoResponse {
    "404 Not Found".with_status(StatusCode::NOT_FOUND)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let app1 = Route::new()
        .nest("/", EmbeddedFilesEndpoint::<FrontendAssets>::new())
        .catch_error(not_found_handler);

    Server::new(TcpListener::bind("127.0.0.1:8081")).run(app1).await?;

    // let app2 = Route::new()
    //     .at("/index.html", EmbeddedFileEndpoint::<FrontendAssets>::new("index.html"))
    //     .catch_error(not_found_handler);

    // Server::new(TcpListener::bind("127.0.0.1:8082")).run(app2).await?;

    Ok(())
}
