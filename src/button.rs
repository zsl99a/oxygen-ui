use leptos::*;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Variant {
    #[default]
    Default,
    Solid,
    Outline,
    Ghost,
    Link,
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::Default => write!(f, "default"),
            Variant::Solid => write!(f, "solid"),
            Variant::Outline => write!(f, "outline"),
            Variant::Ghost => write!(f, "ghost"),
            Variant::Link => write!(f, "link"),
        }
    }
}

#[component]
pub fn Button(#[prop(optional)] variant: Variant, children: Children) -> impl IntoView {
    view! {
        <button class="o-btn" variant=variant.to_string()>
            {children()}
        </button>
    }
}
