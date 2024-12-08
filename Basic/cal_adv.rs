use std::io;

/* 
벡터의 동적 관리
- Vec에서 요소를 추가, 삭제(remove)하는 방법
- 벡터에서 특정 조건을 만족하는 요소를 찾는 position메서드 활용

Rust의 소유권과 참조
- Vec<String>을 사용해 소유권을 관리하고, 문자열 업데이트 시 참조 문제를 방지
- Strin과 &str간 변환(to_string, as_str) 처리

에러 처리와 안전성
- parse 메서드로 문자열을 숫자로 변환할 때 발생할 수 있는 에러 처리
- 연산자 주변에 필요한 피연산자가 존재하는지 확인하는 경계 조건 처리


*/

fn main() {
    loop {
        println!("다중 연산을 입력하세요: ");

        // input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력 실패");

        // parsing
        let mut tokens: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();

        // calculating
        loop {
            if tokens.len() == 1 {
                println!("결과: {}", tokens[0]);
                break;
            }

            // 연산 우선 순위대로 vec를 순회하며 연산자를 할당 받기
            let op_idx = match tokens.iter().position(|c| c == "*" || c == "/") {
                Some(idx) => idx,
                None => match tokens.iter().position(|c| c == "+" || c == "-") {
                    Some(idx) => idx,
                    None => {
                        println!("연산자가 없습니다.");
                        break;
                    }
                },
            };

            if op_idx == 0 || op_idx + 1 >= tokens.len() {
                println!("연산자 주변에 피연산자가 부족합니다.");
                break;
            }

            let operator = &tokens[op_idx];

            let left: f64 = match tokens[op_idx - 1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("잘못된 숫자입니다. 다시 시도하세요.");
                    break;
                }
            };

            let right: f64 = match tokens[op_idx + 1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("잘못된 숫자입니다. 다시 시도하세요.");
                    break;
                }
            };

            let result = match operator.as_str() {
                "+" => left + right,
                "-" => left - right,
                "*" => left * right,
                "/" => {
                    if right == 0.0 {
                        println!("0으로 나눌 수 없습니다.");
                        break;
                    }
                    left / right
                }
                _ => {
                    println!("지원하지 않는 연산자입니다.");
                    break;
                }
            };

            // 결과 갱신
            tokens[op_idx - 1] = result.to_string(); // 결과를 왼쪽 피연산자 위치에 저장
            tokens.remove(op_idx + 1); // 오른쪽 피연산자 제거
            tokens.remove(op_idx); // 연산자 제거
        }
    }
}