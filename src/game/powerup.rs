use bevy::prelude::*;
use rand::seq::SliceRandom;
use std::time::Duration;

use super::constants::{
    BOMB_EXPLOSION_RADIUS, BOMB_INTERVAL, FIRE_RATE, PLAYER_SPEED, PROJECTILE_SPEED,
};
use super::player::{BombTimer, ShootTimer};
use super::{GameState, OnGameScreen};

const OVERLAY_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.75);
const PANEL_COLOR: Color = Color::srgba(0.08, 0.08, 0.12, 0.95);
const BUTTON_COLOR: Color = Color::srgba(0.18, 0.18, 0.28, 0.95);
const BUTTON_HOVER_COLOR: Color = Color::srgba(0.28, 0.28, 0.38, 0.95);
const BUTTON_PRESSED_COLOR: Color = Color::srgba(0.35, 0.65, 0.35, 1.0);

#[derive(Resource, Default, Debug)]
pub struct PowerUpProgress {
    exp_pool: u32,
    pending: u32,
    collected: u32,
}

impl PowerUpProgress {
    pub fn add_experience(&mut self, amount: u32) {
        if amount == 0 {
            return;
        }
        self.exp_pool = self.exp_pool.saturating_add(amount);
        loop {
            let required = self.current_requirement();
            if required == 0 || self.exp_pool < required {
                break;
            }
            self.exp_pool -= required;
            self.pending += 1;
        }
    }

    pub fn has_pending(&self) -> bool {
        self.pending > 0
    }

    pub fn mark_selection_consumed(&mut self) {
        if self.pending > 0 {
            self.pending -= 1;
            self.collected += 1;
        }
    }

    pub fn current_requirement(&self) -> u32 {
        20 + (self.collected + self.pending) * 5
    }

    pub fn progress_to_next(&self) -> (u32, u32) {
        (self.exp_pool, self.current_requirement())
    }
}

#[derive(Resource, Default, Debug)]
pub struct PlayerUpgrades {
    pub rapid_fire_level: u32,
    pub bomb_rate_level: u32,
    pub move_speed_level: u32,
    pub projectile_damage_level: u32,
    pub projectile_speed_level: u32,
    pub explosion_radius_level: u32,
}

impl PlayerUpgrades {
    const RATE_MULTIPLIER: f32 = 0.9;
    const SPEED_INCREMENT: f32 = 0.1;
    const PROJECTILE_DAMAGE_BASE: i32 = 1;
    const PROJECTILE_DAMAGE_INCREMENT: i32 = 1;
    const PROJECTILE_SPEED_INCREMENT: f32 = 0.1;
    const EXPLOSION_RADIUS_INCREMENT: f32 = 0.15;

    pub fn fire_rate_duration(&self) -> f32 {
        FIRE_RATE * Self::RATE_MULTIPLIER.powi(self.rapid_fire_level as i32)
    }

    pub fn bomb_interval_duration(&self) -> f32 {
        BOMB_INTERVAL * Self::RATE_MULTIPLIER.powi(self.bomb_rate_level as i32)
    }

    pub fn movement_speed(&self) -> f32 {
        PLAYER_SPEED * (1.0 + Self::SPEED_INCREMENT * self.move_speed_level as f32)
    }

    pub fn projectile_damage(&self) -> i32 {
        Self::PROJECTILE_DAMAGE_BASE
            + Self::PROJECTILE_DAMAGE_INCREMENT * self.projectile_damage_level as i32
    }

    pub fn projectile_speed(&self) -> f32 {
        PROJECTILE_SPEED
            * (1.0 + Self::PROJECTILE_SPEED_INCREMENT * self.projectile_speed_level as f32)
    }

    pub fn bomb_explosion_radius(&self) -> f32 {
        BOMB_EXPLOSION_RADIUS
            * (1.0 + Self::EXPLOSION_RADIUS_INCREMENT * self.explosion_radius_level as f32)
    }

    pub fn get_level_by_choice(&self, choice: PowerUpChoice) -> u32 {
        match choice {
            PowerUpChoice::RapidFire => self.rapid_fire_level,
            PowerUpChoice::BombRapidFire => self.bomb_rate_level,
            PowerUpChoice::MoveSpeed => self.move_speed_level,
            PowerUpChoice::ProjectileDamage => self.projectile_damage_level,
            PowerUpChoice::ProjectileSpeed => self.projectile_speed_level,
            PowerUpChoice::ExplosionRadius => self.explosion_radius_level,
        }
    }
}

#[derive(Component)]
pub struct PowerUpMenu;

#[derive(Component, Clone, Copy)]
pub struct PowerUpButton {
    pub choice: PowerUpChoice,
}

#[derive(Clone, Copy, Debug)]
pub enum PowerUpChoice {
    RapidFire,
    BombRapidFire,
    MoveSpeed,
    ProjectileDamage,
    ProjectileSpeed,
    ExplosionRadius,
}

impl PowerUpChoice {
    fn label(&self) -> &'static str {
        match self {
            PowerUpChoice::RapidFire => "Rapid Fire",
            PowerUpChoice::BombRapidFire => "Bomb Rapid Fire",
            PowerUpChoice::MoveSpeed => "Move Speed",
            PowerUpChoice::ProjectileDamage => "Bullet Damage",
            PowerUpChoice::ProjectileSpeed => "Bullet Speed",
            PowerUpChoice::ExplosionRadius => "Blast Radius",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            PowerUpChoice::RapidFire => "Shortens the cooldown between normal shots.",
            PowerUpChoice::BombRapidFire => "Shortens the cooldown between bomb placements.",
            PowerUpChoice::MoveSpeed => "Increases movement speed.",
            PowerUpChoice::ProjectileDamage => "Increases projectile damage by one per level.",
            PowerUpChoice::ProjectileSpeed => "Sends bullets flying faster.",
            PowerUpChoice::ExplosionRadius => "Expands bomb explosion size.",
        }
    }
}

const ALL_POWER_UP_CHOICES: [PowerUpChoice; 6] = [
    PowerUpChoice::RapidFire,
    PowerUpChoice::BombRapidFire,
    PowerUpChoice::MoveSpeed,
    PowerUpChoice::ProjectileDamage,
    PowerUpChoice::ProjectileSpeed,
    PowerUpChoice::ExplosionRadius,
];

fn random_powerup_choices() -> Vec<PowerUpChoice> {
    let mut choices = ALL_POWER_UP_CHOICES;
    let mut rng = rand::rng();
    choices.shuffle(&mut rng);
    let count = choices.len().min(3);
    choices[..count].to_vec()
}

pub fn spawn_menu_when_ready(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    progress: Res<PowerUpProgress>,
    menu_query: Query<Entity, With<PowerUpMenu>>,
    screen_root: Single<Entity, With<OnGameScreen>>,
    upgrades: ResMut<PlayerUpgrades>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !progress.has_pending() || !menu_query.is_empty() {
        return;
    }

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let options = random_powerup_choices();
    spawn_menu(&mut commands, *screen_root, font, &options, &upgrades);
    next_state.set(GameState::SelectingPowerUp);
}

pub fn powerup_button_visuals(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<PowerUpButton>),
    >,
) {
    for (interaction, mut color) in &mut query {
        *color = match *interaction {
            Interaction::Pressed => BUTTON_PRESSED_COLOR.into(),
            Interaction::Hovered => BUTTON_HOVER_COLOR.into(),
            Interaction::None => BUTTON_COLOR.into(),
        };
    }
}

pub fn handle_powerup_selection(
    mut commands: Commands,
    mut interactions: Query<(&Interaction, &PowerUpButton), (Changed<Interaction>, With<Button>)>,
    mut progress: ResMut<PowerUpProgress>,
    mut upgrades: ResMut<PlayerUpgrades>,
    mut shoot_timer: ResMut<ShootTimer>,
    mut bomb_timer: ResMut<BombTimer>,
    asset_server: Res<AssetServer>,
    screen_root: Single<Entity, With<OnGameScreen>>,
    menu_root: Query<Entity, With<PowerUpMenu>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        apply_choice(
            button.choice,
            &mut upgrades,
            &mut shoot_timer,
            &mut bomb_timer,
        );
        progress.mark_selection_consumed();

        if let Some(menu_entity) = menu_root.iter().next() {
            commands.entity(menu_entity).despawn();
        }

        if progress.has_pending() {
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            let options = random_powerup_choices();
            spawn_menu(&mut commands, *screen_root, font, &options, &upgrades);
        } else {
            next_state.set(GameState::Playing);
        }

        break;
    }
}

fn spawn_menu(
    commands: &mut Commands,
    parent: Entity,
    font: Handle<Font>,
    choices: &[PowerUpChoice],
    upgrades: &PlayerUpgrades,
) {
    commands.entity(parent).with_children(|parent| {
        parent
            .spawn((
                PowerUpMenu,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(OVERLAY_COLOR),
            ))
            .with_children(|overlay| {
                overlay
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            width: Val::Px(520.0),
                            row_gap: Val::Px(18.0),
                            padding: UiRect::axes(Val::Px(32.0), Val::Px(28.0)),
                            align_items: AlignItems::Stretch,
                            ..default()
                        },
                        BackgroundColor(PANEL_COLOR),
                    ))
                    .with_children(|panel| {
                        panel.spawn((
                            Text::new("Power Up!".to_string()),
                            TextFont {
                                font: font.clone(),
                                font_size: 42.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                        panel.spawn((
                            Text::new("Choose an upgrade".to_string()),
                            TextFont {
                                font: font.clone(),
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgba(0.8, 0.8, 0.85, 1.0)),
                        ));
                        panel
                            .spawn((Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(12.0),
                                ..default()
                            },))
                            .with_children(|list| {
                                for &choice in choices {
                                    list.spawn(button_bundle(
                                        font.clone(),
                                        choice,
                                        upgrades.get_level_by_choice(choice),
                                    ));
                                }
                            });
                    });
            });
    });
}

fn button_bundle(font: Handle<Font>, choice: PowerUpChoice, level: u32) -> impl Bundle {
    (
        Button,
        PowerUpButton { choice },
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            padding: UiRect::axes(Val::Px(16.0), Val::Px(14.0)),
            row_gap: Val::Px(4.0),
            align_items: AlignItems::FlexStart,
            ..default()
        },
        BackgroundColor(BUTTON_COLOR),
        children![
            (
                Text::new(format!("{} ({})", choice.label(), level)),
                TextFont {
                    font: font.clone(),
                    font_size: 26.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                Text::new(choice.description().to_string()),
                TextFont {
                    font,
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgba(0.85, 0.85, 0.9, 1.0)),
            ),
        ],
    )
}

fn apply_choice(
    choice: PowerUpChoice,
    upgrades: &mut PlayerUpgrades,
    shoot_timer: &mut ShootTimer,
    bomb_timer: &mut BombTimer,
) {
    match choice {
        PowerUpChoice::RapidFire => {
            upgrades.rapid_fire_level = upgrades.rapid_fire_level.saturating_add(1);
            refresh_shoot_timer(upgrades, shoot_timer);
        }
        PowerUpChoice::BombRapidFire => {
            upgrades.bomb_rate_level = upgrades.bomb_rate_level.saturating_add(1);
            refresh_bomb_timer(upgrades, bomb_timer);
        }
        PowerUpChoice::MoveSpeed => {
            upgrades.move_speed_level = upgrades.move_speed_level.saturating_add(1);
        }
        PowerUpChoice::ProjectileDamage => {
            upgrades.projectile_damage_level = upgrades.projectile_damage_level.saturating_add(1);
        }
        PowerUpChoice::ProjectileSpeed => {
            upgrades.projectile_speed_level = upgrades.projectile_speed_level.saturating_add(1);
        }
        PowerUpChoice::ExplosionRadius => {
            upgrades.explosion_radius_level = upgrades.explosion_radius_level.saturating_add(1);
        }
    }
}

fn refresh_shoot_timer(upgrades: &PlayerUpgrades, timer: &mut ShootTimer) {
    let new_duration = upgrades.fire_rate_duration();
    timer.0.set_duration(Duration::from_secs_f32(new_duration));
}

fn refresh_bomb_timer(upgrades: &PlayerUpgrades, timer: &mut BombTimer) {
    let new_duration = upgrades.bomb_interval_duration();
    timer.0.set_duration(Duration::from_secs_f32(new_duration));
}
