use nom::error::VerboseError;
use nom::IResult;

pub(crate) type CaosParseResult<I, O> = IResult<I, O, VerboseError<I>>;
