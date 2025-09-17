#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cordcsr0: Cordcsr0,
    oprdx0: Oprdx0,
    oprdy0: Oprdy0,
    oprdz0: Oprdz0,
    cordcsr1: Cordcsr1,
    oprdx1: Oprdx1,
    oprdy1: Oprdy1,
    oprdz1: Oprdz1,
    _reserved8: [u8; 0xe0],
    divcsr0: Divcsr0,
    divdnd0: Divdnd0,
    divsor0: Divsor0,
    divrlt0: Divrlt0,
    divcsr1: Divcsr1,
    divdnd1: Divdnd1,
    divsor1: Divsor1,
    divrlt1: Divrlt1,
}
impl RegisterBlock {
    #[doc = "0x00 - CORDCSR0"]
    #[inline(always)]
    pub const fn cordcsr0(&self) -> &Cordcsr0 {
        &self.cordcsr0
    }
    #[doc = "0x04 - OPRDX0"]
    #[inline(always)]
    pub const fn oprdx0(&self) -> &Oprdx0 {
        &self.oprdx0
    }
    #[doc = "0x08 - OPRDY0"]
    #[inline(always)]
    pub const fn oprdy0(&self) -> &Oprdy0 {
        &self.oprdy0
    }
    #[doc = "0x0c - OPRDZ0"]
    #[inline(always)]
    pub const fn oprdz0(&self) -> &Oprdz0 {
        &self.oprdz0
    }
    #[doc = "0x10 - CORDCSR1"]
    #[inline(always)]
    pub const fn cordcsr1(&self) -> &Cordcsr1 {
        &self.cordcsr1
    }
    #[doc = "0x14 - OPRDX1"]
    #[inline(always)]
    pub const fn oprdx1(&self) -> &Oprdx1 {
        &self.oprdx1
    }
    #[doc = "0x18 - OPRDY1"]
    #[inline(always)]
    pub const fn oprdy1(&self) -> &Oprdy1 {
        &self.oprdy1
    }
    #[doc = "0x1c - OPRDZ1"]
    #[inline(always)]
    pub const fn oprdz1(&self) -> &Oprdz1 {
        &self.oprdz1
    }
    #[doc = "0x100 - DIVCSR0"]
    #[inline(always)]
    pub const fn divcsr0(&self) -> &Divcsr0 {
        &self.divcsr0
    }
    #[doc = "0x104 - DIVDND0"]
    #[inline(always)]
    pub const fn divdnd0(&self) -> &Divdnd0 {
        &self.divdnd0
    }
    #[doc = "0x108 - DIVSOR0"]
    #[inline(always)]
    pub const fn divsor0(&self) -> &Divsor0 {
        &self.divsor0
    }
    #[doc = "0x10c - DIVRLT0"]
    #[inline(always)]
    pub const fn divrlt0(&self) -> &Divrlt0 {
        &self.divrlt0
    }
    #[doc = "0x110 - DIVCSR1"]
    #[inline(always)]
    pub const fn divcsr1(&self) -> &Divcsr1 {
        &self.divcsr1
    }
    #[doc = "0x114 - DIVDND1"]
    #[inline(always)]
    pub const fn divdnd1(&self) -> &Divdnd1 {
        &self.divdnd1
    }
    #[doc = "0x118 - DIVSOR1"]
    #[inline(always)]
    pub const fn divsor1(&self) -> &Divsor1 {
        &self.divsor1
    }
    #[doc = "0x11c - DIVRLT1"]
    #[inline(always)]
    pub const fn divrlt1(&self) -> &Divrlt1 {
        &self.divrlt1
    }
}
#[doc = "CORDCSR0 (rw) register accessor: CORDCSR0\n\nYou can [`read`](crate::Reg::read) this register and get [`cordcsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordcsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cordcsr0`] module"]
#[doc(alias = "CORDCSR0")]
pub type Cordcsr0 = crate::Reg<cordcsr0::Cordcsr0Spec>;
#[doc = "CORDCSR0"]
pub mod cordcsr0;
#[doc = "OPRDX0 (rw) register accessor: OPRDX0\n\nYou can [`read`](crate::Reg::read) this register and get [`oprdx0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oprdx0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oprdx0`] module"]
#[doc(alias = "OPRDX0")]
pub type Oprdx0 = crate::Reg<oprdx0::Oprdx0Spec>;
#[doc = "OPRDX0"]
pub mod oprdx0;
#[doc = "OPRDY0 (rw) register accessor: OPRDY0\n\nYou can [`read`](crate::Reg::read) this register and get [`oprdy0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oprdy0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oprdy0`] module"]
#[doc(alias = "OPRDY0")]
pub type Oprdy0 = crate::Reg<oprdy0::Oprdy0Spec>;
#[doc = "OPRDY0"]
pub mod oprdy0;
#[doc = "OPRDZ0 (rw) register accessor: OPRDZ0\n\nYou can [`read`](crate::Reg::read) this register and get [`oprdz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oprdz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oprdz0`] module"]
#[doc(alias = "OPRDZ0")]
pub type Oprdz0 = crate::Reg<oprdz0::Oprdz0Spec>;
#[doc = "OPRDZ0"]
pub mod oprdz0;
#[doc = "CORDCSR1 (rw) register accessor: CORDCSR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cordcsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordcsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cordcsr1`] module"]
#[doc(alias = "CORDCSR1")]
pub type Cordcsr1 = crate::Reg<cordcsr1::Cordcsr1Spec>;
#[doc = "CORDCSR1"]
pub mod cordcsr1;
#[doc = "OPRDX1 (rw) register accessor: OPRDX1\n\nYou can [`read`](crate::Reg::read) this register and get [`oprdx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oprdx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oprdx1`] module"]
#[doc(alias = "OPRDX1")]
pub type Oprdx1 = crate::Reg<oprdx1::Oprdx1Spec>;
#[doc = "OPRDX1"]
pub mod oprdx1;
#[doc = "OPRDY1 (rw) register accessor: OPRDY1\n\nYou can [`read`](crate::Reg::read) this register and get [`oprdy1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oprdy1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oprdy1`] module"]
#[doc(alias = "OPRDY1")]
pub type Oprdy1 = crate::Reg<oprdy1::Oprdy1Spec>;
#[doc = "OPRDY1"]
pub mod oprdy1;
#[doc = "OPRDZ1 (rw) register accessor: OPRDZ1\n\nYou can [`read`](crate::Reg::read) this register and get [`oprdz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oprdz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oprdz1`] module"]
#[doc(alias = "OPRDZ1")]
pub type Oprdz1 = crate::Reg<oprdz1::Oprdz1Spec>;
#[doc = "OPRDZ1"]
pub mod oprdz1;
#[doc = "DIVCSR0 (rw) register accessor: DIVCSR0\n\nYou can [`read`](crate::Reg::read) this register and get [`divcsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divcsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divcsr0`] module"]
#[doc(alias = "DIVCSR0")]
pub type Divcsr0 = crate::Reg<divcsr0::Divcsr0Spec>;
#[doc = "DIVCSR0"]
pub mod divcsr0;
#[doc = "DIVDND0 (rw) register accessor: DIVDND0\n\nYou can [`read`](crate::Reg::read) this register and get [`divdnd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divdnd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divdnd0`] module"]
#[doc(alias = "DIVDND0")]
pub type Divdnd0 = crate::Reg<divdnd0::Divdnd0Spec>;
#[doc = "DIVDND0"]
pub mod divdnd0;
#[doc = "DIVSOR0 (rw) register accessor: DIVSOR0\n\nYou can [`read`](crate::Reg::read) this register and get [`divsor0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divsor0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divsor0`] module"]
#[doc(alias = "DIVSOR0")]
pub type Divsor0 = crate::Reg<divsor0::Divsor0Spec>;
#[doc = "DIVSOR0"]
pub mod divsor0;
#[doc = "DIVRLT0 (r) register accessor: DIVRLT0\n\nYou can [`read`](crate::Reg::read) this register and get [`divrlt0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divrlt0`] module"]
#[doc(alias = "DIVRLT0")]
pub type Divrlt0 = crate::Reg<divrlt0::Divrlt0Spec>;
#[doc = "DIVRLT0"]
pub mod divrlt0;
#[doc = "DIVCSR1 (rw) register accessor: DIVCSR1\n\nYou can [`read`](crate::Reg::read) this register and get [`divcsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divcsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divcsr1`] module"]
#[doc(alias = "DIVCSR1")]
pub type Divcsr1 = crate::Reg<divcsr1::Divcsr1Spec>;
#[doc = "DIVCSR1"]
pub mod divcsr1;
#[doc = "DIVDND1 (rw) register accessor: DIVDND1\n\nYou can [`read`](crate::Reg::read) this register and get [`divdnd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divdnd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divdnd1`] module"]
#[doc(alias = "DIVDND1")]
pub type Divdnd1 = crate::Reg<divdnd1::Divdnd1Spec>;
#[doc = "DIVDND1"]
pub mod divdnd1;
#[doc = "DIVSOR1 (rw) register accessor: DIVSOR1\n\nYou can [`read`](crate::Reg::read) this register and get [`divsor1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divsor1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divsor1`] module"]
#[doc(alias = "DIVSOR1")]
pub type Divsor1 = crate::Reg<divsor1::Divsor1Spec>;
#[doc = "DIVSOR1"]
pub mod divsor1;
#[doc = "DIVRLT1 (r) register accessor: DIVRLT1\n\nYou can [`read`](crate::Reg::read) this register and get [`divrlt1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divrlt1`] module"]
#[doc(alias = "DIVRLT1")]
pub type Divrlt1 = crate::Reg<divrlt1::Divrlt1Spec>;
#[doc = "DIVRLT1"]
pub mod divrlt1;
