use turbo::*;

// Constants
const GRAVITY: f32 = 0.8;
const JUMP_POWER: f32 = -9.5;
const MIN_JUMP_POWER: f32 = -5.0;
const MAX_JUMP_HOLD: u32 = 12;
const PLAYER_X: f32 = 40.0;
const GROUND_Y: f32 = 100.0;
const SCROLL_SPEED_BASE: f32 = 3.0;
const OBSTACLE_SPAWN_INTERVAL: u32 = 90;
const PARTICLE_SPAWN_RATE: u32 = 2;

// Colors - Christmas palette
const COLOR_SKY_TOP: u32 = 0x001133ff;      // Dark blue night
const COLOR_SKY_MID: u32 = 0x003366ff;      // Medium blue
const COLOR_SKY_BOTTOM: u32 = 0x4488aaff;   // Light blue snowy
const COLOR_AURORA_RED: u32 = 0xff0000ff;   // Christmas red
const COLOR_AURORA_GREEN: u32 = 0x00ff00ff; // Christmas green
const COLOR_AURORA_GOLD: u32 = 0xffd700ff;  // Christmas gold
const COLOR_PLAYER: u32 = 0xffffffff;
const COLOR_PLAYER_GLOW: u32 = 0x00ffffff;
const COLOR_OBSTACLE: u32 = 0x88ffffff;
const COLOR_STAR: u32 = 0xffff00ff;
const COLOR_PRESENT: u32 = 0xff0000ff;
const COLOR_SNOWFLAKE: u32 = 0xeeeeeeff;
const COLOR_TEXT: u32 = 0xffffffff;
const COLOR_UI_BG: u32 = 0x000000aa;
const COLOR_PERFECT: u32 = 0xffff00ff;
const COLOR_SHIELD: u32 = 0x00ffffff;
const COLOR_TREE_TRUNK: u32 = 0x663300ff;
const COLOR_TREE_GREEN: u32 = 0x00aa00ff;

// Game mode enum
#[turbo::serialize]
#[derive(PartialEq)]
enum GameMode {
    Title,
    Playing,
    GameOver,
}

// Obstacle type enum
#[turbo::serialize]
#[derive(PartialEq)]
enum ObstacleType {
    Crystal,
    FloatingRock,
    CandyCane,
}

// Power-up types
#[turbo::serialize]
#[derive(PartialEq)]
enum PowerUpType {
    Shield,
    SlowMo,
    Magnet,
    DoublePoints,
}

// Obstacle struct
#[turbo::serialize]
struct Obstacle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    obstacle_type: ObstacleType,
    passed: bool,
}

impl Obstacle {
    fn new(x: f32, obstacle_type: ObstacleType, rng: &mut u32) -> Self {
        let (y, width, height) = match obstacle_type {
            ObstacleType::Crystal => (GROUND_Y, 12.0, 20.0),
            ObstacleType::FloatingRock => (GROUND_Y - 10.0 - (rand_quick(rng) % 25) as f32, 16.0, 16.0),  // Spawn closer to ground (65-90 range)
            ObstacleType::CandyCane => (GROUND_Y, 16.0, 32.0),
        };

        Self {
            x,
            y,
            width,
            height,
            obstacle_type,
            passed: false,
        }
    }
}

// Star struct
#[turbo::serialize]
struct Star {
    x: f32,
    y: f32,
    collected: bool,
}

// Power-up struct
#[turbo::serialize]
struct PowerUp {
    x: f32,
    y: f32,
    powerup_type: PowerUpType,
    collected: bool,
}

// Particle struct
#[turbo::serialize]
struct Particle {
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
    life: u32,
    max_life: u32,
    particle_type: ParticleType,
    size: f32,
}

// Particle types
#[turbo::serialize]
#[derive(PartialEq)]
enum ParticleType {
    Snowflake,
    Sparkle,
    Trail,
}

// Aurora wave struct
#[turbo::serialize]
struct AuroraWave {
    offset: f32,
    frequency: f32,
    amplitude: f32,
    speed: f32,
    color: u32,
    y_base: f32,
}

// Floating text for feedback
#[turbo::serialize]
struct FloatingText {
    x: f32,
    y: f32,
    text: String,
    life: u32,
    color: u32,
}

// Main game state
#[turbo::game]
struct GameState {
    mode: GameMode,
    
    // Player
    player_x: f32,
    player_y: f32,
    player_velocity_y: f32,
    player_is_jumping: bool,
    player_jump_hold_frames: u32,
    player_is_on_ground: bool,
    player_rotation: f32,
    
    // Game entities
    obstacles: Vec<Obstacle>,
    stars: Vec<Star>,
    powerups: Vec<PowerUp>,
    particles: Vec<Particle>,
    aurora_waves: Vec<AuroraWave>,
    floating_texts: Vec<FloatingText>,
    
    // Gameplay
    score: u32,
    high_score: u32,
    combo: u32,
    max_combo: u32,
    distance: f32,
    scroll_speed: f32,
    
    // Power-up effects
    has_shield: bool,
    shield_timer: u32,
    slow_mo_timer: u32,
    magnet_timer: u32,
    double_points_timer: u32,
    
    // Streak system
    perfect_landings: u32,
    near_miss_count: u32,
    
    // Timing
    frame: u32,
    spawn_timer: u32,
    particle_timer: u32,
    snowflake_timer: u32,
    
    // Effects
    shake_frames: u32,
    shake_intensity: f32,
    title_pulse: f32,
    screen_flash: u32,
    
    // Achievements
    stars_collected: u32,
    total_distance: f32,
    
    // RNG
    rng_state: u32,
}

impl GameState {
    fn new() -> Self {
        let mut rng = 12345u32;
        
        Self {
            mode: GameMode::Title,
            
            // Player
            player_x: PLAYER_X,
            player_y: GROUND_Y,
            player_velocity_y: 0.0,
            player_is_jumping: false,
            player_jump_hold_frames: 0,
            player_is_on_ground: true,
            player_rotation: 0.0,
            
            // Entities
            obstacles: Vec::new(),
            stars: Vec::new(),
            powerups: Vec::new(),
            particles: Vec::new(),
            floating_texts: Vec::new(),
            aurora_waves: vec![
                AuroraWave {
                    offset: (rand_quick(&mut rng) % 100) as f32,
                    frequency: 0.05,
                    amplitude: 15.0,
                    speed: 0.5,
                    color: COLOR_AURORA_RED,
                    y_base: 30.0,
                },
                AuroraWave {
                    offset: (rand_quick(&mut rng) % 100) as f32,
                    frequency: 0.03,
                    amplitude: 20.0,
                    speed: 0.3,
                    color: COLOR_AURORA_GREEN,
                    y_base: 50.0,
                },
                AuroraWave {
                    offset: (rand_quick(&mut rng) % 100) as f32,
                    frequency: 0.04,
                    amplitude: 10.0,
                    speed: 0.4,
                    color: COLOR_AURORA_GOLD,
                    y_base: 70.0,
                },
            ],
            
            // Gameplay
            score: 0,
            high_score: 0,
            combo: 0,
            max_combo: 0,
            distance: 0.0,
            scroll_speed: SCROLL_SPEED_BASE,
            
            // Power-ups
            has_shield: false,
            shield_timer: 0,
            slow_mo_timer: 0,
            magnet_timer: 0,
            double_points_timer: 0,
            
            // Streaks
            perfect_landings: 0,
            near_miss_count: 0,
            
            // Timing
            frame: 0,
            spawn_timer: 0,
            particle_timer: 0,
            snowflake_timer: 0,
            
            // Effects
            shake_frames: 0,
            shake_intensity: 0.0,
            title_pulse: 0.0,
            screen_flash: 0,
            
            // Achievements
            stars_collected: 0,
            total_distance: 0.0,
            
            // RNG
            rng_state: rng,
        }
    }
    
    fn update(&mut self) {
        self.frame += 1;
        self.rng_state = self.rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
        
        let gp = gamepad::get(0);
        
        match self.mode {
            GameMode::Title => self.update_title(&gp),
            GameMode::Playing => self.update_playing(&gp),
            GameMode::GameOver => self.update_game_over(&gp),
        }
        
        self.render();
    }
    
    fn update_title(&mut self, gp: &gamepad::Gamepad) {
        self.title_pulse += 0.1;
        
        // Update aurora waves
        for wave in &mut self.aurora_waves {
            wave.offset += wave.speed;
        }
        
        // Spawn snowflake particles
        self.snowflake_timer += 1;
        if self.snowflake_timer >= 5 {
            let x = (random::u32() % 256) as f32;
            let y = -10.0;
            self.spawn_snowflake(x, y);
            self.snowflake_timer = 0;
        }
        
        // Update particles
        for particle in &mut self.particles {
            particle.x += particle.velocity_x;
            particle.y += particle.velocity_y;
            particle.life += 1;
        }
        self.particles.retain(|p| p.life < p.max_life && p.y < 150.0);
        
        // Start game
        if gp.start.just_pressed() || gp.up.just_pressed() {
            self.reset_game();
        }
    }
    
    fn update_playing(&mut self, gp: &gamepad::Gamepad) {
        // Dynamic scroll speed (slower progression)
        self.scroll_speed = SCROLL_SPEED_BASE + (self.score as f32 / 1000.0).min(2.5);
        
        // Apply slow-mo effect
        let actual_speed = if self.slow_mo_timer > 0 {
            self.slow_mo_timer -= 1;
            self.scroll_speed * 0.5
        } else {
            self.scroll_speed
        };
        
        self.distance += actual_speed;
        self.total_distance += actual_speed;
        
        // Score based on survival
        if self.frame % 10 == 0 {
            let points = 1 + self.combo;
            let multiplier = if self.double_points_timer > 0 { 2 } else { 1 };
            self.score += points * multiplier;
        }
        
        // Update power-up timers
        if self.shield_timer > 0 {
            self.shield_timer -= 1;
            if self.shield_timer == 0 {
                self.has_shield = false;
            }
        }
        if self.double_points_timer > 0 {
            self.double_points_timer -= 1;
        }
        if self.magnet_timer > 0 {
            self.magnet_timer -= 1;
        }
        
        // Player input
        if gp.up.just_pressed() && self.player_is_on_ground {
            self.player_is_jumping = true;
            self.player_velocity_y = JUMP_POWER;
            self.player_jump_hold_frames = 0;
            self.player_is_on_ground = false;
            
            // Play jump sound
            audio::play("jump");
            
            // Perfect landing bonus
            if self.perfect_landings > 0 {
                self.spawn_floating_text(self.player_x, self.player_y - 20.0, "PERFECT!", COLOR_PERFECT);
            }
        }
        
        // Variable jump height
        if gp.up.pressed() && self.player_is_jumping && self.player_jump_hold_frames < MAX_JUMP_HOLD {
            self.player_velocity_y += (JUMP_POWER - MIN_JUMP_POWER) / MAX_JUMP_HOLD as f32;
            self.player_jump_hold_frames += 1;
        } else {
            self.player_is_jumping = false;
        }
        
        // Apply gravity
        self.player_velocity_y += GRAVITY;
        if self.player_velocity_y > 15.0 {
            self.player_velocity_y = 15.0;
        }
        
        // Update position
        self.player_y += self.player_velocity_y;
        
        // Ground collision
        if self.player_y >= GROUND_Y {
            self.player_y = GROUND_Y;
            self.player_velocity_y = 0.0;
            
            // Check for perfect landing
            if !self.player_is_on_ground && self.player_velocity_y.abs() < 2.0 {
                self.perfect_landings += 1;
            } else {
                self.perfect_landings = 0;
            }
            
            self.player_is_on_ground = true;
            self.player_rotation = 0.0;
        } else {
            self.player_rotation = (self.player_velocity_y * 2.0).clamp(-30.0, 30.0);
        }
        
        // Altitude bonus
        let altitude = (GROUND_Y - self.player_y) / GROUND_Y;
        if altitude > 0.3 && self.frame % 5 == 0 {
            let bonus = (altitude * 5.0) as u32;
            let multiplier = if self.double_points_timer > 0 { 2 } else { 1 };
            self.score += bonus * multiplier;
        }
        
        if !audio::is_playing("music-background") {
            audio::play("music-background");
        }
        
        // Update aurora waves
        for wave in &mut self.aurora_waves {
            wave.offset += wave.speed;
        }
        
        // Spawn obstacles
        self.spawn_timer += 1;
        if self.spawn_timer >= OBSTACLE_SPAWN_INTERVAL {
            let obstacle_type = match random::u32() % 10 {
                0..=4 => ObstacleType::Crystal,
                5..=7 => ObstacleType::FloatingRock,
                _ => ObstacleType::CandyCane,
            };
            self.obstacles.push(Obstacle::new(256.0, obstacle_type, &mut self.rng_state));
            self.spawn_timer = 0;
            
            // Spawn star
            if random::u32() % 3 == 0 {
                self.stars.push(Star {
                    x: 256.0 + 40.0,
                    y: 40.0 + (random::u32() % 40) as f32,
                    collected: false,
                });
            }
            
            // Spawn power-ups (more frequent - gifts/presents)
            if random::u32() % 8 == 0 {
                let powerup_type = match random::u32() % 4 {
                    0 => PowerUpType::Shield,
                    1 => PowerUpType::SlowMo,
                    2 => PowerUpType::Magnet,
                    _ => PowerUpType::DoublePoints,
                };
                self.powerups.push(PowerUp {
                    x: 256.0 + 60.0,
                    y: 50.0 + (random::u32() % 30) as f32,
                    powerup_type,
                    collected: false,
                });
            }
        }
        
        // Update obstacles
        let player_x = self.player_x;
        let player_y = self.player_y;
        let mut particles_to_spawn = 0;
        let mut near_miss_detected = false;
        let mut combo_milestones = Vec::new();
        
        for obstacle in &mut self.obstacles {
            obstacle.x -= actual_speed;
            
            if !obstacle.passed && obstacle.x + obstacle.width < player_x {
                obstacle.passed = true;
                self.combo += 1;
                if self.combo > self.max_combo {
                    self.max_combo = self.combo;
                }
                
                let multiplier = if self.double_points_timer > 0 { 2 } else { 1 };
                self.score += 10 * self.combo * multiplier;
                particles_to_spawn += 5;
                
                // Combo milestone feedback
                if self.combo % 10 == 0 {
                    combo_milestones.push(self.combo);
                    self.screen_flash = 10;
                }
            }
            
            // Near miss detection
            let distance_to_obstacle = (obstacle.x - player_x).abs();
            if distance_to_obstacle < 20.0 && distance_to_obstacle > 5.0 && !obstacle.passed {
                near_miss_detected = true;
            }
        }
        
        // Spawn floating text for combo milestones
        for combo_value in combo_milestones {
            audio::play("combo-milestone");
            self.spawn_floating_text(player_x, player_y - 30.0, &format!("{}x COMBO!", combo_value), COLOR_STAR);
        }
        
        // Near miss bonus
        if near_miss_detected {
            self.near_miss_count += 1;
            if self.near_miss_count % 3 == 0 {
                self.spawn_floating_text(player_x, player_y, "CLOSE!", COLOR_AURORA_GREEN);
                self.score += 5;
            }
        }
        
        // Spawn celebration particles
        for _ in 0..particles_to_spawn {
            self.spawn_sparkle(player_x, player_y);
        }
        
        self.obstacles.retain(|o| o.x + o.width >= 0.0);
        
        // Update stars with magnet effect
        for star in &mut self.stars {
            star.x -= actual_speed;
            
            // Magnet pulls stars toward player
            if self.magnet_timer > 0 && !star.collected {
                let dx = self.player_x - star.x;
                let dy = self.player_y - star.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < 80.0 {
                    star.x += dx * 0.1;
                    star.y += dy * 0.1;
                }
            }
        }
        self.stars.retain(|s| s.x >= -8.0);
        
        // Update power-ups
        for powerup in &mut self.powerups {
            powerup.x -= actual_speed;
        }
        self.powerups.retain(|p| p.x >= -16.0);
        
        // Spawn snowflakes
        self.snowflake_timer += 1;
        if self.snowflake_timer >= 10 {
            let x = (random::u32() % 256) as f32;
            self.spawn_snowflake(x, -10.0);
            self.snowflake_timer = 0;
        }
        
        // Spawn trail particles
        self.particle_timer += 1;
        if self.particle_timer >= PARTICLE_SPAWN_RATE && !self.player_is_on_ground {
            self.spawn_trail(self.player_x, self.player_y);
            self.particle_timer = 0;
        }
        
        // Update particles
        for particle in &mut self.particles {
            particle.x += particle.velocity_x;
            particle.y += particle.velocity_y;
            particle.life += 1;
        }
        self.particles.retain(|p| p.life < p.max_life && p.y < 150.0);
        
        // Update floating texts
        for text in &mut self.floating_texts {
            text.y -= 1.0;
            text.life += 1;
        }
        self.floating_texts.retain(|t| t.life < 60);
        
        // Check collisions
        self.check_collisions();
        
        // Update screen effects
        if self.shake_frames > 0 {
            self.shake_frames -= 1;
        }
        if self.screen_flash > 0 {
            self.screen_flash -= 1;
        }
    }
    
    fn update_game_over(&mut self, gp: &gamepad::Gamepad) {
        // Update aurora waves
        for wave in &mut self.aurora_waves {
            wave.offset += wave.speed;
        }
        
        // Spawn snowflakes
        self.snowflake_timer += 1;
        if self.snowflake_timer >= 5 {
            let x = (random::u32() % 256) as f32;
            self.spawn_snowflake(x, -10.0);
            self.snowflake_timer = 0;
        }
        
        // Update particles
        for particle in &mut self.particles {
            particle.x += particle.velocity_x;
            particle.y += particle.velocity_y;
            particle.life += 1;
        }
        self.particles.retain(|p| p.life < p.max_life && p.y < 150.0);
        
        // Restart
        if gp.start.just_pressed() || gp.up.just_pressed() {
            self.reset_game();
        }
    }
    
    fn spawn_snowflake(&mut self, x: f32, y: f32) {
        let speed_x = ((random::u32() % 20) as f32 - 10.0) * 0.1;
        let speed_y = 0.5 + (random::u32() % 10) as f32 * 0.1;
        
        self.particles.push(Particle {
            x,
            y,
            velocity_x: speed_x,
            velocity_y: speed_y,
            life: 0,
            max_life: 200,
            particle_type: ParticleType::Snowflake,
            size: 1.0 + (random::u32() % 3) as f32,
        });
    }
    
    fn spawn_sparkle(&mut self, x: f32, y: f32) {
        let angle = (random::u32() % 360) as f32 * 3.14159 / 180.0;
        let speed = 0.5 + (random::u32() % 20) as f32 / 10.0;
        
        self.particles.push(Particle {
            x,
            y,
            velocity_x: angle.cos() * speed - self.scroll_speed,
            velocity_y: angle.sin() * speed,
            life: 0,
            max_life: 30 + (random::u32() % 20),
            particle_type: ParticleType::Sparkle,
            size: 1.0 + (random::u32() % 3) as f32,
        });
    }
    
    fn spawn_trail(&mut self, x: f32, y: f32) {
        self.particles.push(Particle {
            x,
            y,
            velocity_x: -self.scroll_speed,
            velocity_y: 0.0,
            life: 0,
            max_life: 20,
            particle_type: ParticleType::Trail,
            size: 2.0,
        });
    }
    
    fn spawn_floating_text(&mut self, x: f32, y: f32, text: &str, color: u32) {
        self.floating_texts.push(FloatingText {
            x,
            y,
            text: text.to_string(),
            life: 0,
            color,
        });
    }
    
    fn check_collisions(&mut self) {
        let px = self.player_x - 8.0;
        let py = self.player_y - 8.0;
        let pw = 16.0;
        let ph = 16.0;
        
        // Check obstacle collisions
        for obstacle in &self.obstacles {
            let ox = obstacle.x;
            let oy = obstacle.y - obstacle.height;
            let ow = obstacle.width;
            let oh = obstacle.height;
            
            if px < ox + ow && px + pw > ox && py < oy + oh && py + ph > oy {
                // DEBUG LOG: Print collision details
                let obstacle_name = match obstacle.obstacle_type {
                    ObstacleType::Crystal => "Crystal",
                    ObstacleType::FloatingRock => "FloatingRock",
                    ObstacleType::CandyCane => "CandyCane",
                };
                log!("🔴 COLLISION DETECTED!");
                log!("  Obstacle Type: {}", obstacle_name);
                log!("  Obstacle Pos: x={:.1}, y={:.1}, w={:.1}, h={:.1}", ox, oy, ow, oh);
                log!("  Player Pos: x={:.1}, y={:.1}, w={:.1}, h={:.1}", px, py, pw, ph);
                log!("  Player Y from ground: {:.1}", self.player_y);
                
                if self.has_shield {
                    log!("  Shield absorbed hit!");
                    self.has_shield = false;
                    self.shield_timer = 0;
                    audio::play("shield-break");
                    self.spawn_floating_text(self.player_x, self.player_y, "SHIELD!", COLOR_SHIELD);
                    self.screen_flash = 15;
                    return;
                } else {
                    log!("  GAME OVER!");
                    self.game_over();
                    return;
                }
            }
        }
        
        // Check star collection
        let mut stars_collected = Vec::new();
        for star in &mut self.stars {
            if !star.collected {
                let sx = star.x - 4.0;
                let sy = star.y - 4.0;
                let sw = 8.0;
                let sh = 8.0;
                
                if px < sx + sw && px + pw > sx && py < sy + sh && py + ph > sy {
                    star.collected = true;
                    let multiplier = if self.double_points_timer > 0 { 2 } else { 1 };
                    self.score += 50 * multiplier;
                    self.stars_collected += 1;
                    stars_collected.push((star.x, star.y));
                }
            }
        }
        
        // Spawn particles for collected stars
        for (x, y) in stars_collected {
            audio::play("collect-star");
            for _ in 0..10 {
                self.spawn_sparkle(x, y);
            }
            self.spawn_floating_text(x, y, "+50", COLOR_STAR);
        }
        
        self.stars.retain(|s| !s.collected);
        
        // Check power-up collection
        let mut powerup_effects = Vec::new();
        
        for powerup in &mut self.powerups {
            if !powerup.collected {
                let pux = powerup.x - 8.0;
                let puy = powerup.y - 8.0;
                let puw = 16.0;
                let puh = 16.0;
                
                if px < pux + puw && px + pw > pux && py < puy + puh && py + ph > puy {
                    powerup.collected = true;
                    
                    let (text, color) = match powerup.powerup_type {
                        PowerUpType::Shield => {
                            self.has_shield = true;
                            self.shield_timer = 300;
                            ("SHIELD!", COLOR_SHIELD)
                        }
                        PowerUpType::SlowMo => {
                            self.slow_mo_timer = 180;
                            ("SLOW-MO!", COLOR_AURORA_GREEN)
                        }
                        PowerUpType::Magnet => {
                            self.magnet_timer = 240;
                            ("MAGNET!", COLOR_AURORA_GREEN)
                        }
                        PowerUpType::DoublePoints => {
                            self.double_points_timer = 300;
                            ("2x POINTS!", COLOR_STAR)
                        }
                    };
                    
                    powerup_effects.push((powerup.x, powerup.y, text.to_string(), color));
                }
            }
        }
        
        // Spawn effects after iteration
        for (x, y, text, color) in powerup_effects {
            audio::play("collect-powerup");
            self.spawn_floating_text(x, y, &text, color);
            
            for _ in 0..20 {
                self.spawn_sparkle(x, y);
            }
        }
        
        self.powerups.retain(|p| !p.collected);
    }
    
    fn game_over(&mut self) {
        self.mode = GameMode::GameOver;
        
        audio::stop("music-background");
        // Play game over sound
        audio::play("game-over");
        
        self.shake_frames = 15;
        self.shake_intensity = 5.0;
        self.combo = 0;
        
        if self.score > self.high_score {
            self.high_score = self.score;
        }
        
        for _ in 0..30 {
            self.spawn_sparkle(self.player_x, self.player_y);
        }
    }
    
    fn reset_game(&mut self) {
        self.mode = GameMode::Playing;
        
        audio::stop("music-background");
        // Start background music (looping)
        if !audio::is_playing("music-background") {
            audio::play("music-background");
        }
        
        self.player_x = PLAYER_X;
        self.player_y = GROUND_Y;
        self.player_velocity_y = 0.0;
        self.player_is_jumping = false;
        self.player_jump_hold_frames = 0;
        self.player_is_on_ground = true;
        self.player_rotation = 0.0;
        self.obstacles.clear();
        self.stars.clear();
        self.powerups.clear();
        self.particles.clear();
        self.floating_texts.clear();
        self.score = 0;
        self.combo = 0;
        self.distance = 0.0;
        self.scroll_speed = SCROLL_SPEED_BASE;
        self.spawn_timer = 0;
        self.particle_timer = 0;
        self.snowflake_timer = 0;
        self.shake_frames = 0;
        self.has_shield = false;
        self.shield_timer = 0;
        self.slow_mo_timer = 0;
        self.magnet_timer = 0;
        self.double_points_timer = 0;
        self.perfect_landings = 0;
        self.near_miss_count = 0;
        self.screen_flash = 0;
    }
    
    fn render(&mut self) {
        // Clear screen
        clear(0x00ffffff);
        
        // Flash effect
        if self.screen_flash > 0 {
            let flash_alpha = (self.screen_flash as f32 / 10.0 * 50.0) as u32;
            clear(0xffffff00 | flash_alpha);
        }
        
        // Draw Christmas gradient sky (dark blue to light blue)
        for y in 0..72 {
            let t = y as f32 / 72.0;
            let color = lerp_color(COLOR_SKY_TOP, COLOR_SKY_MID, t);
            rect!(y = y, w = 256, h = 1, color = color);
        }
        
        for y in 72..144 {
            let t = (y - 72) as f32 / 72.0;
            let color = lerp_color(COLOR_SKY_MID, COLOR_SKY_BOTTOM, t);
            rect!(y = y, w = 256, h = 1, color = color);
        }
        
        // Draw Christmas aurora waves (red, green, gold)
        for (i, wave) in self.aurora_waves.iter().enumerate() {
            let xmas_color = match i {
                0 => COLOR_AURORA_RED,
                1 => COLOR_AURORA_GREEN,
                _ => COLOR_AURORA_GOLD,
            };
            
            for x in (0..256).step_by(2) {
                let y1 = wave.y_base + ((x as f32 * wave.frequency + wave.offset).sin() * wave.amplitude);
                let y2 = wave.y_base + (((x + 2) as f32 * wave.frequency + wave.offset).sin() * wave.amplitude);
                let height = ((y2 - y1).abs() + wave.amplitude / 2.0) as u32;
                
                rect!(
                    x = x as i32,
                    y = y1 as i32 - height as i32 / 2,
                    w = 2,
                    h = height.max(1),
                    color = xmas_color & 0xffffff33
                );
            }
        }
        
        // Draw Christmas trees in background (moved below the track)
        let tree_positions = [20, 60, 120, 180, 220];
        for (i, tree_x) in tree_positions.iter().enumerate() {
            let tree_y = 122 - (i % 3) as i32 * 3;  // Moved down from 110 to 122, reduced variance
            let tree_offset = ((self.frame as f32 * 0.02) + (i as f32 * 2.0)).sin() * 2.0;
            
            // Tree trunk
            rect!(x = tree_x + 4, y = tree_y, w = 4, h = 8, color = COLOR_TREE_TRUNK);
            
            // Tree layers (3 triangles)
            // Top layer
            for x_offset in -6i32..=6i32 {
                let y_offset = x_offset.abs() / 2;
                rect!(
                    x = tree_x + 6 + x_offset,
                    y = (tree_y - 15 + y_offset) as i32 + tree_offset as i32,
                    w = 1,
                    h = 1,
                    color = 0x006600ff
                );
            }
            
            // Middle layer
            for x_offset in -8i32..=8i32 {
                let y_offset = x_offset.abs() / 2;
                rect!(
                    x = tree_x + 6 + x_offset,
                    y = (tree_y - 8 + y_offset) as i32 + tree_offset as i32,
                    w = 1,
                    h = 1,
                    color = 0x008800ff
                );
            }
            
            // Bottom layer
            for x_offset in -10i32..=10i32 {
                let y_offset = x_offset.abs() / 2;
                rect!(
                    x = tree_x + 6 + x_offset,
                    y = (tree_y - 1 + y_offset) as i32 + tree_offset as i32,
                    w = 1,
                    h = 1,
                    color = COLOR_TREE_GREEN
                );
            }
            
            // Star on top (blinking)
            if (self.frame / 15 + i as u32) % 2 == 0 {
                circ!(x = tree_x + 6, y = (tree_y - 18) as i32 + tree_offset as i32, d = 3, color = COLOR_STAR);
            }
        }
        
        // Draw snowflake particles (background)
        for particle in &self.particles {
            if particle.particle_type == ParticleType::Snowflake {
                let opacity = 1.0 - (particle.life as f32 / particle.max_life as f32);
                sprite!("snowflake", x = particle.x as i32, y = particle.y as i32, opacity = opacity);
            }
        }
        
        match self.mode {
            GameMode::Title => self.render_title(),
            GameMode::Playing => self.render_playing(),
            GameMode::GameOver => self.render_game_over(),
        }
        
        // Draw sparkle and trail particles (foreground)
        for particle in &self.particles {
            if particle.particle_type != ParticleType::Snowflake {
                let opacity = 1.0 - (particle.life as f32 / particle.max_life as f32);
                let color = if particle.particle_type == ParticleType::Sparkle {
                    apply_opacity(COLOR_STAR, opacity)
                } else {
                    apply_opacity(COLOR_PLAYER_GLOW, opacity)
                };
                circ!(x = particle.x as i32, y = particle.y as i32, d = particle.size as u32, color = color);
            }
        }
        
        // Draw heavy snowfall overlay
        if self.mode == GameMode::Playing {
            for i in 0..20 {
                let x = ((self.frame as u32 * 13 + i * 37) % 256) as i32;
                let y = ((self.frame as u32 * 3 + i * 17) % 144) as i32;
                circ!(x = x, y = y, d = 2, color = 0xffffffaa);
            }
        }
        
        // Draw floating texts
        for ftext in &self.floating_texts {
            let opacity = 1.0 - (ftext.life as f32 / 60.0);
            text!(&ftext.text, x = ftext.x as i32, y = ftext.y as i32, font = "small", color = apply_opacity(ftext.color, opacity));
        }
    }
    
    fn render_title(&self) {
        let pulse = self.title_pulse.sin() * 0.3 + 1.0;
        let title_y = 30 + (self.title_pulse.sin() * 3.0) as i32;
        
        text!("AURORA", x = 80, y = title_y, font = "large", color = apply_opacity(COLOR_AURORA_RED, pulse));
        text!("RUNNER", x = 80, y = title_y + 16, font = "large", color = apply_opacity(COLOR_AURORA_GREEN, pulse));
        
        // Draw cute reindeer on title screen
        sprite!("reindeer", x = 110, y = 55);
        
        if (self.frame / 30) % 2 == 0 {
            text!("PRESS [UP] TO START", x = 60, y = 85, font = "medium", color = COLOR_TEXT);
        }
        
        text!("HIGH SCORE: {}", self.high_score; x = 70, y = 105, font = "small", color = COLOR_TEXT);
        
        if self.max_combo > 0 {
            text!("BEST COMBO: {}x", self.max_combo; x = 70, y = 115, font = "small", color = COLOR_AURORA_GREEN);
        }
        
        text!("Hold [UP] for higher jump!", x = 45, y = 130, font = "small", color = 0xaaaaaaff);
    }
    
    fn render_playing(&self) {
        rect!(y = GROUND_Y as i32 + 8, w = 256, h = 2, color = COLOR_AURORA_GREEN & 0xffffff88);
        
        // Draw stars with sprite
        for star in &self.stars {
            if !star.collected {
                sprite!("star", x = star.x as i32 - 4, y = star.y as i32 - 4);
            }
        }
        
        // Draw power-ups with present sprites
        for powerup in &self.powerups {
            if !powerup.collected {
                let sprite_name = match powerup.powerup_type {
                    PowerUpType::Shield => "present-shield",
                    PowerUpType::SlowMo => "present-slowmo",
                    PowerUpType::Magnet => "present-magnet",
                    PowerUpType::DoublePoints => "present-2x",
                };
                sprite!(sprite_name, x = powerup.x as i32 - 6, y = powerup.y as i32 - 6);
            }
        }
        
        // Draw obstacles with sprites
        for obstacle in &self.obstacles {
            match obstacle.obstacle_type {
                ObstacleType::Crystal => {
                    sprite!("crystal", x = obstacle.x as i32, y = (obstacle.y - obstacle.height) as i32);
                }
                ObstacleType::FloatingRock => {
                    sprite!("floating-rock", x = obstacle.x as i32, y = (obstacle.y - obstacle.height) as i32);
                }
                ObstacleType::CandyCane => {
                    sprite!("candy-cane", x = obstacle.x as i32, y = (obstacle.y - obstacle.height) as i32);
                }
            }
        }
        
        // Draw player with shield effect
        if self.has_shield {
            let shield_pulse = (self.frame as f32 * 0.2).sin() * 0.3 + 0.7;
            sprite!("shield", x = self.player_x as i32 - 16, y = self.player_y as i32 - 16, opacity = shield_pulse);
        }
        
        // Draw reindeer player with Santa hat
        sprite!("santa-hat", x = self.player_x as i32 - 6, y = self.player_y as i32 - 18);
        sprite!("reindeer", x = self.player_x as i32 - 8, y = self.player_y as i32 - 8);
        
        // Draw UI
        rect!(x = 4, y = 4, w = 120, h = 34, color = COLOR_UI_BG);
        text!("SCORE: {}", self.score; x = 8, y = 8, font = "small", color = COLOR_TEXT);
        
        if self.combo > 0 {
            let combo_color = if self.combo >= 20 {
                COLOR_PERFECT
            } else if self.combo >= 10 {
                COLOR_STAR
            } else if self.combo >= 5 {
                COLOR_AURORA_GREEN
            } else {
                COLOR_AURORA_GOLD
            };
            text!("COMBO x{}", self.combo; x = 8, y = 18, font = "small", color = combo_color);
        }
        
        // Power-up indicators
        if self.double_points_timer > 0 {
            text!("2x {}", self.double_points_timer / 60; x = 8, y = 28, font = "small", color = COLOR_STAR);
        }
        if self.magnet_timer > 0 {
            text!("MAG {}", self.magnet_timer / 60; x = 50, y = 28, font = "small", color = COLOR_AURORA_GREEN);
        }
        if self.slow_mo_timer > 0 {
            text!("SLOW {}", self.slow_mo_timer / 60; x = 90, y = 28, font = "small", color = COLOR_AURORA_GREEN);
        }
        
        // Altitude meter
        let altitude = ((GROUND_Y - self.player_y) / GROUND_Y * 100.0) as u32;
        if altitude > 30 {
            rect!(x = 236, y = 4, w = 16, h = 60, color = COLOR_UI_BG);
            let bar_height = (altitude as f32 / 100.0 * 56.0) as u32;
            rect!(x = 238, y = 62 - bar_height as i32, w = 12, h = bar_height, color = COLOR_AURORA_GREEN);
            text!("{}%", altitude; x = 232, y = 68, font = "small", color = COLOR_TEXT);
        }
    }
    
    fn render_game_over(&self) {
        // Draw frozen game with sprites
        for obstacle in &self.obstacles {
            match obstacle.obstacle_type {
                ObstacleType::Crystal => {
                    sprite!("crystal", x = obstacle.x as i32, y = (obstacle.y - obstacle.height) as i32, opacity = 0.5);
                }
                ObstacleType::FloatingRock => {
                    sprite!("floating-rock", x = obstacle.x as i32, y = (obstacle.y - obstacle.height) as i32, opacity = 0.5);
                }
                ObstacleType::CandyCane => {
                    sprite!("candy-cane", x = obstacle.x as i32, y = (obstacle.y - obstacle.height) as i32, opacity = 0.5);
                }
            }
        }
        
        // Draw player
        sprite!("santa-hat", x = self.player_x as i32 - 6, y = self.player_y as i32 - 18);
        sprite!("reindeer", x = self.player_x as i32 - 8, y = self.player_y as i32 - 8);
        
        rect!(w = 256, h = 144, color = 0x00000099);
        rect!(x = 28, y = 30, w = 200, h = 90, color = COLOR_UI_BG);
        
        text!("GAME OVER", x = 75, y = 38, font = "large", color = COLOR_TEXT);
        text!("SCORE: {}", self.score; x = 85, y = 58, font = "medium", color = COLOR_STAR);
        text!("HIGH: {}", self.high_score; x = 75, y = 72, font = "small", color = COLOR_TEXT);
        text!("MAX COMBO: {}x", self.max_combo; x = 60, y = 82, font = "small", color = COLOR_AURORA_GREEN);
        text!("STARS: {}", self.stars_collected; x = 75, y = 92, font = "small", color = COLOR_STAR);
        
        if (self.frame / 30) % 2 == 0 {
            text!("[UP] Play Again", x = 75, y = 105, font = "small", color = COLOR_TEXT);
        }
    }
}

// Utility functions
fn rand_quick(seed: &mut u32) -> u32 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed
}

fn lerp_color(c1: u32, c2: u32, t: f32) -> u32 {
    let r1 = ((c1 >> 24) & 0xff) as f32;
    let g1 = ((c1 >> 16) & 0xff) as f32;
    let b1 = ((c1 >> 8) & 0xff) as f32;
    let a1 = (c1 & 0xff) as f32;
    
    let r2 = ((c2 >> 24) & 0xff) as f32;
    let g2 = ((c2 >> 16) & 0xff) as f32;
    let b2 = ((c2 >> 8) & 0xff) as f32;
    let a2 = (c2 & 0xff) as f32;
    
    let r = (r1 + (r2 - r1) * t) as u32;
    let g = (g1 + (g2 - g1) * t) as u32;
    let b = (b1 + (b2 - b1) * t) as u32;
    let a = (a1 + (a2 - a1) * t) as u32;
    
    (r << 24) | (g << 16) | (b << 8) | a
}

fn apply_opacity(color: u32, opacity: f32) -> u32 {
    let a = (color & 0xff) as f32;
    let new_a = (a * opacity.clamp(0.0, 1.0)) as u32;
    (color & 0xffffff00) | new_a
}
