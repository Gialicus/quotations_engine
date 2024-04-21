use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitOfMeasure {
    Lenght(Lenght),
    Area(Area),
    Footprint(Footprint),
    Volume(Volume),
    Mass(Mass),
    Time(Time),
}
