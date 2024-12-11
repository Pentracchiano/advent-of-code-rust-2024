use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Page(u32);

struct Rules(HashMap<Page, HashSet<Page>>);

fn parse(input: &str) -> (Rules, Vec<Vec<Page>>)
{
    let mut parts = input.split("\n\n");
    let rules_part = parts.next().unwrap();
    let pages_part = parts.next().unwrap();

    (parse_rules(rules_part), parse_pages(pages_part))
}

fn parse_rules(input: &str) -> Rules
{
    let mut rules_map: HashMap<Page, HashSet<Page>> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split('|');
        let must_come_before = Page(parts.next().unwrap().parse().unwrap());
        let must_come_after = Page(parts.next().unwrap().parse().unwrap());

        rules_map.entry(must_come_before)
                 .or_insert_with(HashSet::new)
                 .insert(must_come_after);
    }

    Rules(rules_map)
}

fn parse_pages(input: &str) -> Vec<Vec<Page>>
{
    input.lines().map(|line| {
        line.split(',').map(|part| Page(part.parse().unwrap())).collect()
    }).collect()
}

fn middle(pages: &Vec<Page>) -> Page
{
    pages[pages.len() / 2]
}

fn is_valid(pages: &Vec<Page>, rules: &Rules) -> bool
{
    for (i, page) in pages.iter().enumerate() {
        let afters = match rules.0.get(page) {
            Some(afters) => afters,
            None => continue,
        };

        for j in 0..i {
            if afters.contains(&pages[j]) {
                return false;
            }
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, all_pages) = parse(input);

    Some(all_pages
    .iter()
    .filter(|pages| is_valid(pages, &rules))
    .map(|pages| middle(pages))
    .map(|page| page.0)
    .sum())
}

fn correct_pages(pages: &Vec<Page>, rules: &Rules) -> Vec<Page>
{
    let mut corrected_pages = pages.clone();

    let mut i = 0;
    let mut j = 1;

    while j < corrected_pages.len() {
        if let Some(afters) = rules.0.get(&corrected_pages[j]) {
            if afters.contains(&corrected_pages[i]) {
                corrected_pages.swap(i, j);
            }
        }

        j += 1;
        if j == corrected_pages.len() {
            i += 1;
            j = i + 1;
        }
    }

    corrected_pages
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, all_pages) = parse(input);

    let incorrect_pages = all_pages
    .iter()
    .filter(|pages| !is_valid(pages, &rules));

    Some(incorrect_pages
    .map(|pages| {
        correct_pages(pages, &rules)
    })
    .map(|pages| middle(&pages))
    .map(|page| page.0)
    .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
