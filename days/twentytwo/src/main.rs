use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

fn main() {
    println!("Hello, advent of code day twentytwo!");
    let input = include_str!("../input.txt");
    println!();
    println!("Part one:");
    println!("{}", part_one(input));
    println!();
    println!("Part two:");
    println!("{}", part_two(input));
}
fn part_one(input_string: &str) -> i64 {
    let mut initial_secrets = input_string
        .trim()
        .lines()
        .filter_map(|value| Some(SecretNumber::new(value.trim().parse::<i64>().ok()?)))
        .collect::<Vec<SecretNumber>>();
    initial_secrets.iter_mut().for_each(|secret| {
        for _ in 0..2000 {
            secret.evolve_into_next();
        }
    });
    initial_secrets
        .iter()
        .map(|secret| secret.latest_number)
        .sum()
}
fn part_two(input_string: &str) -> i64 {
    let mut secret_numbers = input_string
        .trim()
        .lines()
        .filter_map(|value| Some(SecretNumber::new(value.trim().parse::<i64>().ok()?)))
        .collect::<Vec<SecretNumber>>();
    let mut offers = MonkeyOffers {
        offers: HashMap::new(),
    };
    secret_numbers.iter_mut().for_each(|secret| {
        let mut new_offers = Vec::new();
        for _ in 0..2000 {
            secret.evolve_into_next();
            new_offers.push(secret.price);
        }
        let mut unique_offers: HashMap<[i64; 4], HidingSpotOffer> = HashMap::new();
        new_offers.windows(4).for_each(|window| {
            let offer = HidingSpotOffer::from(window);
            if !unique_offers.contains_key(&offer.history) {
                unique_offers.insert(offer.history, offer);
            }
        });
        unique_offers.iter().for_each(|(_, offer)| {
            *offers.offers.entry(offer.history).or_insert(0) += offer.price;
        });
    });
    *offers.offers.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1
}
#[cfg(test)]
mod tests {

    use super::*;
    const TEN_SECRET_NUMBERS: &str = r#"15887950
16495136
527345
704524
1553684
12683156
11100544
12249484
7753432
5908254"#;
    const SECRET_INITIAL_NUMBERS: &str = r#"1
10
100
2024
"#;
    const SECRET_FINAL_PRICES: &str = r#"8685429
4700978
15273692
8667524"#;
    #[test]
    fn find_secret_number() {
        let mut initial_secrets = SECRET_INITIAL_NUMBERS
            .trim()
            .lines()
            .filter_map(|value| Some(SecretNumber::new(value.trim().parse::<i64>().ok()?)))
            .collect::<Vec<SecretNumber>>();
        let test_values = SECRET_FINAL_PRICES
            .trim()
            .lines()
            .filter_map(|line| line.parse::<i64>().ok())
            .collect::<Vec<i64>>();
        let mut test_results = Vec::new();
        initial_secrets.iter_mut().for_each(|secret| {
            for _ in 0..2000 {
                secret.evolve_into_next();
            }
            test_results.push(secret.latest_number);
        });
        assert_eq!(test_values, test_results);
        assert_eq!(test_values.iter().sum::<i64>(), 37327623);
    }
    #[test]
    fn find_secret_numbers() {
        let test_values = TEN_SECRET_NUMBERS
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let mut secret_number = SecretNumber::new(123);
        let mut result_values = Vec::new();
        for _ in 0..10 {
            secret_number.evolve_into_next();
            result_values.push(secret_number.latest_number);
        }
        assert_eq!(test_values, result_values);
    }
    const PART_TWO_SECRET_INITIAL_NUMBERS: &str = r#"1
2
3
2024
"#;
    #[test]
    fn bananas() {
        let mut secret_numbers = PART_TWO_SECRET_INITIAL_NUMBERS
            .trim()
            .lines()
            .filter_map(|value| Some(SecretNumber::new(value.trim().parse::<i64>().ok()?)))
            .collect::<Vec<SecretNumber>>();
        let mut offers = MonkeyOffers {
            offers: HashMap::new(),
        };
        secret_numbers.iter_mut().for_each(|secret| {
            let mut new_offers = Vec::new();
            for _ in 0..2000 {
                secret.evolve_into_next();
                new_offers.push(secret.price);
            }
            let mut unique_offers: HashMap<[i64; 4], HidingSpotOffer> = HashMap::new();
            new_offers.windows(4).for_each(|window| {
                let offer = HidingSpotOffer::from(window);
                if !unique_offers.contains_key(&offer.history) {
                    unique_offers.insert(offer.history, offer);
                }
            });
            unique_offers.iter().for_each(|(_, offer)| {
                *offers.offers.entry(offer.history).or_insert(0) += offer.price;
            });
        });
        offers
            .offers
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(offer, count)| {
                println!("{:?} {}", offer, count);
            });
        offers.offers.get(&[-2, 1, -1, 3]).map(|value| {
            println!("{}", value);
        });
    }
    #[test]
    fn price_changes() {
        let mut secret_number = SecretNumber::new(123);
        let mut test_results = Vec::new();
        for _ in 0..10 {
            secret_number.evolve_into_next();
            test_results.push(secret_number.price);
        }
        test_results.iter().for_each(|price| {
            println!("{}", price);
        });
        let mut offers = MonkeyOffers {
            offers: HashMap::new(),
        };
        test_results.windows(4).for_each(|window| {
            let offer = HidingSpotOffer::from(window);
            *offers.offers.entry(offer.history).or_insert(0) += offer.price;
        });
        offers.offers.iter().for_each(|(offer, count)| {
            println!("{:?} {}", offer, count);
        });
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct MonkeyOffers {
    offers: HashMap<OfferSequence, i64>,
}
type OfferSequence = [i64; 4];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HidingSpotOffer {
    price: i64,
    history: OfferSequence,
}
impl From<&[BananaPrice]> for HidingSpotOffer {
    fn from(prices: &[BananaPrice]) -> Self {
        HidingSpotOffer {
            price: prices[3].price,
            history: [
                prices[0].price_change.unwrap(),
                prices[1].price_change.unwrap(),
                prices[2].price_change.unwrap(),
                prices[3].price_change.unwrap(),
            ],
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BananaPrice {
    price: i64,
    price_change: Option<i64>,
}
impl Display for BananaPrice {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "BananaPrice: {} {}",
            self.price,
            match self.price_change {
                Some(value) => format!(" ({})", value),
                None => "".to_string(),
            }
        )
    }
}
struct SecretNumber {
    initial_number: i64,
    latest_number: i64,
    price: BananaPrice,
}
impl SecretNumber {
    fn evolve_into_next(&mut self) {
        // calculate self * 64
        let value = self.latest_number * 64;
        // then MIX the number
        self.mix(value);
        // then PRUNE the number
        self.prune();
        let next_value = self.latest_number / 32;
        self.mix(next_value);
        let last_value = self.latest_number * 2048;

        self.mix(last_value);
        self.prune();
        self.price.price_change = Some(self.last_digit() - self.price.price);
        self.price.price = self.last_digit();
    }
    fn last_digit(&self) -> i64 {
        self.latest_number.abs() % 10
    }
    fn mix(&mut self, new_value: i64) {
        // mix the number
        self.latest_number = new_value ^ self.latest_number;
    }
    fn prune(&mut self) {
        // prune the number
        self.latest_number = self.latest_number % 16777216;
    }
    fn new(number: i64) -> SecretNumber {
        SecretNumber {
            initial_number: number,
            latest_number: number,
            price: BananaPrice {
                price: number.abs() % 10,
                price_change: None,
            },
        }
    }
}
