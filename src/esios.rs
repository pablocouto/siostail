#[allow(dead_code)]
#[derive(Deserialize)]
pub struct IndicatorsElem {
    name: String,
    description: String,
    id: u32,
}

// Some fields ignored.
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Indicators {
    indicators: Vec<IndicatorsElem>,
}

// Some fields ignored.
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct IndicatorInnerValue {
    pub value: f32,
}

// Some fields ignored.
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct IndicatorInner {
    pub values_updated_at: String,
    pub values: Vec<IndicatorInnerValue>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Indicator {
    pub indicator: IndicatorInner,
}
