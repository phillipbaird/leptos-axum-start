use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;

#[server(HelloWorld, "/api")]
pub async fn hello_world() -> Result<(), ServerFnError> {
    crate::server::hello_world();
    Ok(())
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    let hello_world = create_action(cx, |_| hello_world());

    view! {
        cx,
        <Stylesheet id="leptos" href="pkg/project.css"/>
        <Title text="Cargo Leptos Workspace" />
        <div class="flex flex-col bg-green-300 py-32 space-y-8">
            <h1 class="text-3xl text-center">"Hi from your Leptos app with Tailwind!"</h1>
            <button
              class="px-3 py-2 mx-auto border border-black bg-white hover:bg-yellow-200 focus:bg-yellow-200 rounded"
              on:click=move |_| hello_world.dispatch(())
            >
              "Print Hello on the server"
            </button>
        </div>
    }
}

cfg_if! {
  if #[cfg(feature = "ssr")] {
      pub fn register_server_functions() {
          _ = HelloWorld::register();
      }
  }
}
