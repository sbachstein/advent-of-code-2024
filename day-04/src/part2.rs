use crate::custom_error::AocError;

fn get_block(grid: &Vec<&str>, x: usize, y: usize) -> Vec<Vec<String>> {
    (y..y + 3)
        .filter_map(
            |row| grid.get(row).map(
                |&line| (x..x + 3).filter_map(
                    |col| line.chars().nth(col).and_then(|c| Some(c.to_string()))
                ).collect::<Vec<_>>()
            ))
        .collect::<Vec<_>>()
}

fn block_valid(block: Vec<Vec<String>>) -> bool {
    block.get(1).and_then(|row| row.get(1)) == Some(&"A".to_string())
        && (
            block.get(0).and_then(|row| row.get(0)) == Some(&"M".to_string())
                && block.get(2).and_then(|row| row.get(2)) == Some(&"S".to_string())
            || block.get(0).and_then(|row| row.get(0)) == Some(&"S".to_string())
                && block.get(2).and_then(|row| row.get(2)) == Some(&"M".to_string())

        )
        && (
            (
                block.get(0).and_then(|row| row.get(2)) == Some(&"M".to_string())
                && block.get(2).and_then(|row| row.get(0)) == Some(&"S".to_string())
            ) || (
                block.get(0).and_then(|row| row.get(2)) == Some(&"S".to_string())
                    && block.get(2).and_then(|row| row.get(0)) == Some(&"M".to_string())
                )
    )
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let grid = _input.lines().collect::<Vec<_>>();

    let count = (0..grid.len() - 2).flat_map(
        |y| (0..grid[0].len() - 2).map(
            move |x| (x, y)
        )
    )
    .filter_map(
        |(x, y)| {
            println!("{:?}", get_block(&grid, x, y));
            match block_valid(get_block(&grid, x, y)) {
                true => Some(()),
                false => None
            }
        }
    )
    .count();

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
