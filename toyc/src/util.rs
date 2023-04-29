
pub fn miniwarn(msg: &str) {
    eprintln!("\x1b[93m\x1b[1mwarning\x1b[0m\x1b[1m: {}\x1b[0m", msg);
}

pub fn minierr(msg: &str) {
    eprintln!("\x1b[91m\x1b[1merror\x1b[0m\x1b[1m: {}\x1b[0m", msg);
}

pub struct WarnArgs {
    pub line: String,
    pub line_number: i32,
    pub range: Option<(usize, usize)>,
    pub help: Option<String>,
    pub file_name: String,
}

pub fn warn(msg: &str, args: WarnArgs) {
    let nlen = args.line_number.to_string().len();
    miniwarn(msg);
    eprint!("\x1b[m");
    eprintln!(
        "\x1b[96m{}>\x1b[0m\x1b[1m {}:{}\x1b[0m",
        strcol("-".to_string(), nlen + 1),
        args.file_name,
        args.line_number
    );
    eprintln!("\x1b[96m{} |\x1b[0m", strcol(" ".to_string(), nlen));
    eprintln!("\x1b[96m{} |\x1b[0m {}", args.line_number, args.line);
    match args.range {
        None => {}
        Some((s, f)) => {
            eprintln!(
                "\x1b[96m{} |\x1b[0m\x1b[93m{}{}\x1b[0m",
                strcol(" ".to_string(), nlen),
                strcol(" ".to_string(), s),
                strcol("^".to_string(), (f + 1) - s)
            )
        }
    }
    eprintln!("\x1b[96m{} |\x1b[0m", strcol(" ".to_string(), nlen));
    eprint!("\x1b[96m{}\x1b[0m ", strcol("-".to_string(), nlen + 2));
    match args.help {
        None => {
            eprintln!()
        }
        Some(msg) => {
            eprintln!("\x1b[94m\x1b[1mHelp\x1b[0m\x1b[1m: {}\x1b[0m", msg)
        }
    }
    eprintln!();
}

pub fn err(msg: &str, args: WarnArgs) {
    let nlen = args.line_number.to_string().len();
    minierr(msg);
    eprint!("\x1b[m");
    eprintln!(
        "\x1b[95m{}>\x1b[0m\x1b[1m {}:{}\x1b[0m",
        strcol("-".to_string(), nlen + 1),
        args.file_name,
        args.line_number
    );
    eprintln!("\x1b[95m{} |\x1b[0m", strcol(" ".to_string(), nlen));
    eprintln!("\x1b[95m{} |\x1b[0m {}", args.line_number, args.line);
    match args.range {
        None => {}
        Some((s, f)) => {
            eprintln!(
                "\x1b[95m{} |\x1b[0m\x1b[91m{}{}\x1b[0m",
                strcol(" ".to_string(), nlen),
                strcol(" ".to_string(), s),
                strcol("^".to_string(), (f + 1) - s)
            )
        }
    }
    eprintln!("\x1b[95m{} |\x1b[0m", strcol(" ".to_string(), nlen));
    eprint!("\x1b[95m{}\x1b[0m ", strcol("-".to_string(), nlen + 2));
    match args.help {
        None => {
            eprintln!()
        }
        Some(msg) => {
            eprintln!("\x1b[94m\x1b[1mHelp\x1b[0m\x1b[1m: {}\x1b[0m", msg)
        }
    }
    eprintln!();
}

pub fn strcol(j: String, n: usize) -> String {
    let mut r = "".to_string();
    for _ in 0..n {
        r += &j;
    }
    return r;
}