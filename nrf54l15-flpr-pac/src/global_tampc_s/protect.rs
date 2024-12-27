#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PROTECT")]
pub struct Protect {
    domain: [Domain; 1],
    _reserved1: [u8; 0x01e0],
    ap: [Ap; 1],
    _reserved2: [u8; 0x01f8],
    activeshield: Activeshield,
    _reserved3: [u8; 0x30],
    cracentamp: Cracentamp,
    glitchslowdomain: Glitchslowdomain,
    glitchfastdomain: Glitchfastdomain,
    _reserved6: [u8; 0x20],
    extreseten: Extreseten,
    intreseten: Intreseten,
    eraseprotect: Eraseprotect,
}
impl Protect {
    #[doc = "0x00..0x20 - Unspecified"]
    #[inline(always)]
    pub const fn domain(&self, n: usize) -> &Domain {
        &self.domain[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Unspecified"]
    #[inline(always)]
    pub fn domain_iter(&self) -> impl Iterator<Item = &Domain> {
        self.domain.iter()
    }
    #[doc = "0x200..0x208 - Unspecified"]
    #[inline(always)]
    pub const fn ap(&self, n: usize) -> &Ap {
        &self.ap[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x208 - Unspecified"]
    #[inline(always)]
    pub fn ap_iter(&self) -> impl Iterator<Item = &Ap> {
        self.ap.iter()
    }
    #[doc = "0x400..0x408 - Enable active shield detector."]
    #[inline(always)]
    pub const fn activeshield(&self) -> &Activeshield {
        &self.activeshield
    }
    #[doc = "0x438..0x440 - Enable tamper detector from CRACEN."]
    #[inline(always)]
    pub const fn cracentamp(&self) -> &Cracentamp {
        &self.cracentamp
    }
    #[doc = "0x440..0x448 - Enable slow domain glitch detectors."]
    #[inline(always)]
    pub const fn glitchslowdomain(&self) -> &Glitchslowdomain {
        &self.glitchslowdomain
    }
    #[doc = "0x448..0x450 - Enable fast domain glitch detectors."]
    #[inline(always)]
    pub const fn glitchfastdomain(&self) -> &Glitchfastdomain {
        &self.glitchfastdomain
    }
    #[doc = "0x470..0x478 - Trigger a reset when tamper is detected by the external tamper detectors."]
    #[inline(always)]
    pub const fn extreseten(&self) -> &Extreseten {
        &self.extreseten
    }
    #[doc = "0x478..0x480 - Trigger a reset when tamper is detected by the glitch detectors, signal protector or CRACEN tamper detector."]
    #[inline(always)]
    pub const fn intreseten(&self) -> &Intreseten {
        &self.intreseten
    }
    #[doc = "0x480..0x488 - Device erase protection."]
    #[inline(always)]
    pub const fn eraseprotect(&self) -> &Eraseprotect {
        &self.eraseprotect
    }
}
#[doc = "Unspecified"]
pub use self::domain::Domain;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod domain;
#[doc = "Unspecified"]
pub use self::ap::Ap;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod ap;
#[doc = "Enable active shield detector."]
pub use self::activeshield::Activeshield;
#[doc = r"Cluster"]
#[doc = "Enable active shield detector."]
pub mod activeshield;
#[doc = "Enable tamper detector from CRACEN."]
pub use self::cracentamp::Cracentamp;
#[doc = r"Cluster"]
#[doc = "Enable tamper detector from CRACEN."]
pub mod cracentamp;
#[doc = "Enable slow domain glitch detectors."]
pub use self::glitchslowdomain::Glitchslowdomain;
#[doc = r"Cluster"]
#[doc = "Enable slow domain glitch detectors."]
pub mod glitchslowdomain;
#[doc = "Enable fast domain glitch detectors."]
pub use self::glitchfastdomain::Glitchfastdomain;
#[doc = r"Cluster"]
#[doc = "Enable fast domain glitch detectors."]
pub mod glitchfastdomain;
#[doc = "Trigger a reset when tamper is detected by the external tamper detectors."]
pub use self::extreseten::Extreseten;
#[doc = r"Cluster"]
#[doc = "Trigger a reset when tamper is detected by the external tamper detectors."]
pub mod extreseten;
#[doc = "Trigger a reset when tamper is detected by the glitch detectors, signal protector or CRACEN tamper detector."]
pub use self::intreseten::Intreseten;
#[doc = r"Cluster"]
#[doc = "Trigger a reset when tamper is detected by the glitch detectors, signal protector or CRACEN tamper detector."]
pub mod intreseten;
#[doc = "Device erase protection."]
pub use self::eraseprotect::Eraseprotect;
#[doc = r"Cluster"]
#[doc = "Device erase protection."]
pub mod eraseprotect;
