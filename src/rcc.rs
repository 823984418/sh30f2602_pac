#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    cfgr: Cfgr,
    cienr: Cienr,
    cistr: Cistr,
    ciclr: Ciclr,
    ahbrstr: Ahbrstr,
    _reserved6: [u8; 0x04],
    apb0rstr: Apb0rstr,
    ahbenr: Ahbenr,
    _reserved8: [u8; 0x04],
    apb0enr: Apb0enr,
    rststr: Rststr,
    rstclr: Rstclr,
    _reserved11: [u8; 0x04],
    rcclock: Rcclock,
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
    #[doc = "0x08 - CIENR"]
    #[inline(always)]
    pub const fn cienr(&self) -> &Cienr {
        &self.cienr
    }
    #[doc = "0x0c - CISTR"]
    #[inline(always)]
    pub const fn cistr(&self) -> &Cistr {
        &self.cistr
    }
    #[doc = "0x10 - CICLR"]
    #[inline(always)]
    pub const fn ciclr(&self) -> &Ciclr {
        &self.ciclr
    }
    #[doc = "0x14 - AHBRSTR"]
    #[inline(always)]
    pub const fn ahbrstr(&self) -> &Ahbrstr {
        &self.ahbrstr
    }
    #[doc = "0x1c - APB0RSTR"]
    #[inline(always)]
    pub const fn apb0rstr(&self) -> &Apb0rstr {
        &self.apb0rstr
    }
    #[doc = "0x20 - AHBENR"]
    #[inline(always)]
    pub const fn ahbenr(&self) -> &Ahbenr {
        &self.ahbenr
    }
    #[doc = "0x28 - APB0ENR"]
    #[inline(always)]
    pub const fn apb0enr(&self) -> &Apb0enr {
        &self.apb0enr
    }
    #[doc = "0x2c - RSTSTR"]
    #[inline(always)]
    pub const fn rststr(&self) -> &Rststr {
        &self.rststr
    }
    #[doc = "0x30 - RSTCLR"]
    #[inline(always)]
    pub const fn rstclr(&self) -> &Rstclr {
        &self.rstclr
    }
    #[doc = "0x38 - RCCLOCK"]
    #[inline(always)]
    pub const fn rcclock(&self) -> &Rcclock {
        &self.rcclock
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
#[doc = "CIENR (rw) register accessor: CIENR\n\nYou can [`read`](crate::Reg::read) this register and get [`cienr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cienr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cienr`] module"]
#[doc(alias = "CIENR")]
pub type Cienr = crate::Reg<cienr::CienrSpec>;
#[doc = "CIENR"]
pub mod cienr;
#[doc = "CISTR (r) register accessor: CISTR\n\nYou can [`read`](crate::Reg::read) this register and get [`cistr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cistr`] module"]
#[doc(alias = "CISTR")]
pub type Cistr = crate::Reg<cistr::CistrSpec>;
#[doc = "CISTR"]
pub mod cistr;
#[doc = "CICLR (rw) register accessor: CICLR\n\nYou can [`read`](crate::Reg::read) this register and get [`ciclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ciclr`] module"]
#[doc(alias = "CICLR")]
pub type Ciclr = crate::Reg<ciclr::CiclrSpec>;
#[doc = "CICLR"]
pub mod ciclr;
#[doc = "AHBRSTR (rw) register accessor: AHBRSTR\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrstr`] module"]
#[doc(alias = "AHBRSTR")]
pub type Ahbrstr = crate::Reg<ahbrstr::AhbrstrSpec>;
#[doc = "AHBRSTR"]
pub mod ahbrstr;
#[doc = "APB0RSTR (rw) register accessor: APB0RSTR\n\nYou can [`read`](crate::Reg::read) this register and get [`apb0rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb0rstr`] module"]
#[doc(alias = "APB0RSTR")]
pub type Apb0rstr = crate::Reg<apb0rstr::Apb0rstrSpec>;
#[doc = "APB0RSTR"]
pub mod apb0rstr;
#[doc = "AHBENR (rw) register accessor: AHBENR\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbenr`] module"]
#[doc(alias = "AHBENR")]
pub type Ahbenr = crate::Reg<ahbenr::AhbenrSpec>;
#[doc = "AHBENR"]
pub mod ahbenr;
#[doc = "APB0ENR (rw) register accessor: APB0ENR\n\nYou can [`read`](crate::Reg::read) this register and get [`apb0enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb0enr`] module"]
#[doc(alias = "APB0ENR")]
pub type Apb0enr = crate::Reg<apb0enr::Apb0enrSpec>;
#[doc = "APB0ENR"]
pub mod apb0enr;
#[doc = "RSTSTR (r) register accessor: RSTSTR\n\nYou can [`read`](crate::Reg::read) this register and get [`rststr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rststr`] module"]
#[doc(alias = "RSTSTR")]
pub type Rststr = crate::Reg<rststr::RststrSpec>;
#[doc = "RSTSTR"]
pub mod rststr;
#[doc = "RSTCLR (rw) register accessor: RSTCLR\n\nYou can [`read`](crate::Reg::read) this register and get [`rstclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstclr`] module"]
#[doc(alias = "RSTCLR")]
pub type Rstclr = crate::Reg<rstclr::RstclrSpec>;
#[doc = "RSTCLR"]
pub mod rstclr;
#[doc = "RCCLOCK (rw) register accessor: RCCLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`rcclock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcclock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcclock`] module"]
#[doc(alias = "RCCLOCK")]
pub type Rcclock = crate::Reg<rcclock::RcclockSpec>;
#[doc = "RCCLOCK"]
pub mod rcclock;
