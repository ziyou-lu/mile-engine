
pub enum DataType {
    String(String),
    I32(i32),
    F32(f32),
    F64(f64),
}

impl From<String> for DataType {
    fn from(v: String) -> Self {
        DataType::String(v)
    }
}

impl Into<String> for DataType {
    fn into(self) -> String {
        if let DataType::String(v) = self {
            return v;
        } 

        String::new()
    }
}

impl From<i32> for DataType {
    fn from(v: i32) -> Self {
        DataType::I32(v)
    }
}

impl Into<i32> for DataType {
    fn into(self) -> i32 {
        if let DataType::I32(v) = self {
            return v;
        }

        0
    }
}

impl From<f32> for DataType {
    fn from(v: f32) -> Self {
        DataType::F32(v)
    }
}

impl Into<f32> for DataType {
    fn into(self) -> f32 {
        if let DataType::F32(v) = self {
            return v;
        }

        0 as f32
    }
}

impl From<f64> for DataType {
    fn from(v: f64) -> Self {
        DataType::F64(v)
    }
}

impl Into<f64> for DataType {
    fn into(self) -> f64 {
        if let DataType::F64(v) = self {
            return v;
        }
        0 as f64
    }
}

