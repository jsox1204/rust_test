use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 맞춰보세요!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100); // 컴퓨터가 무작위로 생성한 숫자

    loop {
        println!("숫자를 입력하세요.");

        let mut guess = String::new(); // 플레이어가 입력할 숫자

        std::io::stdin()
            .read_line(&mut guess)
            .expect("line 읽기 실패");

        let guess: u32 = guess.trim().parse().expect("숫자로 입력하세요!"); // string 타입을 unsigned 32bit integer 타입으로 변환 (플레이어가 입력한 숫자)

        match guess.cmp(&secret_number) { // 플레이어가 입력한 숫자와 컴퓨터가 무작위로 생성한 숫자를 비교 
            Ordering::Less => println!("숫자가 작습니다!"),
            Ordering::Greater => println!("숫자가 큽니다!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    }
}