#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    cfgr: Cfgr,
    sr: Sr,
    force: Force,
    cnt: Cnt,
    psc: Psc,
    pr: Pr,
    ccmr0: Ccmr0,
    ccmr1: Ccmr1,
    ccmr2: Ccmr2,
    ccr0: Ccr0,
    ccr1: Ccr1,
    ccr2: Ccr2,
    lckr: Lckr,
    adtr: Adtr,
    fltwpr: Fltwpr,
    fltcr: Fltcr,
}
impl RegisterBlock {
    #[doc = "0x00 - CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - CFGR"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x08 - SR"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x0c - FORCE"]
    #[inline(always)]
    pub const fn force(&self) -> &Force {
        &self.force
    }
    #[doc = "0x10 - CNT"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x14 - PSC"]
    #[inline(always)]
    pub const fn psc(&self) -> &Psc {
        &self.psc
    }
    #[doc = "0x18 - PR"]
    #[inline(always)]
    pub const fn pr(&self) -> &Pr {
        &self.pr
    }
    #[doc = "0x1c - CCMR0"]
    #[inline(always)]
    pub const fn ccmr0(&self) -> &Ccmr0 {
        &self.ccmr0
    }
    #[doc = "0x20 - CCMR1"]
    #[inline(always)]
    pub const fn ccmr1(&self) -> &Ccmr1 {
        &self.ccmr1
    }
    #[doc = "0x24 - CCMR2"]
    #[inline(always)]
    pub const fn ccmr2(&self) -> &Ccmr2 {
        &self.ccmr2
    }
    #[doc = "0x28 - CCR0"]
    #[inline(always)]
    pub const fn ccr0(&self) -> &Ccr0 {
        &self.ccr0
    }
    #[doc = "0x2c - CCR1"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    #[doc = "0x30 - CCR2"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    #[doc = "0x34 - LCKR"]
    #[inline(always)]
    pub const fn lckr(&self) -> &Lckr {
        &self.lckr
    }
    #[doc = "0x38 - ADTR"]
    #[inline(always)]
    pub const fn adtr(&self) -> &Adtr {
        &self.adtr
    }
    #[doc = "0x3c - FLTWPR"]
    #[inline(always)]
    pub const fn fltwpr(&self) -> &Fltwpr {
        &self.fltwpr
    }
    #[doc = "0x40 - FLTCR"]
    #[inline(always)]
    pub const fn fltcr(&self) -> &Fltcr {
        &self.fltcr
    }
}
#[doc = "CR (rw) register accessor: CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "CR"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: CFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "CFGR"]
pub mod cfgr;
#[doc = "SR (rw) register accessor: SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "SR"]
pub mod sr;
#[doc = "FORCE (rw) register accessor: FORCE\n\nYou can [`read`](crate::Reg::read) this register and get [`force::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force`] module"]
#[doc(alias = "FORCE")]
pub type Force = crate::Reg<force::ForceSpec>;
#[doc = "FORCE"]
pub mod force;
#[doc = "CNT (rw) register accessor: CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "CNT"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: PSC\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`] module"]
#[doc(alias = "PSC")]
pub type Psc = crate::Reg<psc::PscSpec>;
#[doc = "PSC"]
pub mod psc;
#[doc = "PR (rw) register accessor: PR\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`] module"]
#[doc(alias = "PR")]
pub type Pr = crate::Reg<pr::PrSpec>;
#[doc = "PR"]
pub mod pr;
#[doc = "CCMR0 (rw) register accessor: CCMR0\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr0`] module"]
#[doc(alias = "CCMR0")]
pub type Ccmr0 = crate::Reg<ccmr0::Ccmr0Spec>;
#[doc = "CCMR0"]
pub mod ccmr0;
#[doc = "CCMR1 (rw) register accessor: CCMR1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1`] module"]
#[doc(alias = "CCMR1")]
pub type Ccmr1 = crate::Reg<ccmr1::Ccmr1Spec>;
#[doc = "CCMR1"]
pub mod ccmr1;
#[doc = "CCMR2 (rw) register accessor: CCMR2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2`] module"]
#[doc(alias = "CCMR2")]
pub type Ccmr2 = crate::Reg<ccmr2::Ccmr2Spec>;
#[doc = "CCMR2"]
pub mod ccmr2;
#[doc = "CCR0 (rw) register accessor: CCR0\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr0`] module"]
#[doc(alias = "CCR0")]
pub type Ccr0 = crate::Reg<ccr0::Ccr0Spec>;
#[doc = "CCR0"]
pub mod ccr0;
#[doc = "CCR1 (rw) register accessor: CCR1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`] module"]
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::Ccr1Spec>;
#[doc = "CCR1"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: CCR2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`] module"]
#[doc(alias = "CCR2")]
pub type Ccr2 = crate::Reg<ccr2::Ccr2Spec>;
#[doc = "CCR2"]
pub mod ccr2;
#[doc = "LCKR (rw) register accessor: LCKR\n\nYou can [`read`](crate::Reg::read) this register and get [`lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckr`] module"]
#[doc(alias = "LCKR")]
pub type Lckr = crate::Reg<lckr::LckrSpec>;
#[doc = "LCKR"]
pub mod lckr;
#[doc = "ADTR (rw) register accessor: ADTR\n\nYou can [`read`](crate::Reg::read) this register and get [`adtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adtr`] module"]
#[doc(alias = "ADTR")]
pub type Adtr = crate::Reg<adtr::AdtrSpec>;
#[doc = "ADTR"]
pub mod adtr;
#[doc = "FLTWPR (rw) register accessor: FLTWPR\n\nYou can [`read`](crate::Reg::read) this register and get [`fltwpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltwpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltwpr`] module"]
#[doc(alias = "FLTWPR")]
pub type Fltwpr = crate::Reg<fltwpr::FltwprSpec>;
#[doc = "FLTWPR"]
pub mod fltwpr;
#[doc = "FLTCR (rw) register accessor: FLTCR\n\nYou can [`read`](crate::Reg::read) this register and get [`fltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltcr`] module"]
#[doc(alias = "FLTCR")]
pub type Fltcr = crate::Reg<fltcr::FltcrSpec>;
#[doc = "FLTCR"]
pub mod fltcr;
