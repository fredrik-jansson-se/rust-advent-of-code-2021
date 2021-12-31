use std::collections::HashMap;
use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day25.txt").unwrap();
    println!("day25-1: {}", run_1(&input)?);
    println!("day25-2: {}", run_2(&input)?);
    Ok(())
}

enum Dir {
    East,
    South,
}

fn next_step(width: usize, height: usize) -> impl Fn(Dir, (usize, usize)) -> (usize, usize) {
    move |dir, (row, col)| match dir {
        East => (row, (col + 1) % width),
        South => ((row + 1) % height, col),
    }
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, map) = parse(input).map_err(|e| e.to_owned())?;
    todo!()
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

type Map = HashMap<(usize, usize), Dir>;
fn parse(i: &str) -> nom::IResult<&str, (usize, usize, Map)> {
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn aoc25_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 58);
    }
}
