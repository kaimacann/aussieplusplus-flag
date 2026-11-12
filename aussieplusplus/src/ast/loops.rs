use super::{ExprNode, Stmt, Var};

#[derive(Clone, Debug, PartialEq)]
pub struct ForLoop {
    pub var: Var,
    pub range: (RangeBound<ExprNode>, RangeBound<ExprNode>),
    pub body: Vec<Stmt>,
}

impl ForLoop {
    pub fn new(
        var: Var,
        range: (RangeBound<ExprNode>, RangeBound<ExprNode>),
        body: Vec<Stmt>,
    ) -> Self {
        Self { var, range, body }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WhileLoop {
    pub cond: ExprNode,
    pub body: Vec<Stmt>,
}

impl WhileLoop {
    pub fn new(cond: ExprNode, body: Vec<Stmt>) -> Self {
        Self { cond, body }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RangeBound<T> {
    Inclusive(T),
    Exclusive(T),
}

impl RangeBound<ExprNode> {
    pub fn expr(&self) -> &ExprNode {
        match self {
            Self::Inclusive(expr) => expr,
            Self::Exclusive(expr) => expr,
        }
    }

    pub fn expr_mut(&mut self) -> &mut ExprNode {
        match self {
            Self::Inclusive(expr) => expr,
            Self::Exclusive(expr) => expr,
        }
    }

    pub fn to_evaluated(&self, val: f64) -> RangeBound<f64> {
        match self {
            Self::Inclusive(_) => RangeBound::Inclusive(val),
            Self::Exclusive(_) => RangeBound::Exclusive(val),
        }
    }
}

impl RangeBound<f64> {
    pub fn value(&self) -> f64 {
        match self {
            Self::Inclusive(val) => *val,
            Self::Exclusive(val) => *val,
        }
    }
}

pub trait Range<T> {
    fn satisfied(&self, i: T) -> bool;
    fn values(&self) -> (T, T);
    fn iterate(&self, i: &mut T);
}

impl Range<f64> for (RangeBound<f64>, RangeBound<f64>) {
    fn iterate(&self, val: &mut f64) {
        if self.0.value() < self.1.value() {
            *val += 1f64;
        } else if self.0.value() > self.1.value() {
            *val -= 1f64;
        } else if matches!(self, (RangeBound::Inclusive(_), RangeBound::Inclusive(_))) {
            *val += 1f64;
        }
    }

    fn satisfied(&self, i: f64) -> bool {
        match self {
            (RangeBound::Inclusive(start), RangeBound::Inclusive(end)) => {
                if start < end {
                    i >= *start && i <= *end
                } else if start > end {
                    i >= *end && i <= *start
                } else {
                    let error_margin = f64::EPSILON;
                    (i - *start).abs() < error_margin
                }
            }
            (RangeBound::Inclusive(start), RangeBound::Exclusive(end)) => {
                if start < end {
                    i >= *start && i < *end
                } else if start > end {
                    i <= *start && i > *end
                } else {
                    false
                }
            }
            (RangeBound::Exclusive(start), RangeBound::Inclusive(end)) => {
                if start < end {
                    i > *start && i <= *end
                } else if end < start {
                    i >= *end && i < *start
                } else {
                    false
                }
            }
            (RangeBound::Exclusive(start), RangeBound::Exclusive(end)) => {
                if start < end {
                    i > *start && i < *end
                } else if end < start {
                    i > *end && i < *start
                } else {
                    false
                }
            }
        }
    }

    fn values(&self) -> (f64, f64) {
        match self {
            (RangeBound::Inclusive(a), RangeBound::Inclusive(b)) => (*a, *b),
            (RangeBound::Inclusive(a), RangeBound::Exclusive(b)) => {
                if a < b {
                    (*a, *b + 1f64)
                } else {
                    (*a, *b - 1f64)
                }
            }
            (RangeBound::Exclusive(a), RangeBound::Exclusive(b)) => {
                if a < b {
                    (*a + 1f64, *b - 1f64)
                } else {
                    (*a - 1f64, *b + 1f64)
                }
            }
            (RangeBound::Exclusive(a), RangeBound::Inclusive(b)) => {
                if a < b {
                    (*a + 1f64, *b)
                } else {
                    (*a - 1f64, *b)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Range, RangeBound};

    #[test]
    fn descending_inclusive_to_exclusive_range_includes_start_and_excludes_end() {
        let range = (RangeBound::Inclusive(3.0), RangeBound::Exclusive(0.0));

        assert!(range.satisfied(3.0));
        assert!(range.satisfied(2.0));
        assert!(range.satisfied(1.0));
        assert!(!range.satisfied(0.0));
    }

    #[test]
    fn equal_inclusive_to_exclusive_range_is_empty() {
        let range = (RangeBound::Inclusive(3.0), RangeBound::Exclusive(3.0));

        assert!(!range.satisfied(3.0));
    }

    #[test]
    fn equal_exclusive_to_inclusive_range_is_empty() {
        let range = (RangeBound::Exclusive(3.0), RangeBound::Inclusive(3.0));

        assert!(!range.satisfied(3.0));
    }

    #[test]
    fn equal_inclusive_range_advances_after_its_single_value() {
        let range = (RangeBound::Inclusive(3.0), RangeBound::Inclusive(3.0));
        let (mut value, _) = range.values();

        assert!(range.satisfied(value));
        range.iterate(&mut value);
        assert!(!range.satisfied(value));
    }

    #[test]
    fn bound_inclusivity_is_preserved_for_ascending_and_descending_ranges() {
        let cases: Vec<(RangeBound<f64>, RangeBound<f64>, &[f64])> = vec![
            (
                RangeBound::Inclusive(0.0),
                RangeBound::Inclusive(3.0),
                &[0.0, 1.0, 2.0, 3.0],
            ),
            (
                RangeBound::Inclusive(3.0),
                RangeBound::Inclusive(0.0),
                &[3.0, 2.0, 1.0, 0.0],
            ),
            (
                RangeBound::Inclusive(0.0),
                RangeBound::Exclusive(3.0),
                &[0.0, 1.0, 2.0],
            ),
            (
                RangeBound::Inclusive(3.0),
                RangeBound::Exclusive(0.0),
                &[3.0, 2.0, 1.0],
            ),
            (
                RangeBound::Exclusive(0.0),
                RangeBound::Inclusive(3.0),
                &[1.0, 2.0, 3.0],
            ),
            (
                RangeBound::Exclusive(3.0),
                RangeBound::Inclusive(0.0),
                &[2.0, 1.0, 0.0],
            ),
            (
                RangeBound::Exclusive(0.0),
                RangeBound::Exclusive(3.0),
                &[1.0, 2.0],
            ),
            (
                RangeBound::Exclusive(3.0),
                RangeBound::Exclusive(0.0),
                &[2.0, 1.0],
            ),
        ];

        for (start, end, expected) in cases {
            let range = (start, end);
            let (mut current, _) = range.values();
            let mut values = Vec::new();
            for _ in 0..=expected.len() {
                if !range.satisfied(current) {
                    break;
                }
                values.push(current);
                range.iterate(&mut current);
            }
            assert_eq!(values, expected);
        }
    }
}
