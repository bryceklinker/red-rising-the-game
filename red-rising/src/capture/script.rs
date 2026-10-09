use crate::game_state::GameState;
use bevy::prelude::Resource;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScriptStep {
    AdvanceTicks(u32),
    WaitForState(GameState),
    Capture(&'static str),
    Exit,
}

#[derive(Resource, Debug)]
pub struct ScriptedSession(VecDeque<ScriptStep>);

impl ScriptedSession {
    pub fn new(steps: Vec<ScriptStep>) -> Self {
        Self(steps.into())
    }

    pub fn front(&self) -> Option<&ScriptStep> {
        self.0.front()
    }

    pub fn pop_front(&mut self) -> Option<ScriptStep> {
        self.0.pop_front()
    }

    pub fn push_front(&mut self, step: ScriptStep) {
        self.0.push_front(step);
    }
}
