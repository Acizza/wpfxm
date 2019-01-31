use failure::Fail;

macro_rules! impl_err_conv {
    ($struct:ident, $($from:ty => $to:ident,)+) => {
        $(
        impl From<$from> for $struct {
            fn from(f: $from) -> $struct {
                $struct::$to(f)
            }
        }
        )+
    };
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "config error")]
    Config(#[cause] ConfigError),

    #[fail(display = "prefix error")]
    Prefix(#[cause] PrefixError),

    #[fail(display = "input error")]
    Input(#[cause] InputError),

    #[fail(display = "command error")]
    Command(#[cause] CommandError),
}

impl_err_conv!(Error,
    ConfigError => Config,
    PrefixError => Prefix,
    InputError => Input,
    CommandError => Command,
);

#[derive(Fail, Debug)]
pub enum ConfigError {
    #[fail(display = "failed to get config directory")]
    FailedToGetConfigDir,

    #[fail(display = "failed to read config file")]
    FailedToRead(#[cause] std::io::Error),

    #[fail(display = "failed to parse config file")]
    FailedToParse(#[cause] toml::de::Error),

    #[fail(display = "failed to serialize config")]
    FailedToSerialize(#[cause] toml::ser::Error),

    #[fail(display = "failed to write config to {}", _1)]
    FailedToWrite(#[cause] std::io::Error, String),
}

#[derive(Fail, Debug)]
pub enum PrefixError {
    #[fail(display = "failed to get prefix data directory")]
    FailedToGetDataDir,

    #[fail(display = "failed to read prefix data directory")]
    FailedToReadDataDir(#[cause] std::io::Error),

    #[fail(display = "failed to get hooks directory")]
    FailedToGetHooksDir,

    #[fail(display = "failed to read path {}", _1)]
    FailedToReadPath(#[cause] std::io::Error, String),

    #[fail(display = "failed to read prefix configuration")]
    FailedToReadConfig(#[cause] std::io::Error),

    #[fail(display = "failed to parse prefix configuration")]
    FailedToParseConfig(#[cause] toml::de::Error),

    #[fail(display = "failed to serialize prefix configuration")]
    FailedToSerializeConfig(#[cause] toml::ser::Error),

    #[fail(display = "failed to write prefix config to file")]
    FailedToWriteConfig(#[cause] std::io::Error),

    #[fail(display = "failed to detect prefix architecture")]
    FailedToDetectArch,

    #[fail(display = "hook failed to execute")]
    FailedToRunHook,

    #[fail(display = "unable to find hook {}", _0)]
    HookNotFound(String),

    #[fail(display = "failed to create Wine prefix")]
    FailedToCreatePrefix(#[cause] std::io::Error),

    #[fail(display = "Wine failed to run successfully")]
    WineFailedToExecute,
}

#[derive(Fail, Debug)]
pub enum InputError {
    #[fail(display = "io error")]
    Io(#[cause] std::io::Error),

    #[fail(display = "failed to read line")]
    ReadFailed(#[cause] ::std::io::Error),

    #[fail(display = "failed to parse type: {}", _0)]
    ParseFailed(String),

    #[fail(display = "no list items were provided")]
    NoItemsProvided,
}

impl_err_conv!(InputError,
    std::io::Error => Io,
);

#[derive(Fail, Debug)]
pub enum CommandError {
    #[fail(display = "config error")]
    Config(#[cause] ConfigError),

    #[fail(display = "prefix error")]
    Prefix(#[cause] PrefixError),

    #[fail(display = "input error")]
    Input(#[cause] InputError),

    #[fail(display = "failed to run [{}]", _1)]
    FailedToRunProcess(#[cause] std::io::Error, String),

    #[fail(display = "{} prefix is already being managed", _0)]
    PrefixAlreadyManaged(String),

    #[fail(display = "{} prefix is not managed by wpfxm", _0)]
    PrefixNotManaged(String),

    #[fail(display = "no executables detected in prefix {}", _0)]
    NoExecsDetected(String),

    #[fail(display = "no executables are being managed by wpfxm for this prefix; try using the add command first")]
    NoSavedExecs,

    #[fail(display = "multiple managed executables found, please specify which one to launch with -n")]
    NameNeededToRunExec,

    #[fail(display = "{} is not being managed by wpfxm, please use the add command first to add it", _0)]
    ExecNotManaged(String),

    #[fail(display = "prefix does not exist")]
    PrefixDoesNotExist,

    #[fail(display = "prefix data does not exist")]
    PrefixDataDoesNotExist,

    #[fail(display = "failed to remove {}", _1)]
    FailedToRemovePath(#[cause] std::io::Error, &'static str),

    #[fail(display = "path doesn't exist: {}", _0)]
    PathDoesntExist(String),

    #[fail(display = "unable to find hook {}", _0)]
    HookNotFound(String),
}

impl_err_conv!(CommandError,
    ConfigError => Config,
    PrefixError => Prefix,
    InputError => Input,
);
