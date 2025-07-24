use std::io;

fn main() {
    let mut count = 0;
    loop {
        let mut answer = "".to_string();
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        let _ = io::stdin().read_line(&mut answer);
        count +=1;
        if answer.trim() == "The letter e" {
            println!("Number of trials: {count}");
            break;
        }
    }
}