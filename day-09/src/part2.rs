use std::fmt::{Display, Formatter};
use crate::custom_error::AocError;
use std::num::ParseIntError;
use itertools::Itertools;
use crate::part2::BlockType::{File, Space};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BlockType {
    File(u64),
    Space
}

struct Disk {
    blocks: Vec<(BlockType, u64)>,
}

impl Display for Disk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = self.blocks.iter().map(|(block_type, size)| {
            match block_type {
                File(id) => id.to_string().repeat(*size as usize),
                Space => ".".repeat(*size as usize),
            }
        }).join("");

        write!(f, "{}", string)
    }
}

fn parse(input: &str) -> Result<Disk, ParseIntError> {

    let mut chars = input.trim().chars();

    let mut blocks = vec![(File(0), chars.next().unwrap().to_string().parse::<u64>()?)];
    let mut id = 0;

    while let Some(c) = chars.next() {
        id += 1;
        blocks.push((Space, c.to_string().parse::<u64>()?));
        blocks.push((File(id), chars.next().unwrap().to_string().parse::<u64>()?));
    }

    Ok(Disk{blocks})
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut disk = parse(_input).unwrap();

    let max_id = disk.blocks
        .iter()
        .filter_map(|(t, _)| {
            match t {
                File(id) => Some(*id),
                _ => None,
            }
        }).max().unwrap();

    for id in (0..=max_id).rev() {
        //println!("{}", &disk);
        let file_index = disk.blocks.iter().position(|(t, _)| *t == File(id)).unwrap();
        let file_size = disk.blocks[file_index].1;

        if let Some(space_index) = disk.blocks.iter().position(|(t, s)| {
            (*t == Space) && (*s >= file_size)
        }) {
            if space_index > file_index {
                continue;
            }
            let space_size = disk.blocks[space_index].1;
            disk.blocks.insert(space_index, disk.blocks[file_index]);
            disk.blocks[space_index + 1].1 = space_size - file_size;

            disk.blocks[file_index + 1].0 = Space;

            if let Some((Space, size_succ)) = disk.blocks.get(file_index + 2) {
                disk.blocks[file_index + 1].1 += *size_succ;
                disk.blocks.remove(file_index + 2);
            }

            if let Some((Space, _)) = disk.blocks.get(file_index) {
                disk.blocks[file_index].1 += disk.blocks[file_index + 1].1;
                disk.blocks.remove(file_index + 1);
            }
        }
    }

    let mut sum = 0;
    let mut disk_position = 0;

    for (block_type, size) in disk.blocks.iter() {
        match block_type {
            File(id) => {
                sum += (disk_position..disk_position + *size).sum::<u64>() * *id;
            },
            _ => {},
        }
        disk_position += *size;
    }

    Ok(sum.to_string())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
