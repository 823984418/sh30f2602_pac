#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    tcnt: Tcnt,
    tpr: Tpr,
    psq: Psq,
    timintf: Timintf,
}
impl RegisterBlock {
    #[doc = "0x00 - CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - TCNT"]
    #[inline(always)]
    pub const fn tcnt(&self) -> &Tcnt {
        &self.tcnt
    }
    #[doc = "0x08 - TPR"]
    #[inline(always)]
    pub const fn tpr(&self) -> &Tpr {
        &self.tpr
    }
    #[doc = "0x0c - PSQ"]
    #[inline(always)]
    pub const fn psq(&self) -> &Psq {
        &self.psq
    }
    #[doc = "0x10 - TIMINTF"]
    #[inline(always)]
    pub const fn timintf(&self) -> &Timintf {
        &self.timintf
    }
}
#[doc = "CR (rw) register accessor: CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "CR"]
pub mod cr;
#[doc = "TCNT (rw) register accessor: TCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`tcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt`] module"]
#[doc(alias = "TCNT")]
pub type Tcnt = crate::Reg<tcnt::TcntSpec>;
#[doc = "TCNT"]
pub mod tcnt;
#[doc = "TPR (rw) register accessor: TPR\n\nYou can [`read`](crate::Reg::read) this register and get [`tpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpr`] module"]
#[doc(alias = "TPR")]
pub type Tpr = crate::Reg<tpr::TprSpec>;
#[doc = "TPR"]
pub mod tpr;
#[doc = "PSQ (rw) register accessor: PSQ\n\nYou can [`read`](crate::Reg::read) this register and get [`psq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psq`] module"]
#[doc(alias = "PSQ")]
pub type Psq = crate::Reg<psq::PsqSpec>;
#[doc = "PSQ"]
pub mod psq;
#[doc = "TIMINTF (rw) register accessor: TIMINTF\n\nYou can [`read`](crate::Reg::read) this register and get [`timintf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timintf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timintf`] module"]
#[doc(alias = "TIMINTF")]
pub type Timintf = crate::Reg<timintf::TimintfSpec>;
#[doc = "TIMINTF"]
pub mod timintf;
