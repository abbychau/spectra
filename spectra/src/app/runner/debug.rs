//! The debug runner.

use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance_glfw::surface::{
  Action, GlfwSurface, Key as GlfwKey, Surface, WindowDim, WindowEvent, WindowOpt
};
use std::path::PathBuf;
use structopt::StructOpt;
use warmy::{Store, StoreOpt};

use crate::app::spec::Spec;
use crate::app::runner;
use crate::resource::key::Key;
use crate::time::{DurationSpec, Monotonic};

/// Debug runner.
///
/// This runner shall be used whenever wanted to debug a demo.
pub struct Runner;

#[derive(StructOpt, Debug)]
struct Opt {
  /// Width of the viewport.
  #[structopt(short = "w", long = "width")]
  width: Option<u32>,

  /// Height of the viewport.
  #[structopt(short = "h", long = "height")]
  height: Option<u32>,

  /// Shall the viewport be in fullscreen mode?
  #[structopt(short = "f", long = "fullscreen")]
  fullscreen: bool,

  /// Set a maximum runtime duration. Whenever the time arrives at this duration limit, it will
  /// wrap around to 0. If unset, the demo will run with a forever increasing time.
  ///
  /// The syntax is “MmSs”, where M is optional. M must be a natural specifiying the number of
  /// minutes and S a natural specifying the number of seconds. 30s will then be 30 seconds and 1m42
  /// will be 1 minute and 42 seconds. The number of seconds must not exceed 59.
  #[structopt(short = "z", long = "wrap-at")]
  wrap_at: Option<DurationSpec>,

  /// Start the demo at a given time.
  #[structopt(short = "s", long = "start-at", default_value = "0s")]
  start_at: DurationSpec
}

impl Runner {
  pub fn run<D>(
    title: &str,
    def_width: u32,
    def_height: u32,
    context: &mut D::Context
  ) -> Result<(), runner::Error>
  where D: Spec<Self> {
    info!(context, "starting « {} »", title);

    // get CLI options
    let opt = Opt::from_args();
    let width = opt.width.unwrap_or(def_width);
    let height = opt.height.unwrap_or(def_height);
    let fullscreen = opt.fullscreen;

    // build the WindowDim
    let win_dim = if fullscreen {
      if opt.width.is_some() && opt.height.is_some() {
        info!(context, "window mode: fullscreen restricted ({}×{})", width, height);
        WindowDim::FullscreenRestricted(width, height)
      } else {
        info!(context, "window mode: fullscreen");
        WindowDim::Fullscreen
      }
    } else {
      info!(context, "window mode: windowed ({}×{})", width, height);
      WindowDim::Windowed(width, height)
    };

    let win_opt = WindowOpt::default();

    // create the rendering surface
    let mut surface =
      GlfwSurface::new(win_dim, title, win_opt)
        .map_err(|e| runner::Error::cannot_create_window(format!("{}", e)))?;

    // create the store
    let store_opt = StoreOpt::default().set_root("data");
    let mut store: Store<D::Context, Key> =
      Store::new(store_opt)
        .map_err(|e| runner::Error::cannot_create_store(format!("{}", e)))?;

    // create an instance of our runner to pass to the demo
    let mut runner = Runner;

    // initialize the demo
    let mut demo =
      D::init(&mut runner, &mut store, context)
        .map_err(|e| runner::Error::demo_initialization_failure(format!("{:?}", e)))?;

    // create a bunch of objects needed for rendering
    let mut back_buffer = Framebuffer::back_buffer(surface.size());

    // loop over time and run the demo
    let start_time = Monotonic::now();
    let start_at = opt.start_at;
    let wrap_at = opt.wrap_at;

    info!(context, "initialized; running…");

    'run: loop {
      // treat events first
      for event in surface.poll_events() {
        match event {
          // quit event
          WindowEvent::Close | WindowEvent::Key(GlfwKey::Escape, _, Action::Release, _) => {
            break 'run;
          }

          // resize event
          WindowEvent::FramebufferSize(w, h) => {
            let size = [w as u32, h as u32];

            back_buffer = Framebuffer::back_buffer(size);
            demo.resize(&mut runner, context, size[0], size[1]);
          }

          _ => ()
        }
      }

      // render a frame
      let t = start_time.elapsed_secs();
      let t = if let Some(wrap_t) = wrap_at { t.wrap_around(wrap_t.into()) } else { t };
      let t = t.offset(start_at.into());
      let builder = surface.pipeline_builder();

      demo.render(&mut runner, context, t, &back_buffer, builder);
      surface.swap_buffers();
    }

    Ok(())
  }
}
