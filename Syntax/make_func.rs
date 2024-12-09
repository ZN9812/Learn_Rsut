
// 아무 인자를 필요로 하지 않는다.
// i32를 [-> return type]으로 가진다.
// c => return 8; , rust => 8 (; return)
fn number() -> i32 {
    8  // ; < 반환값에는 ;을 사용하면 안된다. 
}

fn multiply(number_one: i32, number_two: i32) -> i32 {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
    result
}

fn main() {
    println("Hello, world number {}!", number());
    let multiply_result = multiply(8, 9);
}