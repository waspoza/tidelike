use std::env::home_dir;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{MAIN_SEPARATOR, Path, PathBuf};

fn shorten(path: &Path, max_len: usize, home_symbol: &OsStr) -> String {
    let mut out = String::new();
    let mut component_index = 1; // current spot in path (skip root path '/')

    // check if path is in home dir and can be stortened with ~
    if let Some(home) = home_dir()
        && path.starts_with(&home)
    {
        out.push_str(home_symbol.to_str().unwrap());
        component_index = home.components().count();
    }
    // shorten path component when resulting path is longer than max_len
    while out.len() + path_len(path, component_index) > max_len {
        if component_index == path.components().count() - 1 {
            // never short the last component. even if its makes the path longer than max_len.
            break;
        }
        out.push(MAIN_SEPARATOR);
        out.push_str(get_unique_dir(path, component_index));
        component_index += 1;
    }
    //
    // push the rest of the path to the output
    path.iter().skip(component_index).for_each(|comp| {
        out.push(MAIN_SEPARATOR);
        out.push_str(comp.to_str().unwrap());
    });
    //dbg!(&out, out.len());
    out
}

// get shortest unique dir name
// for example in given dir '/home/piotr/Games/battlenet/drive_c/Program Files/Common Files'
// and component_index 4, funcction fill scan directory '/home/piotr/Games'.
// if this dir contains 'battle', 'battlenet', 'battlenet1984', function will return 'battlenet'.
fn get_unique_dir(path: &Path, component_index: usize) -> &str {
    let dir: PathBuf = path.components().take(component_index).collect();
    //dbg!(component_index, &dir);

    let dir_name = path
        .components()
        .nth(component_index)
        .and_then(|comp| comp.as_os_str().to_str())
        .unwrap();
    let first_char = dir_name.chars().next().unwrap();

    let mut dirs = Vec::new(); // list of dirs starting with the same letter
    for entry in fs::read_dir(&dir).unwrap_or_else(|_| panic!("cannot read {:?}", &dir)) {
        let Ok(entry) = entry else {
            continue;
        };
        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            let name = entry.file_name();
            let name_str = name.to_str().unwrap();
            if name_str == dir_name {
                // ignore current directory
                continue;
            }
            if name_str.starts_with(first_char) {
                dirs.push(name_str.to_string());
            }
        }
    }
    //dbg!(&dirs);
    let mut smallest_unique_prefix = dirs
        .iter()
        .map(|s| {
            s.chars()
                .zip(dir_name.chars())
                .take_while(|(a, b)| a == b)
                .count()
        })
        .max()
        .unwrap_or(0);

    if smallest_unique_prefix < dir_name.len() {
        smallest_unique_prefix += 1;
    }
    //dbg!(&dir_name, smallest_unique_prefix);
    &dir_name[0..smallest_unique_prefix]
}

fn path_len(path: &Path, component_index: usize) -> usize {
    path.components()
        .skip(component_index)
        .map(|comp| comp.as_os_str().len() + 1)
        .sum()
}

fn main() {
    let program_name = std::env::args().next().unwrap();
    let program_name = Path::new(&program_name)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let usage = format!(
        "Usage: {} [options]

    Options:

    -p    path to display. (default: current directory)
    -s    home symbol. (default: '~')
    -l    max path length. (default: 40)
    -h    this help screen.
    ",
        program_name
    );
    let mut home_symbol: OsString = "~".into();
    let mut path = std::env::current_dir().unwrap();
    let mut max_len = 40;

    let mut args = std::env::args_os().skip(1);
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("-s") => home_symbol = args.next().unwrap_or(home_symbol),

            Some("-p") => path = args.next().map(PathBuf::from).unwrap_or(path),

            Some("-l") => {
                max_len = args.next().map_or(max_len, |arg| {
                    let input = arg.to_string_lossy();
                    input.parse().unwrap_or_else(|_| {
                        eprintln!("option -l requires valid number, got: '{}'", input);
                        std::process::exit(1);
                    })
                });
            }

            Some("-h" | "--help") => {
                eprintln!("{}", usage);
                return;
            }

            _ => {
                eprintln!("unknown argument: {}", arg.to_str().unwrap());
                std::process::exit(1);
            }
        }
    }

    println!("{}", shorten(&path, max_len, &home_symbol));
}
