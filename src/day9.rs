use std::collections::LinkedList;
use std::fmt::{Display, Formatter, Write};
use anyhow::{anyhow, Error};
use itertools::Itertools;
use crate::aocday::{AocDay, AocInfo, AocInput, AocOutput};
use crate::aocday::AocOutput::Unimplemented;
use crate::day9::Block::{File, Free};

#[derive(Default)]
pub struct Day9 {
    fs: FileSystem,
    block_fs: BlockFileSystem,
}

type Today = Day9;

impl Today {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

}

impl AocDay for Today {

    fn info(&self) -> AocInfo {
        (9, "Disk Fragmenter").into()
    }

    fn init(&mut self, input: AocInput) -> Result<(), Error> {
        let mut file = true;
        let mut id = 0;
        let mut fs: Vec<Block> = Vec::with_capacity(input.len());
        for x in input.raw().trim().chars() {
            let digit = x.to_digit(10).unwrap();
            if file {
                for _ in 0..digit {
                    fs.push(File(id));
                }
                self.block_fs.write_block(File(id), digit as usize);
                id += 1;
            } else {
                for _ in 0..digit {
                    fs.push(Free);
                }
                self.block_fs.write_block(Free, digit as usize);
            }
            file = !file;
        }

        self.fs.fs = fs;

        Ok(())
    }

    fn part1(&self) -> Result<AocOutput, Error> {
        let mut fs = self.fs.clone();

        let mut first_free = fs.first_free(0).unwrap();
        let mut last_file = fs.last_file(fs.fs.len() - 1).unwrap();
        while last_file > first_free {
            fs.move_block(last_file, first_free);
            first_free = fs.first_free(first_free).unwrap();
            last_file = fs.last_file(last_file).unwrap();
        }

        Ok(fs.checksum().into())
    }

    fn part2(&self) -> Result<AocOutput, Error> {
        Ok(Unimplemented)
    }

}

#[derive(Clone, Copy, Debug)]
enum Block {
    File(u16),
    Free,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            File(id) => f.write_fmt(format_args!("{}", id)),
            Free => f.write_char('.'),
        }
    }
}

#[derive(Clone, Default)]
struct FileSystem {
    fs: Vec<Block>,
}

impl FileSystem {
    fn first_free(&self, first_free: usize) -> Option<usize> {
        for i in first_free..self.fs.len() {
            if let Some(Free) = self.fs.get(i) {
                return Some(i);
            }
        }
        None
    }

    fn last_file(&self, last_file: usize) -> Option<usize> {
        for i in (0..=last_file).rev() {
            if let Some(File(_)) = self.fs.get(i) {
                return Some(i);
            }
        }
        None
    }

    fn first_free_block(&self, first_free: usize) -> Option<usize> {
        for i in first_free..self.fs.len() {
            if let Some(Free) = self.fs.get(i) {
                return Some(i);
            }
        }
        None
    }

    fn last_file_block(&self, last_file: usize) -> Option<BFSBlock> {
        for i in (0..=last_file).rev() {
            if let Some(File(_)) = self.fs.get(i) {
                return Some(BFSBlock {
                    start: 0,
                    len: 0,
                    contents: Block::Free,
                });
            }
        }
        None
    }

    fn move_block(&mut self, from: usize, to: usize) {
        self.fs.swap(from, to);
    }

    fn checksum(&self) -> usize {
        let mut checksum = 0;
        for (pos, block) in self.fs.iter().enumerate() {
            if let File(id) = block {
                //dbg!(pos, id, pos * (*id as usize));
                checksum += pos * (*id as usize);
            }
        }
        checksum
    }
}

#[derive(Default)]
struct BlockFileSystem {
    blocks: Vec<BFSBlock>,
    blocks_linked: LinkedList<BFSBlock>,
    len: usize,
}

impl BlockFileSystem {

    fn write_block(&mut self, block: Block, size: usize) {
        let pos = self.len;
        self.len += pos;
        let new_block = BFSBlock {
            start: pos,
            len: size,
            contents: block,
        };
        self.blocks.push(new_block);
        self.blocks_linked.push_back(new_block);
    }

}

#[derive(Clone, Copy)]
struct BFSBlock {
    start: usize,
    len: usize,
    contents: Block,
}

impl BFSBlock {

    fn checksum(&self) -> usize {
        match self.contents {
            File(id) => (id as usize) * (self.len * (2 * self.start + self.len - 1)) / 2,
            Free => 0,
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "2333133121414131402";

    #[test]
    fn example() {
        let day = init(EX);
        assert_eq!(day.part1().expect("Part 1"), 1928usize.into());
        assert_eq!(day.part2().expect("Part 2"), Unimplemented);
    }

    #[test]
    fn bfs_block_checksum() {
        let block = BFSBlock {
            start: 3,
            len: 5,
            contents: File(2),
        };

        assert_eq!(block.checksum(), (3..=7).sum::<usize>() * 2);
    }

    fn init(input: &str) -> Today {
        let mut day = Today::new();
        day.init(AocInput::new(input).trim()).expect("Init failed");
        day
    }

}
