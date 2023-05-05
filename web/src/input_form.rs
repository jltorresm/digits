use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn DigitsForm<'a>(cx: Scope, on_submit: EventHandler<'a, FormEvent>) -> Element<'a> {
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

            h2 { class: "display-6 text-center mb-3", "Numbers to Operate" }

            div {
                class: "row mb-3 operator-container",
                input { name:"operator_1", r#type: "number", required: true,  class: "m-3 form-control operator" }
                input { name:"operator_2", r#type: "number", required: true,  class: "m-3 form-control operator" }
                input { name:"operator_3", r#type: "number", required: true,  class: "m-3 form-control operator" }
            }

            div {
                class: "row mb-3 operator-container",
                input { name:"operator_4", r#type: "number", required: true,  class: "m-3 form-control operator" }
                input { name:"operator_5", r#type: "number", required: true,  class: "m-3 form-control operator" }
                input { name:"operator_6", r#type: "number", required: true,  class: "m-3 form-control operator" }
            }

            button{ r#type:"submit", class: "btn btn-outline-info btn-lg w-100", "Solve!" }
        }
    })
}
