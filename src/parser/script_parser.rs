use crate::commands::{Command};
use crate::parser::CaosParsable;
use crate::parser::{caos_skippable0, caos_skippable1};
use nom::combinator::all_consuming;
use nom::error::VerboseError;
use nom::multi::separated_list0;
use nom::Finish;

pub fn parse_caos_script(input: &str) -> Result<(&str, Vec<Command>), VerboseError<&str>> {
    let do_parse = |input| {
        let (input, _) = caos_skippable0(input)?;
        let (input, v) = separated_list0(caos_skippable1, Command::parse_caos)(input)?;
        let (input, _) = caos_skippable0(input)?;
        Ok((input, v))
    };
    all_consuming(do_parse)(input).finish()
}

#[cfg(test)]
mod tests {
    use crate::commands::{
        Anything, Condition, ConditionType, Decimal, FloatArg, IntArg, Integer, Variable,
    };

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
                    family: IntArg::from_primary(1.into()),
                    genus: IntArg::from_primary(1.into()),
                    species: IntArg::from_primary(114.into()),
                    sprite_file: String::from("blnk").into(),
                    image_count: IntArg::from_primary(1.into()),
                    first_image: IntArg::from_primary(0.into()),
                    plane: IntArg::from_primary(0.into())
                },
                Command::Tick {
                    tick_rate: IntArg::from_primary(9.into())
                },
                Command::Reps {
                    count: IntArg::from_primary(100.into())
                },
                Command::NewSimp {
                    family: IntArg::from_primary(2.into()),
                    genus: IntArg::from_primary(10.into()),
                    species: IntArg::from_primary(37.into()),
                    sprite_file: String::from("graz").into(),
                    image_count: IntArg::from_primary(2.into()),
                    first_image: IntArg::from_primary(218.into()),
                    plane: IntArg::from_primary(3000.into())
                },
                Command::Attr {
                    attributes: IntArg::from_primary(192.into())
                },
                Command::Elas {
                    elasticity: IntArg::from_primary(0.into())
                },
                Command::Setv {
                    var: Variable::Vaxx(0),
                    value: Decimal::Integer(Integer::Rand {
                        value1: Box::new(IntArg::from_primary(0.into())),
                        value2: Box::new(IntArg::from_primary(2.into()))
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
                    acceleration: FloatArg::from_primary(0.1f32.into())
                },
                Command::Elif {
                    condition: Condition::Simple {
                        cond_type: ConditionType::Eq,
                        lhs: Anything::Variable(Variable::Vaxx(0)),
                        rhs: Anything::Decimal(Decimal::Integer(1.into()))
                    }
                },
                Command::Accg {
                    acceleration: FloatArg::from_primary(0.3f32.into())
                },
                Command::Else,
                Command::Accg {
                    acceleration: FloatArg::from_primary(0.4f32.into())
                },
                Command::Endi,
                Command::Mvto {
                    x: FloatArg::from_castable(Integer::Rand {
                        value1: Box::new(IntArg::from_primary(217.into())),
                        value2: Box::new(IntArg::from_primary(2787.into()))
                    }),
                    y: FloatArg::from_primary(1840f32.into()),
                },
                Command::Perm {
                    permiability: IntArg::from_primary(Integer::Rand {
                        value1: Box::new(IntArg::from_primary(0.into())),
                        value2: Box::new(IntArg::from_primary(70.into()))
                    })
                },
                Command::Repe,
            ]
        );
    }
}
