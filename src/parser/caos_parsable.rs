use nom::IResult;

pub trait CaosParsable {
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
