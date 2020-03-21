use std::collections::HashSet;

pub enum Enumerability {
   Countable,
   Singleton,
   Mass,
}

pub enum Flavour {
   Open,
   Mid,
}

pub enum Associability {
   Common,
   Associable,
   Intrinsic,
}

pub struct Noun {
   pub enumerability: Enumerability,
   pub associability: Associability,
   pub flavour: Flavour,
   pub stem: &'static str,
   pub gloss_term: &'static str,
   pub meaning: &'static str,
   pub root_stems: HashSet<&'static str>,
}
