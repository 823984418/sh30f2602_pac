#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwmoe: Pwmoe,
    pwmcon1: Pwmcon1,
    pwmcon2: Pwmcon2,
    pwmp: Pwmp,
    pwmc: Pwmc,
    pwmpsq: Pwmpsq,
    pwm0d: Pwm0d,
    pwm1d: Pwm1d,
    pwm2d: Pwm2d,
    pwm01d: Pwm01d,
    pwm11d: Pwm11d,
    pwm21d: Pwm21d,
    pwmcmp1: Pwmcmp1,
    pwmcmp2: Pwmcmp2,
    pwmcmp3: Pwmcmp3,
    pwmcmp4: Pwmcmp4,
    pwmdt00: Pwmdt00,
    pwmdt01: Pwmdt01,
    pwmdt10: Pwmdt10,
    pwmdt11: Pwmdt11,
    pwmdt20: Pwmdt20,
    pwmdt21: Pwmdt21,
    pmanualcon1: Pmanualcon1,
    pmanualcon2: Pmanualcon2,
    fltcon: Fltcon,
    _reserved25: [u8; 0x0c],
    postdcr: Postdcr,
    pwminten: Pwminten,
    pwmintf: Pwmintf,
    pwmrlden0: Pwmrlden0,
    pwmrlden1: Pwmrlden1,
    fltwen: Fltwen,
    pwmremap: Pwmremap,
}
impl RegisterBlock {
    #[doc = "0x00 - PWMOE"]
    #[inline(always)]
    pub const fn pwmoe(&self) -> &Pwmoe {
        &self.pwmoe
    }
    #[doc = "0x04 - PWMCON1"]
    #[inline(always)]
    pub const fn pwmcon1(&self) -> &Pwmcon1 {
        &self.pwmcon1
    }
    #[doc = "0x08 - PWMCON2"]
    #[inline(always)]
    pub const fn pwmcon2(&self) -> &Pwmcon2 {
        &self.pwmcon2
    }
    #[doc = "0x0c - PWMP"]
    #[inline(always)]
    pub const fn pwmp(&self) -> &Pwmp {
        &self.pwmp
    }
    #[doc = "0x10 - PWMC"]
    #[inline(always)]
    pub const fn pwmc(&self) -> &Pwmc {
        &self.pwmc
    }
    #[doc = "0x14 - PWMPSQ"]
    #[inline(always)]
    pub const fn pwmpsq(&self) -> &Pwmpsq {
        &self.pwmpsq
    }
    #[doc = "0x18 - PWM0D"]
    #[inline(always)]
    pub const fn pwm0d(&self) -> &Pwm0d {
        &self.pwm0d
    }
    #[doc = "0x1c - PWM1D"]
    #[inline(always)]
    pub const fn pwm1d(&self) -> &Pwm1d {
        &self.pwm1d
    }
    #[doc = "0x20 - PWM2D"]
    #[inline(always)]
    pub const fn pwm2d(&self) -> &Pwm2d {
        &self.pwm2d
    }
    #[doc = "0x24 - PWM01D"]
    #[inline(always)]
    pub const fn pwm01d(&self) -> &Pwm01d {
        &self.pwm01d
    }
    #[doc = "0x28 - PWM11D"]
    #[inline(always)]
    pub const fn pwm11d(&self) -> &Pwm11d {
        &self.pwm11d
    }
    #[doc = "0x2c - PWM21D"]
    #[inline(always)]
    pub const fn pwm21d(&self) -> &Pwm21d {
        &self.pwm21d
    }
    #[doc = "0x30 - PWMCMP1"]
    #[inline(always)]
    pub const fn pwmcmp1(&self) -> &Pwmcmp1 {
        &self.pwmcmp1
    }
    #[doc = "0x34 - PWMCMP2"]
    #[inline(always)]
    pub const fn pwmcmp2(&self) -> &Pwmcmp2 {
        &self.pwmcmp2
    }
    #[doc = "0x38 - PWMCMP3"]
    #[inline(always)]
    pub const fn pwmcmp3(&self) -> &Pwmcmp3 {
        &self.pwmcmp3
    }
    #[doc = "0x3c - PWMCMP4"]
    #[inline(always)]
    pub const fn pwmcmp4(&self) -> &Pwmcmp4 {
        &self.pwmcmp4
    }
    #[doc = "0x40 - PWMDT00"]
    #[inline(always)]
    pub const fn pwmdt00(&self) -> &Pwmdt00 {
        &self.pwmdt00
    }
    #[doc = "0x44 - PWMDT01"]
    #[inline(always)]
    pub const fn pwmdt01(&self) -> &Pwmdt01 {
        &self.pwmdt01
    }
    #[doc = "0x48 - PWMDT10"]
    #[inline(always)]
    pub const fn pwmdt10(&self) -> &Pwmdt10 {
        &self.pwmdt10
    }
    #[doc = "0x4c - PWMDT11"]
    #[inline(always)]
    pub const fn pwmdt11(&self) -> &Pwmdt11 {
        &self.pwmdt11
    }
    #[doc = "0x50 - PWMDT20"]
    #[inline(always)]
    pub const fn pwmdt20(&self) -> &Pwmdt20 {
        &self.pwmdt20
    }
    #[doc = "0x54 - PWMDT21"]
    #[inline(always)]
    pub const fn pwmdt21(&self) -> &Pwmdt21 {
        &self.pwmdt21
    }
    #[doc = "0x58 - PMANUALCON1"]
    #[inline(always)]
    pub const fn pmanualcon1(&self) -> &Pmanualcon1 {
        &self.pmanualcon1
    }
    #[doc = "0x5c - PMANUALCON2"]
    #[inline(always)]
    pub const fn pmanualcon2(&self) -> &Pmanualcon2 {
        &self.pmanualcon2
    }
    #[doc = "0x60 - FLTCON"]
    #[inline(always)]
    pub const fn fltcon(&self) -> &Fltcon {
        &self.fltcon
    }
    #[doc = "0x70 - POSTDCR"]
    #[inline(always)]
    pub const fn postdcr(&self) -> &Postdcr {
        &self.postdcr
    }
    #[doc = "0x74 - PWMINTEN"]
    #[inline(always)]
    pub const fn pwminten(&self) -> &Pwminten {
        &self.pwminten
    }
    #[doc = "0x78 - PWMINTF"]
    #[inline(always)]
    pub const fn pwmintf(&self) -> &Pwmintf {
        &self.pwmintf
    }
    #[doc = "0x7c - PWMRLDEN0"]
    #[inline(always)]
    pub const fn pwmrlden0(&self) -> &Pwmrlden0 {
        &self.pwmrlden0
    }
    #[doc = "0x80 - PWMRLDEN1"]
    #[inline(always)]
    pub const fn pwmrlden1(&self) -> &Pwmrlden1 {
        &self.pwmrlden1
    }
    #[doc = "0x84 - FLTWEN"]
    #[inline(always)]
    pub const fn fltwen(&self) -> &Fltwen {
        &self.fltwen
    }
    #[doc = "0x88 - PWMREMAP"]
    #[inline(always)]
    pub const fn pwmremap(&self) -> &Pwmremap {
        &self.pwmremap
    }
}
#[doc = "PWMOE (rw) register accessor: PWMOE\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmoe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmoe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmoe`] module"]
#[doc(alias = "PWMOE")]
pub type Pwmoe = crate::Reg<pwmoe::PwmoeSpec>;
#[doc = "PWMOE"]
pub mod pwmoe;
#[doc = "PWMCON1 (rw) register accessor: PWMCON1\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmcon1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmcon1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmcon1`] module"]
#[doc(alias = "PWMCON1")]
pub type Pwmcon1 = crate::Reg<pwmcon1::Pwmcon1Spec>;
#[doc = "PWMCON1"]
pub mod pwmcon1;
#[doc = "PWMCON2 (rw) register accessor: PWMCON2\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmcon2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmcon2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmcon2`] module"]
#[doc(alias = "PWMCON2")]
pub type Pwmcon2 = crate::Reg<pwmcon2::Pwmcon2Spec>;
#[doc = "PWMCON2"]
pub mod pwmcon2;
#[doc = "PWMP (rw) register accessor: PWMP\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmp`] module"]
#[doc(alias = "PWMP")]
pub type Pwmp = crate::Reg<pwmp::PwmpSpec>;
#[doc = "PWMP"]
pub mod pwmp;
#[doc = "PWMC (rw) register accessor: PWMC\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmc`] module"]
#[doc(alias = "PWMC")]
pub type Pwmc = crate::Reg<pwmc::PwmcSpec>;
#[doc = "PWMC"]
pub mod pwmc;
#[doc = "PWMPSQ (rw) register accessor: PWMPSQ\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmpsq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmpsq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmpsq`] module"]
#[doc(alias = "PWMPSQ")]
pub type Pwmpsq = crate::Reg<pwmpsq::PwmpsqSpec>;
#[doc = "PWMPSQ"]
pub mod pwmpsq;
#[doc = "PWM0D (rw) register accessor: PWM0D\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0d::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0d::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0d`] module"]
#[doc(alias = "PWM0D")]
pub type Pwm0d = crate::Reg<pwm0d::Pwm0dSpec>;
#[doc = "PWM0D"]
pub mod pwm0d;
#[doc = "PWM1D (rw) register accessor: PWM1D\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1d::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1d::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1d`] module"]
#[doc(alias = "PWM1D")]
pub type Pwm1d = crate::Reg<pwm1d::Pwm1dSpec>;
#[doc = "PWM1D"]
pub mod pwm1d;
#[doc = "PWM2D (rw) register accessor: PWM2D\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2d::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2d::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2d`] module"]
#[doc(alias = "PWM2D")]
pub type Pwm2d = crate::Reg<pwm2d::Pwm2dSpec>;
#[doc = "PWM2D"]
pub mod pwm2d;
#[doc = "PWM01D (rw) register accessor: PWM01D\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm01d::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm01d::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm01d`] module"]
#[doc(alias = "PWM01D")]
pub type Pwm01d = crate::Reg<pwm01d::Pwm01dSpec>;
#[doc = "PWM01D"]
pub mod pwm01d;
#[doc = "PWM11D (rw) register accessor: PWM11D\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm11d::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm11d::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm11d`] module"]
#[doc(alias = "PWM11D")]
pub type Pwm11d = crate::Reg<pwm11d::Pwm11dSpec>;
#[doc = "PWM11D"]
pub mod pwm11d;
#[doc = "PWM21D (rw) register accessor: PWM21D\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm21d::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm21d::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm21d`] module"]
#[doc(alias = "PWM21D")]
pub type Pwm21d = crate::Reg<pwm21d::Pwm21dSpec>;
#[doc = "PWM21D"]
pub mod pwm21d;
#[doc = "PWMCMP1 (rw) register accessor: PWMCMP1\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmcmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmcmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmcmp1`] module"]
#[doc(alias = "PWMCMP1")]
pub type Pwmcmp1 = crate::Reg<pwmcmp1::Pwmcmp1Spec>;
#[doc = "PWMCMP1"]
pub mod pwmcmp1;
#[doc = "PWMCMP2 (rw) register accessor: PWMCMP2\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmcmp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmcmp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmcmp2`] module"]
#[doc(alias = "PWMCMP2")]
pub type Pwmcmp2 = crate::Reg<pwmcmp2::Pwmcmp2Spec>;
#[doc = "PWMCMP2"]
pub mod pwmcmp2;
#[doc = "PWMCMP3 (rw) register accessor: PWMCMP3\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmcmp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmcmp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmcmp3`] module"]
#[doc(alias = "PWMCMP3")]
pub type Pwmcmp3 = crate::Reg<pwmcmp3::Pwmcmp3Spec>;
#[doc = "PWMCMP3"]
pub mod pwmcmp3;
#[doc = "PWMCMP4 (rw) register accessor: PWMCMP4\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmcmp4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmcmp4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmcmp4`] module"]
#[doc(alias = "PWMCMP4")]
pub type Pwmcmp4 = crate::Reg<pwmcmp4::Pwmcmp4Spec>;
#[doc = "PWMCMP4"]
pub mod pwmcmp4;
#[doc = "PWMDT00 (rw) register accessor: PWMDT00\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdt00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdt00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmdt00`] module"]
#[doc(alias = "PWMDT00")]
pub type Pwmdt00 = crate::Reg<pwmdt00::Pwmdt00Spec>;
#[doc = "PWMDT00"]
pub mod pwmdt00;
#[doc = "PWMDT01 (rw) register accessor: PWMDT01\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdt01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdt01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmdt01`] module"]
#[doc(alias = "PWMDT01")]
pub type Pwmdt01 = crate::Reg<pwmdt01::Pwmdt01Spec>;
#[doc = "PWMDT01"]
pub mod pwmdt01;
#[doc = "PWMDT10 (rw) register accessor: PWMDT10\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdt10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdt10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmdt10`] module"]
#[doc(alias = "PWMDT10")]
pub type Pwmdt10 = crate::Reg<pwmdt10::Pwmdt10Spec>;
#[doc = "PWMDT10"]
pub mod pwmdt10;
#[doc = "PWMDT11 (rw) register accessor: PWMDT11\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdt11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdt11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmdt11`] module"]
#[doc(alias = "PWMDT11")]
pub type Pwmdt11 = crate::Reg<pwmdt11::Pwmdt11Spec>;
#[doc = "PWMDT11"]
pub mod pwmdt11;
#[doc = "PWMDT20 (rw) register accessor: PWMDT20\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdt20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdt20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmdt20`] module"]
#[doc(alias = "PWMDT20")]
pub type Pwmdt20 = crate::Reg<pwmdt20::Pwmdt20Spec>;
#[doc = "PWMDT20"]
pub mod pwmdt20;
#[doc = "PWMDT21 (rw) register accessor: PWMDT21\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdt21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdt21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmdt21`] module"]
#[doc(alias = "PWMDT21")]
pub type Pwmdt21 = crate::Reg<pwmdt21::Pwmdt21Spec>;
#[doc = "PWMDT21"]
pub mod pwmdt21;
#[doc = "PMANUALCON1 (rw) register accessor: PMANUALCON1\n\nYou can [`read`](crate::Reg::read) this register and get [`pmanualcon1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmanualcon1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmanualcon1`] module"]
#[doc(alias = "PMANUALCON1")]
pub type Pmanualcon1 = crate::Reg<pmanualcon1::Pmanualcon1Spec>;
#[doc = "PMANUALCON1"]
pub mod pmanualcon1;
#[doc = "PMANUALCON2 (rw) register accessor: PMANUALCON2\n\nYou can [`read`](crate::Reg::read) this register and get [`pmanualcon2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmanualcon2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmanualcon2`] module"]
#[doc(alias = "PMANUALCON2")]
pub type Pmanualcon2 = crate::Reg<pmanualcon2::Pmanualcon2Spec>;
#[doc = "PMANUALCON2"]
pub mod pmanualcon2;
#[doc = "FLTCON (rw) register accessor: FLTCON\n\nYou can [`read`](crate::Reg::read) this register and get [`fltcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltcon`] module"]
#[doc(alias = "FLTCON")]
pub type Fltcon = crate::Reg<fltcon::FltconSpec>;
#[doc = "FLTCON"]
pub mod fltcon;
#[doc = "POSTDCR (rw) register accessor: POSTDCR\n\nYou can [`read`](crate::Reg::read) this register and get [`postdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`postdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@postdcr`] module"]
#[doc(alias = "POSTDCR")]
pub type Postdcr = crate::Reg<postdcr::PostdcrSpec>;
#[doc = "POSTDCR"]
pub mod postdcr;
#[doc = "PWMINTEN (rw) register accessor: PWMINTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`pwminten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwminten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwminten`] module"]
#[doc(alias = "PWMINTEN")]
pub type Pwminten = crate::Reg<pwminten::PwmintenSpec>;
#[doc = "PWMINTEN"]
pub mod pwminten;
#[doc = "PWMINTF (rw) register accessor: PWMINTF\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmintf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmintf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmintf`] module"]
#[doc(alias = "PWMINTF")]
pub type Pwmintf = crate::Reg<pwmintf::PwmintfSpec>;
#[doc = "PWMINTF"]
pub mod pwmintf;
#[doc = "PWMRLDEN0 (rw) register accessor: PWMRLDEN0\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmrlden0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmrlden0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmrlden0`] module"]
#[doc(alias = "PWMRLDEN0")]
pub type Pwmrlden0 = crate::Reg<pwmrlden0::Pwmrlden0Spec>;
#[doc = "PWMRLDEN0"]
pub mod pwmrlden0;
#[doc = "PWMRLDEN1 (rw) register accessor: PWMRLDEN1\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmrlden1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmrlden1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmrlden1`] module"]
#[doc(alias = "PWMRLDEN1")]
pub type Pwmrlden1 = crate::Reg<pwmrlden1::Pwmrlden1Spec>;
#[doc = "PWMRLDEN1"]
pub mod pwmrlden1;
#[doc = "FLTWEN (rw) register accessor: FLTWEN\n\nYou can [`read`](crate::Reg::read) this register and get [`fltwen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltwen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltwen`] module"]
#[doc(alias = "FLTWEN")]
pub type Fltwen = crate::Reg<fltwen::FltwenSpec>;
#[doc = "FLTWEN"]
pub mod fltwen;
#[doc = "PWMREMAP (rw) register accessor: PWMREMAP\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmremap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmremap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmremap`] module"]
#[doc(alias = "PWMREMAP")]
pub type Pwmremap = crate::Reg<pwmremap::PwmremapSpec>;
#[doc = "PWMREMAP"]
pub mod pwmremap;
