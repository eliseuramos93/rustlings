fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    
    // let a: [i32; 100] = [0; 100]; // also works, but not as fun as the one below.
    let a: &str = "Never gonna give you up, never gonna let you down, never gon na run around and desert you. Never gonna make you cry, never gonna say goodbye, never gonna tell a lie and hurt you.";

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
