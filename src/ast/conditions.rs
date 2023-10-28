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
