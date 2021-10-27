use poem::{listener::TcpListener, Route};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};

mod corel;
mod controllers;
mod service;

mod api;

use controllers::{ UserApi };
use corel::{db};
use api::{ clockin::{ clock_contro::{ ClockApi, get_clock_api } }};


struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(
        &self,
        #[oai(name = "name", in = "query")] name: Option<String>, // in="query" means this parameter is parsed from Url
    ) -> PlainText<String> { // PlainText is the response type, which means that the response type of the API is a string, and the Content-Type is `text/plain`
        match name {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    // Lunch Database
    if let Ok(_pool) = db::init_db_pool().await {
        println!("数据库启动成功!!!");
    } 

    // Create a TCP listener
    let listener = TcpListener::bind("127.0.0.1:9000");
  
    // Create API service
    let api_service = OpenApiService::new(
        Api.combine(UserApi).combine(get_clock_api())
        )
        .title("Hello LivStyle")
        .server("http://localhost:9000/api");
  
    // Enable the Swagger UI
    let ui = api_service.swagger_ui(); // "http://localhost:9000"

    // Start the server and specify that the root path of the API is /api, and the path of Swagger UI is /
    poem::Server::new(listener)
        .await?
        .run(Route::new().nest("/api", api_service).nest("/", ui))
        .await
}
