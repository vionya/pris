//! Provides all of the methods needed to control and
//! work with a `Player`.
//! 
//! Note that these methods are also all implemented
//! on the `Player` struct.
mod methods_complex;
mod methods_simple;

pub use methods_complex::*;
pub use methods_simple::*;

const INTERFACE: &str = "org.mpris.MediaPlayer2.Player";
