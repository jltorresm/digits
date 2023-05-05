use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct Props {
    hidden: bool,
    results: Vec<String>,
}

#[allow(non_snake_case)]
pub fn Results(cx: Scope<Props>) -> Element {
    let results: Vec<Vec<String>> = cx
        .props
        .results
        .iter()
        .map(|r| r.split('|').map(str::to_string).collect())
        .collect();

    let rendered_results = results
        .into_iter()
        .map(|steps| rsx!(Result { steps: steps }));

    cx.render(rsx! {
        article {
            hidden: "{cx.props.hidden}",
            class: "results p-3 grid gap-2",
            rendered_results,
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn Result(cx: Scope, steps: Vec<String>) -> Element {
    let rendered_steps = steps.iter().map(|step| {
        rsx!(Step {
            value: step.clone()
        })
    });

    cx.render(rsx! {
        div {
            class: "result card g-col-6",
            "data-bs-theme": "dark",
            div {
                class: "card-body grid gap-1",
                rendered_steps
            }
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn Step(cx: Scope, value: String) -> Element {
    cx.render(rsx! {
        span { class: "badge text-bg-light g-col-4", "{value}" }
    })
}
