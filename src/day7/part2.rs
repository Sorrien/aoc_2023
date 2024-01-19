use std::cmp::Ordering;

const CARD_TYPES: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

pub fn solution(input: String) -> u64 {
    let mut hands = input
        .lines()
        .map(|line| {
            let mut split = line.split_ascii_whitespace();
            let hand_str = split.next().unwrap();
            let bid = split.next().unwrap().parse::<u64>().unwrap();

            let hand = eval_hand(hand_str);

            (hand_str, hand, bid)
        })
        .collect::<Vec<_>>();

    hands.sort_by(|(_, a, _), (_, b, _)| a.cmp(b));

    let result = hands
        .iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| {
            let rank = rank as u64 + 1;
            let result = bid * rank;
            result
        })
        .sum();
    result
}

fn eval_hand(hand_str: &str) -> Hand {
    let card_scores = hand_str
        .chars()
        .map(|c| {
            CARD_TYPES
                .iter()
                .position(|char| *char == c)
                .expect("failed to find card") as u64
        })
        .collect::<Vec<_>>();

    let hand_score = get_hand_type(hand_str);
    Hand {
        hand_score,
        card_scores,
    }
}

fn get_hand_type(hand_str: &str) -> u64 {
    let chars = hand_str.chars();

    let mut distinct_chars = chars.clone().collect::<Vec<_>>();
    distinct_chars.sort();
    distinct_chars.dedup();

    let mut card_distribution_chars = distinct_chars
        .iter()
        .map(|distinct_char| {
            (
                distinct_char,
                chars.clone().filter(|c| c == distinct_char).count(),
            )
        })
        .collect::<Vec<_>>();

    card_distribution_chars.sort_by(|(_, a), (_, b)| a.cmp(b));
    card_distribution_chars.reverse();

    let joker_count = if let Some((_, count)) = card_distribution_chars
        .clone()
        .iter()
        .find(|(char, _)| **char == 'J')
    {
        *count
    } else {
        0
    };

    let card_distribution_2 = card_distribution_chars
        .clone()
        .iter()
        .map(|(_, count)| *count)
        .collect::<Vec<_>>();

    let result = if card_distribution_2 == [5] {
        HandType::FiveofaKind
    } else if card_distribution_2 == [4, 1] {
        if joker_count > 0 {
            HandType::FiveofaKind
        } else {
            HandType::FourofaKind
        }
    } else if card_distribution_2 == [3, 2] {
        if joker_count > 0 {
            HandType::FiveofaKind
        } else {
            HandType::FullHouse
        }
    } else if card_distribution_2 == [3, 1, 1] {
        if joker_count > 0 {
            HandType::FourofaKind
        } else {
            HandType::ThreeofaKind
        }
    } else if card_distribution_2 == [2, 2, 1] {
        if joker_count == 2 {
            HandType::FourofaKind
        } else if joker_count == 1 {
            HandType::FullHouse
        } else {
            HandType::TwoPair
        }
    } else if card_distribution_2 == [2, 1, 1, 1] {
        if joker_count == 2 {
            HandType::ThreeofaKind
        } else if joker_count == 1 {
            HandType::ThreeofaKind
        } else {
            HandType::OnePair
        }
    } else if card_distribution_2 == [1, 1, 1, 1, 1] {
        if joker_count == 1 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    } else {
        panic!("we missed a hand distribution!");
    };

    result as u64
}

#[derive(Eq)]
struct Hand {
    pub hand_score: u64,
    pub card_scores: Vec<u64>,
}

impl Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_score == other.hand_score
            && (self
                .card_scores
                .iter()
                .zip(&other.card_scores)
                .filter(|&(a, b)| a == b)
                .count()
                == self.card_scores.len())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_score.cmp(&other.hand_score) {
            core::cmp::Ordering::Equal => {}
            ord => return Some(ord),
        }
        for (x, y) in self.card_scores.iter().zip(&other.card_scores) {
            match x.cmp(y) {
                core::cmp::Ordering::Equal => {}
                ord => return Some(ord),
            }
        }
        Some(core::cmp::Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_score.cmp(&other.hand_score) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        for (x, y) in self.card_scores.iter().zip(&other.card_scores) {
            match x.cmp(y) {
                core::cmp::Ordering::Equal => {}
                ord => return ord,
            }
        }
        core::cmp::Ordering::Equal
    }
}

#[derive(Copy, Clone)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeofaKind = 4,
    FullHouse = 5,
    FourofaKind = 6,
    FiveofaKind = 7,
}
