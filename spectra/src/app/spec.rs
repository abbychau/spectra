//! Quickly create applications.

use luminance::framebuffer::Framebuffer;
pub use luminance::pipeline::Builder;
use luminance::texture::{Dim2, Flat};
pub use warmy::Store;
use std::fmt::Debug;

use crate::logger::Logger;
pub use crate::resource::key::Key;
pub use crate::time::Time;

/// Class of applications’ specifics.
///
/// A “spec” is basically a set of operations that can be done around and inside an event loop.
/// Event loops are provided in another module.
pub trait Spec<Runner>: Sized {
  /// Context carried around.
  type Context: Logger;

  /// Initialization error that might occur.
  type Error: Sized + Debug;

  /// Initialize the application with a given store.
  ///
  /// The runner is passed so that specific initialization is possible.
  fn init(
    runner: &mut Runner,
    store: &mut Store<Self::Context, Key>,
    context: &mut Self::Context
  ) -> Result<Self, Self::Error>;

  /// Called when the viewport gets resized.
  ///
  /// The runner is passed so that specific resizing is possible.
  fn resize(&mut self, runner: &mut Runner, context: &mut Self::Context, width: u32, height: u32);

  /// Render a single frame at a given time.
  ///
  /// The runner is passed so that specific rendering is possible.
  fn render(
    &mut self,
    runner: &mut Runner,
    context: &mut Self::Context,
    t: Time,
    back_buffer: &Backbuffer,
    builder: Builder
  );
}

pub type Backbuffer = Framebuffer<Flat, Dim2, (), ()>;
