use std::{fs, env, error::Error, path::{Path, PathBuf}};

use corofy::rewrite;


fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();

    let src = match args.get(1) {
        Some(path) => Path::new(path),
        None => {
            println!("Missing source file. Please provide a path to a source file and try again.");
            return Ok(());
        },
    };

    let dest = match args.get(2) {
        Some(path) => PathBuf::from(path),
        None => {
            let src_n = src.file_stem().map(|x|x.to_string_lossy()).unwrap_or_default();
            let src_ext = src.extension().map(|x|x.to_string_lossy()).unwrap_or_default();
            let clone = format!("{src_n}_corofied.{src_ext}");

            match src.parent() {
                Some(path) => path.join(&clone).clone(),
                None => PathBuf::from("./").join(&clone),
            }
        },
    };

    let src = fs::read_to_string(src)?;
    // Will truncate if exists
    let dest = fs::File::create(dest)?;

    if let Err(e) = rewrite(src, dest) {
        println!("{e}");
    }
    Ok(())
}

