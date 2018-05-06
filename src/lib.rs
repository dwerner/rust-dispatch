#![feature(test)]

extern crate test;

#[derive(Debug)]
pub struct Point {
  pub x: f32,
  pub y: f32
}

#[derive(Debug)]
pub struct Player<T> where T: Sized {
  pub name: String,
  pub loc: Point,
  pub components: Vec<T>
}

#[derive(Debug)]
pub struct Enemy<T> where T: Sized {
  pub loc: Point,
  pub does_damage: u32,
  pub components: Vec<T>
}

#[derive(Debug)]
pub struct Pathing { pub data: String }

#[derive(Debug)]
pub struct Health { pub hearts: u32 }

#[cfg(test)]
mod static_tests {

  use super::*;
  use test::Bencher;

  #[derive(Debug)]
  pub enum Entity {
    Player(Player<Component>),
    Enemy(Enemy<Component>),
  }

  #[derive(Debug)]
  pub enum Component {
    Pathing(Pathing),
    Health(Health)
  }

  pub trait WithComponents {
    fn mut_components(&mut self) -> &mut Vec<Component>;
  }

  impl WithComponents for Player<Component> {
    fn mut_components(&mut self) -> &mut Vec<Component> {
      &mut self.components
    }
  }

  impl WithComponents for Enemy<Component> {
    fn mut_components(&mut self) -> &mut Vec<Component> {
      &mut self.components
    }
  }

  #[bench]
  fn static_dispatch(b: &mut Bencher) {
    let mut p = Entity::Player( Player{
      name: "Joe".to_string(),
      loc: Point { x: 1f32, y: 2f32 },
      components: vec![
        Component::Health(Health{ hearts: 42 })
      ]
    });

    let mut e = Entity::Enemy( Enemy{
      loc: Point { x: 10f32, y: 20f32 },
      does_damage: 15,
      components: vec![
        Component::Pathing(Pathing{ data: "fedcba".to_string() }),
        Component::Health(Health{ hearts: 77 })
      ]
    });

    b.iter(move || {
      update_entity_components(&mut p);
      update_entity_components(&mut e);
    });
  }

  pub fn update_entity_components(entity: &mut Entity) {
    match entity {
      &mut Entity::Enemy(ref mut e) => {
        let comps = &mut e.components;
        for i in comps.iter_mut() {
          match i {
            &mut Component::Health(ref mut h) => {
              h.hearts = 999;
            },
            &mut Component::Pathing(ref mut p) => {
              p.data = "asdfhjl".to_string() // alloc!!
            }
          }
        }
      },
      &mut Entity::Player(ref mut p) => {
        let comps = &mut p.components;
        for i in comps.iter_mut() {
          match i {
            &mut Component::Health(ref mut h) => {
              h.hearts = 999;
            },
          }
        }
      }
    }
  }

}

#[cfg(test)]
mod dynamic_tests {

  use super::*;
  use test::Bencher;

  trait Component { }
  trait HealthComponent : Component {
    fn set_hearts(&mut self, value: u32);
  }
  trait PathingComponent : Component {
    fn set_data(&mut self, data: String);
  }

  impl Component for Health {}
  impl Component for Pathing {}

  impl HealthComponent for Health {
    fn set_hearts(&mut self, value: u32) {
      self.hearts = value;
    }
  }

  impl PathingComponent for Pathing {
    fn set_data(&mut self, data: String) {
      self.data = data;
    }
  }

  pub trait WithComponents {
    fn mut_components(&mut self) -> &mut Vec<Box<Component>>;
  }

  impl WithComponents for Player<Box<Component>> {
    fn mut_components(&mut self) -> &mut Vec<Box<Component>> {
      &mut self.components
    }
  }

  impl WithComponents for Enemy<Box<Component>> {
    fn mut_components(&mut self) -> &mut Vec<Box<Component>> {
      &mut self.components
    }
  }

  #[bench]
  fn dynamic_dispatch(b: &mut Bencher) {
    let mut p = Player{
      name: "Joe".to_string(),
      loc: Point { x: 1f32, y: 2f32 },
      components: vec![
        Box::new(Health{ hearts: 42 }) as Box<Component>
      ]
    };

    let player = Box::new(p) as Box<WithComponents>;

    let mut e = Enemy{
      loc: Point { x: 10f32, y: 20f32 },
      does_damage: 15,
      components: vec![
        Box::new(Pathing{ data: "fedcba".to_string() }) as Box<Component>, 
        Box::new(Health{ hearts: 77 }) as Box<Component>
      ]
    };

    let enemy = Box::new(e) as Box<WithComponents>;

    b.iter(move || {
      update_trait_components(&mut enemy);
      update_trait_components(&mut player);
    });
  }

  pub fn update_trait_components(entity: &mut Box<WithComponents>) {
    let comps = &mut entity.mut_components();
    for i in comps.iter_mut() {
      let b = i as &mut Box<Component>;
      if let actual_comp = b as &mut Box<HealthComponent> {
        actual_comp.set_hearts(8008);
      }
      if let actual_comp = b as &mut Box<PathingComponent> {
        actual_comp.set_data("garbage".to_string()); //alloc!
      }
    }
  }

}
