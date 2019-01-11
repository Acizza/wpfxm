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

    #[fail(display = "{} prefix is already being managed", _0)]
    PrefixAlreadyManaged(String),

    #[fail(display = "no games detected in prefix \"{}\"", _0)]
    NoGamesDetected(String),

    #[fail(display = "{} prefix is not managed by wpfxm", _0)]
    PrefixNotManaged(String),

    #[fail(display = "failed to run [{}]", _1)]
    FailedToRunProcess(#[cause] std::io::Error, String),

    #[fail(display = "path doesn't exist: {}", _0)]
    PathDoesntExist(String),

    #[fail(display = "unable to find hook {}", _0)]
    HookNotFound(String),

    #[fail(display = "game path not set for prefix; try adding the prefix first")]
    GamePathNotSet,

    #[fail(display = "game already added to prefix")]
    GameAlreadyAdded,

    #[fail(display = "{} is not a valid Windows version; valid options mimic that of winetricks (ex. winxp, win7, win10)", _0)]
    InvalidWindowsVersion(String),
}

impl_err_conv!(Error,
    ConfigError => Config,
    PrefixError => Prefix,
    InputError => Input,
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

    #[fail(display = "failed to set prefix Windows version")]
    FailedToSetWindowsVersion,
}

#[derive(Fail, Debug)]
pub enum InputError {
    #[fail(display = "failed to read line")]
    ReadFailed(#[cause] ::std::io::Error),

    #[fail(display = "failed to parse type: {}", _0)]
    ParseFailed(String),

    #[fail(display = "no list items were provided")]
    NoItemsProvided,
}
