use kiss3d::line_renderer::LineRenderer;
use kiss3d::window::Window;
use nalgebra::Point;
use solid::*;
use std::sync::mpsc;
use std::thread;

enum Command {
  Set(Solid),
  Quit,
}

pub fn display(solid: Solid) -> () {
  let mut disp = KissDisplay::new();
  disp.set(solid);
  disp.join();
}

pub fn quick_display(faces: Vec<Face>) -> () {
  let s = Solid { faces: faces };
  let mut disp = KissDisplay::new();
  disp.set(s);
  disp.join();
}

pub struct KissDisplay {
  handle: thread::JoinHandle<()>,
  sender: mpsc::Sender<Command>,
}

impl KissDisplay {
  pub fn new() -> Self {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
      let mut window = Window::new("kiss3d display");
      window.set_background_color(0.0, 0.0, 0.0);
      let mut target = Solid { faces: Vec::new() };
      while window.render() {
        while let Ok(val) = receiver.try_recv() {
          match val {
            Command::Set(solid) => target = solid,
            _ => (),
          }
        }

        for face in &target.faces {
          for edge in face.edges() {
            let (mut a, mut b) = ([0.0; 3], [0.0; 3]);
            for i in 0..3 {
              a[i] = edge.a.pos.c[i] as f32;
              b[i] = edge.b.pos.c[i] as f32;
            }
            window.draw_line(&a.into(), &b.into(), &[1.0; 3].into());
          }
        }
      }
      window.close();
    });
    KissDisplay {
      sender: sender,
      handle: handle,
    }
  }
  pub fn set(&mut self, s: Solid) -> () {
    self.sender.send(Command::Set(s));
  }
  pub fn join(self) -> () {
    self.handle.join().unwrap();
  }
}
