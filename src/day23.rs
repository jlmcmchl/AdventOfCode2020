use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.as_bytes().iter().map(|e| (e - 48) as usize).collect()
}

fn iter(cups: &mut [usize], times: usize) {
    let mut current = cups[0];

    (0..times).for_each(|_| {
        let taken_1 = cups[current] % cups.len();
        let taken_2 = cups[taken_1] % cups.len();
        let taken_3 = cups[taken_2] % cups.len();
        let taken = vec![taken_1, taken_2, taken_3];
        cups[current] = cups[taken_3] % cups.len();

        // target is decremented from current and not in taken
        let mut target = current - 1;
        if target == 0 {
            target = 9;
        }

        while taken.contains(&target) {
            if target == 1 {
                target = 9;
            } else {
                target -= 1;
            }
        }

        let temp = cups[target];
        cups[target] = taken_1;
        cups[taken_3] = temp;

        current = cups[current] % cups.len();
    });
}

fn iter2(cups: &mut [usize], times: usize) {
    let mut current = cups[0];

    (0..times).for_each(|_| {
        let taken_1 = cups[current] % cups.len();
        let taken_2 = cups[taken_1] % cups.len();
        let taken_3 = cups[taken_2] % cups.len();
        let taken = vec![taken_1, taken_2, taken_3];
        cups[current] = cups[taken_3] % cups.len();

        // target is decremented from current and not in taken
        let mut target = if current == 0 { 999999 } else { current - 1 };
        if target == 0 {
            target = 1000000;
        }

        while taken.contains(&target) {
            if target == 1 {
                target = 1000000;
            } else {
                target -= 1;
            }
        }

        let temp = cups[target % 1000000];
        cups[target % 1000000] = taken_1;
        cups[taken_3] = temp;

        current = cups[current] % cups.len();
    });
}

fn print_state(input: &[usize], start: usize, count: usize) -> usize {
    (0..count).fold(0, |agg, i| {
        let value = if i == 0 { start } else { input[agg % 10] };

        agg * 10 + value
    })
}

#[aoc(day23, part1)]
pub fn solve_p1(input: &[usize]) -> usize {
    let mut cups = (0..10).collect::<Vec<_>>();
    cups[0] = input[0];

    (0..input.len() - 1).for_each(|i| {
        let node = input[i] as usize;
        let next = input[i + 1] as usize;
        cups[node] = next;
        cups[next] = input[0] as usize;
    });

    iter(&mut cups, 100);

    print_state(&cups, cups[1], 8)
}

#[aoc(day23, part2)]
pub fn solve_p2(input: &[usize]) -> usize {
    let mut cups = (1..=1000000).collect::<Vec<_>>();
    cups[0] = input[0];

    (0..input.len() - 1).for_each(|i| {
        let node = input[i] as usize;
        let next = input[i + 1] as usize;
        cups[node] = next;
        cups[next] = 10;
    });

    iter2(&mut cups, 10000000);

    let a1 = cups[1];
    let a2 = cups[a1];
    a1 * a2
}

#[cfg(test)]
mod test {
    use super::*;

    fn p1_wrapper(input: &str) -> usize {
        let parsed = input_generator(input);
        solve_p1(&parsed)
    }

    fn p2_wrapper(input: &str) -> usize {
        let parsed = input_generator(input);
        solve_p2(&parsed)
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1_wrapper("389125467"), 67384529);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2_wrapper("389125467"), 149245887792);
    }
}
