use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;

struct Pair {
    a: usize,
    b: usize,
}
impl FromStr for Pair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("   ");

        let a = parts
            .next()
            .ok_or(anyhow!("input line malformed"))?
            .parse()
            .map_err(|e| anyhow!("failed to parse to int: {e:?}"))?;
        let b = parts
            .next()
            .ok_or(anyhow!("input line malformed"))?
            .parse()
            .map_err(|e| anyhow!("failed to parse to int: {e:?}"))?;
        Ok(Pair { a, b })
    }
}

struct LocationList {
    left: Vec<usize>,
    right: Vec<usize>,
}
impl From<Vec<Pair>> for LocationList {
    fn from(value: Vec<Pair>) -> Self {
        let mut a_vals = value.iter().map(|p| p.a).collect::<Vec<_>>();
        a_vals.sort();

        let mut b_vals = value.iter().map(|p| p.b).collect::<Vec<_>>();
        b_vals.sort();

        LocationList {
            left: a_vals,
            right: b_vals,
        }
    }
}
impl LocationList {
    fn total_distance(&self) -> usize {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum()
    }
    fn similarity_score(&self) -> usize {
        let find_occurrences = |elem: &usize, list: &Vec<usize>| -> usize {
            list.iter().filter(|&x| x == elem).count()
        };

        let mut score_cache: HashMap<usize, usize> = HashMap::new();
        self.left
            .iter()
            .map(|x| {
                if let Some(score) = score_cache.get(x) {
                    *score
                } else {
                    let occurrences = find_occurrences(x, &self.right);
                    let score = x * occurrences;
                    score_cache.insert(*x, score);
                    score
                }
            })
            .sum()
    }
}

pub fn run(input: Vec<String>) -> anyhow::Result<()> {
    let pairs: Vec<Pair> = input.iter().flat_map(|l| match Pair::from_str(l) {
        Ok(pair) => Ok(pair),
        Err(e) => anyhow::bail!("Failed to parse input: {e:?}"),
    }).collect();

    if pairs.is_empty() {
        anyhow::bail!("No valid pairs found in input");
    }

    let loclist = LocationList::from(pairs);
    println!("Total distance: {}", loclist.total_distance());
    println!("Similarity score: {}", loclist.similarity_score());
    Ok(())
}

