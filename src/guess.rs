use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, Default)]
pub enum Strategy {
    #[default]
    BruteForce,
}

pub fn operations(target: u32, numbers: Vec<u32>, strategy: Strategy) -> String {
    match strategy {
        Strategy::BruteForce => {
            let res = Vec::new();
            brute(target, numbers.into(), &res);
            format!("{res:?}")
        }
    }
}

fn brute(target: u32, mut numbers: VecDeque<u32>, path: &[String]) -> bool {
    numbers.make_contiguous().sort_by(|a, b| b.cmp(a));
    let combos = nc2(&numbers);

    for (a, b, rest) in combos {
        let (prod, valid_prod, prod_path) = (
            a * b,
            a > 1 && b > 1,
            push_immutable(path, format!("{a} * {b}")),
        );
        let (add, add_path) = (a + b, push_immutable(path, format!("{a} + {b}")));
        let (div, div_exact, div_path) = (
            a / b,
            a % b == 0 && b != 1,
            push_immutable(path, format!("{a} / {b}")),
        );
        let (sub, sub_positive, sub_path) = (
            a - b,
            a - b > 0 && a - b != b,
            push_immutable(path, format!("{a} - {b}")),
        );

        if valid_prod && prod == target {
            println!("{prod_path:?}");
        } else if add == target {
            println!("{add_path:?}");
        } else if div_exact && div == target {
            println!("{div_path:?}");
        } else if sub_positive && sub == target {
            println!("{sub_path:?}");
        }

        if (valid_prod && prod == target)
            || add == target
            || (div_exact && div == target)
            || (sub_positive && sub == target)
        {
            return true;
        }

        // PRODUCT path: a * b
        if valid_prod {
            let mut prod_nums = rest.clone();
            prod_nums.push_front(prod);
            brute(target, prod_nums, &prod_path);
        }

        // ADDITION path: a + b
        let mut add_nums = rest.clone();
        add_nums.push_front(add);
        brute(target, add_nums, &add_path);

        // DIVISION path: a / b
        if div_exact {
            let mut div_nums = rest.clone();
            div_nums.push_front(div);
            brute(target, div_nums, &div_path);
        }

        // SUBTRACTION path: a - b
        if sub_positive {
            let mut sub_nums = rest.clone();
            sub_nums.push_front(sub);
            brute(target, sub_nums, &sub_path);
        }
    }

    false
}

fn nc2(numbers: &VecDeque<u32>) -> VecDeque<(u32, u32, VecDeque<u32>)> {
    let mut res = VecDeque::new();
    let mut rest = numbers.clone();

    while let Some(n) = rest.pop_front() {
        for m in rest.clone() {
            let others = numbers
                .iter()
                .filter(|i| *i.to_owned() != n && *i.to_owned() != m)
                .map(std::borrow::ToOwned::to_owned)
                .collect::<VecDeque<u32>>();
            res.push_back((n, m, others));
        }
    }

    res
}

fn push_immutable(elements: &[String], insert: String) -> Vec<String> {
    let mut new = elements.to_owned();
    new.push(insert);
    new
}
