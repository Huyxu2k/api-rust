use std::time::Duration;

use tower::{ServiceBuilder, BoxError};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{prelude::*};
use  axum::{Router,
            routing::{get}, error_handling::HandleErrorLayer, http::StatusCode
};


//constants app
const DATABASE_URL:&str=env!("DATABASE_URL");
const APP_ADDRESS:&str=env!("APP_ADDRESS");//"0.0.0.0";//
const APP_PORT:&str=env!("APP_PORT");//"8081";//

#[tokio::main]
async fn main(){

    tracing_subscriber::fmt()
                       .with_max_level(tracing::Level::DEBUG)
                       //.with_env_filter("api-rust=debug,tower_http=debug")
                       .try_init().unwrap();

    let addr=format!("{}:{}",APP_ADDRESS,APP_PORT).parse::<std::net::SocketAddr>().unwrap();
    tracing::debug!("listening on {}",addr);

    //app
    let app= app();

    //server build
    axum::Server::bind(&addr)
         .serve(app.into_make_service())
         .await
         .unwrap();
        
   // Ok(())
}

fn app()->Router{
    Router::new()
          .route("/", get(root))
          .layer(
            ServiceBuilder::new()
                           .layer(HandleErrorLayer::new(|error:BoxError| async move{
                             if error.is::<tower::timeout::error::Elapsed>(){
                                Ok(StatusCode::REQUEST_TIMEOUT)
                             }
                             else {
                                Err((
                                    StatusCode::INTERNAL_SERVER_ERROR,format!("Unhandled internal error:{}",error),
                                ))
                             }
                           }))
                            .timeout(Duration::from_secs(10))
                            .layer(TraceLayer::new_for_http())
                            .into_inner()
          )
}
async fn root()->&'static str{
    "WELLCOME HAHAH!!!"
}