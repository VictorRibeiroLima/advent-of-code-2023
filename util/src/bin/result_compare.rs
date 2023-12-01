fn main() {
    // this is a helper program to compare the results in case i need to debug against the solution
}

fn compare(a: &str, b: &str) {
    for (a_line, my_line) in a.lines().zip(b.lines()) {
        if a_line != my_line {
            println!("a:{} \nb:{}", a_line, my_line);
            println!("----------------")
        }
    }
}
