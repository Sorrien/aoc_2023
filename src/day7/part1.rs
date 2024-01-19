use std::cmp::Ordering;

const CARD_TYPES: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const HAND_TYPE_NAMES: [&str; 7] = [
    "High Card",
    "One Pair",
    "Two Pair",
    "Three of a Kind",
    "Full House",
    "Four of a Kind",
    "Five of a Kind",
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

    hands
        .iter()
        .enumerate()
        .map(|(rank, (hand_str, hand, bid))| {
            let rank = rank as u64 + 1;
            let result = bid * rank;

            println!(
                "hand: {} hand_type: {} result: {} rank: {} bid: {} card_scores: {:?}",
                hand_str,
                HAND_TYPE_NAMES[(hand.hand_score - 1) as usize],
                result,
                rank,
                bid,
                hand.card_scores
            );
            result
        })
        .sum()
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

    let mut card_distribution = distinct_chars
        .iter()
        .map(|distinct_char| chars.clone().filter(|c| c == distinct_char).count())
        .collect::<Vec<_>>();

    card_distribution.sort();
    card_distribution.reverse();

    /*let most_freq_card_occurences = card_distribution[0];

    let sec_most_few_card_occurences = card_distribution.get(1);
     let result = match most_freq_card_occurences {
        //five of a kind
        5 => 7,
        //four of a kind
        4 => 6,
        //full house or 3 of a kind
        3 => {
            if *sec_most_few_card_occurences.unwrap() == 2 {
                //full house
                5
            } else {
                //three of a kind
                4
            }
        }
        2 => {
            if *sec_most_few_card_occurences.unwrap() == 2 {
                //two pair
                3
            } else {
                //one pair
                2
            }
        }
        1 => 1, // High Card
        _ => panic!("what happened?"),
    }; */

    let result = if card_distribution == [5] {
        HandType::FiveofaKind
    } else if card_distribution == [4, 1] {
        HandType::FourofaKind
    } else if card_distribution == [3, 2] {
        HandType::FullHouse
    } else if card_distribution == [3, 1, 1] {
        HandType::ThreeofaKind
    } else if card_distribution == [2, 2, 1] {
        HandType::TwoPair
    } else if card_distribution == [2, 1, 1, 1] {
        HandType::OnePair
    } else {
        HandType::HighCard
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
