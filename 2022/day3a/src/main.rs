mod item {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) struct Item(u8);

    impl TryFrom<u8> for Item {
        type Error = color_eyre::Report;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
                _ => Err(color_eyre::eyre::eyre!(
                    "{} is not a valid item",
                    value as char
                )),
            }
        }
    }

    impl Item {
        pub(crate) fn score(self) -> usize {
            match self {
                Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
                Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
                _ => unreachable!(),
            }
        }
    }
}

use item::Item;
use itertools::Itertools;
use std::collections::HashSet;

fn main() -> color_eyre::Result<()> {
    let sum = include_str!("input.txt")
        .lines()
        .map(|line| -> color_eyre::Result<_> {
            let (first, second) = line.split_at(line.len() / 2);
            let first_items = first
                .bytes()
                .map(Item::try_from)
                .collect::<Result<HashSet<_>, _>>()?;
            itertools::process_results(second.bytes().map(Item::try_from), |mut it| {
                it.find(|&item| first_items.contains(&item))
                    .map(|item| item.score())
                    .ok_or_else(|| color_eyre::eyre::eyre!("compartments have no items in common"))
            })?
        })
        .sum::<color_eyre::Result<usize>>()?;
    dbg!(sum);
    Ok(())
}
