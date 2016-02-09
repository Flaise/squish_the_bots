use area::Area;
use space::*;


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Appearance {
    Floor,
    Bot,
    Block,
    Abyss,
}
impl Area {
    pub fn appearance_at(&self, focus: Position) -> Appearance {
        match self.positions.at(focus) {
            None => Appearance::Floor,
            Some(entity) => match self.appearances.of(entity) {
                None => Appearance::Floor,
                Some(appearance) => appearance,
            }
        }
    }
}
