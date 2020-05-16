use lazy_static::*;
use osm_models::tag::Tagged;
use regex::Regex;

lazy_static! {
    static ref FORBID_PREFIX: Regex = Regex::new("^no_").unwrap();
    static ref REQUIRE_PREFIX: Regex = Regex::new("^only_").unwrap();
    static ref RULE_PREFIX: Regex = Regex::new("^(no|only)_").unwrap();
    static ref NO_ACCESS: Regex = Regex::new("^(no|private)").unwrap();
}

/// Whether mode of transportation is allowed along the given OSM way as
/// indicated by its tags.
pub fn allow_travel_mode<T: Tagged>(
    tagged: &T,
    access_types: &Vec<&str>,
) -> bool {
    let mut allow = true;

    for t in access_types {
        let value = tagged.get_tag(t);
        allow = if let Some(v) = value {
            !NO_ACCESS.is_match(v)
        } else {
            true
        }
    }

    allow
}

#[cfg(test)]
mod tests {
    use super::*;
    use osm_models::{tag, way_type, TagMap, Way};

    #[test]
    fn allow_travel_mode_test() {
        let tags: TagMap = [
            (tag::ROAD_TYPE, Some(way_type::SERVICE_ROAD)),
            (tag::ONE_WAY, Some("yes")),
            (tag::MOTOR_VEHICLE, Some("no")),
        ]
        .iter()
        .cloned()
        .collect();

        let way = Way {
            tags: Some(tags),
            ..Way::default()
        };

        let can_use = vec![
            tag::ACCESS,
            tag::VEHICLE,
            tag::MOTOR_VEHICLE,
            tag::MOTOR_CAR,
        ];

        assert!(!allow_travel_mode(&way, &can_use));
    }
}
