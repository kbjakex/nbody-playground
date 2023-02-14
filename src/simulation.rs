use bevy::prelude::Vec2;
use turborand::{rng::Rng, TurboRand, SeededCore};

#[derive(Clone, Copy)]
pub struct Planet {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32
}

pub struct Simulation {
    pub planets: Vec<Planet>
}

pub fn update_simulation(sim: &mut Simulation) {
    for i in 0..sim.planets.len() {
        let mut p1 = sim.planets[i];
        for j in i+1..sim.planets.len() {
            let p2 = &mut sim.planets[j];
            
            let d = p2.position - p1.position;
            let y = 0.0006;
            let r = d.length().max(1.0);

            // F = G = y * m1 * m2 / r^2
            // F = m*a <=> a = F/m
            let f = (d / r) * (y * p1.mass * p2.mass / r);
            p1.velocity += f / p1.mass;
            p2.velocity -= f / p2.mass;
        }

        p1.position += p1.velocity;

        /* if p1.position.x < -1920./2. || p1.position.x > 1920./2. {
            p1.velocity.x = -p1.velocity.x;
        }
        if p1.position.y < -1080./2. || p1.position.y > 1080./2. {
            p1.velocity.y = -p1.velocity.y;
        } */

        sim.planets[i] = p1;
    }
}

pub fn init_simulation() -> Simulation {
    let mut planets = Vec::new();
    let rng = Rng::with_seed(5);

    for _ in 0..5 { // 10 planets
        // Generate X and Y coordinates between -100 and 100
        let pos_x = rng.f32_normalized() * 400.0;
        let pos_y = rng.f32_normalized() * 400.0;

        // Generate a random initial velocity
        let vel_x = rng.f32_normalized(); // x direction
        let vel_y = rng.f32_normalized(); // y direction
        let speed = 0.0;//rng.f32() * 5.0 + 1.0; // Speed between 1 and 6 (1+5)

        let mass = rng.f32() * 900.0 + 100.0; // Mass between 100 and 1000

        planets.push(Planet {
            position: Vec2{ x: pos_x, y: pos_y },
            velocity: Vec2{ x: vel_x, y: vel_y }.normalize() * speed,
            mass: 10.0 * mass
        })
    }

    Simulation { planets }
}