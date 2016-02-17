use std::io::{self, Write, Cursor};
use appearance::*;


#[derive(PartialEq, Debug)]
pub enum Notification {
    YourTurn,
    YouSee(Appearance),
    YouDied,
    Success,
    TooHeavy,
    NewRound,
}

const CODE_FLOOR: u8 = 0;
const CODE_BOT: u8 = 1;
const CODE_BLOCK: u8 = 2;
const CODE_ABYSS: u8 = 3;


fn appearance_to_code(appearance: Appearance) -> u8 {
    match appearance {
        Appearance::Floor => CODE_FLOOR,
        Appearance::Bot => CODE_BOT,
        Appearance::Block => CODE_BLOCK,
        Appearance::Abyss => CODE_ABYSS,
    }
}


fn serialize_notification(notification: Notification) -> Vec<u8> {
    match notification {
        Notification::YourTurn => vec![1],
        Notification::YouDied => vec![2],
        Notification::Success => vec![3],
        Notification::TooHeavy => vec![4],
        Notification::NewRound => vec![5],
        Notification::YouSee(appearance) => vec![6, appearance_to_code(appearance)],
    }
}


pub fn notify(output: &mut Box<Write>, notification: Notification) -> io::Result<()> {
    output.write_all(serialize_notification(notification).as_ref())
}


#[test]
fn serializing() {
    assert_eq!(serialize_notification(Notification::YourTurn), vec![1]);
    assert_eq!(serialize_notification(Notification::YouDied), vec![2]);
    assert_eq!(serialize_notification(Notification::Success), vec![3]);
    assert_eq!(serialize_notification(Notification::TooHeavy), vec![4]);
    assert_eq!(serialize_notification(Notification::NewRound), vec![5]);
    assert_eq!(serialize_notification(Notification::YouSee(Appearance::Floor)), vec![6, 0]);
    assert_eq!(serialize_notification(Notification::YouSee(Appearance::Bot)),
               vec![6, 1]);
    assert_eq!(serialize_notification(Notification::YouSee(Appearance::Block)),
               vec![6, 2]);
    assert_eq!(serialize_notification(Notification::YouSee(Appearance::Abyss)),
               vec![6, 3]);
}
