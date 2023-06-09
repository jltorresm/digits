use crate::cache::Cache;
use crate::input_form::DigitsForm;
use crate::results::Results;
use anyhow::anyhow;
use digits::guess::Strategy;
use dioxus::prelude::*;
use serde_json::json;
use std::collections::HashMap;

pub fn app(cx: Scope) -> Element {
    let result: &UseState<Option<Vec<String>>> = use_state(cx, || None);
    let cache = Cache::new();

    let handle_submit = move |event: FormEvent| {
        let input: DigitsInput = event
            .data
            .values
            .clone()
            .try_into()
            .expect("input parse error");

        let res = if let Some(res) = cache.get(&input.key()) {
            log::info!("cache hit {} -> {res}", input.key());
            serde_json::from_str(&res).expect("error while deserialising cache entry")
        } else {
            let res =
                digits::guess::operations(input.target, input.operators.clone(), input.strategy);
            cache.set(input.key().as_str(), json!(res).to_string().as_str());
            res
        };

        result.set(Some(res));
    };

    cx.render(rsx! {
        main {
            class: "digits-web text-center",
            style { include_str!("../src/style.css") }
            h1 { class: "display-1 font-bold text-center", "Digits Solver" }
            p {
                class: "text-center mb-1",
                "Naive solver for the New York Times Game ",
                a {
                    href: "https://www.nytimes.com/games/digits",
                    "Digits"
                }
            }
            p {
                class: "mb-5",
                "see the code in "
                a {
                    href: "https://github.com/jltorresm/digits",
                    "GitHub"
                }
            }
            DigitsForm { on_submit: handle_submit }
            Results { hidden: result.is_none(), results: result.get().clone().unwrap_or_default() }
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
    strategy: Strategy,
}

impl DigitsInput {
    pub fn key(&self) -> String {
        let mut ops = self.operators.clone();
        ops.sort_unstable();
        format!("t:{}|o:{:?}|s:{}", self.target.clone(), ops, self.strategy)
    }
}

impl TryFrom<HashMap<String, String>> for DigitsInput {
    type Error = anyhow::Error;

    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        let target = value
            .get("target")
            .ok_or(anyhow!("missing target value"))?
            .parse()?;

        let strategy = value
            .get("strategy")
            .ok_or(anyhow!("missing strategy value"))?
            .parse::<usize>()?
            .into();

        let tmp = value
            .into_iter()
            .filter(|(k, _)| k.contains("operator"))
            .map(|(_, v)| v.parse::<u32>())
            .collect::<Vec<_>>();

        if let Some(Err(e)) = tmp.iter().find(|r| r.is_err()) {
            return Err(anyhow!(e.clone()));
        }

        let operators: Vec<u32> = tmp.into_iter().map(Result::unwrap).collect();

        Ok(DigitsInput {
            target,
            operators,
            strategy,
        })
    }
}
