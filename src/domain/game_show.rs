use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::domain::contestant::Contestant;
use crate::domain::door::Door;

pub(crate) struct GameShow {
    doors: [Door; 3],
    contestant: Contestant,
}

impl GameShow {
    pub(crate) fn new(doors: [Door; 3], contestant: Contestant) -> Self {
        GameShow {
            doors: Self::shuffled_doors(doors),
            contestant,
        }
    }

    pub(crate) fn allow_contestant_to_pick_door(&mut self) {
        self.contestant.pick_a_door_by_index(&self.doors);
    }

    pub(crate) fn open_a_door_without_prize(&mut self) {
        for (i, door) in self.doors.iter_mut().enumerate() {
            if door.is_open()
                || i == self.contestant.chosen_door_index().unwrap()
                || door.contains_prize()
            {
                continue;
            }

            door.open().unwrap();
            break;
        }
    }

    pub(crate) fn allow_contestant_to_switch_doors(&mut self) {
        for (i, door) in self.doors.iter().enumerate() {
            if door.is_open() || i == self.contestant.chosen_door_index().unwrap() {
                continue;
            }

            self.contestant.set_chosen_door_index(i);
            break;
        }
    }

    pub(crate) fn open_picked_door(&mut self) {
        self.doors
            .get_mut(self.contestant.chosen_door_index().unwrap())
            .unwrap()
            .open()
            .unwrap();
    }

    pub(crate) fn contestant_has_won(&self) -> bool {
        self.doors
            .iter()
            .filter(|door| door.is_open() && door.contains_prize())
            .count()
            == 1
    }

    fn shuffled_doors(mut doors: [Door; 3]) -> [Door; 3] {
        let mut rng = thread_rng();
        doors.shuffle(&mut rng);
        doors
    }
}
