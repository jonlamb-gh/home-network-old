use crate::{Error, ParameterFlags, ParameterFrame, ParameterId, ParameterValue};

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Parameter {
    local_time_ms: u64,
    id: ParameterId,
    flags: ParameterFlags,
    value: ParameterValue,
}

impl Parameter {
    pub fn new(id: ParameterId, flags: ParameterFlags) -> Self {
        Parameter {
            local_time_ms: 0,
            id,
            flags,
            value: ParameterValue::default(),
        }
    }

    pub const fn new_with_value(
        id: ParameterId,
        flags: ParameterFlags,
        value: ParameterValue,
    ) -> Self {
        Parameter {
            local_time_ms: 0,
            id,
            flags,
            value,
        }
    }

    pub fn local_time_ms(&self) -> u64 {
        self.local_time_ms
    }

    pub fn id(&self) -> ParameterId {
        self.id
    }

    pub fn flags(&self) -> ParameterFlags {
        self.flags
    }

    pub fn value(&self) -> ParameterValue {
        self.value
    }

    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &ParameterFrame<&T>) -> Result<Self, Error> {
        frame.check_len()?;
        Ok(Parameter {
            local_time_ms: frame.local_time_ms(),
            id: frame.id(),
            flags: frame.flags(),
            value: frame.value(),
        })
    }

    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut ParameterFrame<T>) {
        frame.set_local_time_ms(self.local_time_ms);
        frame.set_id(self.id);
        frame.set_flags(self.flags);
        frame.set_value(self.value);
    }
}
