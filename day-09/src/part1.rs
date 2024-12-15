use crate::custom_error::AocError;
use crate::part1::BlockType::{File, Space};
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy)]
enum BlockType {
    File(u64),
    Space,
}

#[derive(Debug)]
struct Disk {
    blocks: Vec<(BlockType, u8)>,
}

fn parse(input: &str) -> Result<Disk, ParseIntError> {
    let mut chars = input.trim().chars();

    let mut blocks = vec![(File(0), chars.next().unwrap().to_string().parse::<u8>()?)];
    let mut id = 0;

    while let Some(c) = chars.next() {
        id += 1;
        blocks.push((Space, c.to_string().parse::<u8>()?));
        blocks.push((File(id), chars.next().unwrap().to_string().parse::<u8>()?));
    }

    Ok(Disk { blocks })
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut disk = parse(_input).unwrap();
    let mut block_index = 0;
    let mut disk_position: u64 = 0;

    let mut sum: u64 = 0;

    while let Some((block_type, size)) = disk.blocks.get(block_index).copied() {
        match block_type {
            File(id) => {
                sum += (disk_position..disk_position + size as u64).sum::<u64>() * id;
                disk_position += size as u64;
                block_index += 1;
            }
            Space => {
                let (last_block_type, last_size) = disk.blocks.last().copied().unwrap();
                match last_block_type {
                    File(last_id) => {
                        if last_size <= size {
                            // Move last file block entirely
                            sum += (disk_position..disk_position + last_size as u64).sum::<u64>()
                                * last_id;
                            disk_position += last_size as u64;
                            disk.blocks[block_index] = (Space, size - last_size);
                            disk.blocks.pop();
                        } else {
                            sum +=
                                (disk_position..disk_position + size as u64).sum::<u64>() * last_id;
                            disk_position += size as u64;
                            disk.blocks.pop();
                            disk.blocks.push((File(last_id), last_size - size));
                            block_index += 1;
                        }
                    }
                    Space => {
                        disk.blocks.pop();
                    }
                }
            }
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
