use leptos::*;

use crate::{Button, Variant};

#[component]
pub fn DemoPage() -> impl IntoView {
    let colors = vec![
        "gray",
        "gray-dark",
        "blue",
        "indigo",
        "purple",
        "green",
        "red",
        "yellow",
        "orange",
        "pink",
        "cyan",
        "teal",
        "primary",
        "secondary",
    ];

    view! {
        <div style="padding:10px">
            <Button>{"Default"}</Button>
            <Button variant=Variant::Solid>{"Solid"}</Button>
            <Button variant=Variant::Outline>{"Outline"}</Button>
            <Button variant=Variant::Ghost>{"Ghost"}</Button>
            <Button variant=Variant::Link>{"Link"}</Button>
        </div>

        <table>
            <For each=move || { colors.clone() } key=|n| *n let:list>
                <tr>
                    <For each=move || { 1..13 } key=|n| *n let:item>
                        <td style=format!(
                            "background-color:var(--{list}-color-{item});width:140px;height:24px",
                        )>{format!("--{}-color-{}", list, item)}</td>
                    </For>
                </tr>
            </For>
        </table>
    }
}
