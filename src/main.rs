mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

aoc_main::main! {
    year 2023;
    day1             => part1, part2_take1, part2_take2;
    day2 : generator => part1, part2;
    day3 : generator => part1, part2;
    day4 : generator => part1, part2;
    day5 : generator => part1, part1_with_ranges, part2 /*, part2_naive: doesn't complete */;
    day6 : generator => part1, part1_binary, part2, part2_binary;
    day7             => part1, part2;
    day8 : generator => part1, part2;
    day9 : generator => part1, part2;
}
