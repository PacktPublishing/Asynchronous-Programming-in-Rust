use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let mainrs = fs::read_to_string("./src/main.rs").unwrap();
    
    let start = mainrs.find("dude").unwrap();
    
    let mut brackets_counter = 0;
    let mut end = start;
    
    for (i, char) in mainrs[start..].chars().enumerate() {
        end += 1;
        match char {
            '{' => brackets_counter += 1,
            '}' => {
                brackets_counter -= 1;
                if brackets_counter == 0 {
                    break
                }
            }
            _ => (),
        }    
    }
    
    let async_fn = String::from(&mainrs[start..end]);
    
    let rewritten = rewrite_async_fn(&async_fn);
    
    let mut new = fs::File::create("./src/hello.rs").unwrap();
    new.write_all(&mainrs[..start].as_bytes()).unwrap();
    new.write_all(&mainrs[end..].as_bytes()).unwrap();
    new.write_all(rewritten.as_bytes()).unwrap();
    
    println!("cargo:rerun-if-changed=build.rs");
}

fn rewrite_async_fn(s: &str) -> String {
    use std::fmt::Write;
    // find lines that have keyword
    
    let new_fn = &s[6..];
    
    let mut steps = vec![];
    let mut futures = vec![];
    
    let mut buffer = String::new();
    for line in s.lines() {
        if line.contains("chill") {
            steps.push(buffer.clone());
            buffer.clear();
            futures.push(line.to_string());
            
        } else {
            buffer.push_str(line);
            buffer.push_str("\n");
        }
    }
    
    let mut steps_enum = "
    enum Steps {
        Start,
        ".to_string();
    
    for i in 0..steps.len() {
        write!(&mut steps_enum, "Step{}", i+ 1).unwrap();
        write!(&mut steps_enum, "(Box<dyn Future<Output = String>>),\n").unwrap();
    }
    
    writeln!(&mut steps_enum, "Resolved,").unwrap();
    writeln!(&mut steps_enum, "}}").unwrap();
    
    let coroutine = "
    struct Coroutine {
        steps: Steps,
    }
    ";
    
    format!("{steps_enum}\n{coroutine}")
    
}