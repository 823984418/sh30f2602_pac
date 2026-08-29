#[doc = "Register `PWMCON1` reader"]
pub type R = crate::R<Pwmcon1Spec>;
#[doc = "Register `PWMCON1` writer"]
pub type W = crate::W<Pwmcon1Spec>;
#[doc = "Field `PWM0S` reader - "]
pub type Pwm0sR = crate::BitReader;
#[doc = "Field `PWM0S` writer - "]
pub type Pwm0sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1S` reader - "]
pub type Pwm1sR = crate::BitReader;
#[doc = "Field `PWM1S` writer - "]
pub type Pwm1sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2S` reader - "]
pub type Pwm2sR = crate::BitReader;
#[doc = "Field `PWM2S` writer - "]
pub type Pwm2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM01S` reader - "]
pub type Pwm01sR = crate::BitReader;
#[doc = "Field `PWM01S` writer - "]
pub type Pwm01sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM11S` reader - "]
pub type Pwm11sR = crate::BitReader;
#[doc = "Field `PWM11S` writer - "]
pub type Pwm11sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM21S` reader - "]
pub type Pwm21sR = crate::BitReader;
#[doc = "Field `PWM21S` writer - "]
pub type Pwm21sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDCON0` reader - "]
pub type Pdcon0R = crate::BitReader;
#[doc = "Field `PDCON0` writer - "]
pub type Pdcon0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDCON1` reader - "]
pub type Pdcon1R = crate::BitReader;
#[doc = "Field `PDCON1` writer - "]
pub type Pdcon1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDCON2` reader - "]
pub type Pdcon2R = crate::BitReader;
#[doc = "Field `PDCON2` writer - "]
pub type Pdcon2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYM` reader - "]
pub type PwmsymR = crate::BitReader;
#[doc = "Field `PWMSYM` writer - "]
pub type PwmsymW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTMOD` reader - "]
pub type PtmodR = crate::FieldReader;
#[doc = "Field `PTMOD` writer - "]
pub type PtmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `POSTPS` reader - "]
pub type PostpsR = crate::FieldReader;
#[doc = "Field `POSTPS` writer - "]
pub type PostpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POUTMOD` reader - "]
pub type PoutmodR = crate::BitReader;
#[doc = "Field `POUTMOD` writer - "]
pub type PoutmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pwm0s(&self) -> Pwm0sR {
        Pwm0sR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pwm1s(&self) -> Pwm1sR {
        Pwm1sR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pwm2s(&self) -> Pwm2sR {
        Pwm2sR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pwm01s(&self) -> Pwm01sR {
        Pwm01sR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pwm11s(&self) -> Pwm11sR {
        Pwm11sR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pwm21s(&self) -> Pwm21sR {
        Pwm21sR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pdcon0(&self) -> Pdcon0R {
        Pdcon0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pdcon1(&self) -> Pdcon1R {
        Pdcon1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pdcon2(&self) -> Pdcon2R {
        Pdcon2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwmsym(&self) -> PwmsymR {
        PwmsymR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn ptmod(&self) -> PtmodR {
        PtmodR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn postps(&self) -> PostpsR {
        PostpsR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn poutmod(&self) -> PoutmodR {
        PoutmodR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMCON1")
            .field("rev0", &self.rev0())
            .field("poutmod", &self.poutmod())
            .field("postps", &self.postps())
            .field("ptmod", &self.ptmod())
            .field("pwmsym", &self.pwmsym())
            .field("pdcon2", &self.pdcon2())
            .field("pdcon1", &self.pdcon1())
            .field("pdcon0", &self.pdcon0())
            .field("pwm21s", &self.pwm21s())
            .field("pwm11s", &self.pwm11s())
            .field("pwm01s", &self.pwm01s())
            .field("pwm2s", &self.pwm2s())
            .field("pwm1s", &self.pwm1s())
            .field("pwm0s", &self.pwm0s())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pwm0s(&mut self) -> Pwm0sW<'_, Pwmcon1Spec> {
        Pwm0sW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pwm1s(&mut self) -> Pwm1sW<'_, Pwmcon1Spec> {
        Pwm1sW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pwm2s(&mut self) -> Pwm2sW<'_, Pwmcon1Spec> {
        Pwm2sW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pwm01s(&mut self) -> Pwm01sW<'_, Pwmcon1Spec> {
        Pwm01sW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pwm11s(&mut self) -> Pwm11sW<'_, Pwmcon1Spec> {
        Pwm11sW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pwm21s(&mut self) -> Pwm21sW<'_, Pwmcon1Spec> {
        Pwm21sW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pdcon0(&mut self) -> Pdcon0W<'_, Pwmcon1Spec> {
        Pdcon0W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pdcon1(&mut self) -> Pdcon1W<'_, Pwmcon1Spec> {
        Pdcon1W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pdcon2(&mut self) -> Pdcon2W<'_, Pwmcon1Spec> {
        Pdcon2W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwmsym(&mut self) -> PwmsymW<'_, Pwmcon1Spec> {
        PwmsymW::new(self, 9)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn ptmod(&mut self) -> PtmodW<'_, Pwmcon1Spec> {
        PtmodW::new(self, 10)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn postps(&mut self) -> PostpsW<'_, Pwmcon1Spec> {
        PostpsW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn poutmod(&mut self) -> PoutmodW<'_, Pwmcon1Spec> {
        PoutmodW::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwmcon1Spec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMCON1\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmcon1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmcon1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwmcon1Spec;
impl crate::RegisterSpec for Pwmcon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmcon1::R`](R) reader structure"]
impl crate::Readable for Pwmcon1Spec {}
#[doc = "`write(|w| ..)` method takes [`pwmcon1::W`](W) writer structure"]
impl crate::Writable for Pwmcon1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMCON1 to value 0"]
impl crate::Resettable for Pwmcon1Spec {}
