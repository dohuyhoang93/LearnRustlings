// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?

use std::num::ParseIntError;

// Don't change this function.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: Fix the compiler error by changing the signature and body of the
// `main` function.
fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Don't change this line.
    let cost = total_cost(pretend_user_input)?;
// _______________________________________________^__ thêm toán tử `?` là cách đúng để xử lý lỗi khi chương trình phát sinh nhập sai.
// Dùng `unwrap()` tại đây khi lỗi sẽ gây `panic` không cần thiết
    // thread 'main' panicked at error.rs:56:47:
    // called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }
    Ok (()) //Thành công nhưng không chứa giá trị trả về nào (type `()` )
}
