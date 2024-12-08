use std::collections::HashMap;
use std::fs;

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