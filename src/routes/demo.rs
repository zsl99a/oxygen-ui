use leptos::*;

use crate::Button;

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
    ];

    view! {
        <Button>"Hello, World!"</Button>
        <Button>"Hello, World!"</Button>

        <table>
            <For each=move || { colors.clone() } key=|n| *n let:list>
                <tr>
                    <For each=move || { 1..13 } key=|n| *n let:item>
                        <td style=format!(
                            "background-color:var(--{list}-color-{item});width:50px;height:20px",
                        )></td>
                    </For>
                </tr>
            </For>
        </table>
    }
}
