//! Quickly create demoscene applications.

use luminance::framebuffer::Framebuffer;
pub use luminance::pipeline::Builder;
use luminance::texture::{Dim2, Flat};
pub use warmy::Store;
use std::fmt::Debug;

use crate::resource::key::Key;
use crate::time::Time;

/// Class of demo applications.
///
/// A demo is basically just a single function that takes the current time and display something. If
/// you hit escape or close the window, the application must quit.
pub trait Demo: Sized {
  /// Context used to initialize the demo.
  type Context;

  /// Initialization error that might occur.
  type Error: Sized + Debug;

  /// Initialize the demo with a given store.
  fn init(store: &mut Store<Self::Context, Key>) -> Result<Self, Self::Error>;

  /// Resize the demo when the framebuffer gets resized.
  fn resize(&mut self, width: u32, height: u32);

  /// Render the demo at a given time. 
  fn render(&mut self, t: Time, back_buffer: &Backbuffer, builder: Builder);

}

pub type Backbuffer = Framebuffer<Flat, Dim2, (), ()>;
