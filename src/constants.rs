// ─────────────────────────────────────────────────────────────────────────────
// Virtual canvas & world dimensions
// ─────────────────────────────────────────────────────────────────────────────
pub const VW: f32 = 3840.0;
pub const VH: f32 = 2160.0;

/// World is large for exploration — 12 screens wide, 8 screens tall.
pub const WORLD_W: f32 = VW * 12.0;
pub const WORLD_H: f32 = VH * 8.0;

// ─────────────────────────────────────────────────────────────────────────────
// Player
// ─────────────────────────────────────────────────────────────────────────────
pub const PLAYER_W: f32 = 80.0;
pub const PLAYER_H: f32 = 100.0;
pub const PLAYER_THRUST: f32 = 1.4;
pub const PLAYER_STRAFE: f32 = 0.8;
pub const PLAYER_REVERSE: f32 = 0.6;
pub const PLAYER_ROTATE_SPEED: f32 = 5.5;
pub const PLAYER_MAX_SPEED: f32 = 26.0;
pub const PLAYER_RESISTANCE: f32 = 0.975;

pub const SPAWN_X: f32 = WORLD_W * 0.5;
pub const SPAWN_Y: f32 = WORLD_H * 0.5;

// ─────────────────────────────────────────────────────────────────────────────
// Health & shield
// ─────────────────────────────────────────────────────────────────────────────
pub const HULL_MAX: f32 = 100.0;
pub const SHIELD_MAX: f32 = 60.0;
pub const SHIELD_REGEN_RATE: f32 = 0.15; // per tick
pub const SHIELD_REGEN_DELAY: u32 = 180; // ticks (~3s at 60fps) before regen starts
pub const LASER_DAMAGE: f32 = 12.0;
pub const ENEMY_LASER_DAMAGE: f32 = 8.0;

// ─────────────────────────────────────────────────────────────────────────────
// Weapons
// ─────────────────────────────────────────────────────────────────────────────
pub const LASER_SPEED: f32 = 48.0;
pub const LASER_W: f32 = 6.0;
pub const LASER_H: f32 = 28.0;
pub const LASER_LIFETIME: u32 = 90;
pub const FIRE_COOLDOWN: u32 = 6;
pub const LASER_POOL_SIZE: usize = 30;

pub const ENEMY_FIRE_COOLDOWN: u32 = 40;
pub const ENEMY_LASER_POOL_SIZE: usize = 20;

// ─────────────────────────────────────────────────────────────────────────────
// Enemies
// ─────────────────────────────────────────────────────────────────────────────
pub const ENEMY_COUNT: usize = 5;
pub const ENEMY_W: f32 = 70.0;
pub const ENEMY_H: f32 = 70.0;
pub const ENEMY_SPEED: f32 = 5.0;
pub const ENEMY_HULL: f32 = 40.0;
pub const ENEMY_DETECT_RANGE: f32 = 2500.0;

// ─────────────────────────────────────────────────────────────────────────────
// Planets
// ─────────────────────────────────────────────────────────────────────────────
pub const PLANET_GRAVITY_TAG: &str = "planet_grav";

pub struct PlanetDef {
    pub name: &'static str,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub strength: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub atmosphere: f32,
}

pub const PLANETS: &[PlanetDef] = &[
    PlanetDef {
        name: "terra",
        x: WORLD_W * 0.20,
        y: WORLD_H * 0.30,
        radius: 400.0,
        strength: 0.45,
        r: 60,
        g: 140,
        b: 80,
        atmosphere: 0.15,
    },
    PlanetDef {
        name: "inferno",
        x: WORLD_W * 0.75,
        y: WORLD_H * 0.25,
        radius: 300.0,
        strength: 0.35,
        r: 200,
        g: 60,
        b: 30,
        atmosphere: 0.10,
    },
    PlanetDef {
        name: "ice",
        x: WORLD_W * 0.15,
        y: WORLD_H * 0.70,
        radius: 350.0,
        strength: 0.4,
        r: 140,
        g: 180,
        b: 220,
        atmosphere: 0.20,
    },
    PlanetDef {
        name: "giant",
        x: WORLD_W * 0.55,
        y: WORLD_H * 0.75,
        radius: 600.0,
        strength: 0.8,
        r: 180,
        g: 140,
        b: 80,
        atmosphere: 0.25,
    },
    PlanetDef {
        name: "tiny",
        x: WORLD_W * 0.85,
        y: WORLD_H * 0.60,
        radius: 180.0,
        strength: 0.2,
        r: 160,
        g: 100,
        b: 180,
        atmosphere: 0.08,
    },
    PlanetDef {
        name: "moon_a",
        x: WORLD_W * 0.22,
        y: WORLD_H * 0.35,
        radius: 120.0,
        strength: 0.1,
        r: 180,
        g: 180,
        b: 170,
        atmosphere: 0.0,
    },
    PlanetDef {
        name: "moon_b",
        x: WORLD_W * 0.58,
        y: WORLD_H * 0.80,
        radius: 140.0,
        strength: 0.15,
        r: 200,
        g: 190,
        b: 160,
        atmosphere: 0.0,
    },
];

// ─────────────────────────────────────────────────────────────────────────────
// Camera
// ─────────────────────────────────────────────────────────────────────────────
pub const CAMERA_LERP: f32 = 0.12;

// ─────────────────────────────────────────────────────────────────────────────
// HUD
// ─────────────────────────────────────────────────────────────────────────────
pub const HUD_BAR_W: f32 = 400.0;
pub const HUD_BAR_H: f32 = 28.0;
pub const HUD_MARGIN: f32 = 30.0;

// ─────────────────────────────────────────────────────────────────────────────
// Minimap
// ─────────────────────────────────────────────────────────────────────────────
pub const MINIMAP_W: f32 = 320.0;
pub const MINIMAP_H: f32 = 200.0;
pub const MINIMAP_MARGIN: f32 = 30.0;

// ─────────────────────────────────────────────────────────────────────────────
// Gravity debug
// ─────────────────────────────────────────────────────────────────────────────
/// Gravity reach multiplier (ring drawn at radius * this)
pub const GRAVITY_FIELD_MULT: f32 = 3.5;
/// Toggle gravity debug with G key
pub const GRAVITY_DEBUG_RING_ALPHA: u8 = 50;

// ─────────────────────────────────────────────────────────────────────────────
// Space debris
// ─────────────────────────────────────────────────────────────────────────────
pub const DEBRIS_COUNT: usize = 40;
pub const DEBRIS_MIN_SIZE: f32 = 16.0;
pub const DEBRIS_MAX_SIZE: f32 = 60.0;
/// How much damage a large debris piece deals on contact with player
pub const DEBRIS_DAMAGE: f32 = 5.0;
