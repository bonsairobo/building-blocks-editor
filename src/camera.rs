use bevy::{
    app::prelude::*, ecs::prelude::*, math::prelude::*, render::entity::Camera3dBundle,
    transform::components::Transform,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(initialize_camera.system());
    }
}

fn initialize_camera(commands: &mut Commands) {
    let eye = Vec3::new(100.0, 100.0, 100.0);
    let target = Vec3::new(0.0, 0.0, 0.0);
    create_camera_entity(commands, eye, target);
}

fn create_camera_entity(commands: &mut Commands, eye: Vec3, target: Vec3) -> Entity {
    let mut camera_components = Camera3dBundle::default();
    camera_components.transform =
        Transform::from_matrix(Mat4::face_toward(eye, target, Vec3::unit_y()));

    commands.spawn(camera_components).current_entity().unwrap()
}
