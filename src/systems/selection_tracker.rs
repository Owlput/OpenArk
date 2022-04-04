use bevy::{gltf::GltfMesh, prelude::*};
use bevy_mod_picking::PickingEvent;

use crate::{
    environment::gltf_manual_bundle, plugins::freefloat_camera::{ControlEvent, OrbitTarget},
};

#[derive(Component)]
pub struct SelectionRing;
#[derive(Component)]
pub struct Movable;
#[derive(Component)]
pub struct Selected;
#[derive(Default)]
pub struct MovableSelectionLock(pub bool);
#[derive(Default)]
pub struct SelectedMovable(pub Option<Entity>,pub Option<Entity>);//(parent,ring)

pub fn selection_tracker(
    mut commands: Commands,
    mut event_reader: EventReader<PickingEvent>,
    mut disable: ResMut<MovableSelectionLock>,
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
                        if disable.0 != true && query.get(*e).is_ok() {
                            disable.0 = true
                        };
                        selected.0 = Some(e.clone());

                        let entity_handle = commands
                            .spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                                material: materials.add(Color::rgb(0.5, 0.7, 0.6).into()),
                                transform: Transform::from_xyz(0.0, 2.0, 0.0),
                                ..Default::default()
                            })
                            .id();
                        commands.entity(*e).insert(Selected).insert(OrbitTarget);
                        commands.entity(*e).push_children(&[entity_handle]);
                        event_writer.send(ControlEvent::ToggleMode(false))

                    }
                    bevy_mod_picking::SelectionEvent::JustDeselected(e) => {
                        if disable.0 != false && query.get(*e).is_ok() {
                            disable.0 = false
                        };
                        selected.0 = None;
                        commands.entity(*e).remove::<Selected>().remove::<OrbitTarget>();
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
    mut query: Query<&mut Transform>,
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<bevy::gltf::Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
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
                    .spawn_bundle(gltf_manual_bundle(
                        asset_server.load("./selection_ring.gltf"),
                        &assets_gltf,
                        &assets_gltfmesh,
                    ))
                    .insert(SelectionRing)
                    .id(),
            );
            return;
            //The newly added entity won't show up until next frame, so just return to avoid panic
        }
        Some(_) => {} //There is, proceed
                      //有一个，继续
    }
    info!("Try to sync translation");
    //Get the translation of the parent
    //获取祖先的位置
    let parent_pos = query
        .get_component::<Transform>(selected.0.unwrap())
        .unwrap()
        .clone()
        .translation;

    //Move the ring to its parent
    //把环移动过去
    query
        .get_component_mut::<Transform>(selected.1.unwrap())
        .unwrap()
        .translation = parent_pos;
}
