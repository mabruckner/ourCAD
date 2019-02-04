use kiss3d::line_renderer::LineRenderer;
use kiss3d::window::Window;
use nalgebra::Point;
use solid::*;
use std::sync::mpsc;
use std::thread;

enum Command {
  Set(Vec<Edge>),
  Quit,
}

pub fn display(solid: Solid) -> () {
  quick_display(solid.faces);
}

pub fn quick_display(faces: Vec<Face>) -> () {
  let mut disp = KissDisplay::new();
  let mut out = Vec::new();
  for face in faces {
    out.extend(face.edges());
  }
  disp.set(out);
  disp.join();
}

pub fn edge_display(edges: Vec<Edge>) -> () {
  let mut disp = KissDisplay::new();
  disp.set(edges);
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
      let mut target = Vec::new();
      while window.render() {
        while let Ok(val) = receiver.try_recv() {
          match val {
            Command::Set(solid) => target = solid,
            _ => (),
          }
        }

        for edge in &target {
          let (mut a, mut b) = ([0.0; 3], [0.0; 3]);
          for i in 0..3 {
            a[i] = edge.a.pos.c[i] as f32;
            b[i] = edge.b.pos.c[i] as f32;
          }
          window.draw_line(&a.into(), &b.into(), &[1.0; 3].into());
        }
      }
      window.close();
    });
    KissDisplay {
      sender: sender,
      handle: handle,
    }
  }
  pub fn set(&mut self, e: Vec<Edge>) -> () {
    self.sender.send(Command::Set(e));
  }
  pub fn join(self) -> () {
    self.handle.join().unwrap();
  }
}
