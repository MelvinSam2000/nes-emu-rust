use crate::nes::Nes;
use crate::apu::pulsechannel::PulseChannel;
use crate::apu::trianglechannel::TriangleChannel;

pub struct Apu {
    pub pulse1: PulseChannel,
    pub pulse2: PulseChannel,
    pub triangle: TriangleChannel,
    pub updated: ApuUpdateCode,

    pub en_pulse1: bool,
    pub en_pulse2: bool,
    pub en_triangle: bool,
}

#[derive(Clone, Copy)]
pub enum ApuUpdateCode {
    NONE,

    ENABLE_CHANNELS,
    
    PULSE1_DUTYCYCLE_VOLUME,
    PULSE1_FREQUENCY,
    
    PULSE2_DUTYCYCLE_VOLUME,
    PULSE2_FREQUENCY,

    TRIANGLE_FREQUENCY,
    TRIANGLE_MUTE,
}

impl Apu {

    pub fn new() -> Self {
        return Self {
            pulse1: PulseChannel::new(),
            pulse2: PulseChannel::new(),
            triangle: TriangleChannel::new(),
            updated: ApuUpdateCode::NONE,

            en_pulse1: false,
            en_pulse2: false,
            en_triangle: false,
        }
    }
}

pub fn read(nes: &mut Nes, addr: u16) -> u8 {
    
    match addr {
        // PULSE 1
        0x4000 => {
            
        },
        0x4001 => {
            
        },
        0x4002 => {
            
        },
        0x4003 => {
            
        },
        _ => {

        },
    };
    return 0;
}

pub fn write(nes: &mut Nes, addr: u16, data: u8) {
    match addr {

        // PULSE 1
        
        0x4000 => {
            nes.apu.pulse1.set_dutycycle((data & 0b11000000) >> 6);
            nes.apu.pulse1.set_volume(data & 0x0f);
            nes.apu.updated = ApuUpdateCode::PULSE1_DUTYCYCLE_VOLUME;
        },
        0x4001 => {
            
        },
        0x4002 => {
            nes.apu.pulse1.set_period(data, false);
            nes.apu.updated = ApuUpdateCode::PULSE1_FREQUENCY;
        },
        0x4003 => {
            nes.apu.pulse1.set_period(data, true);
            nes.apu.updated = ApuUpdateCode::PULSE1_FREQUENCY;
        },

        // PULSE 2

        0x4004 => {
            nes.apu.pulse2.set_dutycycle((data & 0b11000000) >> 6);
            nes.apu.pulse2.set_volume(data & 0x0f);
            nes.apu.updated = ApuUpdateCode::PULSE2_DUTYCYCLE_VOLUME;
        },
        0x4005 => {
            
        },
        0x4006 => {
            nes.apu.pulse2.set_period(data, false);
            nes.apu.updated = ApuUpdateCode::PULSE2_FREQUENCY;
        },
        0x4007 => {
            nes.apu.pulse2.set_period(data, true);
            nes.apu.updated = ApuUpdateCode::PULSE2_FREQUENCY;
        },

        // TRIANGLE

        0x4008 => {
            nes.apu.triangle.muted = data & 0b11000000 != 0;
            nes.apu.updated = ApuUpdateCode::TRIANGLE_MUTE;
        },
        0x4009 => {
            
        },
        0x400a => {
            nes.apu.triangle.set_period(data, false);
            nes.apu.updated = ApuUpdateCode::TRIANGLE_FREQUENCY;
        },
        0x400b => {
            nes.apu.triangle.set_period(data, true);
            nes.apu.updated = ApuUpdateCode::TRIANGLE_FREQUENCY;
        },


        0x4015 => {

            nes.apu.en_pulse1 = data & (1 << 0) != 0;
            nes.apu.en_pulse2 = data & (1 << 1) != 0;
            nes.apu.en_triangle = data & (1 << 2) != 0;
            nes.apu.updated = ApuUpdateCode::ENABLE_CHANNELS;
        }


        _ => {

        },
    };
}