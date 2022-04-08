use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;

use crate::plugins::camera::ControlEvent;

#[derive(Component)]
pub struct SelectionRing;
#[derive(Component)]
pub struct Movable;
#[derive(Component)]
pub struct Selected;
#[derive(Default)]
pub struct MovableSelectionLock(pub bool);
#[derive(Default)]
pub struct SelectedMovable(pub Option<Entity>, pub Option<Entity>); //(parent,marker)

pub fn selection_tracker(
    mut commands: Commands,
    mut event_reader: EventReader<PickingEvent>,
    mut selected: ResMut<SelectedMovable>,
    mut event_writer: EventWriter<ControlEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<Movable>>,
) {
    for event in event_reader.iter() {
        match event {
            PickingEvent::Selection(e) => {
                match e {
                    bevy_mod_picking::SelectionEvent::JustSelected(e) => {
                        if query.get(*e).is_err() {
                            return;
                        }
                        selected.0 = Some(e.clone());
                        let child_handle = commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                            material: materials.add(Color::rgb(0.5, 0.7, 0.6).into()),
                            transform: Transform::from_xyz(0.0, 2.0, 0.0),
                            ..Default::default()
                        })
                        .insert(SelectionRing)
                        .id();
                        commands.entity(*e).insert(Selected).add_child(child_handle);
                        selected.1 = Some(child_handle);
                        event_writer.send(ControlEvent::ToggleMode(false));
                    }
                    bevy_mod_picking::SelectionEvent::JustDeselected(e) => {
                        if query.get(*e).is_err() {
                            return;
                        }
                        selected.0 = None;
                        let child = selected.1.unwrap();
                        commands.entity(*e).remove::<Selected>().remove_children(&[child]);
                        commands.entity(child).despawn();
                        event_writer.send(ControlEvent::ToggleMode(true));
                    }
                };
            }
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => info!("Gee Willikers, it's a click! {:?}", e),
        }
    }
}