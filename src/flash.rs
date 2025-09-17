#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    acr: Acr,
    mkyr: Mkyr,
    e2kyr: E2kyr,
    sr: Sr,
    cr: Cr,
    cr1: Cr1,
    opr: Opr,
    rpr: Rpr,
    wrpr: Wrpr,
    wrpr1: Wrpr1,
    cntr: Cntr,
    upcntr: Upcntr,
    cntcr: Cntcr,
    ikyr: Ikyr,
    data0: Data0,
    data1: Data1,
    data2: Data2,
    _reserved17: [u8; 0xbc],
    memrmp: Memrmp,
    _reserved18: [u8; 0xfc],
    opr_cust1: OprCust1,
    opr_desi0: OprDesi0,
    opr_desi1: OprDesi1,
    opr_desi2: OprDesi2,
    opr_desi3: OprDesi3,
}
impl RegisterBlock {
    #[doc = "0x00 - ACR"]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0x04 - MKYR"]
    #[inline(always)]
    pub const fn mkyr(&self) -> &Mkyr {
        &self.mkyr
    }
    #[doc = "0x08 - E2KYR"]
    #[inline(always)]
    pub const fn e2kyr(&self) -> &E2kyr {
        &self.e2kyr
    }
    #[doc = "0x0c - SR"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x14 - CR1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x18 - OPR"]
    #[inline(always)]
    pub const fn opr(&self) -> &Opr {
        &self.opr
    }
    #[doc = "0x1c - RPR"]
    #[inline(always)]
    pub const fn rpr(&self) -> &Rpr {
        &self.rpr
    }
    #[doc = "0x20 - WRPR"]
    #[inline(always)]
    pub const fn wrpr(&self) -> &Wrpr {
        &self.wrpr
    }
    #[doc = "0x24 - WRPR1"]
    #[inline(always)]
    pub const fn wrpr1(&self) -> &Wrpr1 {
        &self.wrpr1
    }
    #[doc = "0x28 - CNTR"]
    #[inline(always)]
    pub const fn cntr(&self) -> &Cntr {
        &self.cntr
    }
    #[doc = "0x2c - UPCNTR"]
    #[inline(always)]
    pub const fn upcntr(&self) -> &Upcntr {
        &self.upcntr
    }
    #[doc = "0x30 - CNTCR"]
    #[inline(always)]
    pub const fn cntcr(&self) -> &Cntcr {
        &self.cntcr
    }
    #[doc = "0x34 - IKYR"]
    #[inline(always)]
    pub const fn ikyr(&self) -> &Ikyr {
        &self.ikyr
    }
    #[doc = "0x38 - DATA0"]
    #[inline(always)]
    pub const fn data0(&self) -> &Data0 {
        &self.data0
    }
    #[doc = "0x3c - DATA1"]
    #[inline(always)]
    pub const fn data1(&self) -> &Data1 {
        &self.data1
    }
    #[doc = "0x40 - DATA2"]
    #[inline(always)]
    pub const fn data2(&self) -> &Data2 {
        &self.data2
    }
    #[doc = "0x100 - MEMRMP"]
    #[inline(always)]
    pub const fn memrmp(&self) -> &Memrmp {
        &self.memrmp
    }
    #[doc = "0x200 - OPR_CUST1"]
    #[inline(always)]
    pub const fn opr_cust1(&self) -> &OprCust1 {
        &self.opr_cust1
    }
    #[doc = "0x204 - OPR_DESI0"]
    #[inline(always)]
    pub const fn opr_desi0(&self) -> &OprDesi0 {
        &self.opr_desi0
    }
    #[doc = "0x208 - OPR_DESI1"]
    #[inline(always)]
    pub const fn opr_desi1(&self) -> &OprDesi1 {
        &self.opr_desi1
    }
    #[doc = "0x20c - OPR_DESI2"]
    #[inline(always)]
    pub const fn opr_desi2(&self) -> &OprDesi2 {
        &self.opr_desi2
    }
    #[doc = "0x210 - OPR_DESI3"]
    #[inline(always)]
    pub const fn opr_desi3(&self) -> &OprDesi3 {
        &self.opr_desi3
    }
}
#[doc = "ACR (rw) register accessor: ACR\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`] module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "ACR"]
pub mod acr;
#[doc = "MKYR (w) register accessor: MKYR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mkyr`] module"]
#[doc(alias = "MKYR")]
pub type Mkyr = crate::Reg<mkyr::MkyrSpec>;
#[doc = "MKYR"]
pub mod mkyr;
#[doc = "E2KYR (w) register accessor: E2KYR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`e2kyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@e2kyr`] module"]
#[doc(alias = "E2KYR")]
pub type E2kyr = crate::Reg<e2kyr::E2kyrSpec>;
#[doc = "E2KYR"]
pub mod e2kyr;
#[doc = "SR (rw) register accessor: SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "SR"]
pub mod sr;
#[doc = "CR (rw) register accessor: CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "CR"]
pub mod cr;
#[doc = "CR1 (rw) register accessor: CR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "CR1"]
pub mod cr1;
#[doc = "OPR (r) register accessor: OPR\n\nYou can [`read`](crate::Reg::read) this register and get [`opr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opr`] module"]
#[doc(alias = "OPR")]
pub type Opr = crate::Reg<opr::OprSpec>;
#[doc = "OPR"]
pub mod opr;
#[doc = "RPR (r) register accessor: RPR\n\nYou can [`read`](crate::Reg::read) this register and get [`rpr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpr`] module"]
#[doc(alias = "RPR")]
pub type Rpr = crate::Reg<rpr::RprSpec>;
#[doc = "RPR"]
pub mod rpr;
#[doc = "WRPR (r) register accessor: WRPR\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpr`] module"]
#[doc(alias = "WRPR")]
pub type Wrpr = crate::Reg<wrpr::WrprSpec>;
#[doc = "WRPR"]
pub mod wrpr;
#[doc = "WRPR1 (r) register accessor: WRPR1\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpr1`] module"]
#[doc(alias = "WRPR1")]
pub type Wrpr1 = crate::Reg<wrpr1::Wrpr1Spec>;
#[doc = "WRPR1"]
pub mod wrpr1;
#[doc = "CNTR (rw) register accessor: CNTR\n\nYou can [`read`](crate::Reg::read) this register and get [`cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`] module"]
#[doc(alias = "CNTR")]
pub type Cntr = crate::Reg<cntr::CntrSpec>;
#[doc = "CNTR"]
pub mod cntr;
#[doc = "UPCNTR (rw) register accessor: UPCNTR\n\nYou can [`read`](crate::Reg::read) this register and get [`upcntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upcntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upcntr`] module"]
#[doc(alias = "UPCNTR")]
pub type Upcntr = crate::Reg<upcntr::UpcntrSpec>;
#[doc = "UPCNTR"]
pub mod upcntr;
#[doc = "CNTCR (rw) register accessor: CNTCR\n\nYou can [`read`](crate::Reg::read) this register and get [`cntcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntcr`] module"]
#[doc(alias = "CNTCR")]
pub type Cntcr = crate::Reg<cntcr::CntcrSpec>;
#[doc = "CNTCR"]
pub mod cntcr;
#[doc = "IKYR (w) register accessor: IKYR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ikyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ikyr`] module"]
#[doc(alias = "IKYR")]
pub type Ikyr = crate::Reg<ikyr::IkyrSpec>;
#[doc = "IKYR"]
pub mod ikyr;
#[doc = "DATA0 (rw) register accessor: DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0`] module"]
#[doc(alias = "DATA0")]
pub type Data0 = crate::Reg<data0::Data0Spec>;
#[doc = "DATA0"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: DATA1\n\nYou can [`read`](crate::Reg::read) this register and get [`data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1`] module"]
#[doc(alias = "DATA1")]
pub type Data1 = crate::Reg<data1::Data1Spec>;
#[doc = "DATA1"]
pub mod data1;
#[doc = "DATA2 (rw) register accessor: DATA2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2`] module"]
#[doc(alias = "DATA2")]
pub type Data2 = crate::Reg<data2::Data2Spec>;
#[doc = "DATA2"]
pub mod data2;
#[doc = "MEMRMP (rw) register accessor: MEMRMP\n\nYou can [`read`](crate::Reg::read) this register and get [`memrmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memrmp`] module"]
#[doc(alias = "MEMRMP")]
pub type Memrmp = crate::Reg<memrmp::MemrmpSpec>;
#[doc = "MEMRMP"]
pub mod memrmp;
#[doc = "OPR_CUST1 (r) register accessor: OPR_CUST1\n\nYou can [`read`](crate::Reg::read) this register and get [`opr_cust1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opr_cust1`] module"]
#[doc(alias = "OPR_CUST1")]
pub type OprCust1 = crate::Reg<opr_cust1::OprCust1Spec>;
#[doc = "OPR_CUST1"]
pub mod opr_cust1;
#[doc = "OPR_DESI0 (r) register accessor: OPR_DESI0\n\nYou can [`read`](crate::Reg::read) this register and get [`opr_desi0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opr_desi0`] module"]
#[doc(alias = "OPR_DESI0")]
pub type OprDesi0 = crate::Reg<opr_desi0::OprDesi0Spec>;
#[doc = "OPR_DESI0"]
pub mod opr_desi0;
#[doc = "OPR_DESI1 (r) register accessor: OPR_DESI1\n\nYou can [`read`](crate::Reg::read) this register and get [`opr_desi1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opr_desi1`] module"]
#[doc(alias = "OPR_DESI1")]
pub type OprDesi1 = crate::Reg<opr_desi1::OprDesi1Spec>;
#[doc = "OPR_DESI1"]
pub mod opr_desi1;
#[doc = "OPR_DESI2 (r) register accessor: OPR_DESI2\n\nYou can [`read`](crate::Reg::read) this register and get [`opr_desi2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opr_desi2`] module"]
#[doc(alias = "OPR_DESI2")]
pub type OprDesi2 = crate::Reg<opr_desi2::OprDesi2Spec>;
#[doc = "OPR_DESI2"]
pub mod opr_desi2;
#[doc = "OPR_DESI3 (r) register accessor: OPR_DESI3\n\nYou can [`read`](crate::Reg::read) this register and get [`opr_desi3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opr_desi3`] module"]
#[doc(alias = "OPR_DESI3")]
pub type OprDesi3 = crate::Reg<opr_desi3::OprDesi3Spec>;
#[doc = "OPR_DESI3"]
pub mod opr_desi3;
