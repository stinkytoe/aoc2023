use std::cmp::Ordering;

use anyhow::{anyhow, bail, Error, Result};

pub fn part1(input: &str) -> u32 {
    let lines = input.lines().filter(|line| !line.is_empty());

    let mut hands: Vec<(Hand, u32)> = lines
        .map(|line| line.split(' '))
        .map(|mut split| {
            Ok((
                split.next().ok_or(anyhow!("no first part?"))?,
                split.next().ok_or(anyhow!("no second part?"))?,
                split.next(),
            ))
        })
        .collect::<Result<Vec<_>>>()
        .expect("couldn't parse input!")
        .iter()
        .map(|(hand, bid, rest)| {
            if rest.is_some() {
                bail!("extra data in hand line!")
            } else {
                Ok((Hand::try_from(*hand)?, bid.parse::<u32>()?))
            }
        })
        .collect::<Result<Vec<_>>>()
        .expect("couldn't parse input!");

    hands.sort_by(|(hand1, _), (hand2, _)| hand1.partial_cmp(hand2).unwrap());

    hands.iter().enumerate().map(|(i, (_, bid))| (i as u32 + 1) * bid).sum()
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Card(u8);

const CARD_TO_NUMBER: &[char] = &['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

impl Card {
    pub fn new(input: &char) -> Option<Self> {
        if let Some((i, _)) = CARD_TO_NUMBER.iter().enumerate().find(|(_, ch)| input == *ch) {
            Some(Card(i as u8))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, PartialEq)]
struct Hand {
    data: [Card; 5],
}

impl Hand {
    fn calc_type(&self) -> HandType {
        let mut sorted = self.data.clone();
        sorted.sort();

        let mut i: usize = 0;
        let mut last: Option<Card> = None;
        let mut counts: Vec<u32> = vec![];

        sorted.iter().for_each(|card| {
            if last.is_none() {
                last = Some(card.clone());
                counts.push(1);
            } else if last.as_ref().unwrap().0 == card.0 {
                counts[i] += 1;
            } else {
                i += 1;
                last = Some(card.clone());
                counts.push(1);
            }
        });

        match &counts[..] {
            [5] => HandType::FiveKind,
            [1, 4] | [4, 1] => HandType::FourKind,
            [2, 3] | [3, 2] => HandType::FullHouse,
            [1, 1, 3] | [1, 3, 1] | [3, 1, 1] => HandType::ThreeKind,
            [1, 2, 2] | [2, 1, 2] | [2, 2, 1] => HandType::TwoPair,
            [1, 1, 1, 2] | [1, 1, 2, 1] | [1, 2, 1, 1] | [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl TryFrom<&str> for Hand {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        if value.len() != 5 {
            bail!("not the length of a hand!")
        }

        let mut chars = value.chars();

        let make_card_helper =
            |ch: &char| -> Result<Card> { Card::new(ch).ok_or(anyhow!("card parse error! {value}")) };

        Ok(Self {
            data: [
                make_card_helper(&chars.next().ok_or(anyhow!("failed to get next char!"))?)?,
                make_card_helper(&chars.next().ok_or(anyhow!("failed to get next char!"))?)?,
                make_card_helper(&chars.next().ok_or(anyhow!("failed to get next char!"))?)?,
                make_card_helper(&chars.next().ok_or(anyhow!("failed to get next char!"))?)?,
                make_card_helper(&chars.next().ok_or(anyhow!("failed to get next char!"))?)?,
            ],
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let first_type = self.calc_type();
        let second_type = other.calc_type();
        match first_type.partial_cmp(&second_type).unwrap() {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => self.data.partial_cmp(&other.data),
            Ordering::Greater => Some(Ordering::Greater),
        }
    }
}
