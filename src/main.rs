use std::{env, io, fs};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let var = &args[1];

    if var == "-r" {
        let args = env::args().skip(1);
        for file in args.skip(1) {
            println!("{}",file);
            let file_contents = fs::read_to_string(&file)?;
            let line_list = sort_1(file_contents);
            for line in line_list {
                println!("{}", line);
            }
        }
    }
    else {
        let args = env::args();
        for file in args {
            println!("{}",file);
            let file_contents = fs::read_to_string(&file)?;
            let line_list = sort_2(file_contents);
            for line in line_list {
                println!("{}", line);
            }
        }
    }
    Ok(())
}


fn sort_1(file_contents : String) -> Vec<&'static str> {

    let mut line_list= vec![];
    for lines in file_contents.lines() {
        line_list.push(lines);
        line_list.sort();
    }
    return line_list
}

fn sort_2(file_contents : String) -> Vec<&'static str> {

    let mut line_list= vec![];
    for lines in file_contents.lines() {
        line_list.push(lines);
        line_list.sort();
        line_list.reverse()
    }
    return line_list
}
