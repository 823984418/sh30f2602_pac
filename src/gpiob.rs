#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    moder: Moder,
    idr: Idr,
    odr: Odr,
    bsrr: Bsrr,
    lckr: Lckr,
    ie: Ie,
    otyper: Otyper,
    odrvr: Odrvr,
    pupdr: Pupdr,
    ttlen: Ttlen,
    afrl: Afrl,
    afrh: Afrh,
}
impl RegisterBlock {
    #[doc = "0x00 - MODER"]
    #[inline(always)]
    pub const fn moder(&self) -> &Moder {
        &self.moder
    }
    #[doc = "0x04 - IDR"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x08 - ODR"]
    #[inline(always)]
    pub const fn odr(&self) -> &Odr {
        &self.odr
    }
    #[doc = "0x0c - BSRR"]
    #[inline(always)]
    pub const fn bsrr(&self) -> &Bsrr {
        &self.bsrr
    }
    #[doc = "0x10 - LCKR"]
    #[inline(always)]
    pub const fn lckr(&self) -> &Lckr {
        &self.lckr
    }
    #[doc = "0x14 - IE"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0x18 - OTYPER"]
    #[inline(always)]
    pub const fn otyper(&self) -> &Otyper {
        &self.otyper
    }
    #[doc = "0x1c - ODRVR"]
    #[inline(always)]
    pub const fn odrvr(&self) -> &Odrvr {
        &self.odrvr
    }
    #[doc = "0x20 - PUPDR"]
    #[inline(always)]
    pub const fn pupdr(&self) -> &Pupdr {
        &self.pupdr
    }
    #[doc = "0x24 - TTLEN"]
    #[inline(always)]
    pub const fn ttlen(&self) -> &Ttlen {
        &self.ttlen
    }
    #[doc = "0x28 - AFRL"]
    #[inline(always)]
    pub const fn afrl(&self) -> &Afrl {
        &self.afrl
    }
    #[doc = "0x2c - AFRH"]
    #[inline(always)]
    pub const fn afrh(&self) -> &Afrh {
        &self.afrh
    }
}
#[doc = "MODER (rw) register accessor: MODER\n\nYou can [`read`](crate::Reg::read) this register and get [`moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moder`] module"]
#[doc(alias = "MODER")]
pub type Moder = crate::Reg<moder::ModerSpec>;
#[doc = "MODER"]
pub mod moder;
#[doc = "IDR (r) register accessor: IDR\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "IDR"]
pub mod idr;
#[doc = "ODR (rw) register accessor: ODR\n\nYou can [`read`](crate::Reg::read) this register and get [`odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`] module"]
#[doc(alias = "ODR")]
pub type Odr = crate::Reg<odr::OdrSpec>;
#[doc = "ODR"]
pub mod odr;
#[doc = "BSRR (rw) register accessor: BSRR\n\nYou can [`read`](crate::Reg::read) this register and get [`bsrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr`] module"]
#[doc(alias = "BSRR")]
pub type Bsrr = crate::Reg<bsrr::BsrrSpec>;
#[doc = "BSRR"]
pub mod bsrr;
#[doc = "LCKR (rw) register accessor: LCKR\n\nYou can [`read`](crate::Reg::read) this register and get [`lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckr`] module"]
#[doc(alias = "LCKR")]
pub type Lckr = crate::Reg<lckr::LckrSpec>;
#[doc = "LCKR"]
pub mod lckr;
#[doc = "IE (rw) register accessor: IE\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`] module"]
#[doc(alias = "IE")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "IE"]
pub mod ie;
#[doc = "OTYPER (rw) register accessor: OTYPER\n\nYou can [`read`](crate::Reg::read) this register and get [`otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otyper`] module"]
#[doc(alias = "OTYPER")]
pub type Otyper = crate::Reg<otyper::OtyperSpec>;
#[doc = "OTYPER"]
pub mod otyper;
#[doc = "ODRVR (rw) register accessor: ODRVR\n\nYou can [`read`](crate::Reg::read) this register and get [`odrvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odrvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odrvr`] module"]
#[doc(alias = "ODRVR")]
pub type Odrvr = crate::Reg<odrvr::OdrvrSpec>;
#[doc = "ODRVR"]
pub mod odrvr;
#[doc = "PUPDR (rw) register accessor: PUPDR\n\nYou can [`read`](crate::Reg::read) this register and get [`pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pupdr`] module"]
#[doc(alias = "PUPDR")]
pub type Pupdr = crate::Reg<pupdr::PupdrSpec>;
#[doc = "PUPDR"]
pub mod pupdr;
#[doc = "TTLEN (rw) register accessor: TTLEN\n\nYou can [`read`](crate::Reg::read) this register and get [`ttlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttlen`] module"]
#[doc(alias = "TTLEN")]
pub type Ttlen = crate::Reg<ttlen::TtlenSpec>;
#[doc = "TTLEN"]
pub mod ttlen;
#[doc = "AFRL (rw) register accessor: AFRL\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrl`] module"]
#[doc(alias = "AFRL")]
pub type Afrl = crate::Reg<afrl::AfrlSpec>;
#[doc = "AFRL"]
pub mod afrl;
#[doc = "AFRH (rw) register accessor: AFRH\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrh`] module"]
#[doc(alias = "AFRH")]
pub type Afrh = crate::Reg<afrh::AfrhSpec>;
#[doc = "AFRH"]
pub mod afrh;
