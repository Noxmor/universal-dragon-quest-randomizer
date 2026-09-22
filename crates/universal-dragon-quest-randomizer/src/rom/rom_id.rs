use strum::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, Default, Display, EnumString)]
pub enum RomId {
    #[default]
    Unknown,

    #[strum(to_string = "Dragon Quest Monsters: Joker 1")]
    DragonQuestMonstersJoker1,

    #[strum(to_string = "Dragon Quest Monsters: Joker 2")]
    DragonQuestMonstersJoker2,

    #[strum(to_string = "Dragon Quest Monsters: Joker 2 Professional")]
    DragonQuestMonstersJoker2P,

    #[strum(to_string = "Dragon Quest Monsters: Joker 3")]
    DragonQuestMonstersJoker3,

    #[strum(to_string = "Dragon Quest Monsters: Joker 3 Professional")]
    DragonQuestMonstersJoker3P,
}
