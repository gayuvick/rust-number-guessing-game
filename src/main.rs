use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("🎮 Welcome to the Ultimate Guessing Game! 🔥");

    loop {
        play_game();

        println!("\n🔄 Want to play again? (yes/no)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read line");

        if !play_again.trim().eq_ignore_ascii_case("yes") {
            println!("👋 Thanks for playing! See you next time! 🎉");
            break;
        }
    }
}

fn play_game() {
    let secret_number = rand::rng().random_range(1..=100);
    let mut attempts = 0;

    println!("\n🤖 I have picked a number between 1 and 100. Try to guess it!");

    loop {
        println!("\n🔢 Enter your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please enter a valid number! 😅");
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("📉 Too low! Keep going! 😢"),
            Ordering::Greater => println!("📈 Too high! Try again! 😢"),
            Ordering::Equal => {
                println!("\n🎉🎊 BOOM! You guessed it in {} attempts! 🎊🎉", attempts);
                println!("🔥 You're a pro! Play again to beat your record! 🚀");
                break;
            }
        }
    }
}
