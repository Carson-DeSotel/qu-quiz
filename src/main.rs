use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_from_file (filename: &Path) -> Vec<String> {
    let file = File::open(filename)
                     .expect("file not found...");
    let buf  = BufReader::new(file);
    buf.lines()
       .map(|l| l.unwrap())
       .collect()
}

fn qu_words (lst: &Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for line in lst {
        if line.contains("q") && line.contains("u") {
            res.push(line.to_string());
        }
    }
    res
}

fn explode (s: &String) -> Vec<char> {
    s.chars().collect()
}

fn implode (c: Vec<char>) -> String {
    c.iter().collect()
}

fn sort (s: &String) -> String {
    let mut r = explode(s);
    r.sort_by(|a, b| a.cmp(b));
    implode(r)
}

fn main () {
    let args: Vec<String> = env::args().collect();

    let filename = Path::new(&args[1]);
    let lst = read_from_file(filename);

    let qu_words = qu_words(&lst);
    let qu = "qu".to_string();
    let mut res: Vec<(String, String)> = Vec::new();

    for s in &lst {
        let t = format!("{}{}", &qu, &s); 
        for q in &qu_words {
            if sort(&q) == sort(&t) {
                res.push((s.to_string(), q.to_string()));
            }
        }
    }

    for (s1, s2) in res {
        println!("({}, {})", s1, s2);
    }

    println!("Finished");
}