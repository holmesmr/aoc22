use crate::common::parse_input_as_value_list;
use itertools::Itertools;

/// Get the total of each list in the input.
fn with_totals_from_input<R: std::io::BufRead, F: FnOnce(&mut dyn Iterator<Item = i32>)>(
    input: R,
    f: F,
) {
    let grouped_lists = parse_input_as_value_list::<_, i32>(input).group_by(|item| item.is_ok());
    let mut totals = grouped_lists.into_iter().filter_map(|(is_ok, group)| {
        if is_ok {
            Some(group.filter_map(|item| item.ok()).sum::<i32>())
        } else {
            None
        }
    });

    f(&mut totals as &mut dyn Iterator<Item = i32>);
}

/// Get the largest total calorie count from the list.
///
/// The calorie list is delimited by blank lines.
pub fn solve_part1<R: std::io::BufRead>(input: R) -> anyhow::Result<i32> {
    let mut highest_total = 0;

    with_totals_from_input(input, |totals| {
        highest_total = totals.max().unwrap_or(0);
    });

    Ok(highest_total)
}

/// Get the total of the 3 largest calorie counts from the list.
pub fn solve_part2<R: std::io::BufRead>(input: R) -> anyhow::Result<i32> {
    let mut all_totals = 0;

    with_totals_from_input(input, |totals| {
        let top3 = totals.fold([0, 0, 0], |ys, x| {
            if ys[0] < x {
                let mut out = [x, ys[1], ys[2]];
                out.sort();
                out
            } else {
                ys
            }
        });

        all_totals = top3.iter().sum();
    });

    Ok(all_totals)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const SAMPLE_INPUT: &[u8] = b"\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn part1_sample_produces_expected_result() {
        let input = Cursor::new(SAMPLE_INPUT);

        assert_eq!(solve_part1(input).expect("error parsing input"), 24000);
    }

    #[test]
    fn part2_sample_produces_expected_result() {
        let input = Cursor::new(SAMPLE_INPUT);

        assert_eq!(solve_part2(input).expect("error parsing input"), 45000);
    }
}
