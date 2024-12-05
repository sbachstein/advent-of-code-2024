use crate::custom_error::AocError;

fn get_row(grid: &Vec<&str>, row: usize) -> String {
    grid.get(row).map(|s| s.to_string()).unwrap_or("".to_string())
}

fn get_col(grid: &Vec<&str>, col: usize) -> String {
    let chars = grid.iter().filter_map(|&row| row.chars().nth(col).map(|char| char.to_string())).collect::<Vec<_>>();
    chars.concat()
}

fn get_descending_diagonal(grid: &Vec<&str>, index: usize) -> String {
    (0..grid.len()).filter_map(|i| {
        let x = index as i32 - grid.len() as i32 + i as i32;
        let y = i;
        if x < 0 {
            None
        } else {
            grid.get(y).and_then(|&str| str.chars().nth(x as usize))
        }.and_then(|c| Some(c.to_string()))
    }).collect::<Vec<_>>().concat()
}

fn get_ascending_diagonal(grid: &Vec<&str>, index: usize) -> String {
    (0..grid.len()).filter_map(|i| {
        let x = index as i32 - grid.len() as i32 + i as i32;
        let y = grid.len() - 1 - i;
        if x < 0 {
            None
        } else {
            grid.get(y).and_then(|&str| str.chars().nth(x as usize))
        }.and_then(|c| Some(c.to_string()))
    }).collect::<Vec<_>>().concat()
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let grid = _input.lines().collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    let row_sum = (0..height).map(
        |row| {
            let str = get_row(&grid, row);
            let count = str.match_indices("XMAS").count();
            let rev_count = str.chars().rev().collect::<String>().match_indices("XMAS").count();

            println!("Row {}", row);
            dbg!(str.match_indices("XMAS").collect::<Vec<_>>());
            dbg!(str.chars().rev().collect::<String>().match_indices("XMAS").collect::<Vec<_>>());

            count + rev_count
        }
    ).sum::<usize>();

    let col_sum = (0..width).map(
        |col| {
            let str = get_col(&grid, col);
            let count = str.match_indices("XMAS").count();
            let rev_count = str.chars().rev().collect::<String>().match_indices("XMAS").count();

            println!("Column {}", col);
            dbg!(str.match_indices("XMAS").collect::<Vec<_>>());
            dbg!(str.chars().rev().collect::<String>().match_indices("XMAS").collect::<Vec<_>>());

            count + rev_count
        }
    ).sum::<usize>();

    let desc_diag_sum = (0..(height + width - 1)).map(
        |diag| {
            let str = get_descending_diagonal(&grid, diag);
            let count = str.match_indices("XMAS").count();
            let rev_count = str.chars().rev().collect::<String>().match_indices("XMAS").count();

            println!("Desc Diag {}", diag);
            dbg!(str.match_indices("XMAS").collect::<Vec<_>>());
            dbg!(str.chars().rev().collect::<String>().match_indices("XMAS").collect::<Vec<_>>());

            count + rev_count
        }
    ).sum::<usize>();

    let asc_diag_sum = (0..(height + width - 1)).map(
        |diag| {
            let str = get_ascending_diagonal(&grid, diag);
            let count = str.match_indices("XMAS").count();
            let rev_count = str.chars().rev().collect::<String>().match_indices("XMAS").count();

            println!("Asc Diag {}", diag);
            dbg!(str.match_indices("XMAS").collect::<Vec<_>>());
            dbg!(str.chars().rev().collect::<String>().match_indices("XMAS").collect::<Vec<_>>());

            count + rev_count
        }
    ).sum::<usize>();

    Ok((row_sum + col_sum + desc_diag_sum + asc_diag_sum).to_string())
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
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
