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
