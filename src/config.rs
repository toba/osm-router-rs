use crate::{RouteConfig, RouteMode};
use osm_models::{
    tag,
    way::{travel_by, way_type},
};

pub fn preferences<'a>() -> RouteMode<'a> {
    hashmap! {
        travel_by::CAR => RouteConfig {
            name: travel_by::CAR,
            weights: hashmap! {
                way_type::FREEWAY => 10.0,
                way_type::TRUNK => 10.0,
                way_type::PRIMARY => 2.0,
                way_type::SECONDARY => 1.5,
                way_type::TERTIARY => 1.0,
                way_type::MINOR => 1.0,
                way_type::RESIDENTIAL => 0.7,
                way_type::TWO_TRACK => 0.5,
                way_type::SERVICE_ROAD => 0.5
            },
            can_use: vec![
                tag::ACCESS,
                tag::VEHICLE,
                tag::MOTOR_VEHICLE,
                tag::MOTOR_CAR,
            ],
        },
        travel_by::BUS => RouteConfig {
            name: travel_by::BUS,
            weights: hashmap! {
                way_type::FREEWAY => 10.0,
                way_type::TRUNK => 10.0,
                way_type::PRIMARY => 2.0,
                way_type::SECONDARY => 1.5,
                way_type::TERTIARY => 1.0,
                way_type::MINOR => 1.0,
                way_type::RESIDENTIAL => 0.8,
                way_type::TWO_TRACK => 0.3,
                way_type::SERVICE_ROAD => 0.9
            },
            can_use: vec![
                tag::ACCESS,
                tag::VEHICLE,
                tag::MOTOR_VEHICLE,
                tag::SERVICE_VEHICLE,
                tag::BUS,
            ],
        },
        travel_by::BICYCLE => RouteConfig {
            name: travel_by::BICYCLE,
            weights: hashmap! {
                way_type::TRUNK => 0.05,
                way_type::PRIMARY => 0.3,
                way_type::SECONDARY => 0.9,
                way_type::TERTIARY => 1.0,
                way_type::MINOR => 1.0,
                way_type::BICYCLE_PATH => 2.0,
                way_type::RESIDENTIAL => 2.5,
                way_type::TWO_TRACK => 1.0,
                way_type::HORSE_PATH => 0.8,
                way_type::FOOT_PATH => 0.8,
                way_type::STAIRS => 0.5,
                way_type::PATH => 1.0
            },
            can_use: vec![tag::ACCESS, tag::VEHICLE, tag::BICYCLE],
        },
        travel_by::HORSE => RouteConfig {
            name: travel_by::HORSE,
            weights: hashmap! {
                way_type::PRIMARY => 0.05,
                way_type::SECONDARY => 0.15,
                way_type::TERTIARY => 0.3,
                way_type::MINOR => 1.0,
                way_type::RESIDENTIAL => 1.0,
                way_type::TWO_TRACK => 1.0,
                way_type::SERVICE_ROAD => 1.0,
                way_type::HORSE_PATH => 1.0,
                way_type::FOOT_PATH => 1.2,
                way_type::STAIRS => 1.15,
                way_type::PATH => 1.2
            },
            can_use: vec![tag::ACCESS, tag::HORSE],
        },
        travel_by::TRAM => RouteConfig {
            name: travel_by::TRAM,
            weights: hashmap! {
                way_type::TRAM => 1.0,
                way_type::LIGHT_RAIL => 1.0,
            },
            can_use: vec![tag::ACCESS],
        },
        travel_by::TRAIN => RouteConfig {
            name: travel_by::TRAIN,
            weights: hashmap! {
                way_type::RAIL => 1.0,
                way_type::LIGHT_RAIL => 1.0,
                way_type::SUBWAY => 1.0,
                way_type::NARROW_GAUGE => 1.0,
            },
            can_use: vec![tag::ACCESS],
        },
    }
}
