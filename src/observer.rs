//! Observer Pattern
//! http://gameprogrammingpatterns.com/observer.html

// Used for holding observers.
use std::cell::RefCell;
use std::rc::Rc;

// ================================================================================================
// Defines that we need to get out of our way for examples.
// ================================================================================================

/// Types of events that can be sent by observer subjects.
#[derive(Clone, Copy)]
pub enum Event {
    EntityFell,
}

/// Types of achievements that can be unlocked.
#[derive(PartialEq)]
pub enum Achievement {
    FellOfTheBridge,
}

/// Base trait of Entity in our theoretical game.
pub trait Entity {
    fn is_hero(&self) -> bool;
}

/// Our Hero that will fall of the bridge.
#[derive(Debug, Default)]
pub struct Hero;

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


/// Achievements structure that will become our observer. 
/// Well it will have it's own observer. Expained in `AchievementObserver` struct.
#[derive(Debug, Default)]
pub struct Achievements {
    pub hero_fallen: bool,
}

impl Achievements {
    pub fn new() -> Achievements {
        Achievements { hero_fallen: false }
    }

    fn unlock(&mut self, achievement: Achievement) {
        match achievement {
            Achievement::FellOfTheBridge => {
                if !self.hero_fallen {
                    self.hero_fallen = true;
                    println!("Fall of the bridge achievement unlocked.");
                }
            }
            // Other achievements...
        }
    }
}

/// Physics will hold subjects which we can observe and make our hero fall.
#[derive(Default)]
pub struct Physics {
    fall_event: EntityFallSubject,
}


// ================================================================================================
// Pattern itself
// ================================================================================================

/// Observer type that will be held inside Subject.
pub type ObserverType = Rc<RefCell<Box<Observer>>>;


/// Trait that defines objects which are interested in listening for events.
pub trait Observer {
    fn on_notify(&mut self, entity: &Entity, event: Event);
    /// Id string used to compare with other observers. Mainly for deleting that observer.
    fn id(&self) -> &String;
}


pub struct AchievementObserver {
    // Unlocked Achievements
    unlocked: Vec<Achievement>,
    achievements: Rc<RefCell<Achievements>>,
    id: String,
}

impl AchievementObserver {
    pub fn new(achievements: Rc<RefCell<Achievements>>) -> AchievementObserver {
        AchievementObserver {
            unlocked: Vec::new(),
            achievements: achievements,
            id: "AchievmentObserver".to_owned(),
        }
    }

    pub fn is_unlocked(&self, achievement: Achievement) -> bool {
        for a in &self.unlocked {
            if *a == achievement {
                return true;
            }
        }
        false
    }
}

impl Observer for AchievementObserver {
    fn on_notify(&mut self, entity: &Entity, event: Event) {
        match event {
            Event::EntityFell => {
                if entity.is_hero() && !self.is_unlocked(Achievement::FellOfTheBridge) {
                    self.unlocked.push(Achievement::FellOfTheBridge);
                    self.achievements.borrow_mut().unlock(Achievement::FellOfTheBridge);
                }
                // Handle other events...
            }
        }
    }

    fn id(&self) -> &String {
        &self.id
    }
}


/// Subject trait defines objects that will can be observer. It holds list of observers and sends 
/// notifications to them.
pub trait Subject{
    fn add_observer(&mut self, observer: ObserverType);
    fn remove_observer(&mut self, observer: ObserverType);
    fn notify(&mut self, entity: &Entity, event: Event);
}


/// We'll make Physics hold Subjects that can be observed instead of being a subject.
#[derive(Default)]
pub struct EntityFallSubject {
    pub observers: Vec<ObserverType>,
}

impl EntityFallSubject {
    pub fn new() -> EntityFallSubject {
        EntityFallSubject { observers: Vec::new() }
    }
}

impl Subject for EntityFallSubject {
    fn add_observer(&mut self, observer: ObserverType) {
        self.observers.push(observer);
    }
    fn remove_observer(&mut self, observer: ObserverType) {
        for (i, o) in self.observers.clone().into_iter().enumerate() {
            if o.borrow().id() == observer.borrow().id() {
                self.observers.remove(i);
            }
        }
    }
    fn notify(&mut self, entity: &Entity, event: Event) {
        for o in &self.observers {
            o.borrow_mut().on_notify(entity, event);
        }
    }
}

// And finally our physics implementation. Which holds fall event which can be observed.
impl Physics {
    pub fn new() -> Physics {
        let fall_event = EntityFallSubject::new();
        Physics { fall_event: fall_event }
    }

    pub fn update_entity<E: Entity>(&mut self, entity: &E) {
        // Do some physics...
        // Entity has fallen of the bridge
        self.fall_event.notify(entity, Event::EntityFell);
    }

    pub fn fall_event(&mut self) -> &mut EntityFallSubject {
        &mut self.fall_event
    }
}

#[cfg(test)]
mod tests {
    use super::{Hero, Achievements, Physics, Subject, Observer, AchievementObserver};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    pub fn observer() {
        let hero = Hero::new();
        let achievements = Rc::new(RefCell::new(Achievements::new()));
        let a_observer = AchievementObserver::new(achievements.clone());
        let a_observer = Rc::new(RefCell::new(Box::new(a_observer) as Box<Observer>));
        let mut physics = Physics::new();
        physics.fall_event().add_observer(a_observer);
        physics.update_entity(&hero);
        assert!(achievements.borrow().hero_fallen);
    }
}
