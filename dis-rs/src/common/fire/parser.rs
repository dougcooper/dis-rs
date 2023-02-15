use nom::IResult;
use nom::number::complete::{be_f32, be_u32};
use crate::common::fire::model::Fire;
use crate::common::parser::{entity_id, event_id, location, vec3_f32};
use crate::common::model::PduBody;
use crate::common::parser;
use crate::enumerations::FireTypeIndicator;
use crate::PduHeader;
use crate::v7::model::PduStatus;

pub fn fire_body(header: &PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> + '_ {
    move |input: &[u8]| {
        let fti = header.pdu_status.unwrap_or(PduStatus::default())
            .fire_type_indicator.unwrap_or(FireTypeIndicator::Munition);
        let (input, firing_entity_id) = entity_id(input)?;
        let (input, target_entity_id) = entity_id(input)?;
        let (input, munition_id) = entity_id(input)?;
        let (input, event_id) = event_id(input)?;
        let (input, fire_mission_index) = be_u32(input)?;
        let (input, location_in_world) = location(input)?;
        let (input, descriptor) = parser::descriptor_record_fti(fti)(input)?;
        let (input, velocity) = vec3_f32(input)?;
        let (input, range) = be_f32(input)?;

        let body = Fire {
            firing_entity_id,
            target_entity_id,
            entity_id: munition_id,
            event_id,
            fire_mission_index,
            location_in_world,
            descriptor,
            velocity,
            range,
        };

        Ok((input, PduBody::Fire(body)))
    }
}