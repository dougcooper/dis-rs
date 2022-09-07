use crate::v6::entity_state::model::{ActivityState, Afterburner, AirPlatformsRecord, Camouflage, Concealed, Density, EntityDamage, EntityFirePower, EntityFlamingEffect, EntityHatchState, EntityLights, EntityMobilityKill, EntityPaintScheme, EntitySmoke, EntityTrailingEffect, EnvironmentalsRecord, FrozenStatus, GeneralAppearance, GuidedMunitionsRecord, LandPlatformsRecord, Launcher, LaunchFlash, LifeFormsRecord, LifeFormsState, PowerPlantStatus, Ramp, SpacePlatformsRecord, SpecificAppearance, State, SubsurfacePlatformsRecord, SurfacePlatformRecord, Tent, Weapon};

pub struct GeneralAppearanceBuilder {
    pub entity_paint_scheme : Option<EntityPaintScheme>,
    pub entity_mobility_kill : Option<EntityMobilityKill>,
    pub entity_fire_power : Option<EntityFirePower>,
    pub entity_damage : Option<EntityDamage>,
    pub entity_smoke : Option<EntitySmoke>,
    pub entity_trailing_effect : Option<EntityTrailingEffect>,
    pub entity_hatch_state : Option<EntityHatchState>,
    pub entity_lights : Option<EntityLights>,
    pub entity_flaming_effect : Option<EntityFlamingEffect>,
}

impl GeneralAppearanceBuilder {
    pub fn new() -> Self {
        GeneralAppearanceBuilder {
            entity_paint_scheme: None,
            entity_mobility_kill: None,
            entity_fire_power: None,
            entity_damage: None,
            entity_smoke: None,
            entity_trailing_effect: None,
            entity_hatch_state: None,
            entity_lights: None,
            entity_flaming_effect: None
        }
    }

    pub fn entity_paint_scheme(mut self, entity_paint_scheme : EntityPaintScheme) -> Self {
        self.entity_paint_scheme = Some(entity_paint_scheme);
        self
    }

    pub fn entity_mobility_kill(mut self, entity_mobility_kill : EntityMobilityKill) -> Self {
        self.entity_mobility_kill = Some(entity_mobility_kill);
        self
    }

    pub fn entity_fire_power(mut self, entity_fire_power : EntityFirePower) -> Self {
        self.entity_fire_power = Some(entity_fire_power);
        self
    }

    pub fn entity_damage(mut self, entity_damage : EntityDamage) -> Self {
        self.entity_damage = Some(entity_damage);
        self
    }

    pub fn entity_smoke(mut self, entity_smoke : EntitySmoke) -> Self {
        self.entity_smoke = Some(entity_smoke);
        self
    }

    pub fn entity_trailing_effect(mut self, entity_trailing_effect : EntityTrailingEffect) -> Self {
        self.entity_trailing_effect = Some(entity_trailing_effect);
        self
    }

    pub fn entity_hatch_state(mut self, entity_hatch_state : EntityHatchState) -> Self {
        self.entity_hatch_state = Some(entity_hatch_state);
        self
    }

    pub fn entity_lights(mut self, entity_lights : EntityLights) -> Self {
        self.entity_lights = Some(entity_lights);
        self
    }

    pub fn entity_flaming_effect(mut self, entity_flaming_effect : EntityFlamingEffect) -> Self {
        self.entity_flaming_effect = Some(entity_flaming_effect);
        self
    }

    pub fn validate(&self) -> bool {
        return
            self.entity_paint_scheme.is_some() &&
                self.entity_mobility_kill.is_some() &&
                self.entity_fire_power.is_some() &&
                self.entity_damage.is_some() &&
                self.entity_smoke.is_some() &&
                self.entity_trailing_effect.is_some() &&
                self.entity_hatch_state.is_some() &&
                self.entity_lights.is_some() &&
                self.entity_flaming_effect.is_some();
    }

    pub fn build(&self) -> Result<GeneralAppearance,()> {
        if self.validate() {
            Ok(GeneralAppearance {
                entity_paint_scheme: self.entity_paint_scheme.expect("Should be Some"),
                entity_mobility_kill: self.entity_mobility_kill.expect("Should be Some"),
                entity_fire_power: self.entity_fire_power.expect("Should be Some"),
                entity_damage: self.entity_damage.expect("Should be Some"),
                entity_smoke: self.entity_smoke.expect("Should be Some"),
                entity_trailing_effect: self.entity_trailing_effect.expect("Should be Some"),
                entity_hatch_state: self.entity_hatch_state.expect("Should be Some"),
                entity_lights: self.entity_lights.expect("Should be Some"),
                entity_flaming_effect: self.entity_flaming_effect.expect("Should be Some"),
            })
        } else { Err(()) }
    }
}

pub struct LandPlatformBuilder {
    launcher: Option<Launcher>,
    camouflage_type: Option<Camouflage>,
    concealed: Option<Concealed>,
    frozen_status: Option<FrozenStatus>,
    power_plant_status: Option<PowerPlantStatus>,
    state: Option<State>,
    tent: Option<Tent>,
    ramp: Option<Ramp>,
}

impl LandPlatformBuilder {
    pub fn new() -> Self {
        LandPlatformBuilder {
            launcher: None,
            camouflage_type: None,
            concealed: None,
            frozen_status: None,
            power_plant_status: None,
            state: None,
            tent: None,
            ramp: None
        }
    }

    pub fn launcher(mut self, launcher : Launcher) -> Self {
        self.launcher = Some(launcher);
        self
    }

    pub fn camouflage_type(mut self, camouflage_type : Camouflage) -> Self {
        self.camouflage_type = Some(camouflage_type);
        self
    }

    pub fn concealed(mut self, concealed : Concealed) -> Self {
        self.concealed = Some(concealed);
        self
    }

    pub fn frozen_status(mut self, frozen_status : FrozenStatus) -> Self {
        self.frozen_status = Some(frozen_status);
        self
    }

    pub fn power_plant_status(mut self, power_plant_status : PowerPlantStatus) -> Self {
        self.power_plant_status = Some(power_plant_status);
        self
    }

    pub fn state(mut self, state : State) -> Self {
        self.state = Some(state);
        self
    }

    pub fn tent(mut self, tent : Tent) -> Self {
        self.tent = Some(tent);
        self
    }

    pub fn ramp(mut self, ramp : Ramp) -> Self {
        self.ramp = Some(ramp);
        self
    }

    pub fn validate(&self) -> bool {
        return
            self.launcher.is_some() &&
                self.camouflage_type.is_some() &&
                self.concealed.is_some() &&
                self.frozen_status.is_some() &&
                self.power_plant_status.is_some() &&
                self.state.is_some() &&
                self.tent.is_some() &&
                self.ramp.is_some()
    }

    pub fn build(self) -> Result<SpecificAppearance,()> {
        return if self.validate() {
            Ok(SpecificAppearance::LandPlatform(
                LandPlatformsRecord {
                    launcher: self.launcher.expect("should be set"),
                    camouflage_type: self.camouflage_type.expect("should be set"),
                    concealed: self.concealed.expect("should be set"),
                    frozen_status: self.frozen_status.expect("should be set"),
                    power_plant_status: self.power_plant_status.expect("should be set"),
                    state: self.state.expect("should be set"),
                    tent: self.tent.expect("should be set"),
                    ramp: self.ramp.expect("should be set"),
                }
            ))
        } else {
            return Err(())
        }
    }
}

pub struct AirPlatformBuilder {
    afterburner: Option<Afterburner>,
    frozen_status: Option<FrozenStatus>,
    power_plant_status: Option<PowerPlantStatus>,
    state: Option<State>,
}

impl AirPlatformBuilder {
    pub fn new() -> Self {
        AirPlatformBuilder {
            afterburner: None,
            frozen_status: None,
            power_plant_status: None,
            state: None
        }
    }

    pub fn afterburner(mut self, afterburner : Afterburner) -> Self {
        self.afterburner = Some(afterburner);
        self
    }

    pub fn frozen_status(mut self, frozen_status : FrozenStatus) -> Self {
        self.frozen_status = Some(frozen_status);
        self
    }

    pub fn power_plant_status(mut self, power_plant_status : PowerPlantStatus) -> Self {
        self.power_plant_status = Some(power_plant_status);
        self
    }

    pub fn state(mut self, state : State) -> Self {
        self.state = Some(state);
        self
    }


    pub fn validate(&self) -> bool {
        return
            self.afterburner.is_some() &&
                self.frozen_status.is_some() &&
                self.power_plant_status.is_some() &&
                self.state.is_some()
    }

    pub fn build(self) -> Result<SpecificAppearance,()> {
        return if self.validate() {
            Ok(SpecificAppearance::AirPlatform(
                AirPlatformsRecord {
                    afterburner: self.afterburner.expect("should be set"),
                    frozen_status: self.frozen_status.expect("should be set"),
                    power_plant_status: self.power_plant_status.expect("should be set"),
                    state: self.state.expect("should be set"),
                }
            ))
        } else {
            return Err(())
        }
    }
}

pub struct SurfacePlatformBuilder {
    frozen_status: Option<FrozenStatus>,
    power_plant_status: Option<PowerPlantStatus>,
    state: Option<State>,
}

impl SurfacePlatformBuilder {
    pub fn new() -> Self {
        SurfacePlatformBuilder {
            frozen_status: None,
            power_plant_status: None,
            state: None
        }
    }

    pub fn frozen_status(mut self, frozen_status : FrozenStatus) -> Self {
        self.frozen_status = Some(frozen_status);
        self
    }

    pub fn power_plant_status(mut self, power_plant_status : PowerPlantStatus) -> Self {
        self.power_plant_status = Some(power_plant_status);
        self
    }

    pub fn state(mut self, state : State) -> Self {
        self.state = Some(state);
        self
    }

    pub fn validate(&self) -> bool {
        return
            self.frozen_status.is_some() &&
                self.power_plant_status.is_some() &&
                self.state.is_some()
    }

    pub fn build(self) -> Result<SpecificAppearance,()> {
        return if self.validate() {
            Ok(SpecificAppearance::SurfacePlatform(
                SurfacePlatformRecord {
                    frozen_status: self.frozen_status.expect("should be set"),
                    power_plant_status: self.power_plant_status.expect("should be set"),
                    state: self.state.expect("should be set"),
                }
            ))
        } else {
            return Err(())
        }
    }
}

pub struct SubsurfacePlatformBuilder {
    frozen_status: Option<FrozenStatus>,
    power_plant_status: Option<PowerPlantStatus>,
    state: Option<State>,
}

impl SubsurfacePlatformBuilder {
    pub fn new() -> Self {
        SubsurfacePlatformBuilder {
            frozen_status: None,
            power_plant_status: None,
            state: None
        }
    }

    pub fn frozen_status(mut self, frozen_status : FrozenStatus) -> Self {
        self.frozen_status = Some(frozen_status);
        self
    }

    pub fn power_plant_status(mut self, power_plant_status : PowerPlantStatus) -> Self {
        self.power_plant_status = Some(power_plant_status);
        self
    }

    pub fn state(mut self, state : State) -> Self {
        self.state = Some(state);
        self
    }

    pub fn validate(&self) -> bool {
        return
            self.frozen_status.is_some() &&
                self.power_plant_status.is_some() &&
                self.state.is_some()
    }

    pub fn build(self) -> Result<SpecificAppearance,()> {
        return if self.validate() {
            Ok(SpecificAppearance::SubsurfacePlatform(
                SubsurfacePlatformsRecord {
                    frozen_status: self.frozen_status.expect("should be set"),
                    power_plant_status: self.power_plant_status.expect("should be set"),
                    state: self.state.expect("should be set"),
                }
            ))
        } else {
            return Err(())
        }
    }
}

pub struct SpacePlatformBuilder {
    frozen_status: Option<FrozenStatus>,
    power_plant_status: Option<PowerPlantStatus>,
    state: Option<State>,
}

impl SpacePlatformBuilder {
    pub fn new() -> Self {
        SpacePlatformBuilder {
            frozen_status: None,
            power_plant_status: None,
            state: None
        }
    }

    pub fn frozen_status(mut self, frozen_status : FrozenStatus) -> Self {
        self.frozen_status = Some(frozen_status);
        self
    }

    pub fn power_plant_status(mut self, power_plant_status : PowerPlantStatus) -> Self {
        self.power_plant_status = Some(power_plant_status);
        self
    }

    pub fn state(mut self, state : State) -> Self {
        self.state = Some(state);
        self
    }

    pub fn validate(&self) -> bool {
        return
            self.frozen_status.is_some() &&
                self.power_plant_status.is_some() &&
                self.state.is_some()
    }

    pub fn build(self) -> Result<SpecificAppearance,()> {
        return if self.validate() {
            Ok(SpecificAppearance::SpacePlatform(
                SpacePlatformsRecord {
                    frozen_status: self.frozen_status.expect("should be set"),
                    power_plant_status: self.power_plant_status.expect("should be set"),
                    state: self.state.expect("should be set"),
                }
            ))
        } else {
            return Err(())
        }
    }
}

pub struct GuidedMunitionBuilder {
    launch_flash: Option<LaunchFlash>,
    frozen_status: Option<FrozenStatus>,
    state: Option<State>,
}

impl GuidedMunitionBuilder {
    pub fn new() -> Self {
        GuidedMunitionBuilder {
            launch_flash: None,
            frozen_status: None,
            state: None
        }
    }

    pub fn launch_flash(mut self, launch_flash : LaunchFlash) -> Self {
        self.launch_flash = Some(launch_flash);
        self
    }

    pub fn frozen_status(mut self, frozen_status : FrozenStatus) -> Self {
        self.frozen_status = Some(frozen_status);
        self
    }

    pub fn state(mut self, state : State) -> Self {
        self.state = Some(state);
        self
    }

    pub fn validate(&self) -> bool {
        return
            self.launch_flash.is_some() &&
                self.frozen_status.is_some() &&
                self.state.is_some()
    }

    pub fn build(self) -> Result<SpecificAppearance,()> {
        return if self.validate() {
            Ok(SpecificAppearance::GuidedMunition(
                GuidedMunitionsRecord {
                    launch_flash: self.launch_flash.expect("should be set"),
                    frozen_status: self.frozen_status.expect("should be set"),
                    state: self.state.expect("should be set"),
                }
            ))
        } else {
            return Err(())
        }
    }
}

pub struct LifeFormBuilder {
    life_form_state: Option<LifeFormsState>,
    frozen_status: Option<FrozenStatus>,
    activity_state: Option<ActivityState>,
    weapon_1: Option<Weapon>,
    weapon_2: Option<Weapon>,
}

impl LifeFormBuilder {
    pub fn new() -> Self {
        LifeFormBuilder {
            life_form_state: None,
            frozen_status: None,
            activity_state: None,
            weapon_1: None,
            weapon_2: None,
        }
    }

    pub fn life_form_state(mut self, state : LifeFormsState) -> Self {
        self.life_form_state = Some(state);
        self
    }

    pub fn frozen_status(mut self, frozen_status : FrozenStatus) -> Self {
        self.frozen_status = Some(frozen_status);
        self
    }

    pub fn activity_state(mut self, activity_state : ActivityState) -> Self {
        self.activity_state = Some(activity_state);
        self
    }

    pub fn weapon_1(mut self, weapon : Weapon) -> Self {
        self.weapon_1 = Some(weapon);
        self
    }

    pub fn weapon_2(mut self, weapon : Weapon) -> Self {
        self.weapon_2 = Some(weapon);
        self
    }

    pub fn validate(&self) -> bool {
        return
            self.life_form_state.is_some() &&
                self.frozen_status.is_some() &&
                self.activity_state.is_some() &&
                self.weapon_1.is_some() &&
                self.weapon_2.is_some()
    }

    pub fn build(self) -> Result<SpecificAppearance,()> {
        return if self.validate() {
            Ok(SpecificAppearance::LifeForm(
                LifeFormsRecord {
                    life_form_state: self.life_form_state.expect("should be set"),
                    frozen_status: self.frozen_status.expect("should be set"),
                    activity_state: self.activity_state.expect("should be set"),
                    weapon_1: self.weapon_1.expect("should be set"),
                    weapon_2: self.weapon_2.expect("should be set"),
                }
            ))
        } else {
            return Err(())
        }
    }
}

pub struct EnvironmentalBuilder {
    density: Option<Density>,
}

impl EnvironmentalBuilder {
    pub fn new() -> Self {
        EnvironmentalBuilder {
            density: None,
        }
    }

    pub fn density(mut self, density: Density) -> Self {
        self.density = Some(density);
        self
    }

    pub fn validate(&self) -> bool {
        return
            self.density.is_some()
    }

    pub fn build(self) -> Result<SpecificAppearance,()> {
        return if self.validate() {
            Ok(SpecificAppearance::Environmental(
                EnvironmentalsRecord {
                    density: self.density.expect("should be set"),
                }
            ))
        } else {
            return Err(())
        }
    }
}
