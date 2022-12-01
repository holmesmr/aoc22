use std::str::FromStr;

use anyhow::{anyhow, Context};

#[macro_export]
macro_rules! inline_input {
    ($day:literal) => {{
        let bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/",
            $day,
            ".txt",
        ));

        ::std::io::Cursor::new(bytes)
    }};
}

pub fn transform_input_as_value_list<F, R, T>(
    input: R,
    transformer: F,
) -> impl Iterator<Item = anyhow::Result<T>>
where
    F: Fn(usize, String) -> anyhow::Result<T>,
    R: std::io::BufRead,
{
    input.lines().enumerate().map(move |(i, s)| {
        transformer(
            i,
            s.with_context(|| format!("IO error while reading input on line {}", i + 1))?,
        )
    })
}

pub fn parse_input_as_value_list<R, T>(input: R) -> impl Iterator<Item = anyhow::Result<T>>
where
    R: std::io::BufRead,
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    transform_input_as_value_list(input, |i, s| {
        str::parse::<T>(&*s).with_context(|| format!("Parse error on line {}", i + 1))
    })
}

pub fn parse_input_as_value_list_anyhow<R, T>(input: R) -> impl Iterator<Item = anyhow::Result<T>>
where
    R: std::io::BufRead,
    T: FromStr,
    <T as FromStr>::Err: AsRef<dyn std::error::Error + Send + Sync + 'static>,
{
    transform_input_as_value_list(input, |i, s| {
        str::parse::<T>(&*s).map_err(|e| anyhow!("Parse error on line {}: {}", i + 1, e.as_ref()))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parses_input_list_of_ints_correctly() {
        let input_str = b"1\n2\n3";
        let reader = Cursor::new(input_str);

        let parsed = parse_input_as_value_list(reader)
            .collect::<anyhow::Result<Vec<i32>>>()
            .unwrap();

        assert_eq!(&*parsed, &[1i32, 2i32, 3i32][..]);
    }

    #[test]
    fn allows_trailing_newlines() {
        let input_str = b"1\n2\n3\n";
        let reader = Cursor::new(input_str);

        let parsed = parse_input_as_value_list(reader)
            .collect::<anyhow::Result<Vec<i32>>>()
            .unwrap();

        assert_eq!(&*parsed, &[1i32, 2i32, 3i32][..]);
    }

    #[test]
    fn error_if_not_parseable() {
        let input_str = b"1\nfoo\n3";
        let reader = Cursor::new(input_str);

        assert!(parse_input_as_value_list(reader)
            .collect::<anyhow::Result<Vec<i32>>>()
            .is_err());
    }
}
