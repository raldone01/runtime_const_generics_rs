use std::mem::transmute;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

use replace_with::replace_with_or_abort;

trait GameObject {
  fn run(&mut self);
  fn set_debug(&mut self, flag: bool) -> &mut dyn GameObject;
}

trait GameObjectExt {
  fn set_debug(self: Box<Self>, flag: bool) -> Box<dyn GameObject>;
}

impl GameObjectExt for dyn GameObject {
  fn set_debug(self: Box<Self>, flag: bool) -> Box<dyn GameObject> {
    unsafe {
      let selv = Box::into_raw(self);
      let selv = (&mut *selv).set_debug(flag);
      return Box::from_raw(selv);
    }
  }
}

static ID_CNT: AtomicU32 = AtomicU32::new(0);

struct Node3D<const DEBUG: bool = false> {
  id: u32,
  cnt: u32,
}

impl Node3D {
  const TYPE_NAME: &str = "Node3D";
  fn new() -> Self {
    let id = ID_CNT.fetch_add(1, Ordering::Relaxed);
    let selv = Self { id, cnt: 0 };
    return selv;
  }
}

impl<const DEBUG: bool> GameObject for Node3D<DEBUG> {
  fn run(&mut self) {
    println!("Hello {} from {}@{}!", self.cnt, Node3D::TYPE_NAME, self.id);
    if DEBUG {
      println!("Debug {} from {}@{}!", self.cnt, Node3D::TYPE_NAME, self.id);
    }
    self.cnt += 1;
  }

  fn set_debug(&mut self, flag: bool) -> &mut dyn GameObject {
    unsafe {
      match flag {
        true => transmute::<_, &mut Node3D<true>>(self) as &mut dyn GameObject,
        false => transmute::<_, &mut Node3D<false>>(self) as &mut dyn GameObject,
      }
    }
  }
}

struct Node2D<const DEBUG: bool = false> {
  id: u32,
  cnt: u32,
}

impl Node2D {
  const TYPE_NAME: &str = "Node2D";
  fn new() -> Self {
    let id = ID_CNT.fetch_add(1, Ordering::Relaxed);
    let selv = Self { id, cnt: 0 };
    return selv;
  }
}

impl<const DEBUG: bool> GameObject for Node2D<DEBUG> {
  fn run(&mut self) {
    println!("Hello {} from {}@{}!", self.cnt, Node2D::TYPE_NAME, self.id);
    if DEBUG {
      println!("Debug {} from {}@{}!", self.cnt, Node2D::TYPE_NAME, self.id);
    }
    self.cnt += 1;
  }

  fn set_debug(&mut self, flag: bool) -> &mut dyn GameObject {
    unsafe {
      match flag {
        true => transmute::<_, &mut Node2D<true>>(self) as &mut dyn GameObject,
        false => transmute::<_, &mut Node2D<false>>(self) as &mut dyn GameObject,
      }
    }
  }
}

fn main() {
  let mut objects = Vec::new();
  for _ in 0..10 {
    objects.push(Box::new(Node3D::new()) as Box<dyn GameObject>);
    objects.push(Box::new(Node2D::new()) as Box<dyn GameObject>);
  }

  for o in 0..3 {
    for (i, object) in objects.iter_mut().enumerate() {
      let debug = (o + i) % 2 == 0;
      replace_with_or_abort(object, |object| object.set_debug(debug));
      object.run();
    }
  }
}
