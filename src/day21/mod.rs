use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Error {
    InvalidInput,
}

type ListEntry<'a> = (Vec<&'a str>, Vec<&'a str>);

fn parse(s: &str) -> crate::Result<(HashSet<&str>, Vec<ListEntry>)> {
    let mut all_ingredients = HashSet::new();
    let mut list = Vec::new();
    for l in s.lines() {
        let mut parts = l.split(" (");

        let ingredients: Vec<&str> = parts
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
            .split_whitespace()
            .collect();
        all_ingredients.extend(&ingredients);

        let allergens: Vec<&str> = parts
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
            .trim_start_matches("contains ")
            .trim_end_matches(')')
            .split(", ")
            .collect();

        list.push((ingredients, allergens));
    }

    Ok((all_ingredients, list))
}

fn possibilities<'a>(
    list: &'a [ListEntry],
    ingredients: &HashSet<&'a str>,
) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut possible: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (listed_is, listed_as) in list {
        for allergen in listed_as {
            let possible = possible
                .entry(allergen)
                .or_insert_with(|| ingredients.clone());
            possible.retain(|i| listed_is.contains(i));
        }
    }
    possible
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let (all_ingredients, list) = parse(input)?;
    let possible = possibilities(&list, &all_ingredients);

    let mut safe: Vec<&str> = Vec::new();
    for ingredient in &all_ingredients {
        if possible.values().all(|is| !is.contains(ingredient)) {
            safe.push(ingredient);
        }
    }

    let mut cnt = 0;
    for ingredient in &safe {
        for (listed, _) in &list {
            if listed.contains(ingredient) {
                cnt += 1;
            }
        }
    }

    Ok(cnt)
}

pub fn part2(input: &str) -> crate::Result<String> {
    let (all_ingredients, list) = parse(input)?;
    let mut possible = possibilities(&list, &all_ingredients);

    let mut pairs = Vec::new();
    while let Some(allergen) = possible
        .iter()
        .find(|(_, is)| is.len() == 1)
        .map(|(a, _)| *a)
    {
        let ingredient = possible
            .remove(allergen)
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        pairs.push((ingredient, allergen));
        for is in possible.values_mut() {
            is.remove(ingredient);
        }
    }
    pairs.sort_by_key(|p| p.1);

    let danger_list: Vec<_> = pairs.into_iter().map(|p| p.0).collect();
    let danger_list = danger_list.join(",");
    Ok(danger_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1ex1() {
        let inp = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

        assert_eq!(part1(inp).unwrap(), 5);
        assert_eq!(&part2(inp).unwrap(), "mxmxvkd,sqjhc,fvjkl");
    }
}
