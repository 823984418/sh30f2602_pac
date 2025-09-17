#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    imr: Imr,
    emr: Emr,
    tmsr: Tmsr,
    rtsr: Rtsr,
    ftsr: Ftsr,
    swier: Swier,
    pr: Pr,
    cfg: Cfg,
    cfgh: Cfgh,
    samp: Samp,
}
impl RegisterBlock {
    #[doc = "0x00 - IMR"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x04 - EMR"]
    #[inline(always)]
    pub const fn emr(&self) -> &Emr {
        &self.emr
    }
    #[doc = "0x08 - TMSR"]
    #[inline(always)]
    pub const fn tmsr(&self) -> &Tmsr {
        &self.tmsr
    }
    #[doc = "0x0c - RTSR"]
    #[inline(always)]
    pub const fn rtsr(&self) -> &Rtsr {
        &self.rtsr
    }
    #[doc = "0x10 - FTSR"]
    #[inline(always)]
    pub const fn ftsr(&self) -> &Ftsr {
        &self.ftsr
    }
    #[doc = "0x14 - SWIER"]
    #[inline(always)]
    pub const fn swier(&self) -> &Swier {
        &self.swier
    }
    #[doc = "0x18 - PR"]
    #[inline(always)]
    pub const fn pr(&self) -> &Pr {
        &self.pr
    }
    #[doc = "0x1c - CFG"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x20 - CFGH"]
    #[inline(always)]
    pub const fn cfgh(&self) -> &Cfgh {
        &self.cfgh
    }
    #[doc = "0x24 - SAMP"]
    #[inline(always)]
    pub const fn samp(&self) -> &Samp {
        &self.samp
    }
}
#[doc = "IMR (rw) register accessor: IMR\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "IMR"]
pub mod imr;
#[doc = "EMR (rw) register accessor: EMR\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr`] module"]
#[doc(alias = "EMR")]
pub type Emr = crate::Reg<emr::EmrSpec>;
#[doc = "EMR"]
pub mod emr;
#[doc = "TMSR (rw) register accessor: TMSR\n\nYou can [`read`](crate::Reg::read) this register and get [`tmsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmsr`] module"]
#[doc(alias = "TMSR")]
pub type Tmsr = crate::Reg<tmsr::TmsrSpec>;
#[doc = "TMSR"]
pub mod tmsr;
#[doc = "RTSR (rw) register accessor: RTSR\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr`] module"]
#[doc(alias = "RTSR")]
pub type Rtsr = crate::Reg<rtsr::RtsrSpec>;
#[doc = "RTSR"]
pub mod rtsr;
#[doc = "FTSR (rw) register accessor: FTSR\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr`] module"]
#[doc(alias = "FTSR")]
pub type Ftsr = crate::Reg<ftsr::FtsrSpec>;
#[doc = "FTSR"]
pub mod ftsr;
#[doc = "SWIER (w) register accessor: SWIER\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier`] module"]
#[doc(alias = "SWIER")]
pub type Swier = crate::Reg<swier::SwierSpec>;
#[doc = "SWIER"]
pub mod swier;
#[doc = "PR (rw) register accessor: PR\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`] module"]
#[doc(alias = "PR")]
pub type Pr = crate::Reg<pr::PrSpec>;
#[doc = "PR"]
pub mod pr;
#[doc = "CFG (rw) register accessor: CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "CFG"]
pub mod cfg;
#[doc = "CFGH (rw) register accessor: CFGH\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgh`] module"]
#[doc(alias = "CFGH")]
pub type Cfgh = crate::Reg<cfgh::CfghSpec>;
#[doc = "CFGH"]
pub mod cfgh;
#[doc = "SAMP (rw) register accessor: SAMP\n\nYou can [`read`](crate::Reg::read) this register and get [`samp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@samp`] module"]
#[doc(alias = "SAMP")]
pub type Samp = crate::Reg<samp::SampSpec>;
#[doc = "SAMP"]
pub mod samp;
