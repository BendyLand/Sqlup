use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file = fs::read_to_string(args[1].to_string()).unwrap_or("".to_string());
        let new_file = process_file(file);
        fs::write(args[1].to_string(), new_file).expect("Unable to write file.\n");
    }
    else {
        println!("Usage: sqlup <filepath>");
    }
}

fn process_file(file: String) -> String {
    let lines = file.lines();
    let keywords = get_keywords();
    let mut new_lines = Vec::<String>::new();
    for line in lines {
        let mut new_line = line.to_string();
        for kw in keywords.clone() {
            let word = format!(r"\b{}\b", kw);
            let pattern = Regex::new(&word).unwrap();
            if pattern.is_match(&new_line) {
                new_line = capitalize_keyword(new_line.to_string(), kw);
            }
        }
        new_lines.push(new_line);
    }
    let result = new_lines.join("\n");
    return result;
}

fn capitalize_keyword(line: String, keyword: String) -> String {
    let start = line.find(&keyword).unwrap();
    let mut chars: Vec<char> = line.chars().collect();
    for i in start..start+keyword.len() {
        chars[i] = {
            line
                .chars()
                .nth(i)
                .unwrap()
                .to_uppercase()
                .nth(0)
                .unwrap()
        };
    }
    let result = chars.into_iter().collect::<String>();
    return result;
}

fn get_keywords() -> HashSet<String> {
    let keywords = HashSet::from([
        "add", "all", "alter", "and", "any", "as", "asc", "backup", "begin",
        "between", "by", "case", "check", "column", "constraint", "create",
        "database", "declare", "default", "delete", "desc", "distinct", "drop",
        "end", "exec", "exists", "foreign", "from", "full", "group", "having",
        "in", "index", "inner", "insert", "insert", "into", "is", "join",
        "key", "left", "like", "limit", "not", "null", "on", "or", "order",
        "outer", "primary", "procedure", "references", "replace", "right",
        "rownum", "select", "set", "table", "top", "truncate", "union",
        "unique", "update", "values", "view", "where",
    ].into_iter().map(|x| x.to_string()).collect::<HashSet<String>>());
    return keywords;
}
