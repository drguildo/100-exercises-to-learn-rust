use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaturatingU16(u16);

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16(value.into())
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16((*value).into())
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16(value)
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16(*value)
    }
}

impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        SaturatingU16(self.0.saturating_add(rhs.0))
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16(self.0.saturating_add(rhs))
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        SaturatingU16(self.0.saturating_add(rhs.0))
    }
}

impl Add<SaturatingU16> for u16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> Self::Output {
        SaturatingU16(self.saturating_add(rhs.0))
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}
