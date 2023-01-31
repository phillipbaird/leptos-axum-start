mod error_template;
mod fallback;

use std::sync::Arc;

use axum::{extract::Extension, routing::get, Router};
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

use crate::fallback::file_and_error_handler;

fn app(cx: leptos::Scope) -> impl IntoView {
    use app_package::*;

    view! { cx, <App /> }
}

#[tokio::main]
async fn main() {
    _ = dotenvy::dotenv();

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_address.clone();

    log::info!("serving at {addr}");

    app_package::register_server_functions();

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(app).await;

    // build our application with a route
    let leptos_options = conf.leptos_options;
    let app = Router::new()
        .route("/favicon.ico", get(file_and_error_handler))
        .leptos_routes(leptos_options.clone(), routes, app)
        .fallback(file_and_error_handler)
        .layer(Extension(Arc::new(leptos_options)));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
