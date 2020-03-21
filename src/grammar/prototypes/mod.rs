pub(crate) mod noun;
pub(crate) mod verb;

use crate::grammar::prototypes::noun::Noun;
use crate::grammar::prototypes::verb::Verb;

pub enum Prototype {
   Noun(Noun),
   Verb(Verb),
}
