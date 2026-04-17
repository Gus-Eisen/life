use quartz::*;
use ramp::prism;

mod constants;

use constants::*;

fn build_game_scene(_ctx: &mut prism::Context) -> Scene {
    const BG_PAD: f32 = 60.0;
    let bg_w = VW + BG_PAD * 2.0;
    let bg_h = VH + BG_PAD * 2.0;
    let bg_stars = star_field(bg_w as u32, bg_h as u32, 1800, 42);
    let bg = GameObject::build("bg")
        .image(bg_stars)
        .size(bg_w, bg_h)
        .position(0.0, 0.0)
        .layer(0)
        .ignore_zoom()
        .finish();

    let player_img = Image {
        shape: prism::canvas::ShapeType::Rectangle(0.0, (PLAYER_W, PLAYER_H), 0.0),
        image: player_ship_img().into(),
        color: None,
    };
    let player = GameObject::build("player")
        .image(player_img)
        .size(PLAYER_W, PLAYER_H)
        .position(SPAWN_X - PLAYER_W / 2.0, SPAWN_Y - PLAYER_H / 2.0)
        .tag("player")
        .all_gravity_sources()
        .gravity_influence_mult(GRAVITY_FIELD_MULT)
        .auto_align()
        .auto_align_speed(0.3)
        .auto_align_min_depth(0.5)
        .resistance(PLAYER_RESISTANCE, PLAYER_RESISTANCE)
        .player_layer()
        .layer(5)
        .finish();

    let mut planet_objs: Vec<(String, GameObject)> = Vec::new();
    let mut grav_debug_objs: Vec<(String, GameObject)> = Vec::new();
    for p in PLANETS {
        let size = p.radius * 2.0;
        let img = planet_atmosphere(p.radius as u32, p.r, p.g, p.b, p.atmosphere, size);
        let obj = GameObject::build(p.name)
            .image(img)
            .size(size, size)
            .position(p.x - p.radius, p.y - p.radius)
            .tag(PLANET_GRAVITY_TAG)
            .planet(p.radius)
            .gravity_strength(p.strength)
            .static_object()
            .collision_layer(collision_layers::TERRAIN)
            .layer(2)
            .finish();
        planet_objs.push((p.name.to_string(), obj));

        let field_r = p.radius * GRAVITY_FIELD_MULT;
        let field_d = field_r * 2.0;
        let ring = glow_ring(
            field_d - 6.0,
            field_d - 6.0,
            3.0,
            field_r,
            Color(p.r, p.g, p.b, GRAVITY_DEBUG_RING_ALPHA),
        );
        let dbg_name = format!("{}_grav_ring", p.name);
        let mut dbg_obj = GameObject::build(&dbg_name)
            .image(ring)
            .size(field_d, field_d)
            .position(p.x - field_r, p.y - field_r)
            .layer(1)
            .finish();
        dbg_obj.visible = false;
        grav_debug_objs.push((dbg_name, dbg_obj));
    }

    let player_laser_img = Image {
        shape: prism::canvas::ShapeType::Rectangle(0.0, (LASER_W, LASER_H), 0.0),
        image: laser_img(80, 200, 255).into(),
        color: None,
    };
    let mut laser_objs: Vec<(String, GameObject)> = Vec::new();
    for i in 0..LASER_POOL_SIZE {
        let id = format!("plaser_{i}");
        let mut obj = GameObject::build(&id)
            .image(player_laser_img.clone())
            .size(LASER_W, LASER_H)
            .position(-200.0, -200.0)
            .tag("player_laser")
            .projectile_layer()
            .layer(4)
            .finish();
        obj.visible = false;
        laser_objs.push((id, obj));
    }

    let enemy_laser_img = Image {
        shape: prism::canvas::ShapeType::Rectangle(0.0, (LASER_W, LASER_H), 0.0),
        image: laser_img(255, 60, 40).into(),
        color: None,
    };
    let mut elaser_objs: Vec<(String, GameObject)> = Vec::new();
    for i in 0..ENEMY_LASER_POOL_SIZE {
        let id = format!("elaser_{i}");
        let mut obj = GameObject::build(&id)
            .image(enemy_laser_img.clone())
            .size(LASER_W, LASER_H)
            .position(-200.0, -200.0)
            .tag("enemy_laser")
            .collision_layer(collision_layers::PROJECTILE)
            .collision_mask(collision_layers::PLAYER | collision_layers::TERRAIN)
            .layer(4)
            .finish();
        obj.visible = false;
        elaser_objs.push((id, obj));
    }

    let enemy_img = Image {
        shape: prism::canvas::ShapeType::Rectangle(0.0, (ENEMY_W, ENEMY_H), 0.0),
        image: enemy_ship_img().into(),
        color: None,
    };
    let mut seed: u64 = 0xCAFE_BABE;
    let mut enemy_objs: Vec<(String, GameObject)> = Vec::new();
    let mut enemy_states: Vec<EnemyState> = Vec::new();
    for i in 0..ENEMY_COUNT {
        let id = format!("enemy_{i}");
        let ex = lcg_range(&mut seed, WORLD_W * 0.1, WORLD_W * 0.9);
        let ey = lcg_range(&mut seed, WORLD_H * 0.1, WORLD_H * 0.9);
        let obj = GameObject::build(&id)
            .image(enemy_img.clone())
            .size(ENEMY_W, ENEMY_H)
            .position(ex, ey)
            .tag("enemy")
            .enemy_layer()
            .resistance(0.98, 0.98)
            .layer(5)
            .finish();
        enemy_objs.push((id, obj));
        enemy_states.push(EnemyState {
            alive: true,
            hull: ENEMY_HULL,
            fire_cooldown: (lcg_range(&mut seed, 0.0, ENEMY_FIRE_COOLDOWN as f32)) as u32,
            rotation: 0.0,
        });
    }

    let hull_bar = GameObject::build("hull_bar")
        .image(Image {
            shape: prism::canvas::ShapeType::Rectangle(0.0, (HUD_BAR_W, HUD_BAR_H), 0.0),
            image: bar_img(HUD_BAR_W as u32, HUD_BAR_H as u32, 1.0, 60, 200, 80).into(),
            color: None,
        })
        .size(HUD_BAR_W, HUD_BAR_H)
        .position(HUD_MARGIN, HUD_MARGIN)
        .tag("hud")
        .layer(10)
        .ignore_zoom()
        .finish();

    let shield_bar = GameObject::build("shield_bar")
        .image(Image {
            shape: prism::canvas::ShapeType::Rectangle(0.0, (HUD_BAR_W, HUD_BAR_H), 0.0),
            image: bar_img(HUD_BAR_W as u32, HUD_BAR_H as u32, 1.0, 80, 140, 255).into(),
            color: None,
        })
        .size(HUD_BAR_W, HUD_BAR_H)
        .position(HUD_MARGIN, HUD_MARGIN + HUD_BAR_H + 10.0)
        .tag("hud")
        .layer(10)
        .ignore_zoom()
        .finish();

    let hull_label = GameObject::build("hull_label")
        .size(200.0, 40.0)
        .position(HUD_MARGIN, HUD_MARGIN - 36.0)
        .tag("hud")
        .layer(10)
        .ignore_zoom()
        .finish();

    let score_display = GameObject::build("score_display")
        .size(300.0, 50.0)
        .position(VW - 350.0, HUD_MARGIN)
        .tag("hud")
        .layer(10)
        .ignore_zoom()
        .finish();

    let minimap = GameObject::build("minimap")
        .image(Image {
            shape: prism::canvas::ShapeType::Rectangle(0.0, (MINIMAP_W, MINIMAP_H), 0.0),
            image: minimap_bg().into(),
            color: None,
        })
        .size(MINIMAP_W, MINIMAP_H)
        .position(
            VW - MINIMAP_W - MINIMAP_MARGIN,
            VH - MINIMAP_H - MINIMAP_MARGIN,
        )
        .tag("hud")
        .layer(10)
        .ignore_zoom()
        .finish();

    let mut pause_obj = GameObject::build("pause_overlay")
        .image(Image {
            shape: prism::canvas::ShapeType::Rectangle(0.0, (VW, VH), 0.0),
            image: pause_overlay_img().into(),
            color: None,
        })
        .size(VW, VH)
        .position(0.0, 0.0)
        .tag("hud")
        .layer(20)
        .ignore_zoom()
        .finish();
    pause_obj.visible = false;

    let controls_w = 1200.0f32;
    let controls_h = 900.0f32;
    let mut controls_obj = GameObject::build("controls_panel")
        .image(Image {
            shape: prism::canvas::ShapeType::Rectangle(0.0, (controls_w, controls_h), 0.0),
            image: controls_overlay_img().into(),
            color: None,
        })
        .size(controls_w, controls_h)
        .position(VW / 2.0 - controls_w / 2.0, VH / 2.0 - controls_h / 2.0)
        .tag("hud")
        .layer(21)
        .ignore_zoom()
        .finish();
    controls_obj.visible = false;

    let mut grav_debug_text = GameObject::build("grav_debug_text")
        .size(800.0, 50.0)
        .position(HUD_MARGIN, VH - 80.0)
        .tag("hud")
        .layer(10)
        .ignore_zoom()
        .finish();
    grav_debug_text.visible = false;

    let mut debris_objs: Vec<(String, GameObject)> = Vec::new();
    let mut debris_seed: u64 = 0xDEBB_1E50;
    for i in 0..DEBRIS_COUNT {
        let id = format!("debris_{i}");
        let t = lcg(&mut debris_seed);
        let sz = DEBRIS_MIN_SIZE + t * (DEBRIS_MAX_SIZE - DEBRIS_MIN_SIZE);
        let kind = (i % 3) as u8;
        let dimg = Image {
            shape: prism::canvas::ShapeType::Rectangle(0.0, (sz, sz), 0.0),
            image: debris_img(sz as u32, debris_seed.wrapping_add(i as u64), kind).into(),
            color: None,
        };
        let dx = lcg_range(&mut debris_seed, WORLD_W * 0.05, WORLD_W * 0.95);
        let dy = lcg_range(&mut debris_seed, WORLD_H * 0.05, WORLD_H * 0.95);
        let vx = lcg_range(&mut debris_seed, -1.5, 1.5);
        let vy = lcg_range(&mut debris_seed, -1.5, 1.5);
        let mut builder = GameObject::build(&id)
            .image(dimg)
            .size(sz, sz)
            .position(dx, dy)
            .tag("debris")
            .momentum(vx, vy)
            .all_gravity_sources()
            .gravity_influence_mult(GRAVITY_FIELD_MULT)
            .collision_layer(collision_layers::DEFAULT)
            .collision_mask(
                collision_layers::PLAYER
                    | collision_layers::PROJECTILE
                    | collision_layers::TERRAIN
                    | collision_layers::DEFAULT,
            )
            .layer(3);
        builder = match kind {
            1 => builder.heavy(),
            2 => builder.slippery(),
            _ => builder.bouncy(),
        };
        let obj = builder.finish();
        debris_objs.push((id, obj));
    }

    let mut scene = Scene::new("game")
        .with_object("bg", bg)
        .with_object("player", player)
        .with_object("hull_bar", hull_bar)
        .with_object("shield_bar", shield_bar)
        .with_object("hull_label", hull_label)
        .with_object("score_display", score_display)
        .with_object("minimap", minimap)
        .with_object("pause_overlay", pause_obj)
        .with_object("controls_panel", controls_obj)
        .with_object("grav_debug_text", grav_debug_text);

    for (name, obj) in planet_objs {
        scene = scene.with_object(name, obj);
    }
    for (name, obj) in grav_debug_objs {
        scene = scene.with_object(name, obj);
    }
    for (name, obj) in laser_objs {
        scene = scene.with_object(name, obj);
    }
    for (name, obj) in elaser_objs {
        scene = scene.with_object(name, obj);
    }
    for (name, obj) in enemy_objs {
        scene = scene.with_object(name, obj);
    }
    for (name, obj) in debris_objs {
        scene = scene.with_object(name, obj);
    }

    let init_player_lasers: Vec<LaserState> = (0..LASER_POOL_SIZE)
        .map(|_| LaserState {
            alive: false,
            age: 0,
            angle: 0.0,
        })
        .collect();
    let init_enemy_lasers: Vec<LaserState> = (0..ENEMY_LASER_POOL_SIZE)
        .map(|_| LaserState {
            alive: false,
            age: 0,
            angle: 0.0,
        })
        .collect();

    let state = Arc::new(Mutex::new(State {
        px: SPAWN_X,
        py: SPAWN_Y,
        vx: 0.0,
        vy: 0.0,
        rotation: 0.0,
        hull: HULL_MAX,
        shield: SHIELD_MAX,
        shield_regen_timer: 0,
        fire_cooldown: 0,
        score: 0,
        ticks: 0,
        player_lasers: init_player_lasers,
        enemy_lasers: init_enemy_lasers,
        enemies: enemy_states,
        paused: false,
        show_controls: false,
        game_over: false,
        show_gravity_debug: false,
        seed: 0xDEAD_BEEF,
    }));

    let scene = scene.on_enter({
        let state = state.clone();
        move |canvas| {
            let mut cam = Camera::new((WORLD_W, WORLD_H), (VW, VH));
            cam.follow(Some(Target::name("player")));
            cam.lerp_speed = CAMERA_LERP;
            cam.center_on(SPAWN_X, SPAWN_Y);
            canvas.set_camera(cam);

            const BG_PAD: f32 = 60.0;
            if let Some(obj) = canvas.get_game_object_mut("bg") {
                obj.position = (-BG_PAD, -BG_PAD);
            }

            canvas.run(Action::enable_crystalline());

            let mut exhaust = Emitter::thruster_exhaust((0.0, 0.0));
            exhaust.name = "player_exhaust".into();
            exhaust.size = 6.0;
            exhaust.rate = 120.0;
            exhaust.lifetime = 0.6;
            exhaust.render_layer = 4;
            canvas.run(Action::spawn_emitter(exhaust));
            canvas.run(Action::attach_emitter_at(
                "player_exhaust",
                Target::name("player"),
                Location::on_target(
                    Target::name("player"),
                    Anchor { x: 0.5, y: 0.85 },
                    (0.0, 0.0),
                ),
            ));

            for p in PLANETS {
                let emitter_name = format!("{}_aura", p.name);
                let mut aura = EmitterBuilder::new(&emitter_name)
                    .origin(p.x, p.y)
                    .rate(8.0 + p.radius * 0.02)
                    .lifetime(2.5)
                    .velocity(0.0, 0.0)
                    .spread(p.radius * 0.8, p.radius * 0.8)
                    .size(4.0 + p.atmosphere * 20.0)
                    .color(p.r, p.g, p.b, 40)
                    .gravity_scale(0.0)
                    .render_layer(1)
                    .build();
                aura.render_layer = 1;
                canvas.run(Action::spawn_emitter(aura));
            }

            let st_pause = state.clone();
            canvas.on_key_press(move |c, key| {
                let is_pause = matches!(key, Key::Character(ch) if ch.as_str() == "p");
                if !is_pause {
                    return;
                }
                let mut s = st_pause.lock().unwrap();
                if s.game_over {
                    return;
                }
                if s.paused {
                    s.paused = false;
                    s.show_controls = false;
                    drop(s);
                    c.resume();
                    if let Some(obj) = c.get_game_object_mut("pause_overlay") {
                        obj.visible = false;
                    }
                    if let Some(obj) = c.get_game_object_mut("controls_panel") {
                        obj.visible = false;
                    }
                } else {
                    s.paused = true;
                    drop(s);
                    if let Some(obj) = c.get_game_object_mut("pause_overlay") {
                        obj.position = (0.0, 0.0);
                        obj.visible = true;
                    }
                    c.pause();
                }
            });

            let st_ctrl = state.clone();
            canvas.on_key_press(move |c, key| {
                let is_c = matches!(key, Key::Character(ch) if ch.as_str() == "c");
                if !is_c {
                    return;
                }
                let mut s = st_ctrl.lock().unwrap();
                if !s.paused {
                    return;
                }
                s.show_controls = !s.show_controls;
                let show = s.show_controls;
                drop(s);
                if let Some(obj) = c.get_game_object_mut("controls_panel") {
                    obj.position = (VW / 2.0 - 600.0, VH / 2.0 - 450.0);
                    obj.visible = show;
                }
            });

            let st_grav = state.clone();
            canvas.on_key_press(move |c, key| {
                let is_g = matches!(key, Key::Character(ch) if ch.as_str() == "g");
                if !is_g {
                    return;
                }
                let mut s = st_grav.lock().unwrap();
                s.show_gravity_debug = !s.show_gravity_debug;
                let show = s.show_gravity_debug;
                drop(s);
                for p in PLANETS {
                    let ring_name = format!("{}_grav_ring", p.name);
                    if let Some(obj) = c.get_game_object_mut(&ring_name) {
                        obj.visible = show;
                    }
                }
                if let Some(obj) = c.get_game_object_mut("grav_debug_text") {
                    obj.visible = show;
                }
            });

            let st_fire = state.clone();
            canvas.on_key_press(move |c, key| {
                if !matches!(key, Key::Named(NamedKey::Space)) {
                    return;
                }
                fire_player_laser(&st_fire, c);
            });

            canvas.on_key_press(|c, key| match key {
                Key::Character(ch) if ch.as_str() == "z" => {
                    c.smooth_zoom(c.get_zoom() + 0.2);
                }
                Key::Character(ch) if ch.as_str() == "x" => {
                    c.smooth_zoom((c.get_zoom() - 0.2).max(0.3));
                }
                _ => {}
            });

            let st_fire2 = state.clone();
            canvas.on_mouse_press(move |c, btn, _pos| {
                if btn != MouseButton::Left {
                    return;
                }
                fire_player_laser(&st_fire2, c);
            });

            let hud_font = Arc::new(
                Font::from_bytes(
                    &std::fs::read(
                        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/font.ttf"),
                    )
                    .expect("font not found"),
                )
                .expect("invalid font"),
            );

            let st_tick = state.clone();
            canvas.on_update(move |c| {
                let mut s = st_tick.lock().unwrap();
                if s.game_over || s.paused {
                    return;
                }
                s.ticks += 1;

                if let Some(obj) = c.get_game_object("player") {
                    s.px = obj.position.0 + PLAYER_W / 2.0;
                    s.py = obj.position.1 + PLAYER_H / 2.0;
                    s.vx = obj.momentum.0;
                    s.vy = obj.momentum.1;
                    s.rotation = obj.rotation;
                }

                if c.key("q") {
                    s.rotation -= PLAYER_ROTATE_SPEED;
                }
                if c.key("e") || c.key("r") {
                    s.rotation += PLAYER_ROTATE_SPEED;
                }
                s.rotation = ((s.rotation % 360.0) + 360.0) % 360.0;

                let (fwd_x, fwd_y) = dir_from_angle(s.rotation);
                let (right_x, right_y) = (-fwd_y, fwd_x);

                if c.key("w") || c.key("up") {
                    s.vx += fwd_x * PLAYER_THRUST;
                    s.vy += fwd_y * PLAYER_THRUST;
                }
                if c.key("s") || c.key("down") {
                    s.vx -= fwd_x * PLAYER_REVERSE;
                    s.vy -= fwd_y * PLAYER_REVERSE;
                }
                if c.key("d") || c.key("right") {
                    s.vx += right_x * PLAYER_STRAFE;
                    s.vy += right_y * PLAYER_STRAFE;
                }
                if c.key("a") || c.key("left") {
                    s.vx -= right_x * PLAYER_STRAFE;
                    s.vy -= right_y * PLAYER_STRAFE;
                }

                let spd = (s.vx * s.vx + s.vy * s.vy).sqrt();
                if spd > PLAYER_MAX_SPEED {
                    let scale = PLAYER_MAX_SPEED / spd;
                    s.vx *= scale;
                    s.vy *= scale;
                }

                if s.px < 0.0 {
                    s.px += WORLD_W;
                }
                if s.px > WORLD_W {
                    s.px -= WORLD_W;
                }
                if s.py < 0.0 {
                    s.py += WORLD_H;
                }
                if s.py > WORLD_H {
                    s.py -= WORLD_H;
                }

                if let Some(obj) = c.get_game_object_mut("player") {
                    obj.position = (s.px - PLAYER_W / 2.0, s.py - PLAYER_H / 2.0);
                    obj.rotation = s.rotation;
                    obj.momentum = (s.vx, s.vy);
                }

                if s.fire_cooldown > 0 {
                    s.fire_cooldown -= 1;
                }

                if s.shield < SHIELD_MAX {
                    s.shield_regen_timer += 1;
                    if s.shield_regen_timer >= SHIELD_REGEN_DELAY {
                        s.shield = (s.shield + SHIELD_REGEN_RATE).min(SHIELD_MAX);
                    }
                }

                for i in 0..LASER_POOL_SIZE {
                    let ls = &mut s.player_lasers[i];
                    if !ls.alive {
                        continue;
                    }
                    ls.age += 1;
                    if ls.age > LASER_LIFETIME {
                        ls.alive = false;
                        let id = format!("plaser_{i}");
                        if let Some(obj) = c.get_game_object_mut(&id) {
                            obj.visible = false;
                            obj.position = (-200.0, -200.0);
                        }
                        continue;
                    }
                    let (dx, dy) = dir_from_angle(ls.angle);
                    let id = format!("plaser_{i}");
                    if let Some(obj) = c.get_game_object_mut(&id) {
                        obj.position.0 += dx * LASER_SPEED;
                        obj.position.1 += dy * LASER_SPEED;
                    }
                }

                for i in 0..ENEMY_LASER_POOL_SIZE {
                    let ls = &mut s.enemy_lasers[i];
                    if !ls.alive {
                        continue;
                    }
                    ls.age += 1;
                    if ls.age > LASER_LIFETIME {
                        ls.alive = false;
                        let id = format!("elaser_{i}");
                        if let Some(obj) = c.get_game_object_mut(&id) {
                            obj.visible = false;
                            obj.position = (-200.0, -200.0);
                        }
                        continue;
                    }
                    let (dx, dy) = dir_from_angle(ls.angle);
                    let id = format!("elaser_{i}");
                    if let Some(obj) = c.get_game_object_mut(&id) {
                        obj.position.0 += dx * LASER_SPEED * 0.8;
                        obj.position.1 += dy * LASER_SPEED * 0.8;
                    }
                }

                let player_pos = (s.px, s.py);
                let mut enemy_fire_requests: Vec<(usize, f32, f32, f32)> = Vec::new();

                for i in 0..ENEMY_COUNT {
                    if !s.enemies[i].alive {
                        continue;
                    }
                    let eid = format!("enemy_{i}");
                    let (ex, ey) = if let Some(obj) = c.get_game_object(&eid) {
                        (
                            obj.position.0 + ENEMY_W / 2.0,
                            obj.position.1 + ENEMY_H / 2.0,
                        )
                    } else {
                        continue;
                    };

                    let dx = player_pos.0 - ex;
                    let dy = player_pos.1 - ey;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist < ENEMY_DETECT_RANGE {
                        let target_angle = dy.atan2(dx).to_degrees() + 90.0;
                        let target_angle = ((target_angle % 360.0) + 360.0) % 360.0;
                        let mut diff = target_angle - s.enemies[i].rotation;
                        if diff > 180.0 {
                            diff -= 360.0;
                        }
                        if diff < -180.0 {
                            diff += 360.0;
                        }
                        s.enemies[i].rotation += diff.clamp(-2.5, 2.5);
                        s.enemies[i].rotation = ((s.enemies[i].rotation % 360.0) + 360.0) % 360.0;

                        let (mx, my) = dir_from_angle(s.enemies[i].rotation);
                        if let Some(obj) = c.get_game_object_mut(&eid) {
                            obj.position.0 += mx * ENEMY_SPEED;
                            obj.position.1 += my * ENEMY_SPEED;
                            obj.rotation = s.enemies[i].rotation;
                        }

                        s.enemies[i].fire_cooldown = s.enemies[i].fire_cooldown.saturating_sub(1);
                        if s.enemies[i].fire_cooldown == 0 && dist < ENEMY_DETECT_RANGE * 0.7 {
                            enemy_fire_requests.push((i, ex, ey, s.enemies[i].rotation));
                            s.enemies[i].fire_cooldown = ENEMY_FIRE_COOLDOWN;
                        }
                    } else {
                        let (mx, my) = dir_from_angle(s.enemies[i].rotation);
                        if let Some(obj) = c.get_game_object_mut(&eid) {
                            obj.position.0 += mx * ENEMY_SPEED * 0.3;
                            obj.position.1 += my * ENEMY_SPEED * 0.3;
                        }
                        s.enemies[i].rotation += 0.3;
                    }
                }

                for (_, ex, ey, angle) in enemy_fire_requests {
                    fire_enemy_laser(&mut s, c, ex, ey, angle);
                }

                for li in 0..LASER_POOL_SIZE {
                    if !s.player_lasers[li].alive {
                        continue;
                    }
                    let lid = format!("plaser_{li}");
                    let (lx, ly) = if let Some(obj) = c.get_game_object(&lid) {
                        (obj.position.0, obj.position.1)
                    } else {
                        continue;
                    };

                    for ei in 0..ENEMY_COUNT {
                        if !s.enemies[ei].alive {
                            continue;
                        }
                        let eid = format!("enemy_{ei}");
                        let (ex, ey) = if let Some(obj) = c.get_game_object(&eid) {
                            (obj.position.0, obj.position.1)
                        } else {
                            continue;
                        };

                        if lx > ex && lx < ex + ENEMY_W && ly > ey && ly < ey + ENEMY_H {
                            s.player_lasers[li].alive = false;
                            if let Some(obj) = c.get_game_object_mut(&lid) {
                                obj.visible = false;
                                obj.position = (-200.0, -200.0);
                            }
                            s.enemies[ei].hull -= LASER_DAMAGE;
                            if s.enemies[ei].hull <= 0.0 {
                                s.enemies[ei].alive = false;
                                s.score += 100;
                                let mut expl =
                                    Emitter::explosion((ex + ENEMY_W / 2.0, ey + ENEMY_H / 2.0));
                                expl.render_layer = 6;
                                c.spawn_particle_burst(&expl, 40);
                                if let Some(obj) = c.get_game_object_mut(&eid) {
                                    obj.visible = false;
                                    obj.position = (-500.0, -500.0);
                                }
                            } else {
                                let mut dmg = Emitter::damage_burst((lx, ly));
                                dmg.render_layer = 6;
                                c.spawn_particle_burst(&dmg, 15);
                                let mut spark = Emitter::sparks((lx, ly));
                                spark.render_layer = 6;
                                c.spawn_particle_burst(&spark, 8);
                            }
                            break;
                        }
                    }
                }

                for li in 0..ENEMY_LASER_POOL_SIZE {
                    if !s.enemy_lasers[li].alive {
                        continue;
                    }
                    let lid = format!("elaser_{li}");
                    let (lx, ly) = if let Some(obj) = c.get_game_object(&lid) {
                        (obj.position.0, obj.position.1)
                    } else {
                        continue;
                    };

                    let half_w = PLAYER_W / 2.0;
                    let half_h = PLAYER_H / 2.0;
                    if lx > s.px - half_w
                        && lx < s.px + half_w
                        && ly > s.py - half_h
                        && ly < s.py + half_h
                    {
                        s.enemy_lasers[li].alive = false;
                        if let Some(obj) = c.get_game_object_mut(&lid) {
                            obj.visible = false;
                            obj.position = (-200.0, -200.0);
                        }

                        if s.shield > 0.0 {
                            let absorbed = ENEMY_LASER_DAMAGE.min(s.shield);
                            s.shield -= absorbed;
                            let remaining = ENEMY_LASER_DAMAGE - absorbed;
                            if remaining > 0.0 {
                                s.hull -= remaining;
                            }
                        } else {
                            s.hull -= ENEMY_LASER_DAMAGE;
                        }
                        s.shield_regen_timer = 0;

                        let mut dmg = Emitter::damage_burst((s.px, s.py));
                        dmg.render_layer = 6;
                        c.spawn_particle_burst(&dmg, 15);
                        let mut spark = Emitter::sparks((s.px, s.py));
                        spark.render_layer = 6;
                        c.spawn_particle_burst(&spark, 8);

                        if s.hull <= 0.0 {
                            s.hull = 0.0;
                            s.game_over = true;
                            let mut expl = Emitter::explosion((s.px, s.py));
                            expl.render_layer = 6;
                            c.spawn_particle_burst(&expl, 40);
                            if let Some(obj) = c.get_game_object_mut("player") {
                                obj.visible = false;
                            }
                        }
                    }
                }

                for di in 0..DEBRIS_COUNT {
                    let did = format!("debris_{di}");
                    let (dpos, dsz) = if let Some(obj) = c.get_game_object(&did) {
                        if !obj.visible {
                            continue;
                        }
                        (obj.position, obj.size)
                    } else {
                        continue;
                    };

                    for li in 0..LASER_POOL_SIZE {
                        if !s.player_lasers[li].alive {
                            continue;
                        }
                        let lid = format!("plaser_{li}");
                        let (lx, ly) = if let Some(obj) = c.get_game_object(&lid) {
                            (obj.position.0, obj.position.1)
                        } else {
                            continue;
                        };

                        if lx > dpos.0 && lx < dpos.0 + dsz.0 && ly > dpos.1 && ly < dpos.1 + dsz.1
                        {
                            s.player_lasers[li].alive = false;
                            if let Some(obj) = c.get_game_object_mut(&lid) {
                                obj.visible = false;
                                obj.position = (-200.0, -200.0);
                            }
                            let cx = dpos.0 + dsz.0 / 2.0;
                            let cy = dpos.1 + dsz.1 / 2.0;
                            let mut expl = Emitter::asteroid_debris((cx, cy));
                            expl.render_layer = 6;
                            c.spawn_particle_burst(&expl, 12);
                            let mut dmg = Emitter::damage_burst((cx, cy));
                            dmg.render_layer = 6;
                            c.spawn_particle_burst(&dmg, 15);
                            if let Some(obj) = c.get_game_object_mut(&did) {
                                obj.visible = false;
                                obj.position = (-9999.0, -9999.0);
                            }
                            s.score += 10;
                            break;
                        }
                    }
                }

                if !s.game_over {
                    for di in 0..DEBRIS_COUNT {
                        let did = format!("debris_{di}");
                        let (dpos, dsz) = if let Some(obj) = c.get_game_object(&did) {
                            if !obj.visible {
                                continue;
                            }
                            (obj.position, obj.size)
                        } else {
                            continue;
                        };

                        let dcx = dpos.0 + dsz.0 / 2.0;
                        let dcy = dpos.1 + dsz.1 / 2.0;
                        if (s.px - dcx).abs() < PLAYER_W / 2.0 + dsz.0 / 2.0
                            && (s.py - dcy).abs() < PLAYER_H / 2.0 + dsz.1 / 2.0
                        {
                            let dmg_amt = DEBRIS_DAMAGE * (dsz.0 / DEBRIS_MAX_SIZE);
                            if s.shield > 0.0 {
                                let absorbed = dmg_amt.min(s.shield);
                                s.shield -= absorbed;
                                let remaining = dmg_amt - absorbed;
                                if remaining > 0.0 {
                                    s.hull -= remaining;
                                }
                            } else {
                                s.hull -= dmg_amt;
                            }
                            s.shield_regen_timer = 0;
                            let mut spark = Emitter::sparks((s.px, s.py));
                            spark.render_layer = 6;
                            c.spawn_particle_burst(&spark, 6);
                            if let Some(obj) = c.get_game_object_mut(&did) {
                                obj.visible = false;
                                obj.position = (-9999.0, -9999.0);
                            }
                            if s.hull <= 0.0 {
                                s.hull = 0.0;
                                s.game_over = true;
                                let mut expl = Emitter::explosion((s.px, s.py));
                                expl.render_layer = 6;
                                c.spawn_particle_burst(&expl, 40);
                                if let Some(obj) = c.get_game_object_mut("player") {
                                    obj.visible = false;
                                }
                                break;
                            }
                        }
                    }
                }

                const BG_PAD: f32 = 60.0;
                if let Some(obj) = c.get_game_object_mut("bg") {
                    obj.position = (-BG_PAD, -BG_PAD);
                }

                // ── HUD ──────────────────────────────────────────────────
                if let Some(obj) = c.get_game_object_mut("hull_bar") {
                    obj.position = (HUD_MARGIN, HUD_MARGIN);
                    let fill = s.hull / HULL_MAX;
                    let r = (60.0 + 195.0 * (1.0 - fill)) as u8;
                    let g = (200.0 * fill) as u8;
                    obj.set_image(Image {
                        shape: prism::canvas::ShapeType::Rectangle(
                            0.0,
                            (HUD_BAR_W, HUD_BAR_H),
                            0.0,
                        ),
                        image: bar_img(HUD_BAR_W as u32, HUD_BAR_H as u32, fill, r, g, 80).into(),
                        color: None,
                    });
                }

                if let Some(obj) = c.get_game_object_mut("shield_bar") {
                    obj.position = (HUD_MARGIN, HUD_MARGIN + HUD_BAR_H + 10.0);
                    let fill = s.shield / SHIELD_MAX;
                    obj.set_image(Image {
                        shape: prism::canvas::ShapeType::Rectangle(
                            0.0,
                            (HUD_BAR_W, HUD_BAR_H),
                            0.0,
                        ),
                        image: bar_img(HUD_BAR_W as u32, HUD_BAR_H as u32, fill, 80, 140, 255)
                            .into(),
                        color: None,
                    });
                }

                // Build text before mutable borrows
                let score_text = c.make_text(
                    format!("SCORE: {}", s.score),
                    42.0,
                    Color(255, 255, 255, 255),
                    Align::Right,
                    hud_font.clone(),
                );
                if let Some(obj) = c.get_game_object_mut("score_display") {
                    obj.position = (VW - 350.0, HUD_MARGIN);
                    obj.set_drawable(Box::new(score_text));
                }

                let hull_text = c.make_text(
                    format!("HULL: {:.0}  |  SHIELD: {:.0}", s.hull, s.shield),
                    30.0,
                    Color(200, 220, 240, 255),
                    Align::Left,
                    hud_font.clone(),
                );
                if let Some(obj) = c.get_game_object_mut("hull_label") {
                    obj.position = (HUD_MARGIN, HUD_MARGIN - 36.0);
                    obj.set_drawable(Box::new(hull_text));
                }

                if s.show_gravity_debug {
                    let dominant = c.get_dominant_planet("player").unwrap_or("none");
                    let in_range = c.planets_in_range("player");
                    let grav_msg = if in_range.is_empty() {
                        "Gravity: none".to_string()
                    } else {
                        format!("Dominant: {} | In range: {}", dominant, in_range.join(", "))
                    };
                    let grav_text = c.make_text(
                        grav_msg,
                        22.0,
                        Color(180, 255, 180, 200),
                        Align::Left,
                        hud_font.clone(),
                    );
                    if let Some(obj) = c.get_game_object_mut("grav_debug_text") {
                        obj.position = (HUD_MARGIN, VH - 80.0);
                        obj.set_drawable(Box::new(grav_text));
                    }
                }

                // ── Minimap ──────────────────────────────────────────────
                let mw = MINIMAP_W as u32;
                let mh = MINIMAP_H as u32;
                let mut mimg = minimap_bg();

                for p in PLANETS {
                    let mx = ((p.x / WORLD_W) * mw as f32) as i32;
                    let my = ((p.y / WORLD_H) * mh as f32) as i32;
                    let mr = ((p.radius / WORLD_W) * mw as f32).max(2.0) as i32;
                    for dy in -mr..=mr {
                        for dx in -mr..=mr {
                            if dx * dx + dy * dy <= mr * mr {
                                let px = mx + dx;
                                let py = my + dy;
                                if px >= 0 && px < mw as i32 && py >= 0 && py < mh as i32 {
                                    mimg.put_pixel(
                                        px as u32,
                                        py as u32,
                                        image::Rgba([p.r, p.g, p.b, 200]),
                                    );
                                }
                            }
                        }
                    }
                }

                let pmx = ((s.px / WORLD_W) * mw as f32) as i32;
                let pmy = ((s.py / WORLD_H) * mh as f32) as i32;
                for dy in -2i32..=2 {
                    for dx in -2i32..=2 {
                        let px = pmx + dx;
                        let py = pmy + dy;
                        if px >= 0 && px < mw as i32 && py >= 0 && py < mh as i32 {
                            mimg.put_pixel(px as u32, py as u32, image::Rgba([80, 200, 255, 255]));
                        }
                    }
                }

                for i in 0..ENEMY_COUNT {
                    if !s.enemies[i].alive {
                        continue;
                    }
                    let eid = format!("enemy_{i}");
                    if let Some(eobj) = c.get_game_object(&eid) {
                        let emx =
                            (((eobj.position.0 + ENEMY_W / 2.0) / WORLD_W) * mw as f32) as i32;
                        let emy =
                            (((eobj.position.1 + ENEMY_H / 2.0) / WORLD_H) * mh as f32) as i32;
                        for dy in -1i32..=1 {
                            for dx in -1i32..=1 {
                                let px = emx + dx;
                                let py = emy + dy;
                                if px >= 0 && px < mw as i32 && py >= 0 && py < mh as i32 {
                                    mimg.put_pixel(
                                        px as u32,
                                        py as u32,
                                        image::Rgba([255, 60, 60, 255]),
                                    );
                                }
                            }
                        }
                    }
                }

                for di in 0..DEBRIS_COUNT {
                    let did = format!("debris_{di}");
                    if let Some(dobj) = c.get_game_object(&did) {
                        if !dobj.visible {
                            continue;
                        }
                        let dmx =
                            (((dobj.position.0 + dobj.size.0 / 2.0) / WORLD_W) * mw as f32) as i32;
                        let dmy =
                            (((dobj.position.1 + dobj.size.1 / 2.0) / WORLD_H) * mh as f32) as i32;
                        if dmx >= 0 && dmx < mw as i32 && dmy >= 0 && dmy < mh as i32 {
                            mimg.put_pixel(
                                dmx as u32,
                                dmy as u32,
                                image::Rgba([160, 150, 140, 180]),
                            );
                        }
                    }
                }

                if let Some(obj) = c.get_game_object_mut("minimap") {
                    obj.position = (
                        VW - MINIMAP_W - MINIMAP_MARGIN,
                        VH - MINIMAP_H - MINIMAP_MARGIN,
                    );
                    obj.set_image(Image {
                        shape: prism::canvas::ShapeType::Rectangle(
                            0.0,
                            (MINIMAP_W, MINIMAP_H),
                            0.0,
                        ),
                        image: mimg.into(),
                        color: None,
                    });
                }

                if s.game_over {
                    let over_text = c.make_text(
                        "GAME OVER - Press R to restart".to_string(),
                        52.0,
                        Color(255, 80, 80, 255),
                        Align::Left,
                        hud_font.clone(),
                    );
                    if let Some(obj) = c.get_game_object_mut("hull_label") {
                        obj.set_drawable(Box::new(over_text));
                    }
                }
            });

            let st_restart = state.clone();
            canvas.on_key_press(move |c, key| {
                let is_r = matches!(key, Key::Character(ch) if ch.as_str() == "r");
                if !is_r {
                    return;
                }
                let s = st_restart.lock().unwrap();
                if !s.game_over {
                    return;
                }
                drop(s);
                c.load_scene("game");
            });
        }
    });

    scene
}

pub struct App;

impl App {
    #![allow(clippy::new_ret_no_self)]
    fn new(ctx: &mut Context, _assets: Assets) -> impl Drawable {
        let mut canvas = Canvas::new(ctx, CanvasMode::Landscape);
        // canvas.add_scene(build_game_scene(ctx));
        // canvas.load_scene("game");
        canvas
    }
}

ramp::run! { |ctx: &mut Context, assets: Assets| { App::new(ctx, assets) } }
