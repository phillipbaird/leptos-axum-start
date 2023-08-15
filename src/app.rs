use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[server(HelloWorld, "/api")]
pub async fn hello_world() -> Result<(), ServerFnError> {
    // Call the server internals.
    crate::server::hello_world();
    Ok(())
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="pkg/project.css"/>

        // sets the document title
        <Title text="Leptos Axum Tailwind starter" />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=|| view! {  <HomePage/> }/>
                </Routes>
            </main>
        </Router>

    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let hello_world = create_action(|_| hello_world());

    view! {
        <div class="flex flex-col bg-green-300 py-32 space-y-8">
            <h1 class="text-3xl text-center">"Hi from your Leptos app with Tailwind!"</h1>

            <button
                class="px-3 py-2 mx-auto border border-black bg-white hover:text-white hover:bg-slate-700 rounded"
                on:click=move |_| hello_world.dispatch(())
            >
                "Print Hello on the server"
            </button>

            <button on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}
