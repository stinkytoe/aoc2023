use std::collections::LinkedList;

#[derive(Debug)]
struct PartNumber<'a> {
    start_index: usize,
    length: usize,
    data: &'a str,
}

impl PartNumber<'_> {
    fn is_symbol_in_range(&self, symbol: &Symbol, width: usize, height: usize) -> bool {
        let on_left_edge = self.start_index % width == 0;
        let on_right_edge = (self.start_index + self.length) % width == 0;
        let on_top_edge = self.start_index < width;
        let on_bottom_edge = self.start_index >= width * (height - 1);

        let left_bound: usize = if on_left_edge {
            self.start_index
        } else {
            self.start_index - 1
        };

        let right_bound: usize = if on_right_edge {
            self.start_index + self.length - 1
        } else {
            self.start_index + self.length
        };

        // above
        if !on_top_edge && ((left_bound - width)..=(right_bound - width)).contains(&symbol.index) {
            return true;
        }

        // in line
        if (left_bound..=right_bound).contains(&symbol.index) {
            return true;
        }

        // below
        if !on_bottom_edge && ((left_bound + width)..=(right_bound + width)).contains(&symbol.index) {
            return true;
        }

        false
    }

    fn to_u32(&self) -> u32 {
        self.data.parse().unwrap()
    }
}

#[derive(Debug)]
struct Symbol {
    index: usize,
    _symbol: char,
}

pub fn part1(input: &str) -> u32 {
    let (width, height, chars) = {
        let lines = input.lines().collect::<Vec<&str>>();
        let width = lines[0].len();
        let height = lines.len();
        (
            width,
            height,
            lines.iter().flat_map(|&line| line.chars()).collect::<String>(),
        )
    };

    let mut part_number_list: LinkedList<PartNumber> = LinkedList::new();
    let mut symbol_list: LinkedList<Symbol> = LinkedList::new();
    let mut start_index_option: Option<usize> = None;

    chars.chars().enumerate().for_each(|(index, symbol)| {
        if symbol.is_ascii_digit() {
            if start_index_option.is_none() {
                start_index_option = Some(index);
            }
        } else {
            if let Some(start_index) = start_index_option {
                part_number_list.push_back(PartNumber {
                    start_index,
                    length: index - start_index,
                    data: &chars[start_index..index],
                });
                start_index_option = None;
            }

            if symbol != '.' {
                symbol_list.push_back(Symbol { index, _symbol: symbol })
            }
        }
    });

    let filtered_list: Vec<&PartNumber> = part_number_list
        .iter()
        .filter(|&part_number| {
            symbol_list
                .iter()
                .any(|symbol| part_number.is_symbol_in_range(symbol, width, height))
        })
        .collect();

    filtered_list.iter().map(|part_number| part_number.to_u32()).sum()
}

pub fn part2(input: &str) -> u32 {
    todo!()
}
