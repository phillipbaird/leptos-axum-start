use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use project::app::*;
        use project::fileserv::file_and_error_handler;

        use axum::{routing::post, Router};
        use leptos::*;
        use leptos_axum::{generate_route_list, LeptosRoutes};

        fn app() -> impl IntoView {
          view! { <App /> }
        }

        #[tokio::main]
        async fn main() {
            simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

            // Setting get_configuration(None) means we'll be using cargo-leptos's env values
            // For deployment these variables are:
            // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
            // Alternately a file can be specified such as Some("Cargo.toml")
            // The file would need to be included with the executable when moved to deployment
            let conf = get_configuration(None).await.unwrap();
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;

            // Generate the list of routes in your Leptos App
            let routes = generate_route_list(app).await;

            // build our application with a route
            let app = Router::new()
                .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
                .leptos_routes(&leptos_options, routes, app)
                .fallback(file_and_error_handler)
                .with_state(leptos_options);

            // run our app with hyper
            // `axum::Server` is a re-export of `hyper::Server`
            log!("listening on {}", addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    }

    else
    {
        pub fn main() {

        }
    }

}
