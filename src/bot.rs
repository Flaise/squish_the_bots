use space::*;
use entity::*;


// fn make(ents: Entities<_>, position: Position) -> Result<(), EntityError> {
    
// }


// #[test]
// fn instantiation() {
//     let a = Position::new(0, 0);
//     let b = Position::new(2, 0);
//     let c = Position::new(-1, 3);
    
//     let mut ents = Entities::new();
//     assert!(!ents.occupied(a));
//     assert!(!ents.occupied(b));
//     assert!(!ents.occupied(c));
    
//     // ents.add(Entity::new(a, ())).unwrap();
    
//     make(ents, a).unwrap();
//     assert!(ents.occupied(a));
//     assert!(!ents.occupied(b));
//     assert!(!ents.occupied(c));
    
//     // ents.add(Entity::new(b, ())).unwrap();
//     make(ents, b).unwrap();
//     assert!(ents.occupied(a));
//     assert!(ents.occupied(b));
//     assert!(!ents.occupied(c));
// }
