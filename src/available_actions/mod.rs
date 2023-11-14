use self::{resources::ActorQueue, systems::process_action_queue};
use bevy::prelude::*;

pub mod actions;
pub mod components;
pub mod resources;
mod systems;

// Events
#[derive(Event)]
pub struct TickEvent;
#[derive(Event)]
pub struct NextActorEvent;
#[derive(Event)]
pub struct ActionsCompleteEvent;
#[derive(Event)]
pub struct InvalidPlayerActionEvent;
#[derive(Event)]
pub struct InvalidSystemActionEvent;

pub struct ActionsPlugin;
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()
            .add_event::<InvalidSystemActionEvent>()
            .add_systems(Update, process_action_queue.run_if(on_event::<TickEvent>()));
    }
}
