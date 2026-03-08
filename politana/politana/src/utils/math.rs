pub trait FloatMod {
    fn fmod(self, other: Self) -> Self;
}

impl FloatMod for f64 {
    fn fmod(self, other: Self) -> Self {
        self - other * (self / other).floor()
    }
}
