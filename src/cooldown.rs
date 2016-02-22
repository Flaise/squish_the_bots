use std::collections::hash_map::Entry::*;
use area::*;
use entity::*;
use self::CooldownState::*;


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CooldownState {
    Ready,
    Waiting,
}


impl Area {
    pub fn acted(&mut self, entity: Entity, ticks_used: u8) {
        if ticks_used == 0 {
            debug_unreachable!();
            self.cooldowns.detach(entity);
            return;
        }
        
        self.cooldowns.attach(entity, ticks_used);
    }
    
    pub fn tick(&mut self, entity: Entity) -> CooldownState {
        match self.cooldowns.contents.entry(entity) {
            Vacant(..) => Ready,
            Occupied(mut element) => {
                {
                    let value = element.get_mut();
                    if *value > 1 {
                        *value -= 1;
                        return Waiting;
                    }
                }
                element.remove();
                Ready
            }
        }
    }
}

#[test]
fn ready_after_delay() {
    let mut area = Area::new();
    let bot = area.entities.make();
    
    assert_eq!(area.tick(bot), Ready);
    area.acted(bot, 3);
    assert_eq!(area.tick(bot), Waiting);
    assert_eq!(area.tick(bot), Waiting);
    
    assert_eq!(area.tick(bot), Ready);
}