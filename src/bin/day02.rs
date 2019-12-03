use std::ops::{Add, Mul};

static INPUT: &str = include_str!("../../input/day02.txt");

fn main() {
    let input: Vec<i32> = INPUT.trim().split(",").map(|value| value.parse().unwrap()).collect();

    let state = execute_program_with_modification(&input, 12, 2);
    println!("part 1: {}", state[0]);

    let (noun, verb) = search_desired_output(&input, 19690720);
    println!("part 2: {}", 100 * noun + verb);
}

fn search_desired_output(program: &[i32], desired_output: i32) -> (i32, i32) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let result = execute_program_with_modification(program, noun, verb);

            if result[0] == desired_output {
                return (noun, verb);
            }
        }
    }

    panic!("no match found");
}

fn execute_program_with_modification(program: &[i32], noun_modification: i32, verb_modification: i32) -> Vec<i32> {
    let mut new_program = program.to_owned();

    new_program[1] = noun_modification;
    new_program[2] = verb_modification;

    execute_program(&new_program)
}

fn execute_program(program: &[i32]) -> Vec<i32> {
    let mut state = program.to_owned();
    let mut current_index = 0;

    loop {
        let operator: fn(i32, i32) -> i32 = match state[current_index] {
            1 => Add::add,
            2 => Mul::mul,
            99 => break,
            x => panic!("unknow opcode {}", x)
        };

        let operand1 = state[state[current_index + 1] as usize];
        let operand2 = state[state[current_index + 2] as usize];
        let result_index = state[current_index + 3] as usize;
        let result = operator(operand1, operand2);

        state[result_index] = result;

        current_index += 4;
    };

    state
}

#[test]
fn test_execute_program() {
    assert_eq!(execute_program(&vec![1,9,10,3,2,3,11,0,99,30,40,50]), vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
    assert_eq!(execute_program(&vec![1,0,0,0,99]), vec![2,0,0,0,99]);
    assert_eq!(execute_program(&vec![2,3,0,3,99]), vec![2,3,0,6,99]);
    assert_eq!(execute_program(&vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
    assert_eq!(execute_program(&vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
}
