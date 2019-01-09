use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum ErrorSeverity {
    Warning,
    Fatal,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorSeverity::Warning => write!(f, "warning"),
            ErrorSeverity::Fatal => write!(f, "error"),
        }
    }
}

pub fn error<E>(severity: ErrorSeverity, err: E)
where
    E: Into<failure::Error>,
{
    let err = err.into();

    eprintln!("{}: {}", severity, err);

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
    println!("info: {}", msg.as_ref());
}

pub fn input<S>(msg: S)
where
    S: AsRef<str>,
{
    println!("input: {}", msg.as_ref());
}

pub fn hook<S>(msg: S)
where
    S: AsRef<str>,
{
    println!("hook: {}", msg.as_ref());
}
