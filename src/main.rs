use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "-s" || args[1] == "--string" {
            if args.len() > 2 {
                let file = args[2].clone();
                let new_file = process_file(file);
                println!("\n{}\n", new_file);
            }
            else {
                print_help();
            }
        }
        else {
            let file = fs::read_to_string(args[1].to_string()).unwrap_or("".to_string());
            let new_file = process_file(file);
            fs::write(args[1].to_string(), new_file).expect("Unable to write file.\n");
        }
    }
    else {
        print_help();
    }
}

fn replace_keywords(part: &str, keywords: &HashSet<String>) -> String {
    let mut processed = part.to_string();
    println!("{}", &part);
    for kw in keywords {
        let word = format!(r"\b{}\b", regex::escape(kw));
        let pattern = Regex::new(&word).unwrap();
        processed = pattern
            .replace_all(&processed, |_: &regex::Captures| kw.to_uppercase())
            .to_string();
    }
    return processed;
}

fn process_file(file: String) -> String {
    let lines = file.lines();
    let keywords = get_keywords();
    let mut new_lines = Vec::<String>::new();
    for line in lines {
        // Regex to match text outside quotes
        let re = Regex::new(r#"(?:"[^"]*"|'[^']*')|[^"' ]+"#).unwrap();
        let mut updated_line = String::new();
        let mut last_match_end = 0;
        for mat in re.find_iter(&line) {
            let part = mat.as_str();
            updated_line.push_str(&line[last_match_end..mat.start()]); // Add unmatched text
            if !part.starts_with('"') && !part.starts_with('\'') {
                // Process non-quoted parts to replace keywords
                let processed = replace_keywords(part, &keywords);
                updated_line.push_str(&processed);
            } 
            else {
                // Keep quoted parts unchanged
                updated_line.push_str(part);
            }
            last_match_end = mat.end();
        }
        updated_line.push_str(&line[last_match_end..]); // Add any remaining unmatched text
        new_lines.push(updated_line);
    }
    return new_lines.join("\n");
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

fn print_help() {
    println!("Usage:\nsqlup <filepath>\nsqlup -s <string>\nsqlup --string <string>");
}