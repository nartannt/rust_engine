use crate::transform::Transform;

use legion::EntityStore;
use legion::IntoQuery;
use legion::storage::Component;
use legion::world::WorldOptions;
use legion::world::Entity;
use legion::world::Entry;
use legion::world::World;


pub struct GameObject {
    pub is_active: bool,
    pub is_loaded: bool,
    pub entity: Entity,
    pub transform: Transform,
}

impl GameObject {
    pub fn new(world: &mut World) -> Self {
        let entity = world.push(());
        let test = world.entry(entity).unwrap();
        GameObject {
            is_active: true,
            is_loaded: false,
            transform: Transform::default(),
            entity,
        }
    }

    pub fn is_active(&self) -> bool {
        return self.is_active;
    }
}


pub fn has_component<T: Component>(go: &GameObject, world: &World) -> bool {
    let entry = world.entry_ref(go.entity).unwrap();
    return entry.archetype().layout().has_component::<T>();
}
