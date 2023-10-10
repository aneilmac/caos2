use super::Anything;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Condition {
    Simple {
        cond_type: ConditionType,
        lhs: Anything,
        rhs: Anything,
    },
    Combination {
        c_lhs: Box<Condition>,
        c_rhs: Box<Condition>,
        join_type: JoinType,
    },
}

impl Condition {
    pub fn join(self, other: Condition, new_join_type: JoinType) -> Self {
        match self {
            s @ Self::Simple { .. } => Self::Combination {
                c_lhs: Box::new(s),
                c_rhs: Box::new(other),
                join_type: new_join_type,
            },
            Self::Combination {
                c_lhs,
                c_rhs,
                join_type,
            } => Self::Combination {
                c_lhs,
                c_rhs: Box::new(c_rhs.join(other, new_join_type)),
                join_type,
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum JoinType {
    And,
    Or,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ConditionType {
    Eq,
    Ne,
    Ge,
    Gt,
    Le,
    Lt,
}
