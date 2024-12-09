use std::collections::HashMap;
use std::fs;

/*
파일 읽기
- fs::read_to_string을 사용해 파일 내용을 읽고 문자열로 처리
- 파일이 없거나 읽을 수 없는 경우 에러 처리

문자열 처리
- split_whitespace를 사용해 문자열을 단어 단위로 분리
- matches 메서드로 특정 문자 패턴을 찾아 문장 개수 계산

데이터 구조 활용
- HashMap을 사용해 단어 빈도를 저장하고 업데이트
- entry와 or_insert를 활용한 키-값 쌍 관리
*/

fn main() {
    // 파일 읽기
    let file_content = fs::read_to_string("sample.txt").expect("파일을 읽을 수 없습니다.");

    // 단어 개수 계산
    let word_count = file_content.split_whitespace().count();

    // 단어 빈도 계산
    let mut word_frequency: HashMap<&str, usize> = HashMap::new();
    for word in file_content.split_whitespace() {
        *word_frequency.entry(word).or_insert(0) += 1; 
    }

    // 문장 개수 계산
    let sentence_count = file_content.matches(|c| c == '.' || c == '!' || c == '?').count();

    // 결과 출력
    println!("단어 개수: {}", word_count);
    println!("문장 개수: {}", sentence_count);
    println!("단어 빈도:");
    for (word, count) in word_frequency {
        println!("{}: {}", word, count);
    }
}