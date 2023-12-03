use std::collections::LinkedList;

#[derive(Debug)]
struct PartNumber {
    start_index: usize,
    length: usize,
    data: String,
}

impl PartNumber {
    // fn contains_index(&self, index: usize) -> bool {
    //     self.start_index <= index && index < (self.start_index + self.length)
    // }

    fn generate_surrounding_indices(&self) -> Vec<usize> {
        todo!()
    }
}

#[derive(Debug)]
struct Symbol {
    index: usize,
    symbol: char,
}

pub fn part1(input: &str) -> u32 {
    let (width, height, chars) = {
        let lines = input.lines().collect::<Vec<&str>>();
        let width = lines[0].len();
        let height = lines.len();
        (
            width,
            height,
            lines.iter().flat_map(|&line| line.chars()).collect::<Vec<char>>(),
        )
    };

    let mut part_number_list: LinkedList<PartNumber> = LinkedList::new();
    let mut symbol_list: LinkedList<Symbol> = LinkedList::new();
    let mut start_index_option: Option<usize> = None;

    chars.iter().enumerate().for_each(|(index, &symbol)| {
        if symbol.is_ascii_digit() {
            if start_index_option.is_none() {
                start_index_option = Some(index);
            }
        } else {
            if let Some(start_index) = start_index_option {
                part_number_list.push_back(PartNumber {
                    start_index,
                    length: index - start_index,
                    data: chars[start_index..index].iter().collect(),
                });
                start_index_option = None;
            }

            if symbol != '.' {
                symbol_list.push_back(Symbol { index, symbol })
            }
        }
    });

    todo!()
}
