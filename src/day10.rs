use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u8> {
    let _ = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
    let _ = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
    input.lines().map(|e| e.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn solve_p1(input: &Vec<u8>) -> usize {
    let mut arr = input.clone();
    arr.sort();

    let jumps = arr.windows(2).fold((1, 1), |(ones, threes), window| {
        if window[0] + 1 == window[1] {
            (ones + 1, threes)
        } else if window[0] + 3 == window[1] {
            (ones, threes + 1)
        } else {
            unreachable!()
        }
    });

    // println!("{:?}", jumps);

    jumps.0 * jumps.1
}

#[aoc(day10, part2)]
pub fn solve_p2(input: &Vec<u8>) -> u64 {
    let mut adapters = input.clone();
    adapters.sort();
    let mut permutations_to_end_from_ind = vec![0; adapters.len()];

    adapters
        .iter()
        .enumerate()
        .rev()
        .for_each(|(ind, joltage)| {
            if ind == adapters.len() - 1 {
                permutations_to_end_from_ind[ind] = 1;
            } else if ind == adapters.len() - 2 {
                // specific case: 1 from the end
                let mut permutations_from_here = 0;

                if adapters[ind + 1] - joltage <= 3 {
                    permutations_from_here += permutations_to_end_from_ind[ind + 1];
                }

                permutations_to_end_from_ind[ind] = permutations_from_here;
            } else if ind == adapters.len() - 3 {
                // specific case: we're 2 from the end
                let mut permutations_from_here = 0;

                if adapters[ind + 2] - joltage <= 3 {
                    permutations_from_here += permutations_to_end_from_ind[ind + 2];
                }

                if adapters[ind + 1] - joltage <= 3 {
                    permutations_from_here += permutations_to_end_from_ind[ind + 1];
                }

                permutations_to_end_from_ind[ind] = permutations_from_here;
            } else {
                // try up to 3 adapters after this one
                // general case - we're 3 or more from the end
                let mut permutations_from_here = 0;

                if adapters[ind + 3] - joltage <= 3 {
                    permutations_from_here += permutations_to_end_from_ind[ind + 3];
                }

                if adapters[ind + 2] - joltage <= 3 {
                    permutations_from_here += permutations_to_end_from_ind[ind + 2];
                }

                if adapters[ind + 1] - joltage <= 3 {
                    permutations_from_here += permutations_to_end_from_ind[ind + 1];
                }

                permutations_to_end_from_ind[ind] = permutations_from_here;
            }
            // println!(
            //     "{}, {} -> {}",
            //     ind, joltage, permutations_to_end_from_ind[ind]
            // );
        });

    permutations_to_end_from_ind[0] + permutations_to_end_from_ind[1] + permutations_to_end_from_ind[2]
}
