use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <h1>"Hello from Leptos and Spin"</h1>

        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {move || count.get()}
        </button>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
