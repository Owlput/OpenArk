use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;

use crate::plugins::freefloat_camera::ControlEvent;

#[derive(Component)]
pub struct SelectionRing;
#[derive(Component)]
pub struct Movable;
#[derive(Component)]
pub struct Selected;
#[derive(Default)]
pub struct MovableSelectionLock(pub bool);
#[derive(Default)]
pub struct SelectedMovable(pub Option<Entity>, pub Option<Entity>); //(parent,ring)

pub fn selection_tracker(
    mut commands: Commands,
    mut event_reader: EventReader<PickingEvent>,
    mut selected: ResMut<SelectedMovable>,
    mut event_writer: EventWriter<ControlEvent>,
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
                        commands.entity(*e).insert(Selected);
                        event_writer.send(ControlEvent::ToggleMode(false));
                    }
                    bevy_mod_picking::SelectionEvent::JustDeselected(e) => {
                        if query.get(*e).is_err() {
                            return;
                        }
                        selected.0 = None;
                        commands.entity(*e).remove::<Selected>();
                        event_writer.send(ControlEvent::ToggleMode(true));
                    }
                };
            }
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => info!("Gee Willikers, it's a click! {:?}", e),
        }
    }
}

pub fn movable_selection_ring_handler(
    mut commands: Commands,
    query_parent: Query<&Transform, With<Selected>>,
    mut query_ring: Query<&mut Transform, (With<SelectionRing>,Without<Selected>)>,
    //engine panics when without Without<Selected> for some reasons
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut selected: ResMut<SelectedMovable>,
) {
    if selected.0 == None {
        if let Some(e) = selected.1 {
            info!("remove the ring");
            commands.entity(e).despawn()
        }
        selected.1 = None;
        return;
        //if there's no movable entity selected, just return and clean the ring that may exist
        //如果没有选中任何可移动实体，那就清除可能仍存在的选中环并返回
    }
    //now there's a movable entity selected
    //有选中可移动实体
    match selected.1 {
        None => {
            //There's no selection ring found, spawn one and get it into the resource
            //没有找到关联的选中环，生成一个并添加进资源
            selected.1 = Some(
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                        material: materials.add(Color::rgb(0.5, 0.7, 0.6).into()),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..Default::default()
                    })
                    .insert(SelectionRing)
                    .id(),
            );
            return;
            //The newly added entity won't show up until next frame, so just return to avoid panic
            //新加的实体貌似在本帧不会出现，所以先返回以避免惊恐
            //TODO：把下面的代码分离为单独的系统以增加并发性并及时更新位置
        }
        Some(e) => {
            //Get the translation of the parent
            //获取祖先的位置
            let mut parent_pos = query_parent
                .get_component::<Transform>(selected.0.unwrap())
                .unwrap()
                .clone()
                .translation;
            parent_pos.y += 1.0;
            //Move the ring to its parent
            //把环移动过去
            query_ring
                .get_component_mut::<Transform>(e)
                .unwrap()
                .translation = parent_pos;
        }
    }
}
