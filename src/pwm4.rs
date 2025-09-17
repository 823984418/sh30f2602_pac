#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    pwmlock: Pwmlock,
    pwmpr: Pwmpr,
    pwmdr: Pwmdr,
    pwmdtr: Pwmdtr,
    pwmintf: Pwmintf,
    pwm_adtr: PwmAdtr,
}
impl RegisterBlock {
    #[doc = "0x00 - CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - PWMLOCK"]
    #[inline(always)]
    pub const fn pwmlock(&self) -> &Pwmlock {
        &self.pwmlock
    }
    #[doc = "0x08 - PWMPR"]
    #[inline(always)]
    pub const fn pwmpr(&self) -> &Pwmpr {
        &self.pwmpr
    }
    #[doc = "0x0c - PWMDR"]
    #[inline(always)]
    pub const fn pwmdr(&self) -> &Pwmdr {
        &self.pwmdr
    }
    #[doc = "0x10 - PWMDTR"]
    #[inline(always)]
    pub const fn pwmdtr(&self) -> &Pwmdtr {
        &self.pwmdtr
    }
    #[doc = "0x14 - PWMINTF"]
    #[inline(always)]
    pub const fn pwmintf(&self) -> &Pwmintf {
        &self.pwmintf
    }
    #[doc = "0x18 - PWM_ADTR"]
    #[inline(always)]
    pub const fn pwm_adtr(&self) -> &PwmAdtr {
        &self.pwm_adtr
    }
}
#[doc = "CR (rw) register accessor: CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "CR"]
pub mod cr;
#[doc = "PWMLOCK (rw) register accessor: PWMLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmlock`] module"]
#[doc(alias = "PWMLOCK")]
pub type Pwmlock = crate::Reg<pwmlock::PwmlockSpec>;
#[doc = "PWMLOCK"]
pub mod pwmlock;
#[doc = "PWMPR (rw) register accessor: PWMPR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmpr`] module"]
#[doc(alias = "PWMPR")]
pub type Pwmpr = crate::Reg<pwmpr::PwmprSpec>;
#[doc = "PWMPR"]
pub mod pwmpr;
#[doc = "PWMDR (rw) register accessor: PWMDR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmdr`] module"]
#[doc(alias = "PWMDR")]
pub type Pwmdr = crate::Reg<pwmdr::PwmdrSpec>;
#[doc = "PWMDR"]
pub mod pwmdr;
#[doc = "PWMDTR (rw) register accessor: PWMDTR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmdtr`] module"]
#[doc(alias = "PWMDTR")]
pub type Pwmdtr = crate::Reg<pwmdtr::PwmdtrSpec>;
#[doc = "PWMDTR"]
pub mod pwmdtr;
#[doc = "PWMINTF (rw) register accessor: PWMINTF\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmintf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmintf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmintf`] module"]
#[doc(alias = "PWMINTF")]
pub type Pwmintf = crate::Reg<pwmintf::PwmintfSpec>;
#[doc = "PWMINTF"]
pub mod pwmintf;
#[doc = "PWM_ADTR (rw) register accessor: PWM_ADTR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm_adtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm_adtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_adtr`] module"]
#[doc(alias = "PWM_ADTR")]
pub type PwmAdtr = crate::Reg<pwm_adtr::PwmAdtrSpec>;
#[doc = "PWM_ADTR"]
pub mod pwm_adtr;
