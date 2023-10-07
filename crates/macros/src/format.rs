#[macro_export]
macro_rules! string {
    () => {
        String::new()
    };
    ($x:expr $(,)?) => {
        ToString::to_string(&$x)
    };
}

#[macro_export]
macro_rules! str {
    ($x: expr) => {{
        let out: &'static str = { Box::leak($x.into_boxed_str()) };
        out
    }};
}

#[macro_export]
macro_rules! fmtstr {
    ($($t:tt)*) => {{
        let out: &'static str = { Box::leak(format!($($t)*).into_boxed_str()) };
        out
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use std::io::Write;
        use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("Unable to write to stderr (file handle closed?)");
        write!(&mut stderr, $($arg)*).expect("Unable to write to stderr (file handle closed?)");
    }};
}

#[macro_export]
macro_rules! errorln {
    ($($arg:tt)*) => {{
        use std::io::Write;
        use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("Unable to write to stderr (file handle closed?)");
        writeln!(&mut stderr, $($arg)*).expect("Unable to write to stderr (file handle closed?)");
    }};
}

#[macro_export]
macro_rules! crash {
    ($($arg:tt)*) => {{
        use std::io::Write;
        use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("Unable to write to stderr (file handle closed?)");
        write!(&mut stderr, $($arg)*).expect("Unable to write to stderr (file handle closed?)");
        std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! crashln {
    ($($arg:tt)*) => {{
        use std::io::Write;
        use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("Unable to write to stderr (file handle closed?)");
        writeln!(&mut stderr, $($arg)*).expect("Unable to write to stderr (file handle closed?)");
        std::process::exit(1);
    }};
}
