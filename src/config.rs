#[derive(Clone, Default, Debug)]
#[non_exhaustive]
pub struct Config {
    pub gyro: Gyro,
    pub accel: Accel,
    pub int1: Int1,
    pub pin9: Pin9,
    /// Threshold in bytes for when to trigger int1
    pub fifo_watermark: u16,
}

#[derive(Clone, Default, Debug)]
#[non_exhaustive]
pub struct Gyro {
    pub odr: GyroOdr,
}

#[derive(Clone, Default, Debug)]
#[repr(u8)]
pub enum GyroOdr {
    /// 32 kHz
    _32kHz = 0b0001,
    /// 16 kHz
    _16kHz = 0b0010,
    /// 8 kHz
    _8kHz = 0b0011,
    /// 4 kHz
    _4kHz = 0b0100,
    /// 2 kHz
    _2kHz = 0b0101,
    /// 1 kHz
    #[default]
    _1kHz = 0b0110,
    /// 200 Hz
    _200Hz = 0b0111,
    /// 100 Hz
    _100Hz = 0b1000,
    /// 50 Hz
    _50Hz = 0b1001,
    /// 25 Hz
    _25Hz = 0b1010,
    /// 12.5 Hz
    _12_5Hz = 0b1011,
    /// 500 Hz
    _500Hz = 0b1111,
}

#[derive(Clone, Default, Debug)]
#[non_exhaustive]
pub struct Accel {
    pub odr: AccelOdr,
    pub mode: AccelMode,
}

#[derive(Clone, Default, Debug)]
#[repr(u8)]
pub enum AccelOdr {
    /// 32 kHz (LN mode)
    _32kHz = 0b0001,
    /// 16 kHz (LN mode)
    _16kHz = 0b0010,
    /// 8 kHz (LN mode)
    _8kHz = 0b0011,
    /// 4 kHz (LN mode)
    _4kHz = 0b0100,
    /// 2 kHz (LN mode)
    _2kHz = 0b0101,
    /// 1 kHz (LN mode)
    #[default]
    _1kHz = 0b0110,
    /// 200 Hz (LP or LN mode)
    _200Hz = 0b0111,
    /// 100 Hz (LP or LN mode)
    _100Hz = 0b1000,
    /// 50 Hz (LP or LN mode)
    _50Hz = 0b1001,
    /// 25 Hz (LP or LN mode)
    _25Hz = 0b1010,
    /// 12.5 Hz (LP or LN mode)
    _12_5Hz = 0b1011,
    /// 6.25 Hz (LP mode)
    _6_25Hz = 0b1100,
    /// 3.125 Hz (LP mode)
    _3_125Hz = 0b1101,
    /// 1.5625 Hz (LP mode)
    _1_5625Hz = 0b1110,
    /// 500 Hz (LP or LN mode)
    _500Hz = 0b1111,
}

#[derive(Clone, Default, Debug)]
#[repr(u8)]
pub enum AccelMode {
    Off = 0b00,
    LowPower = 0b10,
    #[default]
    LowNoise = 0b11,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Int1 {
    pub drive: Drive,
    pub polarity: Polarity,
}

impl Default for Int1 {
    fn default() -> Self {
        Self {
            drive: Drive::PushPull,
            polarity: Polarity::ActiveLow,
        }
    }
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Drive {
    OpenDrain = 0,
    PushPull = 1,
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Polarity {
    ActiveLow = 0,
    ActiveHigh = 1,
}

#[derive(Clone, Default, Debug)]
#[non_exhaustive]
pub struct Pin9 {
    pub function: Pin9Function,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin9Function {
    #[default]
    INT2 = 0b00,
    FSYNC = 0b01,
    CLKIN = 0b10,
}
