use bevy::{prelude::*, DefaultPlugins, sprite::MaterialMesh2dBundle, core_pipeline::bloom::{BloomSettings, BloomCompositeMode}};
use simulation::Simulation;
use trails::{Trail, TrailsPlugin};
use turborand::{rng::Rng, TurboRand};

pub mod simulation;
pub mod trails;


#[derive(Resource)]
pub struct BevySimulation(pub Simulation);

#[derive(Component)]
pub struct PlanetId(pub usize);


pub fn main() {
    App::new()  
        .add_plugins(DefaultPlugins)
        .add_plugin(TrailsPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(init)
        .add_system(tick)
        .run();
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let sim = simulation::init_simulation();

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        /* BloomSettings {
            prefilter_settings: bevy::core_pipeline::bloom::BloomPrefilterSettings { threshold: 0.0, ..Default::default() },
            intensity: 5.0,
            ..default()
        }, */
        BloomSettings {
            prefilter_settings: bevy::core_pipeline::bloom::BloomPrefilterSettings { threshold: 0.0, ..Default::default() },
            intensity: 1.0,
            low_frequency_boost: 0.35,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 1.8,
            composite_mode: BloomCompositeMode::Additive,
            ..default()
        }
        /* BloomSettings {
            composite_mode: BloomCompositeMode::EnergyConserving,
            ..Default::default()
        } */
    ));

    let rng = Rng::new();

    // For each planet, spawn a Bevy Entity at the planet's position:
    for (i, planet) in sim.planets.iter().enumerate() {
        let color = Color::hsl(rng.f32() * 360.0, 1.0, 0.8).as_rgba();

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new((planet.mass / 100.0).sqrt()).into()).into(),
                material: materials.add(color.into()),
                transform: Transform::from_xyz(planet.position.x, planet.position.y, 0.0),
                ..default()
            },         
            //Transform::from_xyz(planet.position.x, planet.position.y, 0.0),
            PlanetId(i),
            Trail::new_at(planet.position.extend(0.0), color)
        ));
    }
    

    commands.insert_resource(BevySimulation(sim));
}

fn tick(
    mut sim: ResMut<BevySimulation>,
    mut query: Query<(&mut Transform, &PlanetId)>
) {
    simulation::update_simulation(&mut sim.0);

    // Pull planet positions from Simulation to Bevy
    for (mut transform, id) in query.iter_mut() {
        let planet = &sim.0.planets[id.0];

        transform.translation.x = planet.position.x;
        transform.translation.y = planet.position.y;
    }
}
