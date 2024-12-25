#[derive(Debug)]
struct Rule {
    before: i32,
    after: i32,
}

fn parse_file() -> (Vec<Rule>, Vec<Vec<i32>>) {
    let input = read_file("./d5.txt").unwrap();

    let mut rules = vec![];
    let mut updates = vec![];
    let mut part_one = true;

    for line in input.lines() {
        if line == "" {
            part_one = false;
            continue;
        }

        if part_one {
            let mut split = line.split("|");
            let before = split.next().unwrap().parse::<i32>().unwrap();
            let after = split.next().unwrap().parse::<i32>().unwrap();
            rules.push(Rule { before, after })
        } else {
            let update = line.split(",").map(|x| x.parse::<i32>().unwrap());
            updates.push(update.collect::<Vec<i32>>());
        }
    }

    return (rules, updates);
}

fn check_update(update: &Vec<i32>, rules: &Vec<Rule>) -> bool {
    for (i, number) in update.iter().enumerate() {
        let needs_to_be_after = rules
            .iter()
            .filter(|r| r.after == *number)
            .map(|r| r.before)
            .collect::<Vec<i32>>();

        let invalid = update[i + 1..]
            .iter()
            .any(|n| needs_to_be_after.contains(n));

        if invalid {
            return false;
        }
    }

    return true;
}

fn sum_valid() {
    let (rules, updates) = parse_file();

    let sum = updates
        .iter()
        .filter(|update| check_update(&update, &rules))
        .map(|update| update.get(update.len() / 2))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .sum::<i32>();

    println!("{:?}", sum);
}

fn fix_update(update: &Vec<i32>, rules: &Vec<Rule>) -> Vec<i32> {
    let mut buffer = vec![];

    for number in update {
        let targets = rules
            .iter()
            .filter(|r| r.after == *number)
            .map(|r| r.before)
            .filter(|num| update.contains(num))
            .filter(|num| !buffer.contains(num))
            .collect::<Vec<i32>>();

        targets.iter().for_each(|num| buffer.push(*num));

        if !buffer.contains(number) {
            buffer.push(*number);
        }

        if buffer.len() == update.len() {
            break;
        }
    }

    if !check_update(&buffer, rules) {
        return fix_update(&buffer, rules);
    }

    return buffer;
}

fn sum_corrected() {
    let (rules, updates) = parse_file();

    let sum = updates
        .iter()
        .filter(|update| !check_update(&update, &rules))
        .map(|update| fix_update(&update, &rules))
        .map(|update| update.get(update.len() / 2).cloned())
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .sum::<i32>();

    println!("{:?}", sum);
}

pub fn main() {
    sum_corrected();
}
