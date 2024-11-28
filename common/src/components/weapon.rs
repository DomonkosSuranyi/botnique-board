use crate::metric_dimension::Second;
use crate::resources::weapon::{GunId, GunResource};
use bevy::ecs::component::Component;
use std::time::Duration;

pub use weapon_details::*;

const NUMBER_OF_SLOTS: usize = 3;

/// This is the first approach of the inventory. For now it stores fix number of guns
#[derive(Component)]
pub struct Holster {
    guns: [(Weapon, &'static str); NUMBER_OF_SLOTS],
    selected: usize,
}

impl Holster {
    pub fn new(gun_resource: &GunResource) -> Self {
        let guns = [
            (
                Weapon::new(gun_resource.get_gun(GunId::Revolver)),
                "Revolver",
            ),
            (Weapon::new(gun_resource.get_gun(GunId::Shotgun)), "Shotgun"),
            (Weapon::new(gun_resource.get_gun(GunId::Rifle)), "Rifle"),
        ];

        Holster { guns, selected: 0 }
    }

    pub fn new_with_guns(guns: [(Weapon, &'static str); NUMBER_OF_SLOTS]) -> Self {
        Holster { guns, selected: 0 }
    }

    pub fn switch(&mut self, slot: usize) -> Option<&'static str> {
        if let Some(newly_selected) = self.guns.get(slot) {
            self.selected = slot;
            Some(newly_selected.1)
        } else {
            None
        }
    }

    pub fn active_slot(&self) -> usize {
        self.selected
    }

    pub fn active_gun(&self) -> &Weapon {
        &self.guns[self.selected].0
    }

    pub fn active_gun_mut(&mut self) -> &mut Weapon {
        &mut self.guns[self.selected].0
    }
}

pub struct Weapon {
    /// Time::absolute_time()
    pub last_shot_time: std::time::Duration,
    /// Content of the weapon magazine
    pub bullets_left_in_magazine: u32,
    /// When reload is started. Defined only while reloading.
    pub reload_started_at: Option<Duration>,
    /// Flag required for single/burst shot weapons
    pub input_lifted: bool,
    /// Static details of the weapon.
    pub details: WeaponDetails,
}

impl Weapon {
    pub fn new(details: WeaponDetails) -> Self {
        Weapon {
            last_shot_time: std::time::Duration::ZERO,
            bullets_left_in_magazine: details.magazine_size,
            reload_started_at: None,
            input_lifted: true,
            details,
        }
    }

    pub fn is_allowed_to_shoot(&self, current_absolute_time: std::time::Duration) -> bool {
        let shoot_interval = std::time::Duration::from_secs_f32(1.0 / self.details.fire_rate);
        let need_input_press = match self.details.shot {
            Shot::Single => true,
            Shot::Burst(_) => true,
            Shot::Auto => false,
        };
        let input_ok: bool = !need_input_press || self.input_lifted;

        self.reload_started_at.is_none()
            && input_ok
            && self.bullets_left_in_magazine > 0
            && current_absolute_time > self.last_shot_time + shoot_interval
    }

    pub fn bullet_lifespan_sec(&self) -> Second {
        self.details.bullet_distance_limit / self.details.bullet_speed
    }

    pub fn is_magazine_full(&self) -> bool {
        self.bullets_left_in_magazine == self.details.magazine_size
    }

    pub fn is_allowed_to_reload(&self) -> bool {
        self.reload_started_at.is_none() && !self.is_magazine_full()
    }

}

mod weapon_details {
    use crate::metric_dimension::length::Meter;
    use crate::metric_dimension::{MeterPerSec, Second};
    use bevy::reflect::TypeUuid;
    use serde::Deserialize;

    #[derive(Debug, Eq, PartialEq, Deserialize, Clone)]
    pub enum Shot {
        /// one shot per click (even when player holds down the button)
        Single,
        /// N shot per click
        #[allow(dead_code)] // Please remove this allow when using Burst
        Burst(u32),
        /// constant shooting, it will shoot while mouse button held down
        #[allow(dead_code)] // Please remove this allow when using Auto
        Auto,
    }

    #[derive(Deserialize, Clone, PartialEq, TypeUuid)]
    #[uuid = "d8653dbe-c8a2-46a0-9e64-a7eeeb61bc7f"]
    pub struct WeaponDetails {
        /// Fire rate per seconds [1/s]
        pub fire_rate: f32,
        /// Number of bullets in a single magazine. 0 mean infinite (e.g. laser pistol)
        pub magazine_size: u32,
        /// When magazine_size > 0, amount of time required to reload [seconds]
        pub reload_time: Second,
        /// Damage caused by single bullet
        pub damage: u16,
        /// Bullet spread [degree]
        /// 0 is the perfect gun, always shooting where pointed
        /// 10 is a dumb shotgun
        pub spread: f32,
        /// Shooting distance, bullets disappear after
        pub bullet_distance_limit: Meter,
        pub bullet_speed: MeterPerSec,
        pub shot: Shot,
        /// Number of pellets when shot
        pub pellet_number: u32,
    }
}
