use crate::{
    error::ParseError,
    misc::parser::{FromParser, Parser},
    quick_parser,
};

#[derive(Debug)]
pub struct ActiveSatellites {
    /// Current selection mode
    pub selection: SelectionMode,
    /// Current fix mode
    pub fix: Fix,
    /// IDs of satellites in view
    pub satellites: Box<[SatelliteId]>,
    /// Position (3D) dilution of precision
    pub pdop: f32,
    /// Horizontal dilution of precision
    pub hdop: f32,
    /// Vertical dilution of precision
    pub vdop: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct SatelliteId(u8);

#[derive(Debug, Clone, Copy)]
pub enum SelectionMode {
    /// Forced to operate in 2D or 3D mode
    Manual,
    /// Allowed to automatically switch 2D/3D
    Automatic,
}

#[derive(Debug, Clone, Copy)]
pub enum Fix {
    /// No fix available
    NoFix,
    /// 2D fix
    Fix2D,
    /// 3D fix
    Fix3D,
}

impl ActiveSatellites {
    pub fn parse(sentence: &[u8]) -> Result<ActiveSatellites, ParseError> {
        let mut parser = Parser::new(sentence).take_on_parse(',');
        let selection = parser.parse::<SelectionMode>()?;
        let mode = parser.parse::<Fix>()?;

        let mut satellites = Vec::new();
        for _ in 0..12 {
            if parser.peek().is_some_and(|x| x.is_ascii_digit()) {
                satellites.push(SatelliteId(parser.parse::<u8>()?));
                continue;
            }
            parser.expect(',')?;
        }

        let pdop = parser.parse::<f32>()?;
        let hdop = parser.parse::<f32>()?;
        let vdop = parser.parse::<f32>()?;

        let satellites = satellites.into_boxed_slice();
        Ok(ActiveSatellites {
            selection,
            fix: mode,
            satellites,
            pdop,
            hdop,
            vdop,
        })
    }
}

quick_parser!(SelectionMode, {
    'A' => Automatic,
    'M' => Manual,
});

quick_parser!(Fix, {
    '1' => NoFix,
    '2' => Fix2D,
    '3' => Fix3D,
});
