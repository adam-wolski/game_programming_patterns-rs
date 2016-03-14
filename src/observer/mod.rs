//! http://gameprogrammingpatterns.com/observer.html

#[derive(Clone, Copy)]
enum Event {
    EntityFell,
}


enum Achievement {
    FellOfTheBridge,
}


trait Observer {
    fn on_notify<E: Entity>(&self, entity: &E, event: Event);
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


#[derive(PartialEq)]
struct Achievements {
    hero_on_bridge: bool,
}

impl Achievements {
    pub fn new() -> Achievements {
        Achievements { hero_on_bridge: true }
    }
    fn unlock(&self, achievement: Achievement) {
        match achievement {
            Achievement::FellOfTheBridge => println!("Fall of the bridge achievement unlocked.")
        }
    }
}

impl Observer for Achievements {
    fn on_notify<E: Entity>(&self, entity: &E, event: Event) {
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

trait Subject<'a, T: Observer>{
    fn add_observer(&mut self, observer: &'a T);
    fn remove_observer(&mut self, observer: &'a T);
    fn notify<E: Entity>(&self, entity: &E, event: Event);
}

struct EntityFallSubject<'a, T: 'a> {
    observers: Vec<&'a T>,
}

impl<'a, T> EntityFallSubject<'a, T>
    where T: Observer
{
    pub fn new() -> EntityFallSubject<'a, T> {
        EntityFallSubject {
            observers: Vec::new()
        }
    }
}

impl<'a, T> Subject<'a, T> for EntityFallSubject<'a, T> where T: Observer + PartialEq
{
    fn add_observer(&mut self, observer: &'a T) {
        self.observers.push(observer);
    }
    fn remove_observer(&mut self, observer: &'a T) {
        for (i, o) in self.observers.clone().into_iter().enumerate() {
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


struct Physics<'a, T: 'a + Observer> {
    fall_event: EntityFallSubject<'a, T>,
}

impl<'a, T> Physics<'a, T> where T: Observer + PartialEq
{
    pub fn new() -> Physics<'a, T> {
        let fall_event = EntityFallSubject::new();
        Physics { fall_event: fall_event }
    }

    pub fn update_entity<E: Entity>(&self, entity: &E) {
        // To some physics...
        // Entity has fallen of the bridge
        self.fall_event.notify(entity, Event::EntityFell);
    }

    pub fn fall_event(&mut self) -> &mut EntityFallSubject<'a, T> {
        &mut self.fall_event
    }
}

pub fn test() {
    println!("\n---------------------------");
    println!("Command test.\n");
    let hero = Hero::new();
    let achievements = Achievements::new();
    let mut physics = Physics::new();
    physics.fall_event().add_observer(&achievements);
    physics.update_entity(&hero);
}
