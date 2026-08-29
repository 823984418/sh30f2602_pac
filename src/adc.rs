#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adcon1: Adcon1,
    adcon2: Adcon2,
    adcon3: Adcon3,
    adpch: Adpch,
    addr0: Addr0,
    addr1: Addr1,
    addr2: Addr2,
    addr3: Addr3,
    addr4: Addr4,
    addr5: Addr5,
    addr6: Addr6,
    addr7: Addr7,
    addr8: Addr8,
    addr9: Addr9,
    addr10: Addr10,
    addr11: Addr11,
    addr12: Addr12,
    addr13: Addr13,
    addr14: Addr14,
    addr15: Addr15,
    _reserved20: [u8; 0x0c],
    seqchsel0: Seqchsel0,
    seqchsel1: Seqchsel1,
    adgapon: Adgapon,
    adintf: Adintf,
}
impl RegisterBlock {
    #[doc = "0x00 - ADCON1"]
    #[inline(always)]
    pub const fn adcon1(&self) -> &Adcon1 {
        &self.adcon1
    }
    #[doc = "0x04 - ADCON2"]
    #[inline(always)]
    pub const fn adcon2(&self) -> &Adcon2 {
        &self.adcon2
    }
    #[doc = "0x08 - ADCON3"]
    #[inline(always)]
    pub const fn adcon3(&self) -> &Adcon3 {
        &self.adcon3
    }
    #[doc = "0x0c - ADPCH"]
    #[inline(always)]
    pub const fn adpch(&self) -> &Adpch {
        &self.adpch
    }
    #[doc = "0x10 - ADDR0"]
    #[inline(always)]
    pub const fn addr0(&self) -> &Addr0 {
        &self.addr0
    }
    #[doc = "0x14 - ADDR1"]
    #[inline(always)]
    pub const fn addr1(&self) -> &Addr1 {
        &self.addr1
    }
    #[doc = "0x18 - ADDR2"]
    #[inline(always)]
    pub const fn addr2(&self) -> &Addr2 {
        &self.addr2
    }
    #[doc = "0x1c - ADDR3"]
    #[inline(always)]
    pub const fn addr3(&self) -> &Addr3 {
        &self.addr3
    }
    #[doc = "0x20 - ADDR4"]
    #[inline(always)]
    pub const fn addr4(&self) -> &Addr4 {
        &self.addr4
    }
    #[doc = "0x24 - ADDR5"]
    #[inline(always)]
    pub const fn addr5(&self) -> &Addr5 {
        &self.addr5
    }
    #[doc = "0x28 - ADDR6"]
    #[inline(always)]
    pub const fn addr6(&self) -> &Addr6 {
        &self.addr6
    }
    #[doc = "0x2c - ADDR7"]
    #[inline(always)]
    pub const fn addr7(&self) -> &Addr7 {
        &self.addr7
    }
    #[doc = "0x30 - ADDR8"]
    #[inline(always)]
    pub const fn addr8(&self) -> &Addr8 {
        &self.addr8
    }
    #[doc = "0x34 - ADDR9"]
    #[inline(always)]
    pub const fn addr9(&self) -> &Addr9 {
        &self.addr9
    }
    #[doc = "0x38 - ADDR10"]
    #[inline(always)]
    pub const fn addr10(&self) -> &Addr10 {
        &self.addr10
    }
    #[doc = "0x3c - ADDR11"]
    #[inline(always)]
    pub const fn addr11(&self) -> &Addr11 {
        &self.addr11
    }
    #[doc = "0x40 - ADDR12"]
    #[inline(always)]
    pub const fn addr12(&self) -> &Addr12 {
        &self.addr12
    }
    #[doc = "0x44 - ADDR13"]
    #[inline(always)]
    pub const fn addr13(&self) -> &Addr13 {
        &self.addr13
    }
    #[doc = "0x48 - ADDR14"]
    #[inline(always)]
    pub const fn addr14(&self) -> &Addr14 {
        &self.addr14
    }
    #[doc = "0x4c - ADDR15"]
    #[inline(always)]
    pub const fn addr15(&self) -> &Addr15 {
        &self.addr15
    }
    #[doc = "0x5c - SEQCHSEL0"]
    #[inline(always)]
    pub const fn seqchsel0(&self) -> &Seqchsel0 {
        &self.seqchsel0
    }
    #[doc = "0x60 - SEQCHSEL1"]
    #[inline(always)]
    pub const fn seqchsel1(&self) -> &Seqchsel1 {
        &self.seqchsel1
    }
    #[doc = "0x64 - ADGAPON"]
    #[inline(always)]
    pub const fn adgapon(&self) -> &Adgapon {
        &self.adgapon
    }
    #[doc = "0x68 - ADINTF"]
    #[inline(always)]
    pub const fn adintf(&self) -> &Adintf {
        &self.adintf
    }
}
#[doc = "ADCON1 (rw) register accessor: ADCON1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcon1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcon1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcon1`] module"]
#[doc(alias = "ADCON1")]
pub type Adcon1 = crate::Reg<adcon1::Adcon1Spec>;
#[doc = "ADCON1"]
pub mod adcon1;
#[doc = "ADCON2 (rw) register accessor: ADCON2\n\nYou can [`read`](crate::Reg::read) this register and get [`adcon2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcon2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcon2`] module"]
#[doc(alias = "ADCON2")]
pub type Adcon2 = crate::Reg<adcon2::Adcon2Spec>;
#[doc = "ADCON2"]
pub mod adcon2;
#[doc = "ADCON3 (rw) register accessor: ADCON3\n\nYou can [`read`](crate::Reg::read) this register and get [`adcon3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcon3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcon3`] module"]
#[doc(alias = "ADCON3")]
pub type Adcon3 = crate::Reg<adcon3::Adcon3Spec>;
#[doc = "ADCON3"]
pub mod adcon3;
#[doc = "ADPCH (r) register accessor: ADPCH\n\nYou can [`read`](crate::Reg::read) this register and get [`adpch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adpch`] module"]
#[doc(alias = "ADPCH")]
pub type Adpch = crate::Reg<adpch::AdpchSpec>;
#[doc = "ADPCH"]
pub mod adpch;
#[doc = "ADDR0 (r) register accessor: ADDR0\n\nYou can [`read`](crate::Reg::read) this register and get [`addr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr0`] module"]
#[doc(alias = "ADDR0")]
pub type Addr0 = crate::Reg<addr0::Addr0Spec>;
#[doc = "ADDR0"]
pub mod addr0;
#[doc = "ADDR1 (r) register accessor: ADDR1\n\nYou can [`read`](crate::Reg::read) this register and get [`addr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr1`] module"]
#[doc(alias = "ADDR1")]
pub type Addr1 = crate::Reg<addr1::Addr1Spec>;
#[doc = "ADDR1"]
pub mod addr1;
#[doc = "ADDR2 (r) register accessor: ADDR2\n\nYou can [`read`](crate::Reg::read) this register and get [`addr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr2`] module"]
#[doc(alias = "ADDR2")]
pub type Addr2 = crate::Reg<addr2::Addr2Spec>;
#[doc = "ADDR2"]
pub mod addr2;
#[doc = "ADDR3 (r) register accessor: ADDR3\n\nYou can [`read`](crate::Reg::read) this register and get [`addr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr3`] module"]
#[doc(alias = "ADDR3")]
pub type Addr3 = crate::Reg<addr3::Addr3Spec>;
#[doc = "ADDR3"]
pub mod addr3;
#[doc = "ADDR4 (r) register accessor: ADDR4\n\nYou can [`read`](crate::Reg::read) this register and get [`addr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr4`] module"]
#[doc(alias = "ADDR4")]
pub type Addr4 = crate::Reg<addr4::Addr4Spec>;
#[doc = "ADDR4"]
pub mod addr4;
#[doc = "ADDR5 (r) register accessor: ADDR5\n\nYou can [`read`](crate::Reg::read) this register and get [`addr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr5`] module"]
#[doc(alias = "ADDR5")]
pub type Addr5 = crate::Reg<addr5::Addr5Spec>;
#[doc = "ADDR5"]
pub mod addr5;
#[doc = "ADDR6 (r) register accessor: ADDR6\n\nYou can [`read`](crate::Reg::read) this register and get [`addr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr6`] module"]
#[doc(alias = "ADDR6")]
pub type Addr6 = crate::Reg<addr6::Addr6Spec>;
#[doc = "ADDR6"]
pub mod addr6;
#[doc = "ADDR7 (r) register accessor: ADDR7\n\nYou can [`read`](crate::Reg::read) this register and get [`addr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr7`] module"]
#[doc(alias = "ADDR7")]
pub type Addr7 = crate::Reg<addr7::Addr7Spec>;
#[doc = "ADDR7"]
pub mod addr7;
#[doc = "ADDR8 (r) register accessor: ADDR8\n\nYou can [`read`](crate::Reg::read) this register and get [`addr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr8`] module"]
#[doc(alias = "ADDR8")]
pub type Addr8 = crate::Reg<addr8::Addr8Spec>;
#[doc = "ADDR8"]
pub mod addr8;
#[doc = "ADDR9 (r) register accessor: ADDR9\n\nYou can [`read`](crate::Reg::read) this register and get [`addr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr9`] module"]
#[doc(alias = "ADDR9")]
pub type Addr9 = crate::Reg<addr9::Addr9Spec>;
#[doc = "ADDR9"]
pub mod addr9;
#[doc = "ADDR10 (r) register accessor: ADDR10\n\nYou can [`read`](crate::Reg::read) this register and get [`addr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr10`] module"]
#[doc(alias = "ADDR10")]
pub type Addr10 = crate::Reg<addr10::Addr10Spec>;
#[doc = "ADDR10"]
pub mod addr10;
#[doc = "ADDR11 (r) register accessor: ADDR11\n\nYou can [`read`](crate::Reg::read) this register and get [`addr11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr11`] module"]
#[doc(alias = "ADDR11")]
pub type Addr11 = crate::Reg<addr11::Addr11Spec>;
#[doc = "ADDR11"]
pub mod addr11;
#[doc = "ADDR12 (r) register accessor: ADDR12\n\nYou can [`read`](crate::Reg::read) this register and get [`addr12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr12`] module"]
#[doc(alias = "ADDR12")]
pub type Addr12 = crate::Reg<addr12::Addr12Spec>;
#[doc = "ADDR12"]
pub mod addr12;
#[doc = "ADDR13 (r) register accessor: ADDR13\n\nYou can [`read`](crate::Reg::read) this register and get [`addr13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr13`] module"]
#[doc(alias = "ADDR13")]
pub type Addr13 = crate::Reg<addr13::Addr13Spec>;
#[doc = "ADDR13"]
pub mod addr13;
#[doc = "ADDR14 (r) register accessor: ADDR14\n\nYou can [`read`](crate::Reg::read) this register and get [`addr14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr14`] module"]
#[doc(alias = "ADDR14")]
pub type Addr14 = crate::Reg<addr14::Addr14Spec>;
#[doc = "ADDR14"]
pub mod addr14;
#[doc = "ADDR15 (r) register accessor: ADDR15\n\nYou can [`read`](crate::Reg::read) this register and get [`addr15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr15`] module"]
#[doc(alias = "ADDR15")]
pub type Addr15 = crate::Reg<addr15::Addr15Spec>;
#[doc = "ADDR15"]
pub mod addr15;
#[doc = "SEQCHSEL0 (rw) register accessor: SEQCHSEL0\n\nYou can [`read`](crate::Reg::read) this register and get [`seqchsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqchsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqchsel0`] module"]
#[doc(alias = "SEQCHSEL0")]
pub type Seqchsel0 = crate::Reg<seqchsel0::Seqchsel0Spec>;
#[doc = "SEQCHSEL0"]
pub mod seqchsel0;
#[doc = "SEQCHSEL1 (rw) register accessor: SEQCHSEL1\n\nYou can [`read`](crate::Reg::read) this register and get [`seqchsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqchsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqchsel1`] module"]
#[doc(alias = "SEQCHSEL1")]
pub type Seqchsel1 = crate::Reg<seqchsel1::Seqchsel1Spec>;
#[doc = "SEQCHSEL1"]
pub mod seqchsel1;
#[doc = "ADGAPON (rw) register accessor: ADGAPON\n\nYou can [`read`](crate::Reg::read) this register and get [`adgapon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adgapon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adgapon`] module"]
#[doc(alias = "ADGAPON")]
pub type Adgapon = crate::Reg<adgapon::AdgaponSpec>;
#[doc = "ADGAPON"]
pub mod adgapon;
#[doc = "ADINTF (rw) register accessor: ADINTF\n\nYou can [`read`](crate::Reg::read) this register and get [`adintf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adintf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adintf`] module"]
#[doc(alias = "ADINTF")]
pub type Adintf = crate::Reg<adintf::AdintfSpec>;
#[doc = "ADINTF"]
pub mod adintf;
