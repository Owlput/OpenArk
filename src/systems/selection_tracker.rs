use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;

use crate::{plugins::camera::ControlEvent, rapier_phy::PhyMovable};

#[derive(Component)]
pub struct SelectionRing;
#[derive(Component)]
pub struct Movable;
//A marker for entities that can be moved using camera
//能使用相机移动的实体的标签
#[derive(Component)]
pub struct Selected;
//A marker for the currently selected entites
//目前选中的实体的标签

#[derive(Default)]
pub struct SelectedMovable(pub Option<Entity>, pub Option<Entity>); //(parent,marker)
//Obviously you can only move one entity through camera at a time, 
//so simply quering (With<Selected>,With<Movable>) won't be enough.
//However quering this way can still save us some resource.
//显然你一次只能通过相机移动一个实体，所以查询(With<Selected>,With<Movable>)还不够
//但是这样查询是可以节省一些资源的。

pub fn selection_tracker(
    mut commands: Commands,
    mut event_reader: EventReader<PickingEvent>,
    mut selected: ResMut<SelectedMovable>,
    mut event_writer: EventWriter<ControlEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, Or<(With<Movable>,With<PhyMovable>)>>,
) {
    for event in event_reader.iter() {
        //iterate through all events
        //遍历所有事件
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