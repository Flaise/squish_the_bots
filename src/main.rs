extern crate squish_the_bots;
use squish_the_bots::space::*;

// enum EntityType {
//     Bot,
//     Block,
//     Abyss,
// }

// enum ToClient {
//     RoundStarted,
//     YouSaw(Option<EntityType>),
//     YouDied,
//     YouSurvived,
//     TooHeavyToPush,
//     CommandExecuted,
//     MalformedCommand,
// }

// enum FromClient {
//     LookAt(Offset), // 1 tick
//     Move(Direction), // 3 ticks
//     Drill(Direction), // 7 ticks
// }


enum Entity {
    Bot {
        position: Position,
        ticks_until_active: u8,
    },
    Block {
        position: Position,
    },
    Abyss,
}


fn main() {
}
