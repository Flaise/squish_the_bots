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
        // uses area.positions, area.appearances
        
        match self.positions.at(focus) {
            None => Appearance::Floor,
            Some(entity) => self.appearances.of(entity).unwrap_or(Appearance::Floor),
        }
    }
}
