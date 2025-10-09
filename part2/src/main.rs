// you'll need these two imports to read input form the user
use std::io;
use std::io::Write;

fn gueess_word (corrcet_word :&String,word :&str)->bool {
   for w in corrcet_word.chars(){
       if w.to_string()== word{
           return true;
       }
   }
   return false;
}
fn main() {
   let correct_word = String::from("lobster");
// Make sure the prompt from the previous line gets displayed:
let mut guess_num = correct_word.len() + 5;
let mut count = correct_word.len();
loop{
    if guess_num == 0 {
        print!("error didnot guess right");
        break;
    }
    print!("Please guess a letter: ");
    io::stdout()
        .flush()
        .expect("Error flushing stdout.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line.");
        if gueess_word(&correct_word, &guess[0..1]) {
             count -= 1;
             println!("cool");
        }
        if count == 0{
            println!("succeess");
            break;
        }
         guess_num -=1;
    }
}
