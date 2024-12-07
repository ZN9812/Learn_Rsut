use std::io;

fn main() {
    loop {
        println!("다중 연산을 입력하세요: ");

        // input 
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력 실패");

        // parsing
        let mut tokens: Vec<&str> = input.trim().split_whitespace().collect();
        let token_len: u64 = token.len()

        // calculating 
        while token_len >= 3 {
            let Some(op_idx) = tokens.iter().position(|&c| c == '*' || c == '/')
            let operator = tokens[op_idx];

            let left: f64 = match tokens[op_idx - 1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("잘못된 숫자입니다. 다시 시도하세요");
                    break;
                }
            };

            let rigth: f64 = match tokens[op_idx + 1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("잘못된 숫자입니다. 다시 시도하세요");
                    
                }
            }
        }
    }
}