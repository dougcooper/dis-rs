use nom::{bits, IResult};
use nom::multi::count;
use nom::number::complete::{u8, be_u16, be_u8, be_f32, be_f64, be_u32};
use nom::bits::complete::take as take_bits;
use nom::bytes::complete::take as take_bytes;
use nom::error::{Error};
use nom::sequence::tuple;
use crate::dis::v6::entity_state::model::{ActivityState, Afterburner, AirPlatformsRecord, ApLowBits, Appearance, ApTypeDesignator, ArticulatedParts, ArticulationParameter, Camouflage, Concealed, Country, Density, DrAlgorithm, DrParameters, EntityCapabilities, EntityDamage, EntityFirePower, EntityFlamingEffect, EntityHatchState, EntityId, EntityKind, EntityLights, EntityMarking, EntityMarkingCharacterSet, EntityMobilityKill, EntityPaintScheme, EntitySmoke, EntityState, EntityTrailingEffect, EntityType, EnvironmentalsRecord, ForceId, FrozenStatus, GeneralAppearance, GuidedMunitionsRecord, LandPlatformsRecord, Launcher, LaunchFlash, LifeFormsRecord, LifeFormsState, Location, Orientation, ParameterTypeVariant, PowerPlantStatus, Ramp, SimulationAddress, SpacePlatformsRecord, SpecificAppearance, State, SubsurfacePlatformsRecord, SurfacePlatformRecord, Tent, VectorF32, Weapon};
use crate::dis::v6::model::{Pdu, PduHeader};

pub fn entity_state_body(header: PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], Pdu> {
    move |input: &[u8]| {
        let (input, entity_id_val) = entity_id(input)?;
        let (input, force_id_val) = force_id(input)?;
        let (input, articulated_parts_no) = u8(input)?;
        let (input, entity_type_val) = entity_type(input)?;
        let (input, alternative_entity_type) = entity_type(input)?;
        let (input, entity_linear_velocity) = vec3_f32(input)?;
        let (input, entity_location) = location(input)?;
        let (input, entity_orientation) = orientation(input)?;
        let (input, entity_appearance) = appearance(entity_type_val.clone())(input)?;
        let (input, dead_reckoning_parameters) = dr_parameters(input)?;
        let (input, entity_marking) = entity_marking(input)?;
        let (input, entity_capabilities) = entity_capabilities(input)?;
        let (input, articulation_parameter) = if articulated_parts_no > 0 {
            let (input, params) = count(articulation_record, articulated_parts_no as usize)(input)?;
            (input, Some(params))
        } else { (input, None) };

        let builder = EntityState::builder()
            .header(header)
            .entity_id(entity_id_val)
            .force_id(force_id_val)
            .entity_type(entity_type_val)
            .alt_entity_type(alternative_entity_type)
            .linear_velocity(entity_linear_velocity)
            .location(entity_location)
            .orientation(entity_orientation)
            .appearance(entity_appearance)
            .dead_reckoning(dead_reckoning_parameters)
            .marking(entity_marking)
            .capabilities(entity_capabilities);
        let builder = if let Some(params) = articulation_parameter {
            builder.add_articulation_parameters_vec(params)
        } else { builder };
        let pdu = builder.build();

        Ok((input, Pdu::EntityState(pdu.unwrap())))
    }
}

fn entity_id(input: &[u8]) -> IResult<&[u8], EntityId> {
    let (input, site_id) = be_u16(input)?;
    let (input, application_id) = be_u16(input)?;
    let (input, entity_id) = be_u16(input)?;
    Ok((input, EntityId {
        simulation_address: SimulationAddress {
            site_id,
            application_id,
        },
        entity_id,
    }))
}

fn force_id(input: &[u8]) -> IResult<&[u8], ForceId> {
    let (input, force_id) = be_u8(input)?;
    Ok((input, ForceId::from(force_id)))
}

fn entity_type(input: &[u8]) -> IResult<&[u8], EntityType> {
    let (input, kind) = kind(input)?;
    let (input, domain) = be_u8(input)?;
    let (input, country) = country(input)?;
    let (input, category) = be_u8(input)?;
    let (input, subcategory) = be_u8(input)?;
    let (input, specific) = be_u8(input)?;
    let (input, extra) = be_u8(input)?;
    Ok((input, EntityType {
        kind,
        domain,
        country,
        category,
        subcategory,
        specific,
        extra,
    }))
}

fn kind(input: &[u8]) -> IResult<&[u8], EntityKind> {
    let (input, kind) = be_u8(input)?;
    let kind = EntityKind::from(kind);
    Ok((input, kind))
}

fn country(input: &[u8]) -> IResult<&[u8], Country> {
    let (input, country) = be_u16(input)?;
    let country = Country::from(country);
    Ok((input, country))
}

fn vec3_f32(input: &[u8]) -> IResult<&[u8], VectorF32> {
    let (input, elements) = count(be_f32, 3)(input)?;
    Ok((input, VectorF32 {
        first_vector_component: *elements.get(0).expect("Value supposed to be parsed successfully"),
        second_vector_component: *elements.get(1).expect("Value supposed to be parsed successfully"),
        third_vector_component: *elements.get(2).expect("Value supposed to be parsed successfully"),
    }))
}

fn location(input: &[u8]) -> IResult<&[u8], Location> {
    let (input, locations) = count(be_f64, 3)(input)?;
    Ok((input, Location {
        x_coordinate: *locations.get(0).expect("Value supposed to be parsed successfully"),
        y_coordinate: *locations.get(1).expect("Value supposed to be parsed successfully"),
        z_coordinate: *locations.get(2).expect("Value supposed to be parsed successfully"),
    }))
}

fn orientation(input: &[u8]) -> IResult<&[u8], Orientation> {
    let (input, orientations) = count(be_f32, 3)(input)?;
    Ok((input, Orientation {
        psi: *orientations.get(0).expect("Value supposed to be parsed successfully"),
        theta: *orientations.get(1).expect("Value supposed to be parsed successfully"),
        phi: *orientations.get(2).expect("Value supposed to be parsed successfully"),
    }))
}

fn appearance(entity_type: EntityType) -> impl Fn(&[u8]) -> IResult<&[u8], Appearance> {
    move | input: &[u8] | {
        let (input, general_appearance) = general_appearance(input)?;
        let (input, specific_appearance) = specific_appearance(entity_type.clone())(input)?;
        Ok((input, Appearance {
            general_appearance,
            specific_appearance,
        }))
    }
}

fn general_appearance(input: &[u8]) -> IResult<&[u8], GeneralAppearance> {
    let (input, (
        entity_paint_scheme,
        entity_mobility_kill,
        entity_fire_power,
        entity_damage,
        entity_smoke,
        entity_trailing_effect,
        entity_hatch_state,
        entity_lights,
        entity_flaming_effect)) : (&[u8], (u8,u8,u8,u8,u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(
        tuple(
            (take_bits(1usize),
             take_bits(1usize),
             take_bits(1usize),
             take_bits(2usize),
             take_bits(2usize),
             take_bits(2usize),
             take_bits(3usize),
             take_bits(3usize),
             take_bits(1usize))))(input)?;

    Ok((input, GeneralAppearance{
        entity_paint_scheme : EntityPaintScheme::from(entity_paint_scheme),
        entity_mobility_kill : EntityMobilityKill::from(entity_mobility_kill),
        entity_fire_power : EntityFirePower::from(entity_fire_power),
        entity_damage : EntityDamage::from(entity_damage),
        entity_smoke : EntitySmoke::from(entity_smoke),
        entity_trailing_effect : EntityTrailingEffect::from(entity_trailing_effect),
        entity_hatch_state : EntityHatchState::from(entity_hatch_state),
        entity_lights : EntityLights::from(entity_lights),
        entity_flaming_effect : EntityFlamingEffect::from(entity_flaming_effect),
    }))
}

fn specific_appearance(entity_type: EntityType) -> impl Fn(&[u8]) -> IResult<&[u8], SpecificAppearance> {
    move |input: &[u8]| {
        // domain codes are defined as part of the Entity Type Database > v29.
        let appearance = match (entity_type.kind, entity_type.domain) {
            (EntityKind::Platform, 1u8) => { SpecificAppearance::LandPlatform(land_platform_record(input)?.1) } // land
            (EntityKind::Platform, 2u8) => { SpecificAppearance::AirPlatform(air_platform_record(input)?.1) } // air
            (EntityKind::Platform, 3u8) => { SpecificAppearance::SurfacePlatform(surface_platform_record(input)?.1) } // surface
            (EntityKind::Platform, 4u8) => { SpecificAppearance::SubsurfacePlatform(subsurface_platforms_record(input)?.1) } // subsurface
            (EntityKind::Platform, 5u8) => { SpecificAppearance::SpacePlatform(space_platforms_record(input)?.1) } // space
            (EntityKind::Platform, _) => { SpecificAppearance::Other(other_specific_appearance(input)?.1) } // other: 0 and unspecified
            (EntityKind::Munition, _) => { SpecificAppearance::GuidedMunition(guided_munitions_record(input)?.1) } // guided munition
            (EntityKind::LifeForm, _) => { SpecificAppearance::LifeForm(life_forms_record(input)?.1) } // lifeform
            (EntityKind::Environmental, _) => { SpecificAppearance::Environmental(environmentals_record(input)?.1) } // environmental
            (_, _) => { SpecificAppearance::Other(other_specific_appearance(input)?.1) }
        };
        Ok((input, appearance))
    }
}

fn other_specific_appearance(input: &[u8]) -> IResult<&[u8], [u8;2]> {
    if let Ok((input,slice)) = take_bytes::<usize, &[u8], Error<&[u8]>>(2usize)(input) {
        let two_bytes : [u8;2] = slice.try_into().unwrap();
        Ok((input, two_bytes))
    } else {
        Ok((input, [ 0, 0 ]))
    }
}

fn land_platform_record(input: &[u8]) -> IResult<&[u8], LandPlatformsRecord> {
    let (input,
        (launcher,
            camouflage,
            concealed,
            frozen_status,
            power_plant_status,
            state,
            tent,
            ramp,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(1usize),
         take_bits(2usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(6usize))))(input)?;

    Ok((input, LandPlatformsRecord {
        launcher: Launcher::from(launcher),
        camouflage_type: Camouflage::from(camouflage),
        concealed: Concealed::from(concealed),
        frozen_status: FrozenStatus::from(frozen_status),
        power_plant_status: PowerPlantStatus::from(power_plant_status),
        state: State::from(state),
        tent: Tent::from(tent),
        ramp: Ramp::from(ramp),
    }))
}

fn air_platform_record(input: &[u8]) -> IResult<&[u8], AirPlatformsRecord> {
    let (input,
        (afterburner,
            _unused,
            frozen_status,
            power_plant_status,
            state,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(1usize),
         take_bits(4usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(8usize))))(input)?;

    Ok((input, AirPlatformsRecord {
        afterburner: Afterburner::from(afterburner),
        frozen_status: FrozenStatus::from(frozen_status),
        power_plant_status: PowerPlantStatus::from(power_plant_status),
        state: State::from(state),
    }))
}

fn surface_platform_record(input: &[u8]) -> IResult<&[u8], SurfacePlatformRecord> {
    let (input,
        (_unused,
            frozen_status,
            power_plant_status,
            state,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(5usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(8usize))))(input)?;

    Ok((input, SurfacePlatformRecord {
        frozen_status: FrozenStatus::from(frozen_status),
        power_plant_status: PowerPlantStatus::from(power_plant_status),
        state: State::from(state),
    }))
}

fn subsurface_platforms_record(input: &[u8]) -> IResult<&[u8], SubsurfacePlatformsRecord> {
    let (input,
        (_unused,
            frozen_status,
            power_plant_status,
            state,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(5usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(8usize))))(input)?;

    Ok((input, SubsurfacePlatformsRecord {
        frozen_status: FrozenStatus::from(frozen_status),
        power_plant_status: PowerPlantStatus::from(power_plant_status),
        state: State::from(state),
    }))
}

fn space_platforms_record(input: &[u8]) -> IResult<&[u8], SpacePlatformsRecord> {
    let (input,
        (_unused,
            frozen_status,
            power_plant_status,
            state,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(5usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(8usize))))(input)?;

    Ok((input, SpacePlatformsRecord {
        frozen_status: FrozenStatus::from(frozen_status),
        power_plant_status: PowerPlantStatus::from(power_plant_status),
        state: State::from(state),
    }))
}

fn guided_munitions_record(input: &[u8]) -> IResult<&[u8], GuidedMunitionsRecord> {
    let (input,
        (launch_flash,
            _unused_1,
            frozen_status,
            _unused_2,
            state,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(1usize),
         take_bits(4usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(8usize))))(input)?;

    Ok((input, GuidedMunitionsRecord {
        launch_flash: LaunchFlash::from(launch_flash),
        frozen_status: FrozenStatus::from(frozen_status),
        state: State::from(state),
    }))
}

fn life_forms_record(input: &[u8]) -> IResult<&[u8], LifeFormsRecord> {
    let (input,
        (life_form_state,
            _unused_1,
            frozen_status,
            _unused_2,
            activity_state,
            weapon_1,
            weapon_2,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(4usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(2usize),
         take_bits(2usize),
         take_bits(4usize))))(input)?;

    Ok((input, LifeFormsRecord {
        life_form_state: LifeFormsState::from(life_form_state),
        frozen_status: FrozenStatus::from(frozen_status),
        activity_state: ActivityState::from(activity_state),
        weapon_1: Weapon::from(weapon_1),
        weapon_2: Weapon::from(weapon_2),
    }))
}

fn environmentals_record(input: &[u8]) -> IResult<&[u8], EnvironmentalsRecord> {
    let (input,
        (density,
            _unused,
            _pad_out)) : (&[u8], (u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(4usize),
         take_bits(4usize),
         take_bits(8usize))))(input)?;

    Ok((input, EnvironmentalsRecord {
        density: Density::from(density),
    }))
}

fn dr_parameters(input: &[u8]) -> IResult<&[u8], DrParameters> {
    let (input, algorithm) = be_u8(input)?;
    let (input, other_parameters) = take_bytes(15usize)(input)?;
    let (input, acceleration) = vec3_f32(input)?;
    let (input, velocity) = vec3_f32(input)?;

    let other_parameters = other_parameters.try_into().unwrap();

    Ok((input, DrParameters {
        algorithm: DrAlgorithm::from(algorithm),
        other_parameters,
        linear_acceleration: acceleration,
        angular_velocity: velocity,
    }))
}

// TODO review if this is an efficient way to read the string and trim trailing whitespace
fn entity_marking(input: &[u8]) -> IResult<&[u8], EntityMarking> {
    let mut buf : [u8;11] = [0;11];
    let (input, character_set) = be_u8(input)?;
    let (input, _) = nom::multi::fill(be_u8, &mut buf)(input)?;

    let mut marking = String::from_utf8_lossy(&buf[..]).into_owned();
    marking.truncate(marking.trim_end().len());

    Ok((input, EntityMarking{
        marking_character_set: EntityMarkingCharacterSet::from(character_set),
        marking_string: marking,
    }))
}

fn entity_capabilities(input: &[u8]) -> IResult<&[u8], EntityCapabilities> {
    let (input,
        (ammunition_supply,
            fuel_supply,
            recovery,
            repair,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(28usize))))(input)?;

    Ok((input, EntityCapabilities {
        ammunition_supply: ammunition_supply == 1,
        fuel_supply: fuel_supply == 1,
        recovery: recovery == 1,
        repair: repair == 1,
    }))
}

fn articulation_record(input: &[u8]) -> IResult<&[u8], ArticulationParameter> {
    let (input, parameter_type_designator) = be_u8(input)?;
    let (input, parameter_change_indicator) = be_u8(input)?;
    let (input, articulation_attachment_id) = be_u16(input)?;
    let parameter_type_designator : ApTypeDesignator = ApTypeDesignator::from(parameter_type_designator);
    let (input, parameter_type_variant) = match parameter_type_designator {
        ApTypeDesignator::Articulated => { articulated_part(input)? }
        ApTypeDesignator::Attached => { attached_part(input)? }
    };
    let (input, articulation_parameter_value) = be_f64(input)?;

    Ok((input, ArticulationParameter {
        parameter_type_designator,
        parameter_change_indicator,
        articulation_attachment_id,
        parameter_type_variant,
        articulation_parameter_value,
    }))
}

fn attached_part(input: &[u8]) -> IResult<&[u8], ParameterTypeVariant> {
    let (input, attached_part) = be_u32(input)?;
    Ok((input, ParameterTypeVariant::AttachedParts(attached_part)))
}

fn articulated_part(input: &[u8]) -> IResult<&[u8], ParameterTypeVariant> {
    let (input, low_bits) = be_u16(input)?;
    let (input, high_bits) = be_u16(input)?;

    Ok((input, ParameterTypeVariant::ArticulatedParts(ArticulatedParts {
        low_bits: ApLowBits::from(low_bits),
        high_bits,
    })))
}

#[cfg(test)]
mod tests {
    use crate::dis::v6::entity_state::model::EntityMarkingCharacterSet;
    use crate::dis::v6::entity_state::parser::entity_marking;

    #[test]
    fn parse_marking_ascii() {
        let bytes: [u8; 12] = [0x01, 0x45, 0x59, 0x45, 0x20, 0x31, 0x30, 0x20, 0x20, 0x20, 0x20, 0x20];

        let marking = entity_marking(&bytes);
        assert!(marking.is_ok());
        let marking = marking.unwrap().1;
        assert_eq!(marking.marking_character_set, EntityMarkingCharacterSet::ASCII);
        assert_eq!(marking.marking_string, "EYE 10");
    }
}
