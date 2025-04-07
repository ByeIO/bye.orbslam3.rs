use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiNested {
    pub array_of_arrays: [crate::test_interface_files::msg::Arrays; 3],
    pub array_of_bounded_sequences: [crate::test_interface_files::msg::BoundedSequences; 3],
    pub array_of_unbounded_sequences: [crate::test_interface_files::msg::UnboundedSequences; 3],
    pub bounded_sequence_of_arrays: Vec<crate::test_interface_files::msg::Arrays>,
    pub bounded_sequence_of_bounded_sequences: Vec<crate::test_interface_files::msg::BoundedSequences>,
    pub bounded_sequence_of_unbounded_sequences: Vec<crate::test_interface_files::msg::UnboundedSequences>,
    pub unbounded_sequence_of_arrays: Vec<crate::test_interface_files::msg::Arrays>,
    pub unbounded_sequence_of_bounded_sequences: Vec<crate::test_interface_files::msg::BoundedSequences>,
    pub unbounded_sequence_of_unbounded_sequences: Vec<crate::test_interface_files::msg::UnboundedSequences>,
}

impl Default for MultiNested {
    fn default() -> Self {
        MultiNested {
            array_of_arrays: core::array::from_fn(|_| crate::test_interface_files::msg::Arrays::default()),
            array_of_bounded_sequences: core::array::from_fn(|_| crate::test_interface_files::msg::BoundedSequences::default()),
            array_of_unbounded_sequences: core::array::from_fn(|_| crate::test_interface_files::msg::UnboundedSequences::default()),
            bounded_sequence_of_arrays: Vec::new(),
            bounded_sequence_of_bounded_sequences: Vec::new(),
            bounded_sequence_of_unbounded_sequences: Vec::new(),
            unbounded_sequence_of_arrays: Vec::new(),
            unbounded_sequence_of_bounded_sequences: Vec::new(),
            unbounded_sequence_of_unbounded_sequences: Vec::new(),
        }
    }
}

impl ros2_client::Message for MultiNested {}
