use std::{
    fs::File,
    io::{BufWriter, Write},
};

fn main() {
    let file = std::fs::File::create("output3.txt").unwrap();
    let mut write = BufWriter::new(file);
    /*
    let a = include_str!("../../../output.txt");
    let b = include_str!("../../../output2.txt");
    compare(a, b, &mut write);
    */
}

fn compare(a: &str, b: &str, write: &mut BufWriter<File>) {
    for (a_line, my_line) in a.lines().zip(b.lines()) {
        if a_line != my_line {
            write
                .write_fmt(format_args!("a:{} \nb:{}\n", a_line, my_line))
                .unwrap();

            write.write_fmt(format_args!("----------------\n")).unwrap();
            println!("a:{} \nb:{}", a_line, my_line);
            println!("----------------")
        }
    }
}
