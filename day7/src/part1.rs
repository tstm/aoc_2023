#![allow(dead_code, unused_variables)]

use rayon::prelude::*;
use std::cmp::Ordering;

#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Debug, Copy)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn parse(symbol: char) -> Card {
        match symbol {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Unknown card type"),
        }
    }
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Debug)]
enum Combination {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl Combination {
    fn parse(cards: &[Card; 5]) -> Combination {
        match Combination::highest_count(cards) {
            1 => Combination::HighCard,
            2 => {
                if Combination::unique_count(cards) == 3 {
                    Combination::TwoPair
                } else {
                    Combination::OnePair
                }
            }
            3 => {
                if Combination::unique_count(cards) == 2 {
                    Combination::FullHouse
                } else {
                    Combination::ThreeKind
                }
            }
            4 => Combination::FourKind,
            5 => Combination::FiveKind,
            _ => panic!("No combination found"),
        }
    }

    fn unique_count(cards: &[Card; 5]) -> usize {
        let mut cardvec = cards.to_vec();
        cardvec.sort();
        cardvec.dedup();
        cardvec.len()
    }

    fn highest_count(cards: &[Card; 5]) -> usize {
        cards
            .into_iter()
            .map(|card| cards.iter().filter(|c| c == &card).count())
            .max()
            .unwrap()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.combination.cmp(&other.combination) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let mut i = 0;
                loop {
                    match self.cards[i].cmp(&other.cards[i]) {
                        Ordering::Less => break Ordering::Less,
                        Ordering::Greater => break Ordering::Greater,
                        Ordering::Equal => (),
                    }
                    i += 1;
                    if i == 5 {
                        break Ordering::Equal;
                    }
                }
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    combination: Combination,
    bid: usize,
}

impl Hand {
    fn parse(input: &str) -> Hand {
        let (cards, bid) = input
            .split_once(" ")
            .expect("There should be cards and bid");
        let cards: [Card; 5] = cards
            .chars()
            .map(|c| Card::parse(c))
            .collect::<Vec<Card>>()
            .try_into()
            .expect("There should be exactly 5 cards.");
        let bid = bid.parse::<usize>().expect("to have a bid");

        Hand {
            cards,
            combination: Combination::parse(&cards),
            bid,
        }
    }
}

pub fn run(input: &str) -> Result<usize, String> {
    let mut hands: Vec<_> = input.par_lines().map(|line| Hand::parse(line)).collect();
    hands.par_sort();

    Ok(hands
        .iter()
        .enumerate()
        .map(|(n, hand)| (n + 1) * hand.bid)
        .sum())
}
