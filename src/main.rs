use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <h1>"Hello from Leptos and Spin"</h1>

        <h2>"Spin Cloud Preview - Testing, Testing, 1..2..3! And making a trivial change..."</h2>

        <p>
            "For ease of development, run `trunk watch` in one terminal while also running `spin watch` in another terminal -- on changes, the only thing you need to do is reload the browser window: Leptos and Spin will take care of the rest!"
        </p>

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
