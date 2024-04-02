use crate::rw_container::RwContainer;
use core::fmt::Error;
use runtime::trace::Trace;

pub struct EntryBuilder {
    pub rw_container: RwContainer,
}

impl Default for EntryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EntryBuilder {
    pub fn new() -> EntryBuilder {
        Self {
            rw_container: RwContainer::new(),
        }
    }

    pub fn build(&mut self, trace: &Trace) -> Result<(), Error> {
        for (index, step) in trace.steps.iter().enumerate() {
            // Register may not all be zero.
            if index == 0 {
                self.rw_container.register = step.registers.clone();
            }
            // match step.instruction.opcode
            // TODO: store rw operations to container
            dbg!(&self.rw_container.rw_register_ops);
            self.rw_container.step(step)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use runtime::trace::{self, Step, Trace};
    use serde_json::map::Entry;

    use super::EntryBuilder;

    fn get_trace_from_file(path: &str) -> Vec<Step> {
        let file = File::open(path).expect("open file");
        let reader = BufReader::new(file);
        let trace = serde_json::from_reader(reader).expect("read json");
        trace
    }

    #[test]
    fn deserialize_trace() {
        let steps = get_trace_from_file("trace.json");
        dbg!(&steps);
    }

    #[test]
    fn test_entry_builder() {
        let mut entry_builder = EntryBuilder::new();
        let steps = get_trace_from_file("trace.json");
        let trace = Trace {
            cycles: 0,
            return_value: 0,
            steps,
        };
        entry_builder.build(&trace).expect("build entry");
    }
}
