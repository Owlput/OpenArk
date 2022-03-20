use bevy::{math::Vec3, prelude::{App, Plugin, EventReader, Query, With, Transform}};

use crate::{general_components::{Movable, Selected}, control::pickable_movement::pickable_movement_controller};

#[derive(Default)]
pub struct PickableMovementPlugin;

impl Plugin for PickableMovementPlugin{
    fn build(&self,app:&mut App){
        app.add_system(control_system)
        .add_event::<ControlEvent>()
        .add_system(pickable_movement_controller);
    }
}

pub enum ControlEvent{
    Translate(Vec3)
}

fn control_system(
    mut events:EventReader<ControlEvent>,
    mut query:Query<&mut Transform,(With<Movable>,With<Selected>)>,
){
    let mut transform = if let Some(transform) = query.iter_mut().next(){
        transform
    }else{return};
    
    for event in events.iter(){
        match event{
            &ControlEvent::Translate(delta)=>{
                println!("event received");
                transform.translation += delta
            }
        }
    }

}