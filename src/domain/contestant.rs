use rand::seq::IteratorRandom;
use rand::thread_rng;

use crate::domain::door::Door;

pub(crate) struct Contestant {
    chosen_door_index: Option<usize>,
}

impl Contestant {
    pub(crate) fn new() -> Self {
        Contestant {
            chosen_door_index: None,
        }
    }

    pub(crate) fn chosen_door_index(&self) -> Option<usize> {
        self.chosen_door_index
    }

    pub(crate) fn set_chosen_door_index(&mut self, index: usize) {
        self.chosen_door_index = Some(index);
    }

    pub(crate) fn pick_a_door_by_index(&mut self, doors: &[Door; 3]) {
        let mut rng = thread_rng();
        let choice = doors
            .iter()
            .enumerate()
            .map(|(index, _)| index)
            .choose(&mut rng)
            .unwrap();
        self.chosen_door_index = Some(choice);
    }
}
