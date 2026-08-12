use std::ops::Deref;

use crate::components::*;
use bevy::{
    camera,
    ecs::system::command,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    transform,
    window::PrimaryWindow,
};
pub struct CameraPlugin;

#[derive(Component)]
struct CameraSettings {
    translation_sensitivity: f32,
}
impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            translation_sensitivity: 10.0,
        }
    }
}
#[derive(Resource)]
struct CameraZoomSettings {
    maximum_zoom: f32,
    minimum_zoom: f32,
    zoom_sensitivity: f32,
}
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraZoomSettings {
            maximum_zoom: 0.25,
            minimum_zoom: 10.0,
            zoom_sensitivity: 0.1,
        });

        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (move_camera, camera_zoom));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, CameraSettings::default(), Selected));
}

fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_query: Single<(&mut Transform, &CameraSettings), (With<Selected>, With<Camera>)>,
) {
    let (mut transform, settings) = camera_query.into_inner();

    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += settings.translation_sensitivity * transform.scale.x;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= settings.translation_sensitivity * transform.scale.x;
    }
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += settings.translation_sensitivity * transform.scale.x;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= settings.translation_sensitivity * transform.scale.x;
    }
}

fn camera_zoom(
    mut scroll_event: MessageReader<MouseWheel>,
    camera_query: Single<
        (&mut Transform, &Camera, &GlobalTransform, &mut Projection),
        (With<Selected>, With<Camera2d>),
    >,
    zoom_settings: Res<CameraZoomSettings>,
    window: Single<&Window, With<PrimaryWindow>>,
) { 

    let (mut transform, camera, global_transform, mut projection) = camera_query.into_inner();

    for scroll_tick in scroll_event.read() {
        if let Some(cursor_position) = window.cursor_position() {

            
            let cursor_world_pos = camera.viewport_to_world_2d(&global_transform, cursor_position).unwrap();

            if let Projection::Orthographic(ref mut ortho) = *projection {

                let oldscale = ortho.scale;

                if ortho.scale <= zoom_settings.minimum_zoom
                    && ortho.scale >= zoom_settings.maximum_zoom
                {
                    ortho.scale -= scroll_tick.y * zoom_settings.zoom_sensitivity * ortho.scale;
                }
                //les deux conditions test si le scale à dépasser la valeur maxi ou mini de zoom et le corige si nécéssaire.
                if ortho.scale > zoom_settings.minimum_zoom {
                    ortho.scale = zoom_settings.minimum_zoom;
                }

                if ortho.scale < zoom_settings.maximum_zoom {
                    ortho.scale = zoom_settings.maximum_zoom;
                }

                //calcul de l'offset pour le focus curseur.
                let delta = cursor_world_pos-(global_transform.translation().truncate()+((ortho.scale/oldscale)*(cursor_world_pos-global_transform.translation().truncate())));
                transform.translation+=delta.extend(0.0);
            }
        }
    }
}
