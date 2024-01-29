// //! Describes the parsing for procedures
// //!
// //! This file gets deserialized to our [`Components`] datastructure using [`serde`].

mod install_path;
pub mod procedures;
pub use install_path::InstallPath;
pub use procedures::Procedure;
