#[allow(dead_code)]
pub fn term_rprint(color: term::color::Color, status_text: &str, text: &str) {
    let mut t = term::stdout().unwrap();

    t.carriage_return().unwrap();
    t.delete_line().unwrap();
    t.attr(term::Attr::Bold).unwrap();
    t.fg(color).unwrap();
    write!(t, "{} ", status_text).unwrap();
    let _ = t.reset();
    write!(t, "{}", text).unwrap();
    t.flush().unwrap();
}

#[allow(dead_code)]
pub fn term_rprint_finish() {
    let mut t = term::stdout().unwrap();

    writeln!(t).unwrap();
    t.flush().unwrap();
}

#[allow(dead_code)]
pub fn term_print(color: term::color::Color, status_text: &str, text: &str) {
    term_print_(color, status_text, text, false);
}

pub fn term_println(color: term::color::Color, status_text: &str, text: &str) {
    term_print_(color, status_text, text, true);
}

fn term_print_(color: term::color::Color, status_text: &str, text: &str, newline: bool) {
    let mut t = term::stdout().unwrap();

    t.attr(term::Attr::Bold).unwrap();
    t.fg(color).unwrap();
    write!(t, "{} ", status_text).unwrap();
    let _ = t.reset();

    if newline {
        writeln!(t, "{}", text).unwrap();
    } else {
        write!(t, "{}", text).unwrap();
        t.flush().unwrap();
    }
}

#[cfg(not(debug_assertions))]
pub fn term_panic(text: &str) {
    term_println(term::color::BRIGHT_RED, "Failure:", text);
}
