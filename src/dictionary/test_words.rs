use lazy_static::lazy_static;

use crate::grammar::prototypes::noun::{Associability, Enumerability, Flavour, Noun};
use crate::grammar::prototypes::verb::{Receiver, Valency, Verb};
use crate::grammar::prototypes::Prototype;
use std::collections::HashSet;

lazy_static! {
   static ref APPLE: Prototype = Prototype::Noun(Noun {
      enumerability: Enumerability::Countable,
      associability: Associability::Associable,
      flavour: Flavour::Open,
      stem: "lep",
      gloss_term: "apple",
      meaning: "apple",
      root_stems: HashSet::new(),
   });
   static ref EAT: Prototype = Prototype::Verb(Verb {
      receiver: Receiver::Single,
      valency: Valency::One,
      instantaneous: false,
      integral: false,
      creation: false,
      stem: "buli",
      meaning: "eat",
      root_stems: Default::default(),
      argument_meanings: &["eater", "food item"],
   });
}
