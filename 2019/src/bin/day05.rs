use std::collections::VecDeque;
use std::convert::TryInto;

static INPUT: &str = include_str!("../../input/day05.txt");

fn main() {
    let input: Vec<i32> = INPUT.trim().split(",").map(|value| value.parse().unwrap()).collect();

    let (_state, output) = execute_program(&input, &vec![1]);
    println!("part 1: {}", output.last().unwrap());

    let (_state, output) = execute_program(&input, &vec![5]);
    println!("part 2: {}", output.last().unwrap());
}

fn execute_program(program: &[i32], input: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut state = program.to_owned();
    let mut input = VecDeque::from(input.to_owned());
    let mut output = vec![];
    let mut current_index = 0;

    loop {
        let operation = get_operation(state[current_index] % 100);
        let operand1mode = get_mode(state[current_index] / 100 % 10);
        let operand2mode = get_mode(state[current_index] / 1000 % 10);
        let operand3mode = get_mode(state[current_index] / 10000 % 10);

        match operation {
            Operation::Add => {
                if operand3mode == Mode::Immediate { panic!("output operand cant be immediate") };

                let operand1 = state[current_index + 1];
                let operand2 = state[current_index + 2];
                let operand3 = state[current_index + 3];

                let argument1 = get_argument(&state, operand1, &operand1mode);
                let argument2 = get_argument(&state, operand2, &operand2mode);

                state[operand3 as usize] = argument1 + argument2;

                current_index += 4;
            },
            Operation::Mul => {
                if operand3mode == Mode::Immediate { panic!("output operand cant be immediate") };

                let operand1 = state[current_index + 1];
                let operand2 = state[current_index + 2];
                let operand3 = state[current_index + 3];

                let argument1 = get_argument(&state, operand1, &operand1mode);
                let argument2 = get_argument(&state, operand2, &operand2mode);

                state[operand3 as usize] = argument1 * argument2;

                current_index += 4;
            },
            Operation::Input => {
                if operand1mode == Mode::Immediate { panic!("output operand cant be immediate") };

                let operand1 = state[current_index + 1];

                let value = input.pop_front().expect("input is empty");

                state[operand1 as usize] = value;

                current_index += 2;
            },
            Operation::Output => {
                let operand1 = state[current_index + 1];

                let argument1 = get_argument(&state, operand1, &operand1mode);

                output.push(argument1);

                current_index += 2;
            },
            Operation::Equal => {
                if operand3mode == Mode::Immediate { panic!("output operand cant be immediate") };

                let operand1 = state[current_index + 1];
                let operand2 = state[current_index + 2];
                let operand3 = state[current_index + 3];

                let argument1 = get_argument(&state, operand1, &operand1mode);
                let argument2 = get_argument(&state, operand2, &operand2mode);

                state[operand3 as usize] = if argument1 == argument2 { 1 } else { 0 };

                current_index += 4;
            },
            Operation::LessThan => {
                if operand3mode == Mode::Immediate { panic!("output operand cant be immediate") };

                let operand1 = state[current_index + 1];
                let operand2 = state[current_index + 2];
                let operand3 = state[current_index + 3];

                let argument1 = get_argument(&state, operand1, &operand1mode);
                let argument2 = get_argument(&state, operand2, &operand2mode);

                state[operand3 as usize] = if argument1 < argument2 { 1 } else { 0 };

                current_index += 4;
            },
            Operation::JumpIfTrue => {
                let operand1 = state[current_index + 1];
                let operand2 = state[current_index + 2];

                let argument1 = get_argument(&state, operand1, &operand1mode);
                let argument2 = get_argument(&state, operand2, &operand2mode);

                if argument1 != 0 {
                    current_index = argument2.try_into().unwrap();
                } else {
                    current_index += 3;
                }
            },
            Operation::JumpIfFalse => {
                let operand1 = state[current_index + 1];
                let operand2 = state[current_index + 2];

                let argument1 = get_argument(&state, operand1, &operand1mode);
                let argument2 = get_argument(&state, operand2, &operand2mode);

                if argument1 == 0 {
                    current_index = argument2.try_into().unwrap();
                } else {
                    current_index += 3;
                }
            },
            Operation::Break => break,
        };
    };

    (state, output)
}

#[derive(PartialEq, Eq, Debug)]
enum Operation {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equal,
    Break,
}

fn get_operation(int: i32) -> Operation {
    match int {
        1 => Operation::Add,
        2 => Operation::Mul,
        3 => Operation::Input,
        4 => Operation::Output,
        5 => Operation::JumpIfTrue,
        6 => Operation::JumpIfFalse,
        7 => Operation::LessThan,
        8 => Operation::Equal,
        99 => Operation::Break,
        x => panic!("invalid operation {}", x),
    }
}

#[derive(PartialEq, Eq)]
enum Mode {
    Position,
    Immediate,
}

fn get_mode(int: i32) -> Mode {
    match int {
        0 => Mode::Position,
        1 => Mode::Immediate,
        x => panic!("invalid mode {}", x),
    }
}

fn get_argument(state: &[i32], operand: i32, mode: &Mode) -> i32 {
    match mode {
        Mode::Position => state[operand as usize],
        Mode::Immediate => operand,
    }
}

#[test]
fn test_execute_program_basic() {
    assert_eq!(execute_program(&vec![1,9,10,3,2,3,11,0,99,30,40,50], &vec![]), (vec![3500,9,10,70,2,3,11,0,99,30,40,50], vec![]));
    assert_eq!(execute_program(&vec![1,0,0,0,99], &vec![]), (vec![2,0,0,0,99], vec![]));
    assert_eq!(execute_program(&vec![2,3,0,3,99], &vec![]), (vec![2,3,0,6,99], vec![]));
    assert_eq!(execute_program(&vec![2,4,4,5,99,0], &vec![]), (vec![2,4,4,5,99,9801], vec![]));
    assert_eq!(execute_program(&vec![1,1,1,4,99,5,6,0,99], &vec![]), (vec![30,1,1,4,2,5,6,0,99], vec![]));
}

#[test]
fn test_execute_program_io() {
    assert_eq!(execute_program(&vec![3,5,99,0,0,0], &vec![666]), (vec![3,5,99,0,0,666], vec![]));
    assert_eq!(execute_program(&vec![4,5,99,0,0,666], &vec![]), (vec![4,5,99,0,0,666], vec![666]));
    assert_eq!(execute_program(&vec![3,0,4,0,99], &vec![123]), (vec![123,0,4,0,99], vec![123]));
}

#[test]
fn test_execute_program_immediate_mode() {
    assert_eq!(execute_program(&vec![1002,4,3,4,33], &vec![]), (vec![1002,4,3,4,99], vec![]));
}

#[test]
fn test_execute_program_comparison() {
    let program = vec![3,9,8,9,10,9,4,9,99,-1,8];
    assert_eq!(execute_program(&program, &vec![1]).1, vec![0]);
    assert_eq!(execute_program(&program, &vec![7]).1, vec![0]);
    assert_eq!(execute_program(&program, &vec![8]).1, vec![1]);
    assert_eq!(execute_program(&program, &vec![10]).1, vec![0]);

    let program = vec![3,9,7,9,10,9,4,9,99,-1,8];
    assert_eq!(execute_program(&program, &vec![1]).1, vec![1]);
    assert_eq!(execute_program(&program, &vec![7]).1, vec![1]);
    assert_eq!(execute_program(&program, &vec![8]).1, vec![0]);
    assert_eq!(execute_program(&program, &vec![10]).1, vec![0]);

    let program = vec![3,3,1108,-1,8,3,4,3,99];
    assert_eq!(execute_program(&program, &vec![1]).1, vec![0]);
    assert_eq!(execute_program(&program, &vec![7]).1, vec![0]);
    assert_eq!(execute_program(&program, &vec![8]).1, vec![1]);
    assert_eq!(execute_program(&program, &vec![10]).1, vec![0]);

    let program = vec![3,3,1107,-1,8,3,4,3,99];
    assert_eq!(execute_program(&program, &vec![1]).1, vec![1]);
    assert_eq!(execute_program(&program, &vec![7]).1, vec![1]);
    assert_eq!(execute_program(&program, &vec![8]).1, vec![0]);
    assert_eq!(execute_program(&program, &vec![10]).1, vec![0]);
}

#[test]
fn test_execute_program_jump() {
    let program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    assert_eq!(execute_program(&program, &vec![0]).1, vec![0]);
    assert_eq!(execute_program(&program, &vec![-1]).1, vec![1]);
    assert_eq!(execute_program(&program, &vec![1]).1, vec![1]);
    assert_eq!(execute_program(&program, &vec![10]).1, vec![1]);

    let program = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
    assert_eq!(execute_program(&program, &vec![0]).1, vec![0]);
    assert_eq!(execute_program(&program, &vec![-1]).1, vec![1]);
    assert_eq!(execute_program(&program, &vec![1]).1, vec![1]);
    assert_eq!(execute_program(&program, &vec![10]).1, vec![1]);
}

#[test]
fn test_execute_program_complex() {
    let program = vec![
        3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,
        125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
    ];

    assert_eq!(execute_program(&program, &vec![7]).1, vec![999]);
    assert_eq!(execute_program(&program, &vec![8]).1, vec![1000]);
    assert_eq!(execute_program(&program, &vec![9]).1, vec![1001]);
}
