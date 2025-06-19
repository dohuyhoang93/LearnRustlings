// TODO: Fix the compiler error in the `main` function without changing this function.
fn is_a_color_word(word: String) -> bool {
    word == "green" || word == "blue" || word == "red"
}

fn main() {
    let words = String::from("greens"); // Don't change this line.

    if is_a_color_word(words) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
