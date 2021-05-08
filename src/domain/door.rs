pub(crate) struct Door {
    position: DoorPosition,
    prize: Option<()>,
}

impl Door {
    pub(crate) fn new(prize: Option<()>) -> Self {
        Door {
            position: DoorPosition::Closed,
            prize,
        }
    }

    pub(crate) fn open(&mut self) -> Result<(), OpenDoorError> {
        match self.position {
            DoorPosition::Open => Err(DoorAlreadyOpenError.into()),
            DoorPosition::Closed => {
                self.position = DoorPosition::Open;
                Ok(())
            }
        }
    }

    pub(crate) fn contains_prize(&self) -> bool {
        self.prize.is_some()
    }

    pub(crate) fn is_open(&self) -> bool {
        matches!(self.position, DoorPosition::Open)
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum OpenDoorError {
    #[error(transparent)]
    AlreadyOpen(#[from] DoorAlreadyOpenError),
}

#[derive(Debug, thiserror::Error)]
#[error("Door is already open")]
pub(crate) struct DoorAlreadyOpenError;

pub(crate) enum DoorPosition {
    Open,
    Closed,
}
