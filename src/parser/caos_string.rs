use super::{
    parse_anything, parse_decimal_arg, parse_int_arg, parse_string_literal, parse_variable,
};
use crate::{ast::SString, ast::SStringArg, CaosError, Rule};
use pest::iterators::Pair;

pub fn parse_string(pair: Pair<Rule>) -> Result<SString, CaosError> {
    if pair.as_rule() != Rule::string {
        return Err(CaosError::new_parse_error(pair));
    }

    let pair = pair
        .clone()
        .into_inner()
        .next()
        .ok_or(CaosError::new_parse_error(pair))?;

    match pair.as_rule() {
        Rule::literal_string => parse_string_literal(pair).map(SString::Literal),
        Rule::string_catx => {
            let mut it = pair.clone().into_inner();
            let category_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Catx { category_id })
        }
        Rule::string_hand => Ok(SString::Hand),
        Rule::string_wild => {
            let mut it = pair.clone().into_inner();
            let family = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let genus = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let species = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let tag_stub = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let offset = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Wild {
                family,
                genus,
                species,
                tag_stub,
                offset,
            })
        }
        Rule::string_bkgd => {
            let mut it = pair.clone().into_inner();
            let metaroom_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Bkgd { metaroom_id })
        }
        Rule::string_ptxt => Ok(SString::Ptxt),
        Rule::string_face => Ok(SString::Face),
        Rule::string_dbg => {
            let mut it = pair.clone().into_inner();
            let variable = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Dbg { variable })
        }
        Rule::string_dbga => {
            let mut it = pair.clone().into_inner();
            let variable = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Dbga { variable })
        }
        Rule::string_fvwm => {
            let mut it = pair.clone().into_inner();
            let name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::Fvwm { name })
        }
        Rule::string_innl => Ok(SString::Innl),
        Rule::string_gtos => {
            let mut it = pair.clone().into_inner();
            let slot = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Gtos { slot })
        }
        Rule::string_hist_foto => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let event_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::HistFoto { moniker, event_no })
        }
        Rule::string_hist_mon1 => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let event_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::HistMon1 { moniker, event_no })
        }
        Rule::string_hist_mon2 => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let event_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::HistMon2 { moniker, event_no })
        }
        Rule::string_hist_name => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::HistName { moniker })
        }
        Rule::string_hist_next => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::HistNext { moniker })
        }
        Rule::string_hist_prev => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::HistPrev { moniker })
        }
        Rule::string_hist_utxt => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let event_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::HistUtxt { moniker, event_no })
        }
        Rule::string_hist_wnam => {
            let mut it = pair.clone().into_inner();
            let moniker = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let event_no = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::HistWnam { moniker, event_no })
        }
        Rule::string_bkds => {
            let mut it = pair.clone().into_inner();
            let metaroom_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Bkds { metaroom_id })
        }
        Rule::string_emid => Ok(SString::Emid),
        Rule::string_erid => {
            let mut it = pair.clone().into_inner();
            let metaroom_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Erid { metaroom_id })
        }
        Rule::string_mloc => {
            let mut it = pair.clone().into_inner();
            let metaroom_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Mloc { metaroom_id })
        }
        Rule::string_rate => {
            let mut it = pair.clone().into_inner();
            let room_type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let ca_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Rate {
                room_type,
                ca_index,
            })
        }
        Rule::string_rloc => {
            let mut it = pair.clone().into_inner();
            let room_id = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Rloc { room_id })
        }
        Rule::string_pray_agts => {
            let mut it = pair.clone().into_inner();
            let resource_name = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let string_tag = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let default_value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::PrayAgts {
                resource_name,
                string_tag,
                default_value,
            })
        }
        Rule::string_pray_next => {
            let mut it = pair.clone().into_inner();
            let resource_type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let last_known = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::PrayNext {
                resource_type,
                last_known,
            })
        }
        Rule::string_pray_prev => {
            let mut it = pair.clone().into_inner();
            let resource_type = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let last_known = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::PrayPrev {
                resource_type,
                last_known,
            })
        }
        Rule::string_caos => {
            let mut it = pair.clone().into_inner();
            let inline = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let state_trans = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let p1 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_anything)
                .map(Box::new)?;
            let p2 = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_anything)
                .map(Box::new)?;
            let commands = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let throws = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let catches = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let report = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_variable)
                .map(Box::new)?;
            Ok(SString::Caos {
                inline,
                state_trans,
                p1,
                p2,
                commands,
                throws,
                catches,
                report,
            })
        }
        Rule::string_rmsc => {
            let mut it = pair.clone().into_inner();
            let x = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let y = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Rmsc { x, y })
        }
        Rule::string_vois => Ok(SString::Vois),
        Rule::string_rtif => {
            let mut it = pair.clone().into_inner();
            let real_time = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let format = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::Rtif { real_time, format })
        }
        Rule::string_gamn => {
            let mut it = pair.clone().into_inner();
            let previous = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            Ok(SString::Gamn { previous })
        }
        Rule::string_gnam => Ok(SString::Gnam),
        Rule::string_read => {
            let mut it = pair.clone().into_inner();
            let catalogue_tag = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let offset = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Read {
                catalogue_tag,
                offset,
            })
        }
        Rule::string_subs => {
            let mut it = pair.clone().into_inner();
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_string_arg)
                .map(Box::new)?;
            let start = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            let count = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Subs {
                value,
                start,
                count,
            })
        }
        Rule::string_vtos => {
            let mut it = pair.clone().into_inner();
            let value = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_decimal_arg)
                .map(Box::new)?;
            Ok(SString::Vtos { value })
        }
        Rule::string_pswd => {
            let mut it = pair.clone().into_inner();
            let world_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Pswd { world_index })
        }
        Rule::string_wnam => Ok(SString::Wnam),
        Rule::string_wrld => {
            let mut it = pair.clone().into_inner();
            let world_index = it
                .next()
                .ok_or(CaosError::new_parse_error(pair.clone()))
                .and_then(parse_int_arg)
                .map(Box::new)?;
            Ok(SString::Wrld { world_index })
        }
        Rule::string_wuid => Ok(SString::Wuid),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

pub fn parse_string_arg(pair: Pair<Rule>) -> Result<SStringArg, CaosError> {
    match pair.as_rule() {
        Rule::string => parse_string(pair).map(SStringArg::String),
        Rule::variable => parse_variable(pair).map(SStringArg::Variable),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Anything, Decimal, DecimalArg, IntArg, Integer, SString, Variable},
        parser::CaosParser,
    };
    use pest::Parser;

    use super::*;

    #[test]
    fn test_string_arg() {
        for p in CaosParser::parse(Rule::string_arg, r#""Hello""#).expect("Parsed") {
            assert_eq!(
                parse_string_arg(p).expect("Parsed variable"),
                SStringArg::String(SString::Literal("Hello".into())),
            );
        }
        for p in CaosParser::parse(Rule::float_arg, "MV32").expect("Parsed") {
            assert_eq!(
                parse_string_arg(p).expect("Parsed variable"),
                SStringArg::Variable(Variable::Mvxx(32)),
            );
        }
    }

    #[test]
    fn test_string_literal() {
        for p in CaosParser::parse(Rule::string, r#""Hello""#).expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Literal("Hello".into()),
            );
        }
    }

    #[test]
    fn test_string_catx() {
        for p in CaosParser::parse(Rule::string, "CATX ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Catx {
                    category_id: Box::new(IntArg::Primary(Integer::Attr))
                },
            );
        }
    }

    #[test]
    fn test_string_hand() {
        for p in CaosParser::parse(Rule::string, "hand").expect("Parsed") {
            assert_eq!(parse_string(p).expect("Parsed variable"), SString::Hand,);
        }
    }

    #[test]
    fn test_string_wild() {
        for p in CaosParser::parse(Rule::string, "wild ATTR attr ATTR HAND ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Wild {
                    family: Box::new(IntArg::Primary(Integer::Attr)),
                    genus: Box::new(IntArg::Primary(Integer::Attr)),
                    species: Box::new(IntArg::Primary(Integer::Attr)),
                    tag_stub: Box::new(SStringArg::String(SString::Hand)),
                    offset: Box::new(IntArg::Primary(Integer::Attr)),
                },
            );
        }
    }

    #[test]
    fn test_string_bkgd() {
        for p in CaosParser::parse(Rule::string, "bkgd ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Bkgd {
                    metaroom_id: Box::new(IntArg::Primary(Integer::Attr))
                },
            );
        }
    }

    #[test]
    fn test_string_ptxt() {
        for p in CaosParser::parse(Rule::string, "ptxt").expect("Parsed") {
            assert_eq!(parse_string(p).expect("Parsed variable"), SString::Ptxt,);
        }
    }

    #[test]
    fn test_string_face() {
        for p in CaosParser::parse(Rule::string, "face").expect("Parsed") {
            assert_eq!(parse_string(p).expect("Parsed variable"), SString::Face,);
        }
    }

    #[test]
    fn test_string_dbg() {
        for p in CaosParser::parse(Rule::string, "dbg# ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Dbg {
                    variable: Box::new(IntArg::Primary(Integer::Attr))
                },
            );
        }
    }

    #[test]
    fn test_string_dbga() {
        for p in CaosParser::parse(Rule::string, "dbga ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Dbga {
                    variable: Box::new(IntArg::Primary(Integer::Attr))
                },
            );
        }
    }

    #[test]
    fn test_string_fvwm() {
        for p in CaosParser::parse(Rule::string, "fvwm face").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Fvwm {
                    name: Box::new(SStringArg::String(SString::Face))
                },
            );
        }
    }

    #[test]
    fn test_string_innl() {
        for p in CaosParser::parse(Rule::string, "innl").expect("Parsed") {
            assert_eq!(parse_string(p).expect("Parsed variable"), SString::Innl,);
        }
    }

    #[test]
    fn test_string_gtos() {
        for p in CaosParser::parse(Rule::string, "gtos ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Gtos {
                    slot: Box::new(IntArg::Primary(Integer::Attr))
                },
            );
        }
    }

    #[test]
    fn test_string_hist_foto() {
        for p in CaosParser::parse(Rule::string, "hist foto HAND ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::HistFoto {
                    moniker: Box::new(SStringArg::String(SString::Hand)),
                    event_no: Box::new(IntArg::Primary(Integer::Attr))
                },
            );
        }
    }

    #[test]
    fn test_string_hist_mon1() {
        for p in CaosParser::parse(Rule::string, "hist mon1 HAND ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::HistMon1 {
                    moniker: Box::new(SStringArg::String(SString::Hand)),
                    event_no: Box::new(IntArg::Primary(Integer::Attr))
                },
            );
        }
    }

    #[test]
    fn test_string_hist_mon2() {
        for p in CaosParser::parse(Rule::string, "hist mon2 HAND ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::HistMon2 {
                    moniker: Box::new(SStringArg::String(SString::Hand)),
                    event_no: Box::new(IntArg::Primary(Integer::Attr))
                },
            );
        }
    }

    #[test]
    fn test_string_hist_name() {
        for p in CaosParser::parse(Rule::string, "hist name HAND").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::HistName {
                    moniker: Box::new(SStringArg::String(SString::Hand))
                },
            );
        }
    }

    #[test]
    fn test_string_hist_next() {
        for p in CaosParser::parse(Rule::string, "hist next HAND").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::HistNext {
                    moniker: Box::new(SStringArg::String(SString::Hand))
                },
            );
        }
    }

    #[test]
    fn test_string_hist_prev() {
        for p in CaosParser::parse(Rule::string, "hist prev HAND").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::HistPrev {
                    moniker: Box::new(SStringArg::String(SString::Hand))
                },
            );
        }
    }

    #[test]
    fn test_string_hist_utxt() {
        for p in CaosParser::parse(Rule::string, "hist utxt HAND ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::HistUtxt {
                    moniker: Box::new(SStringArg::String(SString::Hand)),
                    event_no: Box::new(IntArg::Primary(Integer::Attr))
                },
            );
        }
    }

    #[test]
    fn test_string_hist_wnam() {
        for p in CaosParser::parse(Rule::string, "hist wnam HAND ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::HistWnam {
                    moniker: Box::new(SStringArg::String(SString::Hand)),
                    event_no: Box::new(IntArg::Primary(Integer::Attr))
                },
            );
        }
    }

    #[test]
    fn test_string_bkds() {
        for p in CaosParser::parse(Rule::string, "bkds ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Bkds {
                    metaroom_id: Box::new(IntArg::Primary(Integer::Attr))
                },
            );
        }
    }

    #[test]
    fn test_string_emid() {
        for p in CaosParser::parse(Rule::string, "EMID").expect("Parsed") {
            assert_eq!(parse_string(p).expect("Parsed variable"), SString::Emid,);
        }
    }

    #[test]
    fn test_string_erid() {
        for p in CaosParser::parse(Rule::string, "ERID ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Erid {
                    metaroom_id: Box::new(IntArg::Primary(Integer::Attr))
                },
            );
        }
    }

    #[test]
    fn test_string_mloc() {
        for p in CaosParser::parse(Rule::string, "MLOC ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Mloc {
                    metaroom_id: Box::new(IntArg::Primary(Integer::Attr))
                },
            );
        }
    }

    #[test]
    fn test_string_rate() {
        for p in CaosParser::parse(Rule::string, "RATE ATTR ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Rate {
                    room_type: Box::new(IntArg::Primary(Integer::Attr)),
                    ca_index: Box::new(IntArg::Primary(Integer::Attr))
                }
            );
        }
    }

    #[test]
    fn test_string_rloc() {
        for p in CaosParser::parse(Rule::string, "RLOc ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Rloc {
                    room_id: Box::new(IntArg::Primary(Integer::Attr))
                }
            );
        }
    }

    #[test]
    fn test_string_pray_agts() {
        for p in CaosParser::parse(Rule::string, "pray agts HAND HAND HAND").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::PrayAgts {
                    resource_name: Box::new(SStringArg::String(SString::Hand)),
                    string_tag: Box::new(SStringArg::String(SString::Hand)),
                    default_value: Box::new(SStringArg::String(SString::Hand))
                }
            );
        }
    }

    #[test]
    fn test_string_pray_next() {
        for p in CaosParser::parse(Rule::string, "pray NEXT HAND HAND").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::PrayNext {
                    resource_type: Box::new(SStringArg::String(SString::Hand)),
                    last_known: Box::new(SStringArg::String(SString::Hand))
                }
            );
        }
    }

    #[test]
    fn test_string_pray_prev() {
        for p in CaosParser::parse(Rule::string, "PRAY prev hand HAND").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::PrayPrev {
                    resource_type: Box::new(SStringArg::String(SString::Hand)),
                    last_known: Box::new(SStringArg::String(SString::Hand))
                }
            );
        }
    }

    #[test]
    fn test_string_caos() {
        for p in CaosParser::parse(Rule::string, "CAOS ATTR ATTR ATTR ATTR HAND ATTR attr MV00")
            .expect("Parsed")
        {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Caos {
                    inline: Box::new(IntArg::Primary(Integer::Attr)),
                    state_trans: Box::new(IntArg::Primary(Integer::Attr)),
                    p1: Box::new(Anything::Decimal(Decimal::Integer(Integer::Attr))),
                    p2: Box::new(Anything::Decimal(Decimal::Integer(Integer::Attr))),
                    commands: Box::new(SStringArg::String(SString::Hand)),
                    throws: Box::new(IntArg::Primary(Integer::Attr)),
                    catches: Box::new(IntArg::Primary(Integer::Attr)),
                    report: Box::new(Variable::Mvxx(0)),
                }
            );
        }
    }

    #[test]
    fn test_string_rmsc() {
        for p in CaosParser::parse(Rule::string, "RMSC ATTR ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Rmsc {
                    x: Box::new(IntArg::Primary(Integer::Attr)),
                    y: Box::new(IntArg::Primary(Integer::Attr)),
                }
            );
        }
    }

    #[test]
    fn test_string_vois() {
        for p in CaosParser::parse(Rule::string, "VOIS").expect("Parsed") {
            assert_eq!(parse_string(p).expect("Parsed variable"), SString::Vois);
        }
    }

    #[test]
    fn test_string_rtif() {
        for p in CaosParser::parse(Rule::string, "RTIF ATTR HAND").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Rtif {
                    real_time: Box::new(IntArg::Primary(Integer::Attr)),
                    format: Box::new(SStringArg::String(SString::Hand))
                }
            );
        }
    }

    #[test]
    fn test_string_gamn() {
        for p in CaosParser::parse(Rule::string, "GAMN HAND").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Gamn {
                    previous: Box::new(SStringArg::String(SString::Hand))
                }
            );
        }
    }

    #[test]
    fn test_string_gnam() {
        for p in CaosParser::parse(Rule::string, "GNAM").expect("Parsed") {
            assert_eq!(parse_string(p).expect("Parsed variable"), SString::Gnam);
        }
    }

    #[test]
    fn test_string_read() {
        for p in CaosParser::parse(Rule::string, "READ hAND ATTr").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Read {
                    catalogue_tag: Box::new(SStringArg::String(SString::Hand)),
                    offset: Box::new(IntArg::Primary(Integer::Attr))
                }
            );
        }
    }

    #[test]
    fn test_string_subs() {
        for p in CaosParser::parse(Rule::string, "SUBS hAND ATTR ATTR").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Subs {
                    value: Box::new(SStringArg::String(SString::Hand)),
                    start: Box::new(IntArg::Primary(Integer::Attr)),
                    count: Box::new(IntArg::Primary(Integer::Attr))
                }
            );
        }
    }

    #[test]
    fn test_string_vtos() {
        for p in CaosParser::parse(Rule::string, "VTOS %1").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Vtos {
                    value: Box::new(DecimalArg::Decimal(Decimal::Integer(Integer::Literal(1))))
                }
            );
        }
    }

    #[test]
    fn test_string_pswd() {
        for p in CaosParser::parse(Rule::string, "PSWD 0").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Pswd {
                    world_index: Box::new(IntArg::Primary(Integer::Literal(0)))
                }
            );
        }
    }

    #[test]
    fn test_string_wnam() {
        for p in CaosParser::parse(Rule::string, "WNAM").expect("Parsed") {
            assert_eq!(parse_string(p).expect("Parsed variable"), SString::Wnam);
        }
    }

    #[test]
    fn test_string_wrld() {
        for p in CaosParser::parse(Rule::string, "wrld 0").expect("Parsed") {
            assert_eq!(
                parse_string(p).expect("Parsed variable"),
                SString::Wrld {
                    world_index: Box::new(IntArg::Primary(Integer::Literal(0)))
                }
            );
        }
    }

    #[test]
    fn test_string_wuid() {
        for p in CaosParser::parse(Rule::string, "WUID").expect("Parsed") {
            assert_eq!(parse_string(p).expect("Parsed variable"), SString::Wuid);
        }
    }
}
