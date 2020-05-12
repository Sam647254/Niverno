use crate::grammar::morphology::derivations::Derivation;
use crate::grammar::morphology::inflections::Inflection;
use crate::grammar::prototypes::Prototype;
use std::collections::HashSet;

mod derivations;
mod inflections;

pub struct DerivedPrototype {
   pub base: Prototype,
   pub derivations: [Derivation],
}

pub fn derivations_of(prototype: Prototype) -> HashSet<Derivation> {
   unimplemented!()
}

pub fn inflections_of(prototype: &Prototype) -> HashSet<Inflection> {
   HashSet::new()
}

#[cfg(test)]
mod tests {
   use super::*;
   use crate::grammar::morphology::inflections::Inflection::*;
   use crate::dictionary::test_words::APPLE;
   use std::iter::FromIterator;

   #[test]
   fn apple_inflections() {
      let correct : HashSet<Inflection> = vec![
         Nonspecific,
         Specific,
         Anaphor,
         Archetype,
         Associated
      ].into_iter().collect();
      assert_eq!(correct, inflections_of(&*APPLE));
   }
}
