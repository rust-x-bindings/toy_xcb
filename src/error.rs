
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Xcb(xcb::Error),
}

impl From<xcb::Error> for Error {
    fn from(err: xcb::Error) -> Error {
        Error::Xcb(err)
    }
}

impl From<xcb::ConnError> for Error {
    fn from(err: xcb::ConnError) -> Error {
        Error::Xcb(err.into())
    }
}

impl From<xcb::ProtocolError> for Error {
    fn from(err: xcb::ProtocolError) -> Error {
        Error::Xcb(err.into())
    }
}
