use nom::bytes::streaming::take_while1;
use nom::character::is_space;
use nom::character::streaming::space1;
use nom::multi::separated_nonempty_list;
use nom::IResult;

// Experimenting. Broken.

fn tokens(input: &str) -> IResult<&str, Vec<&str>> {
    let token = take_while1(|x| !is_space(x));
    let spaces = space1;
    separated_nonempty_list(spaces, token)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;

    #[test]
    fn tokens_test() {
        assert_eq!(
            tokens("X = 123"),
            Ok((
                "",
                vec![data::token("X"), data::token("="), data::token("123")]
            ))
        )
    }
}
