use app_dirs::AppDirsError;
use isahc::http::Error as HttpError;
use isahc::Error as IsahcError;
use std::convert::From;
use std::io::Error as IOError;
use url::ParseError;

#[derive(Debug)]
pub enum Error {
    AppDirs(AppDirsError),
    IO(IOError),
    Http(IsahcError),
    Url(ParseError),
}

impl From<AppDirsError> for Error {
    fn from(err: AppDirsError) -> Self {
        Error::AppDirs(err)
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

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::Url(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
