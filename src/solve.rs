use crate::{ExpressionKind, Tableau};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Solution {
    variables: Vec<(String, bool)>,
}

impl Solution {
    pub const fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<bool> {
        self.variables
            .iter()
            .find(|(n, _)| n == name.as_ref())
            .map(|(_, v)| *v)
    }

    pub fn contains(&self, name: impl AsRef<str>, value: bool) -> bool {
        self.variables
            .iter()
            .any(|(n, v)| n == name.as_ref() && *v == value)
    }

    pub fn push(&mut self, name: impl Into<String>, value: bool) {
        self.variables.push((name.into(), value));
    }

    pub fn len(&self) -> usize {
        self.variables.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, bool)> {
        self.variables.iter().map(|(n, v)| (n.as_str(), *v))
    }
}

#[derive(Clone, Debug, Default)]
pub struct Solutions {
    solutions: Vec<Solution>,
}

impl Solutions {
    pub const fn new() -> Self {
        Self {
            solutions: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.solutions.is_empty()
    }

    pub fn len(&self) -> usize {
        self.solutions.len()
    }

    /// Pushes a variable on to all solutions.
    pub fn push(&mut self, name: impl Into<String>, value: bool) {
        let name = name.into();

        self.solutions.retain_mut(|solution| {
            if let Some(v) = solution.get(&name) {
                v == value
            } else {
                solution.push(name.clone(), value);

                true
            }
        });
    }

    /// Removes all redundant solutions.
    pub fn clean(&mut self) {
        let mut redunant = Vec::new();

        for (i, s_a) in self.solutions.iter().enumerate() {
            for (j, s_b) in self.solutions.iter().enumerate() {
                if s_a == s_b && !redunant.contains(&i) && i != j {
                    redunant.push(j);
                }

                if s_a.len() < s_b.len() {
                    if s_a.iter().all(|(n, v)| s_b.contains(n, v)) {
                        redunant.push(j);
                    }
                }
            }
        }

        let mut i = 0;
        self.solutions.retain(|_| {
            let retain = !redunant.contains(&i);
            i += 1;
            retain
        });
    }

    /// Returns an [`Iterator`] over all solutions.
    pub fn iter(&self) -> impl Iterator<Item = &Solution> {
        self.solutions.iter()
    }
}

impl From<&Tableau> for Solutions {
    fn from(tableau: &Tableau) -> Self {
        let mut this = Solutions::new();

        if tableau.branches.is_empty() {
            this.solutions.push(Solution::new());
        }

        for branch in tableau.branches.iter() {
            let mut solutions = Self::from(branch);

            this.solutions.append(&mut solutions.solutions);
        }

        for expectation in tableau.expectations.iter() {
            match expectation.expr.kind.as_ref() {
                ExpressionKind::Atomic(atomic) => {
                    this.push(&atomic.ident, expectation.truth_value);
                }
                ExpressionKind::TruthValue(truth_value) => {
                    if truth_value.value != expectation.truth_value {
                        this.solutions.clear();
                    }
                }
                _ => {}
            }
        }

        this
    }
}
