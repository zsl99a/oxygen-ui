use leptos::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! { <button class="o-btn">{children()}</button> }
}
