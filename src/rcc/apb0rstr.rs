#[doc = "Register `APB0RSTR` reader"]
pub type R = crate::R<Apb0rstrSpec>;
#[doc = "Register `APB0RSTR` writer"]
pub type W = crate::W<Apb0rstrSpec>;
#[doc = "Field `TIM7RST` reader - "]
pub type Tim7rstR = crate::BitReader;
#[doc = "Field `TIM7RST` writer - "]
pub type Tim7rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8RST` reader - "]
pub type Tim8rstR = crate::BitReader;
#[doc = "Field `TIM8RST` writer - "]
pub type Tim8rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0RST` reader - "]
pub type Uart0rstR = crate::BitReader;
#[doc = "Field `UART0RST` writer - "]
pub type Uart0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1RST` reader - "]
pub type Uart1rstR = crate::BitReader;
#[doc = "Field `UART1RST` writer - "]
pub type Uart1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3RST` reader - "]
pub type Pwm3rstR = crate::BitReader;
#[doc = "Field `PWM3RST` writer - "]
pub type Pwm3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0RST` reader - "]
pub type Spi0rstR = crate::BitReader;
#[doc = "Field `SPI0RST` writer - "]
pub type Spi0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM4RST` reader - "]
pub type Pwm4rstR = crate::BitReader;
#[doc = "Field `PWM4RST` writer - "]
pub type Pwm4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDTRST` reader - "]
pub type WwdtrstR = crate::BitReader;
#[doc = "Field `WWDTRST` writer - "]
pub type WwdtrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMOCRST` reader - "]
pub type AmocrstR = crate::BitReader;
#[doc = "Field `AMOCRST` writer - "]
pub type AmocrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCA0RST` reader - "]
pub type Pca0rstR = crate::BitReader;
#[doc = "Field `PCA0RST` writer - "]
pub type Pca0rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NTESTRST` reader - "]
pub type NtestrstR = crate::BitReader;
#[doc = "Field `NTESTRST` writer - "]
pub type NtestrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tim7rst(&self) -> Tim7rstR {
        Tim7rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tim8rst(&self) -> Tim8rstR {
        Tim8rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uart0rst(&self) -> Uart0rstR {
        Uart0rstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart1rst(&self) -> Uart1rstR {
        Uart1rstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pwm3rst(&self) -> Pwm3rstR {
        Pwm3rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi0rst(&self) -> Spi0rstR {
        Spi0rstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwm4rst(&self) -> Pwm4rstR {
        Pwm4rstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wwdtrst(&self) -> WwdtrstR {
        WwdtrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn amocrst(&self) -> AmocrstR {
        AmocrstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pca0rst(&self) -> Pca0rstR {
        Pca0rstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ntestrst(&self) -> NtestrstR {
        NtestrstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB0RSTR")
            .field("rev0", &self.rev0())
            .field("ntestrst", &self.ntestrst())
            .field("rev1", &self.rev1())
            .field("pca0rst", &self.pca0rst())
            .field("amocrst", &self.amocrst())
            .field("wwdtrst", &self.wwdtrst())
            .field("pwm4rst", &self.pwm4rst())
            .field("spi0rst", &self.spi0rst())
            .field("pwm3rst", &self.pwm3rst())
            .field("uart1rst", &self.uart1rst())
            .field("uart0rst", &self.uart0rst())
            .field("rev2", &self.rev2())
            .field("tim8rst", &self.tim8rst())
            .field("tim7rst", &self.tim7rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tim7rst(&mut self) -> Tim7rstW<'_, Apb0rstrSpec> {
        Tim7rstW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tim8rst(&mut self) -> Tim8rstW<'_, Apb0rstrSpec> {
        Tim8rstW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, Apb0rstrSpec> {
        Rev2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uart0rst(&mut self) -> Uart0rstW<'_, Apb0rstrSpec> {
        Uart0rstW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart1rst(&mut self) -> Uart1rstW<'_, Apb0rstrSpec> {
        Uart1rstW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pwm3rst(&mut self) -> Pwm3rstW<'_, Apb0rstrSpec> {
        Pwm3rstW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi0rst(&mut self) -> Spi0rstW<'_, Apb0rstrSpec> {
        Spi0rstW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwm4rst(&mut self) -> Pwm4rstW<'_, Apb0rstrSpec> {
        Pwm4rstW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wwdtrst(&mut self) -> WwdtrstW<'_, Apb0rstrSpec> {
        WwdtrstW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn amocrst(&mut self) -> AmocrstW<'_, Apb0rstrSpec> {
        AmocrstW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pca0rst(&mut self) -> Pca0rstW<'_, Apb0rstrSpec> {
        Pca0rstW::new(self, 10)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, Apb0rstrSpec> {
        Rev1W::new(self, 11)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ntestrst(&mut self) -> NtestrstW<'_, Apb0rstrSpec> {
        NtestrstW::new(self, 14)
    }
    #[doc = "Bits 15:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Apb0rstrSpec> {
        Rev0W::new(self, 15)
    }
}
#[doc = "APB0RSTR\n\nYou can [`read`](crate::Reg::read) this register and get [`apb0rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb0rstrSpec;
impl crate::RegisterSpec for Apb0rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb0rstr::R`](R) reader structure"]
impl crate::Readable for Apb0rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb0rstr::W`](W) writer structure"]
impl crate::Writable for Apb0rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB0RSTR to value 0"]
impl crate::Resettable for Apb0rstrSpec {}
