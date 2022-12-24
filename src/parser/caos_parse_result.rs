use nom::error::VerboseError;
use nom::IResult;

pub type CaosParseResult<I, O> = IResult<I, O, VerboseError<I>>;
