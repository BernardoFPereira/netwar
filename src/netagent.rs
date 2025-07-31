use crate::prelude::*;

// Idea for the future: Add different agent types with different behaviour.
// i.e. a type that doesn't flee from more than 1 enemy and instead seeks
// aggressively

pub struct Agent {
    position: Vec2,
    velocity: Vec2,
    target: Option<Vec2>,
}
impl Agent {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: vec2(0., 0.),
            target: None,
        }
    }
}

pub struct Units {
    pub agents: Vec<Agent>,
}
impl Units {
    pub fn new() -> Self {
        Self { agents: vec![] }
    }

    fn spawn_unit(&mut self, position: Vec2) {
        self.agents.push(Agent::new(position));
    }

    pub fn draw_units(&self) {
        for agent in &self.agents {
            draw_rectangle(agent.position.x, agent.position.y, 2., 2., BLUE);
        }
    }
}
