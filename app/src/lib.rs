use leptos::*;
use leptos_meta::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="pkg/project.css"/>
        <Title text="Cargo Leptos Workspace" />
        <div class="bg-green-300 py-32">
            <h1 class="w-full text-3xl text-center">"Hi from your Leptos app with Tailwind!"</h1>
        </div>
    }
}
