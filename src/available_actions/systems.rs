use super::{
    components::Actor, resources::ActorQueue, ActionsCompleteEvent, InvalidPlayerActionEvent,
    InvalidSystemActionEvent, NextActorEvent,
};
use crate::player::components::Player;
use bevy::prelude::*;

pub fn process_action_queue(world: &mut World) {
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else { return };
    let Some(entity) = queue.0.pop_front() else {
        world.send_event(ActionsCompleteEvent);
        return;
    };
    let Some(mut actor) = world.get_mut::<Actor>(entity) else { return };
    let Some(action) = actor.0.take() else { return };

    let action_result = action.execute(world);
    if !action_result && world.get::<Player>(entity).is_some() {
        world.send_event(InvalidPlayerActionEvent);
        return;
    } else if !action_result {
        world.send_event(InvalidSystemActionEvent);
        return;
    }

    world.send_event(NextActorEvent);
}
