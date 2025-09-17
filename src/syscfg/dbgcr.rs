#[doc = "Register `DBGCR` reader"]
pub type R = crate::R<DbgcrSpec>;
#[doc = "Register `DBGCR` writer"]
pub type W = crate::W<DbgcrSpec>;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::BitReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STOP` reader - "]
pub type DbgStopR = crate::BitReader;
#[doc = "Field `DBG_STOP` writer - "]
pub type DbgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::FieldReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_PWM` reader - "]
pub type DbgPwmR = crate::BitReader;
#[doc = "Field `DBG_PWM` writer - "]
pub type DbgPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDT` reader - "]
pub type DbgIwdtR = crate::BitReader;
#[doc = "Field `DBG_IWDT` writer - "]
pub type DbgIwdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDT` reader - "]
pub type DbgWwdtR = crate::BitReader;
#[doc = "Field `DBG_WWDT` writer - "]
pub type DbgWwdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_PCA` reader - "]
pub type DbgPcaR = crate::BitReader;
#[doc = "Field `DBG_PCA` writer - "]
pub type DbgPcaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM` reader - "]
pub type DbgTimR = crate::BitReader;
#[doc = "Field `DBG_TIM` writer - "]
pub type DbgTimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_MCM` reader - "]
pub type DbgMcmR = crate::BitReader;
#[doc = "Field `DBG_MCM` writer - "]
pub type DbgMcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_UART` reader - "]
pub type DbgUartR = crate::BitReader;
#[doc = "Field `DBG_UART` writer - "]
pub type DbgUartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_SPI` reader - "]
pub type DbgSpiR = crate::BitReader;
#[doc = "Field `DBG_SPI` writer - "]
pub type DbgSpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::BitReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - "]
pub type LockR = crate::FieldReader<u16>;
#[doc = "Field `LOCK` writer - "]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DbgStopR {
        DbgStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dbg_pwm(&self) -> DbgPwmR {
        DbgPwmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dbg_iwdt(&self) -> DbgIwdtR {
        DbgIwdtR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dbg_wwdt(&self) -> DbgWwdtR {
        DbgWwdtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dbg_pca(&self) -> DbgPcaR {
        DbgPcaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dbg_tim(&self) -> DbgTimR {
        DbgTimR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dbg_mcm(&self) -> DbgMcmR {
        DbgMcmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dbg_uart(&self) -> DbgUartR {
        DbgUartR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dbg_spi(&self) -> DbgSpiR {
        DbgSpiR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, DbgcrSpec> {
        Rev3W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DbgStopW<'_, DbgcrSpec> {
        DbgStopW::new(self, 1)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, DbgcrSpec> {
        Rev2W::new(self, 2)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, DbgcrSpec> {
        Rev1W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dbg_pwm(&mut self) -> DbgPwmW<'_, DbgcrSpec> {
        DbgPwmW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dbg_iwdt(&mut self) -> DbgIwdtW<'_, DbgcrSpec> {
        DbgIwdtW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dbg_wwdt(&mut self) -> DbgWwdtW<'_, DbgcrSpec> {
        DbgWwdtW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dbg_pca(&mut self) -> DbgPcaW<'_, DbgcrSpec> {
        DbgPcaW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dbg_tim(&mut self) -> DbgTimW<'_, DbgcrSpec> {
        DbgTimW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dbg_mcm(&mut self) -> DbgMcmW<'_, DbgcrSpec> {
        DbgMcmW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dbg_uart(&mut self) -> DbgUartW<'_, DbgcrSpec> {
        DbgUartW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dbg_spi(&mut self) -> DbgSpiW<'_, DbgcrSpec> {
        DbgSpiW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, DbgcrSpec> {
        Rev0W::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, DbgcrSpec> {
        LockW::new(self, 16)
    }
}
#[doc = "DBGCR\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgcrSpec;
impl crate::RegisterSpec for DbgcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgcr::R`](R) reader structure"]
impl crate::Readable for DbgcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgcr::W`](W) writer structure"]
impl crate::Writable for DbgcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGCR to value 0"]
impl crate::Resettable for DbgcrSpec {}
