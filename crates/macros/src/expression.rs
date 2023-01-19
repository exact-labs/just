#[macro_export]
macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {
            $v
        } else {
            $v1
        }
    };
}

#[macro_export]
macro_rules! attempt {
   (@recurse ($a:expr) { } catch ($e:ident) $b:block) => {
      if let Err ($e) = $a $b
   };
   (@recurse ($a:expr) { $e:expr; $($tail:tt)* } $($handler:tt)*) => {
      attempt!{@recurse ($a.and_then (|_| $e)) { $($tail)* } $($handler)*}
   };
   ({ $e:expr; $($tail:tt)* } $($handler:tt)*) => {
      attempt!{@recurse ($e) { $($tail)* } $($handler)* }
   };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        use std::io::Write;
        use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("Unable to write to stderr (file handle closed?)");
        writeln!(&mut stderr, $($arg)*).expect("Unable to write to stderr (file handle closed?)");
    )
}

#[macro_export]
macro_rules! crash {
    ($($arg:tt)*) => (
        use std::io::Write;
        use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("Unable to write to stderr (file handle closed?)");
        writeln!(&mut stderr, $($arg)*).expect("Unable to write to stderr (file handle closed?)");
        std::process::exit(1);
    )
}
