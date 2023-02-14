use bevy::{prelude::*, sprite::{Mesh2dHandle, MaterialMesh2dBundle}, render::render_resource::PrimitiveTopology};


const TRAIL_LENGTH : usize = 256; // 2^8

pub struct TrailsPlugin;

impl Plugin for TrailsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init)
            .add_system(update_trails);
    }
}


// History of positions to draw the (past) trajectory of the object
// Second field is the index to the last added element
#[derive(Component)]
pub struct Trail(Box<[Vec3]>, usize, Color);

impl Trail {
    pub fn new_at(initial_pos: Vec3, color: Color) -> Self {
        Self(vec![initial_pos; TRAIL_LENGTH].into(), 0, color)
    }
}

#[derive(Component)]
struct TrailsEntityTag;

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    // Spawn the entity responsible for drawing trails 
    // (initially holds an empty mesh because no trails exist yet)
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::new(PrimitiveTopology::LineList)).into(),
            material: materials.add(Color::WHITE.into()),
            ..default()
        },
        TrailsEntityTag
    ));
}

fn update_trails(
    mut meshes: ResMut<Assets<Mesh>>,
    mut pos_buffer: Local<Vec<Vec3>>,
    mut color_buffer: Local<Vec<Vec4>>,
    mut objects_with_trails_query: Query<(&Transform, &mut Trail), Without<TrailsEntityTag>>,
    mesh_handle_query: Query<&mut Mesh2dHandle, With<TrailsEntityTag>>,
) {
    for (transform, mut trail) in objects_with_trails_query.iter_mut() {
        let idx = (trail.1 + 1) % TRAIL_LENGTH;
        trail.1 = idx;
        trail.0[idx] = transform.translation;

        /* let color = Vec4::from(trail.2) * 3.0;
        for i in 0..TRAIL_LENGTH {
            pos_buffer.push(trail.0[idx.wrapping_sub(i) % TRAIL_LENGTH]);
            color_buffer.push(color * (1.0 - i as f32 / TRAIL_LENGTH as f32).powf(16.0));
        } */

        let color = Vec4::from(trail.2) * 6.0; // 3.0
        pos_buffer.push(transform.translation);
        color_buffer.push(color);
        for i in 0..TRAIL_LENGTH-1 {
            pos_buffer.push(trail.0[idx.wrapping_sub(i) % TRAIL_LENGTH]);
            color_buffer.push(color * (1.0 - i as f32 / TRAIL_LENGTH as f32).powf(16.0));

            pos_buffer.push(trail.0[idx.wrapping_sub(i) % TRAIL_LENGTH]);
            color_buffer.push(color * (1.0 - i as f32 / TRAIL_LENGTH as f32).powf(16.0));
        }

        pos_buffer.push(trail.0[idx.wrapping_sub(TRAIL_LENGTH-1) % TRAIL_LENGTH]);
        color_buffer.push(color * (1.0 / TRAIL_LENGTH as f32).powf(16.0));
    }

    let mesh = meshes.get_mut(&mesh_handle_query.single().0).unwrap();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, std::mem::take(&mut *pos_buffer));
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, std::mem::take(&mut *color_buffer));
}
