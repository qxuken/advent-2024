use std log

def log_levels [] { ["t", "trace", "d", "debug", "i", "info"] }
def available_days [] { ls src/solutions | select name | each {|r| $r.name | parse --regex '(\d+)' |  get capture0 } | flatten }

# Test aoc given day task
export def test [
    day: int@available_days # Day of the aoc
    --star (-s) # Should run second star problem
] {
    if $star {
        ^cargo test $"solutions::day($day)::test::validate_second_star_example"
    } else {
        ^cargo test $"solutions::day($day)::test::validate_one_star_example"
    }
}
export alias t = test

# Run aoc given day task
export def run [
    day: int@available_days # Day of the aoc
    --input (-i): string # Custom input file <FILE>
    --release (-r) # Should build and run in release mode
    --star (-s) # Should run second star problem
    --log (-l): string@log_levels = "t" # Logger levels t|trace,d|debug,i|info
] {

    let target = if $release { "release" } else { "debug" }

    log debug $"Building project in ($target) mode..."
    if $release {
        ^cargo build --release
    } else {
        ^cargo build
    }

    let input_file = if ($input | is-not-empty) { $input } else { $"./inputs/day($day).txt"}

    let args = [$input_file, $"day($day)", "--logger", "json"]

    let args = match $log {
        "trace" | "t" => ($args | append "-vv"),
        "debug" | "d" => ($args | append "-v"),
        _ => $args,
    }

    let args = match $star {
        true => ($args | append "-s"),
        false => $args,
    }

    let exec_path = $"./target/($target)/advent_2024"
    log debug $"Running second-star puzzle for ($day) with input file ($input)..."
    let logs = run-external $exec_path ...$args
    $logs | lines | each {|r| $r | from json }
}
export alias r = run
