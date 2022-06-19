use crate::graph::edge::{
    PULL_FORCE_FORCE_AT_TWICE_DISTANCE, PULL_FORCE_MIN_DISTANCE, PUSH_FORCE_DISTANCE,
    PUSH_FORCE_FORCE,
};
use crate::graph::gravity::{PullForceConfig, PushForceConfig};
use std::borrow::BorrowMut;

#[derive(PartialEq)]
pub enum UiMode {
    Edit,
    Algorithm,
}

pub struct UiData {
    mode: UiMode,

    is_directed: bool,

    //   force:
    push_conf: PushForceConfig,
    pull_conf: PullForceConfig,

    //   random-gen:
    node_count: u32,
    edge_count: u32,
}

impl UiData {
    pub fn new() -> UiData {
        UiData {
            is_directed: true,
            push_conf: PushForceConfig::new(PUSH_FORCE_FORCE, PUSH_FORCE_DISTANCE),
            pull_conf: PullForceConfig::new(
                PULL_FORCE_MIN_DISTANCE,
                PULL_FORCE_FORCE_AT_TWICE_DISTANCE,
            ),
            node_count: 10,
            edge_count: 15,
            mode: UiMode::Edit,
        }
    }

    pub fn is_directed(&self) -> bool {
        self.is_directed
    }

    pub fn state_mut(&mut self) -> &mut UiMode {
        self.mode.borrow_mut()
    }

    pub fn state(&self) -> &UiMode {
        &self.mode
    }

    pub fn directed_mut(&mut self) -> &mut bool {
        &mut self.is_directed
    }

    pub fn node_count_mut(&mut self) -> &mut u32 {
        &mut self.node_count
    }

    pub fn edge_count(&mut self) -> &mut u32 {
        &mut self.edge_count
    }

    pub fn push_conf_mut(&mut self) -> &mut PushForceConfig {
        &mut self.push_conf
    }

    pub fn pull_conf_mut(&mut self) -> &mut PullForceConfig {
        &mut self.pull_conf
    }

    pub fn directed(&self) -> bool {
        self.is_directed
    }

    pub fn push_conf(&self) -> &PushForceConfig {
        &self.push_conf
    }

    pub fn pull_conf(&self) -> &PullForceConfig {
        &self.pull_conf
    }
}

impl Default for UiData {
    fn default() -> Self {
        Self::new()
    }
}
