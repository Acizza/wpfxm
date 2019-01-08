use failure::Fail;

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

    #[fail(display = "failed to write config to file")]
    FailedToWrite(#[cause] std::io::Error),

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
}
