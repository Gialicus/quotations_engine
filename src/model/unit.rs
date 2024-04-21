use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LenghtType {
    Meter,
    Centimeter,
    Millimeter,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum VolumeType {
    Liter,
    Centiliter,
    Milliliter,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MassType {
    Kilogram,
    Gram,
    Centigram,
    Milligram,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TimeType {
    Second,
    Minute,
    Hour,
    Day,
    Month,
    Year,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lenght {
    pub uom: LenghtType,
    pub lenght: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Area {
    pub uom: LenghtType,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Footprint {
    pub uom: LenghtType,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub uom: VolumeType,
    pub liter: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mass {
    pub uom: MassType,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    pub uom: TimeType,
    pub value: f64,
}

impl Lenght {
    pub fn new(uom: LenghtType, lenght: f64) -> Self {
        Lenght { uom, lenght }
    }
}

impl Area {
    pub fn new(uom: LenghtType, x: f64, y: f64) -> Self {
        Area { uom, x, y }
    }
}

impl Footprint {
    pub fn new(uom: LenghtType, x: f64, y: f64, z: f64) -> Self {
        Footprint { uom, x, y, z }
    }
}

impl Volume {
    pub fn new(uom: VolumeType, liter: f64) -> Self {
        Volume { uom, liter }
    }
}

impl Mass {
    pub fn new(uom: MassType, weight: f64) -> Self {
        Mass { uom, weight }
    }
}

impl Time {
    pub fn new(uom: TimeType, value: f64) -> Self {
        Time { uom, value }
    }
}

impl fmt::Display for LenghtType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LenghtType::Meter => write!(f, "m"),
            LenghtType::Centimeter => write!(f, "cm"),
            LenghtType::Millimeter => write!(f, "mm"),
        }
    }
}

impl fmt::Display for VolumeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            VolumeType::Liter => write!(f, "l"),
            VolumeType::Centiliter => write!(f, "cl"),
            VolumeType::Milliliter => write!(f, "ml"),
        }
    }
}

impl fmt::Display for MassType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MassType::Kilogram => write!(f, "Kg"),
            MassType::Gram => write!(f, "g"),
            MassType::Centigram => write!(f, "cg"),
            MassType::Milligram => write!(f, "mg"),
        }
    }
}

impl fmt::Display for TimeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TimeType::Second => write!(f, "second"),
            TimeType::Minute => write!(f, "minute"),
            TimeType::Hour => write!(f, "hour"),
            TimeType::Day => write!(f, "day"),
            TimeType::Month => write!(f, "month"),
            TimeType::Year => write!(f, "year"),
        }
    }
}

impl fmt::Display for Lenght {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - {}", self.uom, self.lenght)
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - x: {}, y: {}", self.uom, self.x, self.y)
    }
}

impl fmt::Display for Footprint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} - x: {}, y: {}, z: {}",
            self.uom, self.x, self.y, self.z
        )
    }
}

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - {}", self.uom, self.liter)
    }
}

impl fmt::Display for Mass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - {}", self.uom, self.weight)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - {}", self.uom, self.value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitOfMeasure {
    Lenght(Lenght),
    Area(Area),
    Footprint(Footprint),
    Volume(Volume),
    Mass(Mass),
    Time(Time),
}

impl fmt::Display for UnitOfMeasure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnitOfMeasure::Lenght(l) => write!(f, "{}", l),
            UnitOfMeasure::Area(a) => write!(f, "{}", a),
            UnitOfMeasure::Footprint(fp) => write!(f, "{}", fp),
            UnitOfMeasure::Volume(v) => write!(f, "{}", v),
            UnitOfMeasure::Mass(m) => write!(f, "{}", m),
            UnitOfMeasure::Time(t) => write!(f, "{}", t),
        }
    }
}
