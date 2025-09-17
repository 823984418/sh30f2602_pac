#[doc = "Register `OPR_DESI3` reader"]
pub type R = crate::R<OprDesi3Spec>;
#[doc = "Field `OPT0` reader - "]
pub type Opt0R = crate::FieldReader;
#[doc = "Field `OPT1` reader - "]
pub type Opt1R = crate::FieldReader;
#[doc = "Field `OPT2` reader - "]
pub type Opt2R = crate::FieldReader;
#[doc = "Field `OPT3` reader - "]
pub type Opt3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn opt0(&self) -> Opt0R {
        Opt0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn opt1(&self) -> Opt1R {
        Opt1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn opt2(&self) -> Opt2R {
        Opt2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn opt3(&self) -> Opt3R {
        Opt3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "OPR_DESI3\n\nYou can [`read`](crate::Reg::read) this register and get [`opr_desi3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OprDesi3Spec;
impl crate::RegisterSpec for OprDesi3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opr_desi3::R`](R) reader structure"]
impl crate::Readable for OprDesi3Spec {}
#[doc = "`reset()` method sets OPR_DESI3 to value 0"]
impl crate::Resettable for OprDesi3Spec {}
