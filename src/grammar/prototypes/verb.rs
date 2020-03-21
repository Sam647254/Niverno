use std::collections::HashSet;

pub enum Receiver {
   Single,
   Multiple,
   Global,
}

pub enum Valency {
   Zero,
   One,
   Two,
}

pub struct Verb {
   pub receiver: Receiver,
   pub valency: Valency,
   pub instantaneous: bool,
   pub integral: bool,
   pub creation: bool,
   pub stem: &'static str,
   pub meaning: &'static str,
   pub root_stems: HashSet<&'static str>,
   pub argument_meanings: &'static [&'static str],
}
