use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" (contains ").unwrap();
            (
                left.split(' ').map(|i| i.to_owned()).collect(),
                right[..right.len() - 1]
                    .split(", ")
                    .map(|i| i.to_owned())
                    .collect(),
            )
        })
        .collect()
}

#[aoc(day21, part1)]
pub fn solve_p1(rules: &[(Vec<String>, Vec<String>)]) -> usize {
    let all_ingredients = rules
        .iter()
        .flat_map(|i| i.0.clone())
        .collect::<HashSet<_>>();

    let mut allergies = HashMap::new();

    for (ingredients, allergens) in rules.iter() {
        let ingredients = ingredients.iter().cloned().collect::<HashSet<_>>();
        for allergen in allergens {
            let diff = allergies
                .entry(allergen)
                .or_insert_with(|| all_ingredients.clone())
                .intersection(&ingredients)
                .cloned()
                .collect();
            allergies.entry(allergen).and_modify(|value| *value = diff);
        }
    }

    // println!("{:?}", allergies);

    let good_ingredients = allergies
        .iter()
        .fold(all_ingredients, |acc, (_, ingredients)| {
            acc.difference(ingredients).cloned().collect()
        });

    // println!("{:?}", good_ingredients);

    rules
        .iter()
        .flat_map(|(ingredients, _)| {
            ingredients
                .iter()
                .filter(|ingredient| good_ingredients.contains(*ingredient))
        })
        .count()
}

#[aoc(day21, part2)]
pub fn solve_p2(rules: &[(Vec<String>, Vec<String>)]) -> String {
    let all_ingredients = rules
        .iter()
        .flat_map(|i| i.0.clone())
        .collect::<HashSet<_>>();

    let mut allergies = HashMap::new();

    for (ingredients, allergens) in rules.iter() {
        let ingredients = ingredients.iter().cloned().collect::<HashSet<_>>();
        for allergen in allergens {
            let diff = allergies
                .entry(allergen)
                .or_insert_with(|| all_ingredients.clone())
                .intersection(&ingredients)
                .cloned()
                .collect();
            allergies.entry(allergen).and_modify(|value| *value = diff);
        }
    }

    while allergies
        .iter()
        .filter(|(_, candidates)| candidates.len() > 1)
        .count()
        > 0
    {
        allergies = allergies
            .iter()
            .map(|(k, ingredients)| {
                let ingredients = allergies
                    .iter()
                    .filter(|(bad, ingredients)| ingredients.len() == 1 && *bad != k)
                    .fold(ingredients.clone(), |acc, (_, ingredients)| {
                        acc.difference(ingredients).cloned().collect()
                    });
                (*k, ingredients)
            })
            .collect();
    }

    // println!("{:?}", allergies);

    let mut bad_stuff = allergies.iter().collect::<Vec<_>>();
    bad_stuff.sort_by_cached_key(|(k, _)| **k);
    bad_stuff.iter().map(|(_, v)| v.iter().cloned().next().unwrap()).collect::<Vec<_>>().join(",")
}

#[cfg(test)]
mod test {
    use super::*;

    fn p1_wrapper(input: &str) -> usize {
        let parsed = input_generator(input);
        solve_p1(&parsed)
    }

    fn p2_wrapper(input: &str) -> String {
        let parsed = input_generator(input);
        solve_p2(&parsed)
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1_wrapper("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)"), 5);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2_wrapper("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)"), "mxmxvkd,sqjhc,fvjkl");
    }
}
