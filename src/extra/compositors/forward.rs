use luminance::{Dim2, Flat, Mode, RGBA32F, Unit};
use luminance_gl::gl33::{Framebuffer, Pipe, Pipeline, RenderCommand, ShadingCommand, Tessellation,
                         Texture, Uniform};

use id::Id;
use scene::Scene;
use shader::Program;

pub type Texture2D<A> = Texture<Flat, Dim2, A>;

const FORWARD_SOURCE: Uniform<Unit> = Uniform::new(0);

pub struct Forward<'a> {
  program: Id<'a, Program>,
  quad: Tessellation,
  w: u32,
  h: u32
}

impl<'a> Forward<'a> {
  pub fn new_from(w: u32, h: u32, scene: &mut Scene<'a>) -> Self {
    let program = get_id!(scene, "spectra/compositors/forward.glsl", vec![Uniform::<Unit>::sem("source")]).unwrap();

    // update the texture uniform once and for all
    {
      let program: &Program = &scene.get_by_id(&program).unwrap();
      program.update(&FORWARD_SOURCE, Unit::new(0));
    }

    Forward {
      program: program,
      quad: Tessellation::attributeless(Mode::TriangleStrip, 4),
      w: w,
      h: h
    }
  }

  pub fn composite(&mut self, scene: &mut Scene<'a>, source: &Texture2D<RGBA32F>) {
    let program = scene.get_by_id(&self.program).unwrap();
    let back_fb = Framebuffer::default((self.w, self.h));
    let textures = &[source.into()];

    Pipeline::new(&back_fb, [0., 0., 0., 0.], textures, &[], vec![
      Pipe::new(|_| {}, ShadingCommand::new(&program, vec![
        Pipe::new(|_| {}, RenderCommand::new(None, true, vec![
          Pipe::new(|_|{}, &self.quad)], 1, None))
        ]))
    ]).run();
  }
}