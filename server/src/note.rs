use bevy::prelude::*;
use common::note::Note;
use lightyear::prelude::{Replicate, ReplicationSender};

pub const DEFAULT_NOTE_SIZE: Vec2 = Vec2::new(128.0, 128.0);

pub(crate) struct NotePlugin;

impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, new_note));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(NoteHandles {
        mesh: meshes.add(Rectangle::new(DEFAULT_NOTE_SIZE.x, DEFAULT_NOTE_SIZE.y)),
        material: materials.add(Color::linear_rgb(1.0, 0.92549, 0.35686)),
    });
}

fn new_note(mut commands: Commands, note_handles: Res<NoteHandles>) {
    let pos = Vec2::new(0.0, 0.0);
    let text = "Test Text";

    commands.spawn((
        Note,
        Text2d(text.to_string()),
        Transform::from_xyz(pos.x, pos.y, 0.1),
        Mesh2d(note_handles.mesh.clone()),
        MeshMaterial2d(note_handles.material.clone()),
        Replicate::default(),
        ReplicationSender::default(),
    ));
}

#[derive(Debug, Resource)]
pub struct NoteHandles {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}
