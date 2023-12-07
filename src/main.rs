mod day1;
mod day2;
mod day3;
mod day4;

aoc_main::main! {
    year 2023;
    day1             => part1, part2_take1, part2_take2;
    day2 : generator => part1, part2;
    day3 : generator => part1, part2;
    day4 : generator => part1;
}
