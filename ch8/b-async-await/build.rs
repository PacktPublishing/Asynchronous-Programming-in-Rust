use std::error::Error;
use std::fmt::{write, Write as WriteFmt};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::{env, fs};

const FN_KW: &str = "dude";
const W_KW: &str = "chill";

fn main() {
    let mainrs = fs::read_to_string("./src/main.rs").unwrap();

    if !mainrs.starts_with("// REWRITE") {
        return;
    }
    // remove rewrite line in the start
    let mainrs: String = mainrs.lines().skip(1).map(|l| format!("{l}\n")).collect();

    // Find the start of the async function
    let start = mainrs.find(FN_KW).unwrap();

    // Find the end of the async function
    let mut brackets_counter = 0;
    let mut end = start;

    for char in mainrs[start..].chars() {
        end += 1;
        match char {
            '{' => brackets_counter += 1,
            '}' => {
                brackets_counter -= 1;
                if brackets_counter == 0 {
                    break;
                }
            }
            _ => (),
        }
    }

    // Store the async function in a variable since we'll
    // rewrite it
    let async_fn = String::from(&mainrs[start..end-1]);

    // Rewrite the async function to a state machine instead
    let rewritten = rewrite_async_fn(&async_fn).unwrap();

    // Truncate main.rs and rewrite what's there
    let mut new = fs::File::create("./src/main.rs").unwrap();
    // Write everything befor async function back to the file
    new.write_all(&mainrs[..start].as_bytes()).unwrap();

    // Write everything after the async function to the file
    // (we put the rewritten code last in the file since it's easier
    // to see what's rewritten)
    new.write_all(&mainrs[end..].as_bytes()).unwrap();

    // Comment out the async function
    new.write_all(comment_orig(&async_fn).as_bytes()).unwrap();

    // write new function body
    let (fn_body, _) = mainrs[start + FN_KW.len() + 1..].lines().nth(0).unwrap().split_once("{").unwrap();
    let fn_body = format!("{fn_body} -> impl Future {{\n");
    new.write_all(fn_body.as_bytes()).unwrap();
    new.write_all("Coroutine::new()\n}\n".as_bytes()).unwrap();

    // Write the coroutine implementation
    new.write_all(rewritten.as_bytes()).unwrap();

}

/// Format and comment out the original "async" function
fn comment_orig(orig: &str) -> String {
    let mut res = String::new();
    writeln!(&mut res, "
// =================================
// We rewrite this:
// =================================
    ").unwrap();
    for line in orig.lines() {
        writeln!(&mut res, "// {line}").unwrap();
    }
    writeln!(&mut res, "
// }}

// =================================
// Into this:
// =================================
").unwrap();

    res
}

/// Rewrite the async function (this is very brittle, but does
/// the job for our example)
fn rewrite_async_fn(s: &str) -> Result<String, Box<dyn Error>> {
    let w_kw_len = W_KW.len();

    // Store the code in each "step" in this variable
    let mut steps = vec![];
    // Store the future call that we yield on
    let mut futures = vec![];

    let mut buffer = String::new();
    // Skip the first line since that's the function definition
    for line in s.lines().skip(1) {
        // If the line contains the keyword it's an await-point
        if line.contains(W_KW) {
            // Store the steps since last await point as a "step"
            steps.push(buffer.clone());
            buffer.clear();
            // Remove the keyword itself
            let l = &line[..line.len() - w_kw_len - 1];
            // we need both the future call and the variable name since
            // we most likely reference this variable name in the next "step"
            // This could be:
            // `let txt = Http::get("...").await`
            // or simply
            // `join_all(futures).await`
            match l.split_once("=") {
                Some((var, fut)) =>  {
                    // This could fail in so many ways...
                    let varname = &var[var.find("let").unwrap() + 3..].trim();
                    futures.push((varname.to_string(), fut.to_string()));
                }
                None => futures.push(("_".to_string(), l.trim().to_string())),
            }

            // We store the variable name and the future as a tuple since they're connected

        } else {
            buffer.push_str(line);
            buffer.push_str("\n");
        }
    }

    steps.push(buffer);

    // Write our steps enum. We know it will start with "Start" and end with "Resolved"
    // but we need to add one step for each await point
    let mut steps_enum = "
    enum State {
        Start,
        ".to_string();

    for i in 0..steps.len() - 1 {
        write!(&mut steps_enum, "Wait{}", i + 1)?;
        write!(&mut steps_enum, "(Box<dyn Future<Output = String>>),\n")?;
    }

    writeln!(&mut steps_enum,"
        Resolved,
    }}")?;

    // So, our `State` enum is finished, we create a coroutine struct and a simple
    // `new` implementation
    let coroutine = "
    struct Coroutine {
        state: State,
    }

    impl Coroutine {
        fn new() -> Self {
            Self { state: State::Start }
        }
    }
    ";


    // This is our future implementation
    let mut imp = "
    impl Future for Coroutine {
        type Output = ();

        fn poll(&mut self) -> PollState<()> {
                    "
    .to_string();

    for (i, step) in steps.iter().enumerate() {
        // This is the index for the next step in the state machine
        let next = i + 1;

        // We need to special case the first call since that
        // happens before we reach an `await` point
        if i == 0 {
            let futname = &futures[i].1;
            write!(&mut imp, "
                match self.state {{ \nState::Start => {{
                {step}
                let fut{next} = Box::new({futname});
                self.state = State::Wait{next}(fut{next});
                PollState::NotReady
                }}
            ")?;

        // These steps are await-ponts where we await a future
        } else if i < steps.len() - 1 {
            let varname =  &futures[i-1].0;
            let fut = &futures[i].1;
            write!(&mut imp, "
                State::Wait{i}(ref mut f{i}) => {{
                    match f{i}.poll() {{
                        PollState::Ready({varname}) => {{
                            {step}
                            let fut{next} = Box::new({fut});
                            self.state = State::Wait{next}(fut{next});
                            PollState::NotReady
                        }}
                        PollState::NotReady => PollState::NotReady,
                    }}
                }}
            ")?;

        // This is the part after the last await point. There is no need to yield any more
        } else {
            let varname =  &futures[i-1].0;
            write!(&mut imp, "
                State::Wait{i}(ref mut f{i}) => {{
                    match f{i}.poll() {{
                        PollState::Ready({varname}) => {{
                            {step}
                            self.state = State::Resolved;
                            PollState::Ready(())
                        }}
                        PollState::NotReady => PollState::NotReady,
                    }}
                }}
            ")?;

        }
    }

    // If we poll the future after it has resolved, we panic
    writeln!(&mut imp, "
                State::Resolved => panic!(\"Polled a resolved future\")
            }}
        }}
    }}

    ")?;

    // Format the different parts of the Coroutine implementation to a string
    Ok(format!("{steps_enum}\n{coroutine}\n{imp}"))
}
