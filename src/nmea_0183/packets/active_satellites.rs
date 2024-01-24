use crate::{
    error::ParseError,
    misc::parser::{FromParser, Parser},
    quick_parser,
};

#[derive(Debug)]
pub struct ActiveSatellites {
    /// Current selection mode
    selection: SelectionMode,
    /// Current fix mode
    mode: Mode,
    /// IDs of satellites in view
    satellites: Box<[SatelliteId]>,
    /// Position (3D) dilution of precision
    pdop: f32,
    /// Horizontal dilution of precision
    hdop: f32,
    /// Vertical dilution of precision
    vdop: f32,
}

#[derive(Debug)]
pub struct SatelliteId(u8);

#[derive(Debug)]
pub enum SelectionMode {
    /// Forced to operate in 2D or 3D mode
    Manual,
    /// Allowed to automatically switch 2D/3D
    Automatic,
}

#[derive(Debug)]
pub enum Mode {
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
        let mode = parser.parse::<Mode>()?;

        let mut satellites = Vec::new();
        for _ in 0..12 {
            if parser.peek().is_some_and(|x| x.is_ascii_digit()) {
                satellites.push(SatelliteId(parser.parse::<u8>()?));
                continue;
            }
            parser.skip(1);
        }

        let pdop = parser.parse::<f32>()?;
        let hdop = parser.parse::<f32>()?;
        let vdop = parser.parse::<f32>()?;

        let satellites = satellites.into_boxed_slice();
        Ok(ActiveSatellites {
            selection,
            mode,
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

quick_parser!(Mode, {
    '1' => NoFix,
    '2' => Fix2D,
    '3' => Fix3D,
});
