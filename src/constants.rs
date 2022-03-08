use phf::phf_map;

use crate::timezones::TimeZone;

// Each city name will have a mapping to an RGB color value
pub static TEAM_COLORS: phf::Map<&'static str, (u8, u8, u8)> = phf_map! {
    "Atlanta" => (225, 68, 52),
    "Boston" => (0, 122, 51),
    "Brooklyn" => (0, 0, 0),
    "Charlotte" => (29, 17, 96),
    "Chicago" => (206, 17, 65),
    "Cleveland" => (134, 0, 56),
    "Dallas" => (0, 83, 188),
    "Denver" => (13, 34, 64),
    "Detroit" => (200, 16, 46),
    "Golden State" => (29, 66, 138),
    "Houston" => (206, 17, 65),
    "Indiana" => (0, 45, 98),
    "LA Clippers" => (200, 16, 46),
    "LA Lakers" => (85, 37, 130),
    "Memphis" => (93, 118, 169),
    "Miami" => (152, 0, 46),
    "Milwaukee" => (0, 71, 27),
    "Minnesota" => (12, 35, 64),
    "New Orleans" => (0, 22, 65),
    "New York" => (0, 107, 182),
    "Oklahoma City" => (0, 125, 195),
    "Orlando" => (0, 125, 197),
    "Philadelphia" => (0, 107, 182),
    "Phoenix" => (229, 95, 32),
    "Portland" => (224, 58, 62),
    "Sacramento" => (91, 43, 130),
    "San Antonio" => (196, 206, 211),
    "Toronto" => (206, 17, 65),
    "Utah" => (0, 43, 92),
    "Washington" => (0, 43, 92),
};

pub static MY_TIMEZONE: TimeZone = TimeZone::Eastern;
