#[doc = "Register `CMPINTF` reader"]
pub type R = crate::R<CmpintfSpec>;
#[doc = "Register `CMPINTF` writer"]
pub type W = crate::W<CmpintfSpec>;
#[doc = "Field `C0IF` reader - "]
pub type C0ifR = crate::BitReader;
#[doc = "Field `C1IF` reader - "]
pub type C1ifR = crate::BitReader;
#[doc = "Field `C2IF` reader - "]
pub type C2ifR = crate::BitReader;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader<u16>;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `C0IFC` reader - "]
pub type C0ifcR = crate::BitReader;
#[doc = "Field `C0IFC` writer - "]
pub type C0ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1IFC` reader - "]
pub type C1ifcR = crate::BitReader;
#[doc = "Field `C1IFC` writer - "]
pub type C1ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2IFC` reader - "]
pub type C2ifcR = crate::BitReader;
#[doc = "Field `C2IFC` writer - "]
pub type C2ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn c0if(&self) -> C0ifR {
        C0ifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn c1if(&self) -> C1ifR {
        C1ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn c2if(&self) -> C2ifR {
        C2ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:15"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn c0ifc(&self) -> C0ifcR {
        C0ifcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn c1ifc(&self) -> C1ifcR {
        C1ifcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn c2ifc(&self) -> C2ifcR {
        C2ifcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPINTF")
            .field("rev0", &self.rev0())
            .field("c2ifc", &self.c2ifc())
            .field("c1ifc", &self.c1ifc())
            .field("c0ifc", &self.c0ifc())
            .field("rev1", &self.rev1())
            .field("c2if", &self.c2if())
            .field("c1if", &self.c1if())
            .field("c0if", &self.c0if())
            .finish()
    }
}
impl W {
    #[doc = "Bits 3:15"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, CmpintfSpec> {
        Rev1W::new(self, 3)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn c0ifc(&mut self) -> C0ifcW<'_, CmpintfSpec> {
        C0ifcW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn c1ifc(&mut self) -> C1ifcW<'_, CmpintfSpec> {
        C1ifcW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn c2ifc(&mut self) -> C2ifcW<'_, CmpintfSpec> {
        C2ifcW::new(self, 18)
    }
    #[doc = "Bits 19:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, CmpintfSpec> {
        Rev0W::new(self, 19)
    }
}
#[doc = "CMPINTF\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpintf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpintf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpintfSpec;
impl crate::RegisterSpec for CmpintfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpintf::R`](R) reader structure"]
impl crate::Readable for CmpintfSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpintf::W`](W) writer structure"]
impl crate::Writable for CmpintfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPINTF to value 0"]
impl crate::Resettable for CmpintfSpec {}
