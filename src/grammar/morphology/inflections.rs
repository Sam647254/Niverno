#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Inflection {
   Nonspecific,
   Specific,
   Anaphor,
   Archetype,
   Associated,
   Detached,
   StartImperative,
   StopImperative,
   Started,
   Finished,
   NotStarted,
   Paused,
   Stopped,
   Cancelled,
   Right,
   Left,
}
