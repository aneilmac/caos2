use nom::IResult;

/// Represents a type which can be parsed from a string into
///  a CAOS expression.
pub(crate) trait CaosParsable {
    fn parse_caos(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

impl<T> CaosParsable for Box<T>
where
    T: CaosParsable,
{
    fn parse_caos(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (input, t) = T::parse_caos(input)?;
        Ok((input, Box::new(t)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use caos_macros::CaosParsable;
    use nom::character::complete::digit1;
    use nom::combinator::map_res;
    use nom::error::ErrorKind;
    use nom::Finish;

    #[derive(Eq, PartialEq, Debug, CaosParsable)]
    enum Foo {
        #[syntax]
        Helo,
        #[syntax]
        Wrld { recursive: Box<Foo> },
        #[syntax]
        Rec2 {
            recursive1: Box<Foo>,
            recursive2: Box<Foo>,
        },
        #[syntax(name = "cus: name")]
        CusName,
        #[syntax(with_parser = "magic_parse")]
        MagicParse(u32),
    }

    fn magic_parse(input: &str) -> IResult<&str, Foo> {
        map_res(digit1, |s: &str| s.parse::<u32>().map(Foo::MagicParse))(input)
    }

    #[test]
    fn test_failed_args() {
        assert_eq!(
            Foo::parse_caos("not: vald")
                .finish()
                .expect_err("Not a valid string")
                .code,
            ErrorKind::Fail
        );
    }

    #[test]
    fn test_no_args_case_insensitive() {
        assert_eq!(
            Foo::parse_caos("helo").expect("Successful parse").1,
            Foo::Helo
        );
        assert_eq!(
            Foo::parse_caos("HeLo").expect("Successful parse").1,
            Foo::Helo
        );
        assert_eq!(
            Foo::parse_caos("HELO").expect("Successful parse").1,
            Foo::Helo
        );
    }

    #[test]
    fn test_newline() {
        assert_eq!(
            Foo::parse_caos("wrld   \n   helo").expect("A valid pass").1,
            Foo::Wrld {
                recursive: Box::new(Foo::Helo)
            }
        );
    }

    #[test]
    fn test_splitname() {
        assert_eq!(
            Foo::parse_caos("wrld    cus:  \n\t  name")
                .expect("A valid pass")
                .1,
            Foo::Wrld {
                recursive: Box::new(Foo::CusName)
            }
        );
    }

    #[test]
    fn test_custom_name() {
        assert_eq!(
            Foo::parse_caos("cus: name").expect("A valid pass").1,
            Foo::CusName
        );
    }

    #[test]
    fn test_custom_parse() {
        assert_eq!(
            Foo::parse_caos("7").expect("A valid pass").1,
            Foo::MagicParse(7)
        );
    }

    #[test]
    fn test_single_arg() {
        assert_eq!(
            Foo::parse_caos("wrld helo").expect("A valid pass").1,
            Foo::Wrld {
                recursive: Box::new(Foo::Helo)
            }
        );

        assert_eq!(
            Foo::parse_caos("wrld cus: name").expect("A valid pass").1,
            Foo::Wrld {
                recursive: Box::new(Foo::CusName)
            }
        );
    }

    #[test]
    fn test_multi_args_simple() {
        assert_eq!(
            Foo::parse_caos("rec2 helo helo").expect("A valid pass").1,
            Foo::Rec2 {
                recursive1: Box::new(Foo::Helo),
                recursive2: Box::new(Foo::Helo)
            }
        );
    }

    #[test]
    fn test_multi_args_complex() {
        assert_eq!(
            Foo::parse_caos("rec2 wrld helo 2").expect("A valid pass").1,
            Foo::Rec2 {
                recursive1: Box::new(Foo::Wrld {
                    recursive: Box::new(Foo::Helo)
                }),
                recursive2: Box::new(Foo::MagicParse(2))
            }
        );
    }
}
