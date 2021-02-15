use std::{env, io, fs};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let var = &args[1];
    if var == "-r" {
        let args = env::args()
            .skip(2)
            .collect::<Vec<String>>();
        let mut line_list = vec![];
        for file in args.clone() {
            let file_contents = fs::read_to_string(&file)?;
            for lines in file_contents.lines() {
                &line_list.push(lines.to_string());
                &line_list.sort();
                &line_list.reverse();
                if lines == file_contents.lines().last().unwrap() {
                    for line in line_list.clone() {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    else {
        let args = env::args()
            .skip(1)
            .collect::<Vec<String>>();
        let mut line_list = vec![];
        for file in args.clone() {
            let file_contents = fs::read_to_string(&file)?;
            for lines in file_contents.lines() {
                &line_list.push(lines.to_string());
                &line_list.sort();
                if lines == file_contents.lines().last().unwrap(){
                    for line in line_list.clone() {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}
