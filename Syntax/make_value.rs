fn main() {
    // 코드 블록을 사용해 값을 반환할 수 있다.
    let my_number = {
        let second_number = 8;
            second_number + 9
    };
    println!("My number is: {}", my_number);

    let my_number2 = {
        let second_number = 8;
            second_number + 9; // 블록 마지막에 ; 을 추가하면 반환 값이 없기 때문에
                               // () 아무것도 없음이 반환된다.
    };

    println!("My number is: {:?}", my_number2); // {:?} 을 사용하면 아무것도 없어도 출력
}