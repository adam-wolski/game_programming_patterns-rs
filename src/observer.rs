//! http://gameprogrammingpatterns.com/observer.html
use std::rc::Rc;

enum Event {
    EntityFell,
}


enum Achievement {
    FellOfTheBridge,
}


trait Observer {
    fn on_notify<E: Entity>(&self, entity: &E, event: Event) where Self: Sized;
}


trait Entity {
    fn is_hero(&self) -> bool;
}


struct Hero;
impl Hero {
    pub fn new() -> Hero {
        Hero
    }
}

impl Entity for Hero {
    fn is_hero(&self) -> bool {
        true
    }
}


struct Achievements {
    hero_on_bridge: bool,
}

impl Achievements {
    pub fn new() -> Achievements {
        Achievements { hero_on_bridge: true }
    }
    fn unlock(&self, achievement: Achievement) {
        println!("Unlocked achievement, yay");
    }
}

impl Observer for Achievements {
    fn on_notify<E: Entity>(&self, entity: &E, event: Event) where Self: Sized {
        match event {
            Event::EntityFell => {
                if entity.is_hero() && self.hero_on_bridge {
                    self.unlock(Achievement::FellOfTheBridge)
                }
            // Handle other events..
            }
        }
    }
}

trait Subject {
    fn add_observer<T: Observer>(&self, observer: Rc<T>);
    fn remove_observer<T: Observer>(&self, observer: Rc<T>);
    fn notify<E: Entity>(&self, entity: &E, event: Event);
}

struct EntityFallSubject {
    observers: Vec<Rc<Observer>>
}

impl Subject for EntityFallSubject 
{
    fn add_observer<T: Observer>(&self, observer: Rc<T>) {
        self.observers.push(observer);
    }
    fn remove_observer<T: Observer>(&self, observer: Rc<T>) {
        for (i, o) in self.observers.into_iter().enumerate() {
            if o == observer {
                self.observers.remove(i);
            }
        }
    }
    fn notify<E: Entity>(&self, entity: &E, event: Event) {
        for o in &self.observers {
            o.on_notify(entity, event);
        }
    }
}


struct Physics {
    fall_event: EntityFallSubject,
}

impl Physics {
    pub fn new() -> Physics {
        Physics {  }
    }

    pub fn update_entity<E: Entity>(&self, entity: E) {
        // To some physics...
        // Entity has fallen of the bridge
        self.fall_event.notify(entity, Event::EntityFell);
    }

    pub fn fall_event(&self) -> &mut EntityFallSubject {
        self.fall_event
    }
}

pub fn test() {
    let hero = Hero::new();
    let physics = Physics::new();
    let achievements = Achievements::new();
    physics.fall_event.add_observer(achievements);
    physics.update_entity(hero);
}
