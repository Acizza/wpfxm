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

    #[fail(display = "{} prefix is already being managed", _0)]
    PrefixAlreadyManaged(String),

    #[fail(display = "no games detected in prefix {}", _0)]
    NoGamesDetected(String),
}

impl_err_conv!(Error,
    ConfigError => Config,
    PrefixError => Prefix,
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

    #[fail(display = "failed to create config dir")]
    FailedToCreateConfigDir(#[cause] std::io::Error),
}

#[derive(Fail, Debug)]
pub enum PrefixError {
    #[fail(display = "failed to get local data directory")]
    FailedToGetDataDir,

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
}
