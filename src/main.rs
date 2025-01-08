use copypasta::{ClipboardContext, ClipboardProvider};
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = args[1..].to_vec();
    let flags = check_arg_flags(&args);
    let file_name = find_file_name(&args);
    let file = {
        if file_name.len() > 0 { read_to_string(file_name.clone()).unwrap_or("".to_string()) } 
        else { find_string_arg(&args) }
    };
    if flags.len() > 0 {
        if flags.contains(&Arg::STRING) {
            if flags.contains(&Arg::COPY) {
                let new_file = process_file(file);
                copy_to_clipboard(new_file);
            }
            else if file.len() > 0 {
                let new_file = process_file(file);
                println!("\n{}\n", new_file);
            }
            else { print_help(); }
        }
        else if flags.contains(&Arg::COPY) {
            if file.len() > 0 {
                let new_file = process_file(file);
                copy_to_clipboard(new_file);
            }
            else { print_help(); }
        }
        else {
            let file = fs::read_to_string(args[1].to_string()).unwrap_or("".to_string());
            let new_file = process_file(file);
            fs::write(args[1].to_string(), new_file).expect("Unable to write file.\n");
        }
    }
    else { 
        if file.len() > 0 {
            let new_file = process_file(file);
            fs::write(file_name, new_file).expect("Unable to write file.\n");
        }
        else {
            print_help(); 
        }
    }
}

fn replace_keywords(part: &str, keywords: &HashSet<String>) -> String {
    let mut processed = part.to_string();
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
        "add", "after", "all", "alter", "and", "any", "as", "asc", "backup",
        "begin", "before", "between", "by", "case", "check", "column",
        "constraint", "create", "database", "declare", "default", "delete",
        "desc", "distinct", "drop", "each", "end", "exec", "execute",
        "exists", "foreign", "for", "from", "full", "group", "having", "in",
        "index", "inner", "insert", "insert", "into", "is", "join", "key",
        "left", "like", "limit", "not", "null", "on", "or", "order", "outer",
        "primary", "procedure", "references", "replace", "right", "row",
        "rownum", "select", "set", "table", "top", "trigger", "truncate",
        "union", "unique", "update", "values", "view", "where", "when"
    ].into_iter().map(|x| x.to_string()).collect::<HashSet<String>>());
    return keywords;
}

fn print_help() {
println!(
"
Usage:

sqlup     <filepath>
sqlup -c  <filepath> or sqlup --copy   <filepath>      (to copy results to clipboard)
sqlup -s  <string>   or sqlup --string <string>        (to provide a string instead of a filename)

Flags may also be combined:
sqlup -sc <string>   or sqlup --copy --string <string> (to copy a provided string)
"
);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Arg {
    STRING,
    COPY,
}

fn check_arg_flags(args: &Vec<String>) -> Vec<Arg> {
    let mut result = Vec::<Arg>::new();
    let pattern = Regex::new(r"-[sc]*[sc]").unwrap();
    for arg in args {
        if pattern.is_match(&arg) {
            if arg.contains("s") { result.push(Arg::STRING); }
            if arg.contains("c") { result.push(Arg::COPY); }
        }
        else if arg.contains("--") {
            if arg.contains("string") { result.push(Arg::STRING); }
            if arg.contains("copy") { result.push(Arg::COPY); }
        }
    }
    return result;
}

fn copy_to_clipboard(text: String) {
    if !text.is_empty() {
        let mut ctx = ClipboardContext::new().expect("Failed to initialize system clipboard.");
        ctx.set_contents(text).expect("Unable to set clipboard contents");
        println!("Result copied to clipboard!");
    }
    else { print_help(); }
}

fn find_file_name(args: &Vec<String>) -> String {
    let mut result = String::new();
    let pattern = Regex::new(r"\.d.l").unwrap();
    for arg in args {
        let matched = {
            arg.contains("ql") ||
            arg.contains("db") ||
            pattern.is_match(arg)
        };
        if matched {
            result = arg.clone();
            break;
        }
    }
    return result;
}

fn find_string_arg(args: &Vec<String>) -> String {
    let mut result = String::new();
    for arg in args {
        if arg.contains(";") {
            result = arg.clone();
            break;
        }
    }
    if result.is_empty() {
        let keywords = get_keywords();
        for arg in args {
            if contains_any(&arg, &keywords) {
                result = arg.clone();
                break;
            }
        }
    }
    return result;
}

fn contains_any(target: &String, checks: &HashSet<String>) -> bool {
    for check in checks {
        if target.contains(check) { return true; }
    }
    return false;
}