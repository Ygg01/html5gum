use crate::utils::State;
use crate::Emitter;

pub(crate) struct MachineHelper {
    pub temporary_buffer: String,
    pub character_reference_code: u32,
    pub state: State,
    return_state: Option<State>,
}

impl Default for MachineHelper {
    fn default() -> Self {
        MachineHelper {
            temporary_buffer: String::new(),
            character_reference_code: 0,
            state: State::Data,
            return_state: None,
        }
    }
}

impl MachineHelper {
    pub(crate) fn is_consumed_as_part_of_an_attribute(&self) -> bool {
        matches!(
            self.return_state,
            Some(
                State::AttributeValueDoubleQuoted
                    | State::AttributeValueSingleQuoted
                    | State::AttributeValueUnquoted
            )
        )
    }
    pub(crate) fn flush_code_points_consumed_as_character_reference<E: Emitter>(
        &mut self,
        emitter: &mut E,
    ) {
        if self.is_consumed_as_part_of_an_attribute() {
            emitter.push_attribute_value(&self.temporary_buffer);
            self.temporary_buffer.clear();
        } else {
            self.flush_buffer_characters(emitter);
        }
    }
    pub(crate) fn flush_buffer_characters<E: Emitter>(&mut self, emitter: &mut E) {
        emitter.emit_string(&self.temporary_buffer);
        self.temporary_buffer.clear();
    }

    pub(crate) fn enter_state(&mut self, state: State) {
        self.return_state = Some(self.state);
        self.state = state;
    }

    pub(crate) fn pop_return_state(&mut self) -> State {
        self.return_state.take().unwrap()
    }

    pub(crate) fn exit_state(&mut self) {
        self.state = self.pop_return_state();
    }
}