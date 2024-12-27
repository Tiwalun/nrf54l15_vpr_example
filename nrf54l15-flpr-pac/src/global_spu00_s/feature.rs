#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "FEATURE")]
pub struct Feature {
    _reserved0: [u8; 0x80],
    dppic: Dppic,
    gpiote: [Gpiote; 2],
    _reserved2: [u8; 0x80],
    _reserved_2_gpio: [u8; 0x0184],
    _reserved3: [u8; 0x037c],
    grtc: Grtc,
}
impl Feature {
    #[doc = "0x80..0x100 - Unspecified"]
    #[inline(always)]
    pub const fn dppic(&self) -> &Dppic {
        &self.dppic
    }
    #[doc = "0x100..0x180 - Unspecified"]
    #[inline(always)]
    pub const fn gpiote(&self, n: usize) -> &Gpiote {
        &self.gpiote[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x180 - Unspecified"]
    #[inline(always)]
    pub fn gpiote_iter(&self) -> impl Iterator<Item = &Gpiote> {
        self.gpiote.iter()
    }
    #[doc = "0x200..0x384 - Unspecified"]
    #[inline(always)]
    pub const fn cracen(&self) -> &Cracen {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x200..0x380 - Unspecified"]
    #[inline(always)]
    pub const fn gpio(&self, n: usize) -> &Gpio {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(512)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x380 - Unspecified"]
    #[inline(always)]
    pub fn gpio_iter(&self) -> impl Iterator<Item = &Gpio> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(512)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x700..0x7c0 - Unspecified"]
    #[inline(always)]
    pub const fn grtc(&self) -> &Grtc {
        &self.grtc
    }
}
#[doc = "Unspecified"]
pub use self::dppic::Dppic;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod dppic;
#[doc = "Unspecified"]
pub use self::gpiote::Gpiote;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod gpiote;
#[doc = "Unspecified"]
pub use self::gpio::Gpio;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod gpio;
#[doc = "Unspecified"]
pub use self::cracen::Cracen;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod cracen;
#[doc = "Unspecified"]
pub use self::grtc::Grtc;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod grtc;
