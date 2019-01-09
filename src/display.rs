use colored::Colorize;
use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum ErrorSeverity {
    Warning,
    Fatal,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &str = (*self).into();
        write!(f, "{}", s)
    }
}

impl<'a> Into<&'a str> for ErrorSeverity {
    fn into(self) -> &'a str {
        match self {
            ErrorSeverity::Warning => "warning",
            ErrorSeverity::Fatal => "error",
        }
    }
}

pub fn error<E>(severity: ErrorSeverity, err: E)
where
    E: Into<failure::Error>,
{
    let err = err.into();

    let severity_str = {
        let s: &str = severity.into();

        match severity {
            ErrorSeverity::Warning => s.yellow(),
            ErrorSeverity::Fatal => s.red(),
        }
    };

    eprintln!("{}: {}", severity_str, err);

    for cause in err.iter_chain().skip(1) {
        eprintln!("  cause: {}", cause);
    }

    let backtrace = err.backtrace().to_string();

    if !backtrace.is_empty() {
        eprintln!("{}", backtrace);
    }
}

pub fn info<S>(msg: S)
where
    S: AsRef<str>,
{
    println!("{}: {}", "info".cyan(), msg.as_ref());
}

pub fn input<S>(msg: S)
where
    S: AsRef<str>,
{
    println!("{}: {}", "input".magenta(), msg.as_ref());
}

pub fn hook<S>(msg: S)
where
    S: AsRef<str>,
{
    println!("{}: {}", "hook".green(), msg.as_ref());
}
