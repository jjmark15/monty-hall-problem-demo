use rand::seq::IteratorRandom;
use rand::thread_rng;

use crate::domain::door::Door;

pub(crate) struct Contestant {}

impl Contestant {
    pub(crate) fn new() -> Self {
        Contestant {}
    }

    pub(crate) fn choose_a_closed_door_by_index(&self, doors: &[Door; 3]) -> usize {
        let mut rng = thread_rng();
        doors
            .iter()
            .enumerate()
            .filter(|(_i, door)| !door.is_open())
            .map(|(index, _)| index)
            .choose(&mut rng)
            .unwrap()
    }
}
