use std::io; // 표준 입출력 위한 모듈 가져오기

fn main(){
    loop { // 무한 루프를 사용하여 사용자가 종료할 때까지 반복
        println!("연산을 입력하세요 (예: 5 + 3): "); // 사용자에게 입력 요청
        
        let mut input = String::new(); // 입력값을 저장할 문자열 변수 생성
        io::stdin().read_line(&mut input).expect("입력 실패"); // 사용자 입력 읽기
        
        let tokens: Vec<&str> = input.trim().split_whitespace().collect(); // 입력값을 공백으로 분리하여 토큰화
        if tokens.len() < 3 {
            println!("잘못된 입력 형식입니다. 다시 시도하세요."); // 입력 형식이 잘못된 경우 메세지 출력 
            continue; // 다음 반복으로 넘어가기
        }
        
        // 첫 번째 숫자를 파싱
        let left: f64 = match tokens[0].parse() { 
            Ok(num) => num, // 성공적으로 숫자로 변환된 경우
            Err(_) => { // 숫자로 변환 실패 시
                println!("잘못된 숫자입니다. 다시 시도하세요.");
                continue; 
            }
        };
        
        let operator = tokens[1]; // 연산자 추출
        
        // 두 번째 숫자를 파싱
        let right: f64 = match tokens[2].parse() { 
            Ok(num) => num,
            Err(_) => {
                println!("잘못된 숫자입니다. 다시 시도하세요.");
                continue;
            }
        };
        
        // 연산자에 따른 계산 수행
        let result = match operator {
            "+" => left + right, // 덧셈
            "-" => left - right, // 뺄셈
            "*" => left * right, // 곱셈
            "/" => {
                if right == 0.0 {
                    println!("0으로 나눌 수 없습니다."); // 나눗셈에서 0으로 나누는 경우
                    continue; 
                }
                left / right // 나눗셈
            } _ => {
                println!("지원하지 않는 연산자입니다. 다시 시도하세요.");
                continue;
            }
        };
        
        println!("결과: {}", result);
    }
    
}