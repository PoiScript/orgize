use app_dirs::AppDirsError;
use dotenv::Error as EnvError;
use isahc::http::Error as HttpError;
use isahc::Error as IsahcError;
use std::convert::From;
use std::io::Error as IOError;
use toml::de::Error as TomlDeError;
use toml::ser::Error as TomlSerError;
use url::ParseError;

#[derive(Debug)]
pub enum Error {
    AppDirs(AppDirsError),
    Env(EnvError),
    Http(IsahcError),
    IO(IOError),
    TomlDe(TomlDeError),
    TomlSer(TomlSerError),
    Url(ParseError),
}

impl From<AppDirsError> for Error {
    fn from(err: AppDirsError) -> Self {
        Error::AppDirs(err)
    }
}

impl From<EnvError> for Error {
    fn from(err: EnvError) -> Self {
        Error::Env(err)
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        Error::IO(err)
    }
}

impl From<IsahcError> for Error {
    fn from(err: IsahcError) -> Self {
        Error::Http(err)
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Self {
        Error::Http(err.into())
    }
}

impl From<TomlDeError> for Error {
    fn from(err: TomlDeError) -> Self {
        Error::TomlDe(err)
    }
}

impl From<TomlSerError> for Error {
    fn from(err: TomlSerError) -> Self {
        Error::TomlSer(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::Url(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
