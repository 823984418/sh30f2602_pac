#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwrcr: Pwrcr,
    pwrsr: Pwrsr,
    safr: Safr,
    _reserved3: [u8; 0x04],
    dbgcr: Dbgcr,
    gpiobcr: Gpiobcr,
    _reserved5: [u8; 0x68],
    hldocr: Hldocr,
}
impl RegisterBlock {
    #[doc = "0x00 - PWRCR"]
    #[inline(always)]
    pub const fn pwrcr(&self) -> &Pwrcr {
        &self.pwrcr
    }
    #[doc = "0x04 - PWRSR"]
    #[inline(always)]
    pub const fn pwrsr(&self) -> &Pwrsr {
        &self.pwrsr
    }
    #[doc = "0x08 - SAFR"]
    #[inline(always)]
    pub const fn safr(&self) -> &Safr {
        &self.safr
    }
    #[doc = "0x10 - DBGCR"]
    #[inline(always)]
    pub const fn dbgcr(&self) -> &Dbgcr {
        &self.dbgcr
    }
    #[doc = "0x14 - GPIOBCR"]
    #[inline(always)]
    pub const fn gpiobcr(&self) -> &Gpiobcr {
        &self.gpiobcr
    }
    #[doc = "0x80 - HLDOCR"]
    #[inline(always)]
    pub const fn hldocr(&self) -> &Hldocr {
        &self.hldocr
    }
}
#[doc = "PWRCR (rw) register accessor: PWRCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcr`] module"]
#[doc(alias = "PWRCR")]
pub type Pwrcr = crate::Reg<pwrcr::PwrcrSpec>;
#[doc = "PWRCR"]
pub mod pwrcr;
#[doc = "PWRSR (rw) register accessor: PWRSR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrsr`] module"]
#[doc(alias = "PWRSR")]
pub type Pwrsr = crate::Reg<pwrsr::PwrsrSpec>;
#[doc = "PWRSR"]
pub mod pwrsr;
#[doc = "SAFR (rw) register accessor: SAFR\n\nYou can [`read`](crate::Reg::read) this register and get [`safr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`safr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@safr`] module"]
#[doc(alias = "SAFR")]
pub type Safr = crate::Reg<safr::SafrSpec>;
#[doc = "SAFR"]
pub mod safr;
#[doc = "DBGCR (rw) register accessor: DBGCR\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgcr`] module"]
#[doc(alias = "DBGCR")]
pub type Dbgcr = crate::Reg<dbgcr::DbgcrSpec>;
#[doc = "DBGCR"]
pub mod dbgcr;
#[doc = "GPIOBCR (rw) register accessor: GPIOBCR\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiobcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiobcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiobcr`] module"]
#[doc(alias = "GPIOBCR")]
pub type Gpiobcr = crate::Reg<gpiobcr::GpiobcrSpec>;
#[doc = "GPIOBCR"]
pub mod gpiobcr;
#[doc = "HLDOCR (rw) register accessor: HLDOCR\n\nYou can [`read`](crate::Reg::read) this register and get [`hldocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hldocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hldocr`] module"]
#[doc(alias = "HLDOCR")]
pub type Hldocr = crate::Reg<hldocr::HldocrSpec>;
#[doc = "HLDOCR"]
pub mod hldocr;
