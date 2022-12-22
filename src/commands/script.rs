use crate::parser::{caos_skippable1, CaosParsable};
use nom::{bytes::complete::tag_no_case, character::complete::u32};

pub struct ScriptHeader {
    family: u32,
    genus: u32,
    species: u32,
    script_number: u32,
}

impl CaosParsable for ScriptHeader {
    fn parse_caos(input: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        let (input, _) = tag_no_case("scrp")(input)?;
        let (input, _) = caos_skippable1(input)?;
        let (input, family) = u32(input)?;
        let (input, _) = caos_skippable1(input)?;
        let (input, genus) = u32(input)?;
        let (input, _) = caos_skippable1(input)?;
        let (input, species) = u32(input)?;
        let (input, _) = caos_skippable1(input)?;
        let (input, script_number) = u32(input)?;
        Ok((
            input,
            ScriptHeader {
                family,
                genus,
                species,
                script_number,
            },
        ))
    }
}
