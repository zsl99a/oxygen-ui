use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    error_template::{AppError, ErrorTemplate},
    DemoPage, HomePage,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let theme = "light";
    // let theme = "dark";

    view! {
        <Html lang="en" attributes=vec![("theme", Attribute::String(theme.into())),]/>

        <Stylesheet id="leptos" href="/pkg/oxygen-ui.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="demo" view=DemoPage/>
                </Routes>
            </main>
        </Router>
    }
}
