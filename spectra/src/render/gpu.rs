//! The GPU, giving access to renderer interfacing.
//!
//! The [`GPU`] is used to interface and implement renderers.

use luminance::context::GraphicsContext;
use luminance::framebuffer::{Framebuffer, FramebufferError};

use crate::render::target::{Target, TargetProperty};

struct GPU<T> where T: GraphicsContext {
  context: T
}

impl<T> GPU<T> where T: GraphicsContext {
  pub fn new_target<P>(
    &mut self,
    width: u32,
    height: u32,
    mipmaps: usize
  ) -> Result<Target<P>, GPUError>
  where P: TargetProperty {
    Framebuffer::new(&mut self.context, [width, height], mipmaps)?
      .map(Target::new)
  }
}

#[derive(Debug)]
pub enum GPUError {
  TargetError(TargetError)
}

#[derive(Debug)]
pub enum TargetError {
  TextureError(String),
  Incomplete(String)
}

impl From<FramebufferError> for TargetError {
  fn from(fbe: FramebufferError) -> Self {
    match fbe {
      FramebufferError::TextureError(e) => TargetError::TextureError(format!("{}", e)),
      FramebufferError::Incomplete(e) => TargetError::Incomplete(format!("{}", e)),
    }
  }
}

impl From<TargetError> for GPUError {
  fn from(te: TargetError) -> Self {
    GPUError::TargetError(te)
  }
}

impl From<FramebufferError> for GPUError {
  fn from(fbe: FramebufferError) -> Self {
    let te: TargetError = fbe.into();
    te.into()
  }
}
