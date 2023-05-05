use crate::input_form::digits_form;
use anyhow::anyhow;
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use std::collections::HashMap;

pub fn app(cx: Scope) -> Element {
    let result = use_state(cx, Vec::<String>::new);

    let handle_submit = move |event: FormEvent| {
        let input: DigitsInput = event
            .data
            .values
            .clone()
            .try_into()
            .expect("input parse error");
        let res = digits::guess::operations(
            input.target,
            input.operators,
            digits::guess::Strategy::Shortest,
        );
        result.set(res);
    };

    cx.render(rsx! {
        main {
            class: "digits-web",
            style { include_str!("../src/style.css") }
            h1 { class: "display-1 font-bold text-center mb-5", "Digits Solver" }
            Router {
                Route { to: "/", digits_form { on_submit: handle_submit } "{result:?}" },
                Route { to: "/spin", Spinner { alt: "Loading...".to_string(), hidden: false } },
                Route { to: "", "not found" },
            }
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
pub fn Spinner(cx: Scope, alt: String, hidden: bool) -> Element {
    cx.render(rsx! {
        div {
            hidden: "{hidden}",
            class: "spinner-grow text-light",
            role: "status",
            span { class: "visually-hidden", "{alt}"}
        }
    })
}

#[derive(Clone, Debug)]
pub struct DigitsInput {
    target: u32,
    operators: Vec<u32>,
}

impl TryFrom<HashMap<String, String>> for DigitsInput {
    type Error = anyhow::Error;

    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        let target = value
            .get("target")
            .ok_or(anyhow!("missing target value"))?
            .parse()?;

        let tmp = value
            .into_iter()
            .filter(|(k, _)| k.contains("operator"))
            .map(|(_, v)| v.parse::<u32>())
            .collect::<Vec<_>>();

        if let Some(Err(e)) = tmp.iter().find(|r| r.is_err()) {
            return Err(anyhow!(e.clone()));
        }

        let operators: Vec<u32> = tmp.into_iter().map(Result::unwrap).collect();

        Ok(DigitsInput { target, operators })
    }
}
