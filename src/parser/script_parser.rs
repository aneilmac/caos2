use crate::commands::Command;
use crate::parser::{caos_skippable0, caos_skippable1};
use crate::parser::CaosParsable;
use nom::Finish;
use nom::multi::separated_list0;
use nom::error::VerboseError;

pub fn parse_caos_script(input: &str) -> Result<(&str, Vec<Command>), VerboseError<&str>> {
    let do_parse = |input| {
        let (input, _) = caos_skippable0(input)?;
        let (input, v) = separated_list0(caos_skippable1, Command::parse_caos)(input)?;
        let (input, _) = caos_skippable0(input)?;
        Ok((input, v))
    };
    do_parse(input).finish()
}

#[cfg(test)]
mod tests {
    use crate::commands::{Anything, Condition, ConditionType, Decimal, Float, Integer, Variable};

    use super::*;

    const CONTENT: &str = r#"
* Simple test
* Of multiline strings.
inst
**moisture monitor
new: simp 1 1 114 "blnk" 1 0 0
tick 9

******CREATE SOME MUCK - to seed the area
reps 100
    new: simp 2 10 37 "graz" 2 218 3000
    attr 192
    elas 0
    setv va00 rand 0 2
    doif va00 eq 0
        accg 0.1
    elif va00 eq 1
        accg 0.3
    else
        accg 0.4
    endi


    mvto rand 217 2787 1840
    perm rand 0 70
repe
"#;

    #[test]
    fn test_script_file_parse() {
        let (_, v) = parse_caos_script(CONTENT).expect("Successful parse");

        assert_eq!(
            v,
            vec![
                Command::Inst,
                Command::NewSimp {
                    family: 1.into(),
                    genus: 1.into(),
                    species: 114.into(),
                    sprite_file: String::from("blnk").into(),
                    image_count: 1.into(),
                    first_image: 0.into(),
                    plane: 0.into()
                },
                Command::Tick {
                    tick_rate: 9.into()
                },
                Command::Reps { count: 100.into() },
                Command::NewSimp {
                    family: 2.into(),
                    genus: 10.into(),
                    species: 37.into(),
                    sprite_file: String::from("graz").into(),
                    image_count: 2.into(),
                    first_image: 218.into(),
                    plane: 3000.into()
                },
                Command::Attr {
                    attributes: 192.into()
                },
                Command::Elas {
                    elasticity: 0.into()
                },
                Command::Setv {
                    var: Variable::Vaxx(0),
                    value: Decimal::Integer(Integer::Rand {
                        value1: Box::new(0.into()),
                        value2: Box::new(2.into())
                    })
                },
                Command::Doif {
                    condition: Condition::Simple {
                        cond_type: ConditionType::Eq,
                        lhs: Anything::Variable(Variable::Vaxx(0)),
                        rhs: Anything::Decimal(Decimal::Integer(0.into()))
                    }
                },
                Command::Accg {
                    acceleration: 0.1f32.into()
                },
                Command::Elif {
                    condition: Condition::Simple {
                        cond_type: ConditionType::Eq,
                        lhs: Anything::Variable(Variable::Vaxx(0)),
                        rhs: Anything::Decimal(Decimal::Integer(1.into()))
                    }
                },
                Command::Accg {
                    acceleration: 0.3f32.into()
                },
                Command::Else,
                Command::Accg {
                    acceleration: 0.4f32.into()
                },
                Command::Endi,
                Command::Mvto {
                    x: Float::FromInteger(Box::new(Integer::Rand {
                        value1: Box::new(217.into()),
                        value2: Box::new(2787.into())
                    })),
                    y: 1840f32.into(),
                },
                Command::Perm {
                    permiability: Integer::Rand {
                        value1: Box::new(0.into()),
                        value2: Box::new(70.into())
                    }
                },
                Command::Repe,
            ]
        );
    }
}
