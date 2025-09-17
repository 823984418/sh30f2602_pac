#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmp0con: Cmp0con,
    cmp1con: Cmp1con,
    cmpintf: Cmpintf,
    opcon: Opcon,
    cmp2con: Cmp2con,
}
impl RegisterBlock {
    #[doc = "0x00 - CMP0CON"]
    #[inline(always)]
    pub const fn cmp0con(&self) -> &Cmp0con {
        &self.cmp0con
    }
    #[doc = "0x04 - CMP1CON"]
    #[inline(always)]
    pub const fn cmp1con(&self) -> &Cmp1con {
        &self.cmp1con
    }
    #[doc = "0x08 - CMPINTF"]
    #[inline(always)]
    pub const fn cmpintf(&self) -> &Cmpintf {
        &self.cmpintf
    }
    #[doc = "0x0c - OPCON"]
    #[inline(always)]
    pub const fn opcon(&self) -> &Opcon {
        &self.opcon
    }
    #[doc = "0x10 - CMP2CON"]
    #[inline(always)]
    pub const fn cmp2con(&self) -> &Cmp2con {
        &self.cmp2con
    }
}
#[doc = "CMP0CON (rw) register accessor: CMP0CON\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp0con::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp0con::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp0con`] module"]
#[doc(alias = "CMP0CON")]
pub type Cmp0con = crate::Reg<cmp0con::Cmp0conSpec>;
#[doc = "CMP0CON"]
pub mod cmp0con;
#[doc = "CMP1CON (rw) register accessor: CMP1CON\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1con::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1con::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1con`] module"]
#[doc(alias = "CMP1CON")]
pub type Cmp1con = crate::Reg<cmp1con::Cmp1conSpec>;
#[doc = "CMP1CON"]
pub mod cmp1con;
#[doc = "CMPINTF (rw) register accessor: CMPINTF\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpintf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpintf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpintf`] module"]
#[doc(alias = "CMPINTF")]
pub type Cmpintf = crate::Reg<cmpintf::CmpintfSpec>;
#[doc = "CMPINTF"]
pub mod cmpintf;
#[doc = "OPCON (rw) register accessor: OPCON\n\nYou can [`read`](crate::Reg::read) this register and get [`opcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opcon`] module"]
#[doc(alias = "OPCON")]
pub type Opcon = crate::Reg<opcon::OpconSpec>;
#[doc = "OPCON"]
pub mod opcon;
#[doc = "CMP2CON (rw) register accessor: CMP2CON\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp2con::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp2con::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp2con`] module"]
#[doc(alias = "CMP2CON")]
pub type Cmp2con = crate::Reg<cmp2con::Cmp2conSpec>;
#[doc = "CMP2CON"]
pub mod cmp2con;
