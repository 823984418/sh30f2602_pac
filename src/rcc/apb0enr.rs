#[doc = "Register `APB0ENR` reader"]
pub type R = crate::R<Apb0enrSpec>;
#[doc = "Register `APB0ENR` writer"]
pub type W = crate::W<Apb0enrSpec>;
#[doc = "Field `TIM7EN` reader - "]
pub type Tim7enR = crate::BitReader;
#[doc = "Field `TIM7EN` writer - "]
pub type Tim7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8EN` reader - "]
pub type Tim8enR = crate::BitReader;
#[doc = "Field `TIM8EN` writer - "]
pub type Tim8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0EN` reader - "]
pub type Uart0enR = crate::BitReader;
#[doc = "Field `UART0EN` writer - "]
pub type Uart0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1EN` reader - "]
pub type Uart1enR = crate::BitReader;
#[doc = "Field `UART1EN` writer - "]
pub type Uart1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3EN` reader - "]
pub type Pwm3enR = crate::BitReader;
#[doc = "Field `PWM3EN` writer - "]
pub type Pwm3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0EN` reader - "]
pub type Spi0enR = crate::BitReader;
#[doc = "Field `SPI0EN` writer - "]
pub type Spi0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM4EN` reader - "]
pub type Pwm4enR = crate::BitReader;
#[doc = "Field `PWM4EN` writer - "]
pub type Pwm4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDTEN` reader - "]
pub type WwdtenR = crate::BitReader;
#[doc = "Field `WWDTEN` writer - "]
pub type WwdtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMOCEN` reader - "]
pub type AmocenR = crate::BitReader;
#[doc = "Field `AMOCEN` writer - "]
pub type AmocenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCA0EN` reader - "]
pub type Pca0enR = crate::BitReader;
#[doc = "Field `PCA0EN` writer - "]
pub type Pca0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NTESTEN` reader - "]
pub type NtestenR = crate::BitReader;
#[doc = "Field `NTESTEN` writer - "]
pub type NtestenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tim7en(&self) -> Tim7enR {
        Tim7enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tim8en(&self) -> Tim8enR {
        Tim8enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uart0en(&self) -> Uart0enR {
        Uart0enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart1en(&self) -> Uart1enR {
        Uart1enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pwm3en(&self) -> Pwm3enR {
        Pwm3enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi0en(&self) -> Spi0enR {
        Spi0enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwm4en(&self) -> Pwm4enR {
        Pwm4enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wwdten(&self) -> WwdtenR {
        WwdtenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn amocen(&self) -> AmocenR {
        AmocenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pca0en(&self) -> Pca0enR {
        Pca0enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ntesten(&self) -> NtestenR {
        NtestenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB0ENR")
            .field("rev0", &self.rev0())
            .field("ntesten", &self.ntesten())
            .field("rev1", &self.rev1())
            .field("pca0en", &self.pca0en())
            .field("amocen", &self.amocen())
            .field("wwdten", &self.wwdten())
            .field("pwm4en", &self.pwm4en())
            .field("spi0en", &self.spi0en())
            .field("pwm3en", &self.pwm3en())
            .field("uart1en", &self.uart1en())
            .field("uart0en", &self.uart0en())
            .field("rev2", &self.rev2())
            .field("tim8en", &self.tim8en())
            .field("tim7en", &self.tim7en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tim7en(&mut self) -> Tim7enW<'_, Apb0enrSpec> {
        Tim7enW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tim8en(&mut self) -> Tim8enW<'_, Apb0enrSpec> {
        Tim8enW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, Apb0enrSpec> {
        Rev2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uart0en(&mut self) -> Uart0enW<'_, Apb0enrSpec> {
        Uart0enW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart1en(&mut self) -> Uart1enW<'_, Apb0enrSpec> {
        Uart1enW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pwm3en(&mut self) -> Pwm3enW<'_, Apb0enrSpec> {
        Pwm3enW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi0en(&mut self) -> Spi0enW<'_, Apb0enrSpec> {
        Spi0enW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwm4en(&mut self) -> Pwm4enW<'_, Apb0enrSpec> {
        Pwm4enW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wwdten(&mut self) -> WwdtenW<'_, Apb0enrSpec> {
        WwdtenW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn amocen(&mut self) -> AmocenW<'_, Apb0enrSpec> {
        AmocenW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pca0en(&mut self) -> Pca0enW<'_, Apb0enrSpec> {
        Pca0enW::new(self, 10)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, Apb0enrSpec> {
        Rev1W::new(self, 11)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ntesten(&mut self) -> NtestenW<'_, Apb0enrSpec> {
        NtestenW::new(self, 14)
    }
    #[doc = "Bits 15:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Apb0enrSpec> {
        Rev0W::new(self, 15)
    }
}
#[doc = "APB0ENR\n\nYou can [`read`](crate::Reg::read) this register and get [`apb0enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb0enrSpec;
impl crate::RegisterSpec for Apb0enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb0enr::R`](R) reader structure"]
impl crate::Readable for Apb0enrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb0enr::W`](W) writer structure"]
impl crate::Writable for Apb0enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB0ENR to value 0"]
impl crate::Resettable for Apb0enrSpec {}
