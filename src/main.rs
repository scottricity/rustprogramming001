fn am_i_your_dad(legalbool: bool) -> &'static str {
    if legalbool { return "yes" } else {return "nah g"}
}

fn main() {
    let dad = true;
    print!("{}", am_i_your_dad(dad));
}