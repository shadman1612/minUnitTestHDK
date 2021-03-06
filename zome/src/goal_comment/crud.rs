use crate::{error::Error, validate_helpers::entry_from_element_create_or_update};
use dna_help::{WrappedAgentPubKey, WrappedHeaderHash};
use hdk::prelude::*;

#[hdk_entry(id = "goal_comment")]
#[derive(Clone, PartialEq)]
pub struct GoalComment {
    pub goal_address: WrappedHeaderHash,
    pub content: String,
    pub agent_address: WrappedAgentPubKey,
    pub unix_timestamp: f64,
    pub is_imported: bool,
}

// can be updated
impl TryFrom<&Element> for GoalComment {
    type Error = Error;
    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        entry_from_element_create_or_update::<GoalComment>(element)
    }
}

impl GoalComment {
    pub fn new(
        goal_address: WrappedHeaderHash,
        content: String,
        agent_address: WrappedAgentPubKey,
        unix_timestamp: f64,
        is_imported: bool,
    ) -> Self {
        Self {
            goal_address,
            content,
            agent_address,
            unix_timestamp,
            is_imported,
        }
    }
}
