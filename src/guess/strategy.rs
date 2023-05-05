use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Default)]
pub enum Strategy {
    #[default]
    Shortest = 0,
    Longest = 1,
}

impl From<usize> for Strategy {
    fn from(value: usize) -> Self {
        for s in Strategy::all() {
            if s as usize == value {
                return s;
            }
        }

        panic!("unknown Strategy variant");
    }
}

impl Display for Strategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rep = match self {
            Strategy::Shortest => "Shortest",
            Strategy::Longest => "Longest",
        };
        write!(f, "{rep}")
    }
}

impl Strategy {
    #[must_use]
    pub fn all() -> [Strategy; 2] {
        [Strategy::Shortest, Strategy::Longest]
    }

    pub fn filter(self, mut solutions: Vec<Vec<String>>) -> Vec<String> {
        log::info!("number of raw solutions found -> {}", solutions.len());

        match self {
            Strategy::Shortest => {
                solutions.sort_by_key(Vec::len);
                let shortest_len = solutions[0].len();
                solutions
                    .into_iter()
                    .filter(|a| a.len() == shortest_len)
                    .map(|a| a.join(" | "))
                    .collect()
            }
            Strategy::Longest => {
                solutions.sort_by_key(|a| std::cmp::Reverse(a.len()));
                let longest_len = solutions[0].len();
                solutions
                    .into_iter()
                    .filter(|a| a.len() == longest_len)
                    .map(|a| a.join(" | "))
                    .collect()
            }
        }
    }
}
