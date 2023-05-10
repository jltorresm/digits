use digits::guess::Strategy;
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn DigitsForm<'a>(cx: Scope, on_submit: EventHandler<'a, FormEvent>) -> Element<'a> {
    let strategies = Strategy::all()
        .into_iter()
        .map(|strategy| rsx!(option { value: "{strategy as usize}", "{strategy}" }));

    cx.render(rsx! {
        form {
            onsubmit: move |e| on_submit.call(e),
            method: "post",
            prevent_default: "onsubmit",

            div {
                class: "form-floating mb-5 rounded target-container",
                input {
                    r#type: "number",
                    required: true,
                    class: "form-control",
                    id: "target",
                    name: "target",
                    placeholder: 77
                }
                label {r#for: "target", "Target Number"}
            }

            h3 { class: "text-center", "Numbers to Operate" }

            div {
                class: "row operator-container",
                input { name: "operator_1", r#type: "number", required: true,  class: "m-3 form-control operator" }
                input { name: "operator_2", r#type: "number", required: true,  class: "m-3 form-control operator" }
                input { name: "operator_3", r#type: "number", required: true,  class: "m-3 form-control operator" }
            }

            div {
                class: "row mb-1 operator-container",
                input { name: "operator_4", r#type: "number", required: true,  class: "m-3 form-control operator" }
                input { name: "operator_5", r#type: "number", required: true,  class: "m-3 form-control operator" }
                input { name: "operator_6", r#type: "number", required: true,  class: "m-3 form-control operator" }
            }

            select {
                name: "strategy",
                required: true,
                class: "form-select mb-2 w-auto m-auto",
                option { value: "", "Select your strategy" },
                strategies
            }

            button{ r#type:"submit", class: "btn btn-outline-info btn-lg w-100", "Solve!" }
        }
    })
}
