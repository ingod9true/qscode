// //! Error and Result module.
// use std::error::Error as StdError;
// use std::fmt;
//
// /// Result type often returned from methods that can have hyper `Error`s.
// pub type Result<T> = std::result::Result<T, Error>;
//
// type Cause = Box<dyn StdError + Send + Sync>;
//
// /// Represents errors that can occur handling HTTP streams.
// pub struct Error {
//     inner: Box<ErrorImpl>,
// }
//
// struct ErrorImpl {
//     kind: Kind,
//     cause: Option<Cause>,
// }
//
// #[derive(Debug, PartialEq)]
// pub(super) enum Kind {
//     Parse(Parse),
//     User(User),
//     IncompleteMessage,
//     UnexpectedMessage,
//     Canceled,
//     ChannelClosed,
//     Connect,
//     Listen,
//     Accept,
// }
//
// #[derive(Debug, PartialEq)]
// pub(super) enum Parse {
//     Method,
//     Version,
//     VersionH2,
//     Uri,
//     Header,
//     TooLarge,
//     Status,
// }
//
// #[derive(Debug, PartialEq)]
// pub(super) enum User {
//     Body,
//     MakeService,
//     Service,
//     UnexpectedHeader,
//     UnsupportedVersion,
//     NoUpgrade,
//     AbortedByCallback,
// }
//
// // Sentinel type to indicate the error was caused by a timeout.
// #[derive(Debug)]
// pub(super) struct TimedOut;
//
// impl Error {
//
//     /// Returns true if this was about a `Request` that was canceled.
//     pub fn is_canceled(&self) -> bool {
//         self.inner.kind == Kind::Canceled
//     }
//
//
//
//     /// Returns true if the error was caused by a timeout.
//     pub fn is_timeout(&self) -> bool {
//         self.find_source::<TimedOut>().is_some()
//     }
//
//     /// Consumes the error, returning its cause.
//     pub fn into_cause(self) -> Option<Box<dyn StdError + Send + Sync>> {
//         self.inner.cause
//     }
//
//     pub(super) fn new(kind: Kind) -> Error {
//         Error {
//             inner: Box::new(ErrorImpl { kind, cause: None }),
//         }
//     }
//
//     pub(super) fn new_canceled() -> Error {
//         Error::new(Kind::Canceled)
//     }
//
//     #[cfg(feature = "http1")]
//     pub(super) fn new_incomplete() -> Error {
//         Error::new(Kind::IncompleteMessage)
//     }
//
//     #[cfg(feature = "http1")]
//     pub(super) fn new_too_large() -> Error {
//         Error::new(Kind::Parse(Parse::TooLarge))
//     }
//
//     #[cfg(feature = "http1")]
//     pub(super) fn new_version_h2() -> Error {
//         Error::new(Kind::Parse(Parse::VersionH2))
//     }
//
//     #[cfg(feature = "http1")]
//     pub(super) fn new_unexpected_message() -> Error {
//         Error::new(Kind::UnexpectedMessage)
//     }
//
//     #[cfg(any(feature = "http1", feature = "http2"))]
//     pub(super) fn new_io(cause: std::io::Error) -> Error {
//         Error::new(Kind::Io).with(cause)
//     }
//
//
//     pub(super) fn new_closed() -> Error {
//         Error::new(Kind::ChannelClosed)
//     }
//
//     #[cfg(any(feature = "http1", feature = "http2", feature = "stream"))]
//     pub(super) fn new_body<E: Into<Cause>>(cause: E) -> Error {
//         Error::new(Kind::Body).with(cause)
//     }
//
//     #[cfg(any(feature = "http1", feature = "http2"))]
//     pub(super) fn new_body_write<E: Into<Cause>>(cause: E) -> Error {
//         Error::new(Kind::BodyWrite).with(cause)
//     }
//
//     pub(super) fn new_body_write_aborted() -> Error {
//         Error::new(Kind::BodyWriteAborted)
//     }
//
//     fn new_user(user: User) -> Error {
//         Error::new(Kind::User(user))
//     }
//
//
//
//     pub(super) fn new_user_body<E: Into<Cause>>(cause: E) -> Error {
//         Error::new_user(User::Body).with(cause)
//     }
//
//     pub(super) fn new_shutdown(cause: std::io::Error) -> Error {
//         Error::new(Kind::Shutdown).with(cause)
//     }
//
//
//     pub(super) fn new_user_aborted_by_callback() -> Error {
//         Error::new_user(User::AbortedByCallback)
//     }
//
//     fn description(&self) -> &str {
//         match self.inner.kind {
//             Kind::Parse(Parse::Method) => "invalid HTTP method parsed",
//             Kind::Accept => "error accepting connection",
//             Kind::User(User::Body) => "error from user's HttpBody stream",
//             Kind::IncompleteMessage => "connection closed before message completed",
//            _ => "connection closed before message completed",
//         }
//     }
// }
//
// impl fmt::Debug for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut f = f.debug_tuple("hyper::Error");
//         f.field(&self.inner.kind);
//         if let Some(ref cause) = self.inner.cause {
//             f.field(cause);
//         }
//         f.finish()
//     }
// }
//
// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if let Some(ref cause) = self.inner.cause {
//             write!(f, "{}: {}", self.description(), cause)
//         } else {
//             f.write_str(self.description())
//         }
//     }
// }
//
// impl StdError for Error {
//     fn source(&self) -> Option<&(dyn StdError + 'static)> {
//         self.inner
//             .cause
//             .as_ref()
//             .map(|cause| &**cause as &(dyn StdError + 'static))
//     }
// }
//
