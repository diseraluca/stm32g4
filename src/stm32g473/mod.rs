#![deny(unused_allocation)]
#![deny(unused_comparisons)]
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
///Number available in the NVIC for configuring priority
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD_PVM();
    fn RTC_TAMP_CSS_LSE();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CH1();
    fn DMA1_CH2();
    fn DMA1_CH3();
    fn DMA1_CH4();
    fn DMA1_CH5();
    fn DMA1_CH6();
    fn DMA1_CH7();
    fn ADC1_2();
    fn USB_HP();
    fn USB_LP();
    fn FDCAN1_INTR1_IT();
    fn FDCAN1_INTR0_IT();
    fn EXTI9_5();
    fn TIM1_BRK_TIM15();
    fn TIM1_UP_TIM16();
    fn TIM1_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn USBWAKEUP();
    fn TIM8_BRK();
    fn TIM8_UP();
    fn TIM8_TRG_COM();
    fn TIM8_CC();
    fn ADC3();
    fn FMC();
    fn LPTIM1();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6_DACUNDER();
    fn TIM7();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn ADC4();
    fn ADC5();
    fn UCPD1();
    fn COMP1_2_3();
    fn COMP4_5_6();
    fn COMP7();
    fn CRS();
    fn SAI();
    fn TIM20_BRK();
    fn TIM20_UP();
    fn TIM20_TRG_COM();
    fn TIM20_CC();
    fn FPU();
    fn I2C4_EV();
    fn I2C4_ER();
    fn SPI4();
    fn AES();
    fn FDCAN2_INTR0();
    fn FDCAN2_INTR1();
    fn FDCAN3_INTR0();
    fn FDCAN3_INTR1();
    fn RNG();
    fn LPUART();
    fn I2C3_EV();
    fn I2C3_ER();
    fn DMAMUX_OVR();
    fn QUADSPI();
    fn DMA1_CH8();
    fn DMA2_CH6();
    fn DMA2_CH7();
    fn DMA2_CH8();
    fn CORDIC();
    fn FMAC();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 102] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD_PVM },
    Vector {
        _handler: RTC_TAMP_CSS_LSE,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector { _handler: DMA1_CH1 },
    Vector { _handler: DMA1_CH2 },
    Vector { _handler: DMA1_CH3 },
    Vector { _handler: DMA1_CH4 },
    Vector { _handler: DMA1_CH5 },
    Vector { _handler: DMA1_CH6 },
    Vector { _handler: DMA1_CH7 },
    Vector { _handler: ADC1_2 },
    Vector { _handler: USB_HP },
    Vector { _handler: USB_LP },
    Vector {
        _handler: FDCAN1_INTR1_IT,
    },
    Vector {
        _handler: FDCAN1_INTR0_IT,
    },
    Vector { _handler: EXTI9_5 },
    Vector {
        _handler: TIM1_BRK_TIM15,
    },
    Vector {
        _handler: TIM1_UP_TIM16,
    },
    Vector {
        _handler: TIM1_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector {
        _handler: USBWAKEUP,
    },
    Vector { _handler: TIM8_BRK },
    Vector { _handler: TIM8_UP },
    Vector {
        _handler: TIM8_TRG_COM,
    },
    Vector { _handler: TIM8_CC },
    Vector { _handler: ADC3 },
    Vector { _handler: FMC },
    Vector { _handler: LPTIM1 },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector {
        _handler: TIM6_DACUNDER,
    },
    Vector { _handler: TIM7 },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector { _handler: ADC4 },
    Vector { _handler: ADC5 },
    Vector { _handler: UCPD1 },
    Vector {
        _handler: COMP1_2_3,
    },
    Vector {
        _handler: COMP4_5_6,
    },
    Vector { _handler: COMP7 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: CRS },
    Vector { _handler: SAI },
    Vector {
        _handler: TIM20_BRK,
    },
    Vector { _handler: TIM20_UP },
    Vector {
        _handler: TIM20_TRG_COM,
    },
    Vector { _handler: TIM20_CC },
    Vector { _handler: FPU },
    Vector { _handler: I2C4_EV },
    Vector { _handler: I2C4_ER },
    Vector { _handler: SPI4 },
    Vector { _handler: AES },
    Vector {
        _handler: FDCAN2_INTR0,
    },
    Vector {
        _handler: FDCAN2_INTR1,
    },
    Vector {
        _handler: FDCAN3_INTR0,
    },
    Vector {
        _handler: FDCAN3_INTR1,
    },
    Vector { _handler: RNG },
    Vector { _handler: LPUART },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector {
        _handler: DMAMUX_OVR,
    },
    Vector { _handler: QUADSPI },
    Vector { _handler: DMA1_CH8 },
    Vector { _handler: DMA2_CH6 },
    Vector { _handler: DMA2_CH7 },
    Vector { _handler: DMA2_CH8 },
    Vector { _handler: CORDIC },
    Vector { _handler: FMAC },
];
///Enumeration of all the interrupts
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    ///0 - Window Watchdog interrupt
    WWDG = 0,
    ///1 - PVD through EXTI line detection
    PVD_PVM = 1,
    ///2 - RTC_TAMP_CSS_LSE
    RTC_TAMP_CSS_LSE = 2,
    ///3 - RTC Wakeup timer
    RTC_WKUP = 3,
    ///4 - FLASH
    FLASH = 4,
    ///5 - RCC
    RCC = 5,
    ///6 - EXTI Line0 interrupt
    EXTI0 = 6,
    ///7 - EXTI Line1 interrupt
    EXTI1 = 7,
    ///8 - EXTI Line2 interrupt
    EXTI2 = 8,
    ///9 - EXTI Line3 interrupt
    EXTI3 = 9,
    ///10 - EXTI Line4 interrupt
    EXTI4 = 10,
    ///11 - DMA1 channel 1 interrupt
    DMA1_CH1 = 11,
    ///12 - DMA1 channel 2 interrupt
    DMA1_CH2 = 12,
    ///13 - DMA1 channel 3 interrupt
    DMA1_CH3 = 13,
    ///14 - DMA1 channel 4 interrupt
    DMA1_CH4 = 14,
    ///15 - DMA1 channel 5 interrupt
    DMA1_CH5 = 15,
    ///16 - DMA1 channel 6 interrupt
    DMA1_CH6 = 16,
    ///17 - DMA1 channel 7 interrupt
    DMA1_CH7 = 17,
    ///18 - ADC1 and ADC2 global interrupt
    ADC1_2 = 18,
    ///19 - USB_HP
    USB_HP = 19,
    ///20 - USB_LP
    USB_LP = 20,
    ///21 - fdcan1_intr1_it
    FDCAN1_INTR1_IT = 21,
    ///22 - fdcan1_intr0_it
    FDCAN1_INTR0_IT = 22,
    ///23 - EXTI9_5
    EXTI9_5 = 23,
    ///24 - TIM1_BRK_TIM15
    TIM1_BRK_TIM15 = 24,
    ///25 - TIM1_UP_TIM16
    TIM1_UP_TIM16 = 25,
    ///26 - TIM1_TRG_COM/
    TIM1_TRG_COM = 26,
    ///27 - TIM1 capture compare interrupt
    TIM1_CC = 27,
    ///28 - TIM2
    TIM2 = 28,
    ///29 - TIM3
    TIM3 = 29,
    ///30 - TIM4
    TIM4 = 30,
    ///31 - I2C1_EV
    I2C1_EV = 31,
    ///32 - I2C1_ER
    I2C1_ER = 32,
    ///33 - I2C2_EV
    I2C2_EV = 33,
    ///34 - I2C2_ER
    I2C2_ER = 34,
    ///35 - SPI1
    SPI1 = 35,
    ///36 - SPI2
    SPI2 = 36,
    ///37 - USART1
    USART1 = 37,
    ///38 - USART2
    USART2 = 38,
    ///39 - USART3
    USART3 = 39,
    ///40 - EXTI15_10
    EXTI15_10 = 40,
    ///41 - RTC_ALARM
    RTC_ALARM = 41,
    ///42 - USBWakeUP
    USBWAKEUP = 42,
    ///43 - TIM8_BRK
    TIM8_BRK = 43,
    ///44 - TIM8_UP
    TIM8_UP = 44,
    ///45 - TIM8_TRG_COM
    TIM8_TRG_COM = 45,
    ///46 - TIM8_CC
    TIM8_CC = 46,
    ///47 - ADC3
    ADC3 = 47,
    ///48 - FMC
    FMC = 48,
    ///49 - LPTIM1
    LPTIM1 = 49,
    ///50 - TIM5
    TIM5 = 50,
    ///51 - SPI3
    SPI3 = 51,
    ///52 - UART4
    UART4 = 52,
    ///53 - UART5
    UART5 = 53,
    ///54 - TIM6_DACUNDER
    TIM6_DACUNDER = 54,
    ///55 - TIM7
    TIM7 = 55,
    ///56 - DMA2_CH1
    DMA2_CH1 = 56,
    ///57 - DMA2_CH2
    DMA2_CH2 = 57,
    ///58 - DMA2_CH3
    DMA2_CH3 = 58,
    ///59 - DMA2_CH4
    DMA2_CH4 = 59,
    ///60 - DMA2_CH5
    DMA2_CH5 = 60,
    ///61 - ADC4
    ADC4 = 61,
    ///62 - ADC5
    ADC5 = 62,
    ///63 - UCPD1
    UCPD1 = 63,
    ///64 - COMP1_2_3
    COMP1_2_3 = 64,
    ///65 - COMP4_5_6
    COMP4_5_6 = 65,
    ///66 - COMP7
    COMP7 = 66,
    ///75 - CRS
    CRS = 75,
    ///76 - SAI
    SAI = 76,
    ///77 - TIM20_BRK
    TIM20_BRK = 77,
    ///78 - TIM20_UP
    TIM20_UP = 78,
    ///79 - TIM20_TRG_COM
    TIM20_TRG_COM = 79,
    ///80 - TIM20_CC
    TIM20_CC = 80,
    ///81 - Floating point unit interrupt
    FPU = 81,
    ///82 - I2C4_EV
    I2C4_EV = 82,
    ///83 - I2C4_ER
    I2C4_ER = 83,
    ///84 - SPI4
    SPI4 = 84,
    ///85 - AES
    AES = 85,
    ///86 - FDCAN2_intr0
    FDCAN2_INTR0 = 86,
    ///87 - FDCAN2_intr1
    FDCAN2_INTR1 = 87,
    ///88 - FDCAN3_intr0
    FDCAN3_INTR0 = 88,
    ///89 - FDCAN3_intr1
    FDCAN3_INTR1 = 89,
    ///90 - RNG
    RNG = 90,
    ///91 - LPUART
    LPUART = 91,
    ///92 - I2C3_EV
    I2C3_EV = 92,
    ///93 - I2C3_ER
    I2C3_ER = 93,
    ///94 - DMAMUX_OVR
    DMAMUX_OVR = 94,
    ///95 - QUADSPI
    QUADSPI = 95,
    ///96 - DMA1_CH8
    DMA1_CH8 = 96,
    ///97 - DMA2_CH6
    DMA2_CH6 = 97,
    ///98 - DMA2_CH7
    DMA2_CH7 = 98,
    ///99 - DMA2_CH8
    DMA2_CH8 = 99,
    ///100 - Cordic
    CORDIC = 100,
    ///101 - FMAC
    FMAC = 101,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
///Cyclic redundancy check calculation unit
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x4002_3000 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
///Cyclic redundancy check calculation unit
pub mod crc;
///Inter-integrated circuit
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5400 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
///Inter-integrated circuit
pub mod i2c1;
///Inter-integrated circuit
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5800 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
///Inter-integrated circuit
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_7800 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C3::ptr() }
    }
}
///Inter-integrated circuit
pub struct I2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C4 {}
impl I2C4 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_8400 as *const _
    }
}
impl Deref for I2C4 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C4::ptr() }
    }
}
///Flash
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x4002_2000 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH::ptr() }
    }
}
///Flash
pub mod flash;
///Debug support
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBGMCU {}
impl DBGMCU {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dbgmcu::RegisterBlock {
        0xe004_2000 as *const _
    }
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DBGMCU::ptr() }
    }
}
///Debug support
pub mod dbgmcu;
///Reset and clock control
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const rcc::RegisterBlock {
        0x4002_1000 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RCC::ptr() }
    }
}
///Reset and clock control
pub mod rcc;
///Power control
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const pwr::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWR::ptr() }
    }
}
///Power control
pub mod pwr;
///Random number generator
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        0x5006_0800 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG::ptr() }
    }
}
///Random number generator
pub mod rng;
///Advanced encryption standard hardware accelerator
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const aes::RegisterBlock {
        0x5006_0000 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES::ptr() }
    }
}
///Advanced encryption standard hardware accelerator
pub mod aes;
///General-purpose I/Os
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4800_0000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
///General-purpose I/Os
pub mod gpioa;
///General-purpose I/Os
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x4800_0400 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
///General-purpose I/Os
pub mod gpiob;
///General-purpose I/Os
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4800_0800 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
///General-purpose I/Os
pub mod gpioc;
///General-purpose I/Os
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4800_0c00 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOD::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4800_1000 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOE::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4800_1400 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOF::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4800_1800 as *const _
    }
}
impl Deref for GPIOG {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOG::ptr() }
    }
}
///General purpose timers
pub struct TIM15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM15 {}
impl TIM15 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim15::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for TIM15 {
    type Target = tim15::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM15::ptr() }
    }
}
///General purpose timers
pub mod tim15;
///General purpose timers
pub struct TIM16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM16 {}
impl TIM16 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim16::RegisterBlock {
        0x4001_4400 as *const _
    }
}
impl Deref for TIM16 {
    type Target = tim16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM16::ptr() }
    }
}
///General purpose timers
pub mod tim16;
///General purpose timers
pub struct TIM17 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM17 {}
impl TIM17 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim16::RegisterBlock {
        0x4001_4800 as *const _
    }
}
impl Deref for TIM17 {
    type Target = tim16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM17::ptr() }
    }
}
///Advanced-timers
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        0x4001_2c00 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM1::ptr() }
    }
}
///Advanced-timers
pub mod tim1;
///Advanced-timers
pub struct TIM20 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM20 {}
impl TIM20 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        0x4001_5000 as *const _
    }
}
impl Deref for TIM20 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM20::ptr() }
    }
}
///Advanced-timers
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM8 {}
impl TIM8 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        0x4001_3400 as *const _
    }
}
impl Deref for TIM8 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM8::ptr() }
    }
}
///Advanced-timers
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM2::ptr() }
    }
}
///Advanced-timers
pub mod tim2;
///Advanced-timers
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_0400 as *const _
    }
}
impl Deref for TIM3 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM3::ptr() }
    }
}
///Advanced-timers
pub struct TIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM4 {}
impl TIM4 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_0800 as *const _
    }
}
impl Deref for TIM4 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM4::ptr() }
    }
}
///Advanced-timers
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM5 {}
impl TIM5 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_0c00 as *const _
    }
}
impl Deref for TIM5 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM5::ptr() }
    }
}
///Basic-timers
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM6 {}
impl TIM6 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM6::ptr() }
    }
}
///Basic-timers
pub mod tim6;
///Basic-timers
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM7 {}
impl TIM7 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        0x4000_1400 as *const _
    }
}
impl Deref for TIM7 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM7::ptr() }
    }
}
///Low power timer
pub struct LPTIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIMER1 {}
impl LPTIMER1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const lptimer1::RegisterBlock {
        0x4000_7c00 as *const _
    }
}
impl Deref for LPTIMER1 {
    type Target = lptimer1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIMER1::ptr() }
    }
}
///Low power timer
pub mod lptimer1;
///Universal synchronous asynchronous receiver transmitter
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_3800 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub mod usart1;
///Universal synchronous asynchronous receiver transmitter
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_4400 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_4800 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART3::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const uart4::RegisterBlock {
        0x4000_4c00 as *const _
    }
}
impl Deref for UART4 {
    type Target = uart4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART4::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub mod uart4;
///Universal synchronous asynchronous receiver transmitter
pub struct UART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART5 {}
impl UART5 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const uart4::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for UART5 {
    type Target = uart4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART5::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub struct LPUART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART1 {}
impl LPUART1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const lpuart1::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for LPUART1 {
    type Target = lpuart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART1::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub mod lpuart1;
///Serial peripheral interface/Inter-IC sound
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
///Serial peripheral interface/Inter-IC sound
pub mod spi1;
///Serial peripheral interface/Inter-IC sound
pub struct SPI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI4 {}
impl SPI4 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi4::RegisterBlock {
        0x4001_3c00 as *const _
    }
}
impl Deref for SPI4 {
    type Target = spi4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI4::ptr() }
    }
}
///Serial peripheral interface/Inter-IC sound
pub mod spi4;
///Serial peripheral interface/Inter-IC sound
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi4::RegisterBlock {
        0x4000_3c00 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI3::ptr() }
    }
}
///Serial peripheral interface/Inter-IC sound
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi4::RegisterBlock {
        0x4000_3800 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
///External interrupt/event controller
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        0x4001_0400 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EXTI::ptr() }
    }
}
///External interrupt/event controller
pub mod exti;
///Real-time clock
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_2800 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
///Real-time clock
pub mod rtc;
///Flexible memory controller
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0xa000_0000 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMC::ptr() }
    }
}
///Flexible memory controller
pub mod fmc;
///DMA controller
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA1::ptr() }
    }
}
///DMA controller
pub mod dma1;
///DMA controller
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x4002_0400 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA2::ptr() }
    }
}
///DMAMUX
pub struct DMAMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX {}
impl DMAMUX {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dmamux::RegisterBlock {
        0x4002_0800 as *const _
    }
}
impl Deref for DMAMUX {
    type Target = dmamux::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAMUX::ptr() }
    }
}
///DMAMUX
pub mod dmamux;
///System configuration controller
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCFG::ptr() }
    }
}
///System configuration controller
pub mod syscfg;
///Voltage reference buffer
pub struct VREFBUF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREFBUF {}
impl VREFBUF {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const vrefbuf::RegisterBlock {
        0x4001_0030 as *const _
    }
}
impl Deref for VREFBUF {
    type Target = vrefbuf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VREFBUF::ptr() }
    }
}
///Voltage reference buffer
pub mod vrefbuf;
///Comparator control and status register
pub struct COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP {}
impl COMP {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const comp::RegisterBlock {
        0x4001_0200 as *const _
    }
}
impl Deref for COMP {
    type Target = comp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMP::ptr() }
    }
}
///Comparator control and status register
pub mod comp;
///Operational amplifiers
pub struct OPAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OPAMP {}
impl OPAMP {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const opamp::RegisterBlock {
        0x4001_0300 as *const _
    }
}
impl Deref for OPAMP {
    type Target = opamp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OPAMP::ptr() }
    }
}
///Operational amplifiers
pub mod opamp;
///QuadSPI interface
pub struct QUADSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QUADSPI {}
impl QUADSPI {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const quadspi::RegisterBlock {
        0xa000_1000 as *const _
    }
}
impl Deref for QUADSPI {
    type Target = quadspi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QUADSPI::ptr() }
    }
}
///QuadSPI interface
pub mod quadspi;
///Digital-to-analog converter
pub struct DAC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC1 {}
impl DAC1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dac1::RegisterBlock {
        0x5000_0800 as *const _
    }
}
impl Deref for DAC1 {
    type Target = dac1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC1::ptr() }
    }
}
///Digital-to-analog converter
pub mod dac1;
///Digital-to-analog converter
pub struct DAC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC2 {}
impl DAC2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dac1::RegisterBlock {
        0x5000_0c00 as *const _
    }
}
impl Deref for DAC2 {
    type Target = dac1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC2::ptr() }
    }
}
///Digital-to-analog converter
pub struct DAC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC3 {}
impl DAC3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dac1::RegisterBlock {
        0x5000_1000 as *const _
    }
}
impl Deref for DAC3 {
    type Target = dac1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC3::ptr() }
    }
}
///Digital-to-analog converter
pub struct DAC4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC4 {}
impl DAC4 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dac1::RegisterBlock {
        0x5000_1400 as *const _
    }
}
impl Deref for DAC4 {
    type Target = dac1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC4::ptr() }
    }
}
///Analog-to-Digital Converter
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC1::ptr() }
    }
}
///Analog-to-Digital Converter
pub mod adc1;
///Analog-to-Digital Converter
pub struct ADC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC2 {}
impl ADC2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x5000_0100 as *const _
    }
}
impl Deref for ADC2 {
    type Target = adc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC2::ptr() }
    }
}
///Analog-to-Digital Converter
pub struct ADC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC3 {}
impl ADC3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const adc3::RegisterBlock {
        0x5000_0400 as *const _
    }
}
impl Deref for ADC3 {
    type Target = adc3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC3::ptr() }
    }
}
///Analog-to-Digital Converter
pub mod adc3;
///Analog-to-Digital Converter
pub struct ADC4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC4 {}
impl ADC4 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x5000_0500 as *const _
    }
}
impl Deref for ADC4 {
    type Target = adc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC4::ptr() }
    }
}
///Analog-to-Digital Converter
pub struct ADC5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC5 {}
impl ADC5 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const adc3::RegisterBlock {
        0x5000_0600 as *const _
    }
}
impl Deref for ADC5 {
    type Target = adc3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC5::ptr() }
    }
}
///Analog-to-Digital Converter
pub struct ADC12_COMMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC12_COMMON {}
impl ADC12_COMMON {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const adc12_common::RegisterBlock {
        0x5000_0200 as *const _
    }
}
impl Deref for ADC12_COMMON {
    type Target = adc12_common::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC12_COMMON::ptr() }
    }
}
///Analog-to-Digital Converter
pub mod adc12_common;
///Analog-to-Digital Converter
pub struct ADC345_COMMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC345_COMMON {}
impl ADC345_COMMON {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const adc12_common::RegisterBlock {
        0x5000_0700 as *const _
    }
}
impl Deref for ADC345_COMMON {
    type Target = adc12_common::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC345_COMMON::ptr() }
    }
}
///Filter Math Accelerator
pub struct FMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMAC {}
impl FMAC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const fmac::RegisterBlock {
        0x4002_1400 as *const _
    }
}
impl Deref for FMAC {
    type Target = fmac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMAC::ptr() }
    }
}
///Filter Math Accelerator
pub mod fmac;
///CORDIC Co-processor
pub struct CORDIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CORDIC {}
impl CORDIC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const cordic::RegisterBlock {
        0x4002_0c00 as *const _
    }
}
impl Deref for CORDIC {
    type Target = cordic::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CORDIC::ptr() }
    }
}
///CORDIC Co-processor
pub mod cordic;
///Serial audio interface
pub struct SAI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI {}
impl SAI {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const sai::RegisterBlock {
        0x4001_5400 as *const _
    }
}
impl Deref for SAI {
    type Target = sai::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI::ptr() }
    }
}
///Serial audio interface
pub mod sai;
///Tamper and backup registers
pub struct TAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TAMP {}
impl TAMP {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tamp::RegisterBlock {
        0x4000_2400 as *const _
    }
}
impl Deref for TAMP {
    type Target = tamp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TAMP::ptr() }
    }
}
///Tamper and backup registers
pub mod tamp;
///SysTick timer
pub struct STK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STK {}
impl STK {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const stk::RegisterBlock {
        0xe000_e010 as *const _
    }
}
impl Deref for STK {
    type Target = stk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*STK::ptr() }
    }
}
///SysTick timer
pub mod stk;
///Floating point unit CPACR
pub struct FPU_CPACR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU_CPACR {}
impl FPU_CPACR {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const fpu_cpacr::RegisterBlock {
        0xe000_ef08 as *const _
    }
}
impl Deref for FPU_CPACR {
    type Target = fpu_cpacr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FPU_CPACR::ptr() }
    }
}
///Floating point unit CPACR
pub mod fpu_cpacr;
///System control block ACTLR
pub struct SCB_ACTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB_ACTRL {}
impl SCB_ACTRL {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const scb_actrl::RegisterBlock {
        0xe000_e008 as *const _
    }
}
impl Deref for SCB_ACTRL {
    type Target = scb_actrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB_ACTRL::ptr() }
    }
}
///System control block ACTLR
pub mod scb_actrl;
///FDCAN
pub struct FDCAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FDCAN {}
impl FDCAN {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const fdcan::RegisterBlock {
        0x4000_a400 as *const _
    }
}
impl Deref for FDCAN {
    type Target = fdcan::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FDCAN::ptr() }
    }
}
///FDCAN
pub mod fdcan;
///FDCAN
pub struct FDCAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FDCAN1 {}
impl FDCAN1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const fdcan::RegisterBlock {
        0x4000_6400 as *const _
    }
}
impl Deref for FDCAN1 {
    type Target = fdcan::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FDCAN1::ptr() }
    }
}
///FDCAN
pub struct FDCAN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FDCAN2 {}
impl FDCAN2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const fdcan::RegisterBlock {
        0x4000_6800 as *const _
    }
}
impl Deref for FDCAN2 {
    type Target = fdcan::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FDCAN2::ptr() }
    }
}
///FDCAN
pub struct FDCAN3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FDCAN3 {}
impl FDCAN3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const fdcan::RegisterBlock {
        0x4000_6c00 as *const _
    }
}
impl Deref for FDCAN3 {
    type Target = fdcan::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FDCAN3::ptr() }
    }
}
///UCPD1
pub struct UCPD1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UCPD1 {}
impl UCPD1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const ucpd1::RegisterBlock {
        0x4000_a000 as *const _
    }
}
impl Deref for UCPD1 {
    type Target = ucpd1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UCPD1::ptr() }
    }
}
///UCPD1
pub mod ucpd1;
///USB_FS_device
pub struct USB_FS_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB_FS_DEVICE {}
impl USB_FS_DEVICE {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usb_fs_device::RegisterBlock {
        0x4000_5c00 as *const _
    }
}
impl Deref for USB_FS_DEVICE {
    type Target = usb_fs_device::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB_FS_DEVICE::ptr() }
    }
}
///USB_FS_device
pub mod usb_fs_device;
///CRS
pub struct CRS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRS {}
impl CRS {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const crs::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for CRS {
    type Target = crs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRS::ptr() }
    }
}
///CRS
pub mod crs;
///System window watchdog
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const wwdg::RegisterBlock {
        0x4000_2c00 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDG::ptr() }
    }
}
///System window watchdog
pub mod wwdg;
///WinWATCHDOG
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const iwdg::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IWDG::ptr() }
    }
}
///WinWATCHDOG
pub mod iwdg;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
///All the peripherals
#[allow(non_snake_case)]
pub struct Peripherals {
    ///CRC
    pub CRC: CRC,
    ///I2C1
    pub I2C1: I2C1,
    ///I2C2
    pub I2C2: I2C2,
    ///I2C3
    pub I2C3: I2C3,
    ///I2C4
    pub I2C4: I2C4,
    ///FLASH
    pub FLASH: FLASH,
    ///DBGMCU
    pub DBGMCU: DBGMCU,
    ///RCC
    pub RCC: RCC,
    ///PWR
    pub PWR: PWR,
    ///RNG
    pub RNG: RNG,
    ///AES
    pub AES: AES,
    ///GPIOA
    pub GPIOA: GPIOA,
    ///GPIOB
    pub GPIOB: GPIOB,
    ///GPIOC
    pub GPIOC: GPIOC,
    ///GPIOD
    pub GPIOD: GPIOD,
    ///GPIOE
    pub GPIOE: GPIOE,
    ///GPIOF
    pub GPIOF: GPIOF,
    ///GPIOG
    pub GPIOG: GPIOG,
    ///TIM15
    pub TIM15: TIM15,
    ///TIM16
    pub TIM16: TIM16,
    ///TIM17
    pub TIM17: TIM17,
    ///TIM1
    pub TIM1: TIM1,
    ///TIM20
    pub TIM20: TIM20,
    ///TIM8
    pub TIM8: TIM8,
    ///TIM2
    pub TIM2: TIM2,
    ///TIM3
    pub TIM3: TIM3,
    ///TIM4
    pub TIM4: TIM4,
    ///TIM5
    pub TIM5: TIM5,
    ///TIM6
    pub TIM6: TIM6,
    ///TIM7
    pub TIM7: TIM7,
    ///LPTIMER1
    pub LPTIMER1: LPTIMER1,
    ///USART1
    pub USART1: USART1,
    ///USART2
    pub USART2: USART2,
    ///USART3
    pub USART3: USART3,
    ///UART4
    pub UART4: UART4,
    ///UART5
    pub UART5: UART5,
    ///LPUART1
    pub LPUART1: LPUART1,
    ///SPI1
    pub SPI1: SPI1,
    ///SPI4
    pub SPI4: SPI4,
    ///SPI3
    pub SPI3: SPI3,
    ///SPI2
    pub SPI2: SPI2,
    ///EXTI
    pub EXTI: EXTI,
    ///RTC
    pub RTC: RTC,
    ///FMC
    pub FMC: FMC,
    ///DMA1
    pub DMA1: DMA1,
    ///DMA2
    pub DMA2: DMA2,
    ///DMAMUX
    pub DMAMUX: DMAMUX,
    ///SYSCFG
    pub SYSCFG: SYSCFG,
    ///VREFBUF
    pub VREFBUF: VREFBUF,
    ///COMP
    pub COMP: COMP,
    ///OPAMP
    pub OPAMP: OPAMP,
    ///QUADSPI
    pub QUADSPI: QUADSPI,
    ///DAC1
    pub DAC1: DAC1,
    ///DAC2
    pub DAC2: DAC2,
    ///DAC3
    pub DAC3: DAC3,
    ///DAC4
    pub DAC4: DAC4,
    ///ADC1
    pub ADC1: ADC1,
    ///ADC2
    pub ADC2: ADC2,
    ///ADC3
    pub ADC3: ADC3,
    ///ADC4
    pub ADC4: ADC4,
    ///ADC5
    pub ADC5: ADC5,
    ///ADC12_COMMON
    pub ADC12_COMMON: ADC12_COMMON,
    ///ADC345_COMMON
    pub ADC345_COMMON: ADC345_COMMON,
    ///FMAC
    pub FMAC: FMAC,
    ///CORDIC
    pub CORDIC: CORDIC,
    ///SAI
    pub SAI: SAI,
    ///TAMP
    pub TAMP: TAMP,
    ///STK
    pub STK: STK,
    ///FPU_CPACR
    pub FPU_CPACR: FPU_CPACR,
    ///SCB_ACTRL
    pub SCB_ACTRL: SCB_ACTRL,
    ///FDCAN
    pub FDCAN: FDCAN,
    ///FDCAN1
    pub FDCAN1: FDCAN1,
    ///FDCAN2
    pub FDCAN2: FDCAN2,
    ///FDCAN3
    pub FDCAN3: FDCAN3,
    ///UCPD1
    pub UCPD1: UCPD1,
    ///USB_FS_DEVICE
    pub USB_FS_DEVICE: USB_FS_DEVICE,
    ///CRS
    pub CRS: CRS,
    ///WWDG
    pub WWDG: WWDG,
    ///IWDG
    pub IWDG: IWDG,
}
impl Peripherals {
    ///Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    ///Unchecked version of `Peripherals::take`
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            CRC: CRC {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            I2C4: I2C4 {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            DBGMCU: DBGMCU {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            GPIOG: GPIOG {
                _marker: PhantomData,
            },
            TIM15: TIM15 {
                _marker: PhantomData,
            },
            TIM16: TIM16 {
                _marker: PhantomData,
            },
            TIM17: TIM17 {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            TIM20: TIM20 {
                _marker: PhantomData,
            },
            TIM8: TIM8 {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            TIM4: TIM4 {
                _marker: PhantomData,
            },
            TIM5: TIM5 {
                _marker: PhantomData,
            },
            TIM6: TIM6 {
                _marker: PhantomData,
            },
            TIM7: TIM7 {
                _marker: PhantomData,
            },
            LPTIMER1: LPTIMER1 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            UART5: UART5 {
                _marker: PhantomData,
            },
            LPUART1: LPUART1 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI4: SPI4 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            DMAMUX: DMAMUX {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            VREFBUF: VREFBUF {
                _marker: PhantomData,
            },
            COMP: COMP {
                _marker: PhantomData,
            },
            OPAMP: OPAMP {
                _marker: PhantomData,
            },
            QUADSPI: QUADSPI {
                _marker: PhantomData,
            },
            DAC1: DAC1 {
                _marker: PhantomData,
            },
            DAC2: DAC2 {
                _marker: PhantomData,
            },
            DAC3: DAC3 {
                _marker: PhantomData,
            },
            DAC4: DAC4 {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            ADC2: ADC2 {
                _marker: PhantomData,
            },
            ADC3: ADC3 {
                _marker: PhantomData,
            },
            ADC4: ADC4 {
                _marker: PhantomData,
            },
            ADC5: ADC5 {
                _marker: PhantomData,
            },
            ADC12_COMMON: ADC12_COMMON {
                _marker: PhantomData,
            },
            ADC345_COMMON: ADC345_COMMON {
                _marker: PhantomData,
            },
            FMAC: FMAC {
                _marker: PhantomData,
            },
            CORDIC: CORDIC {
                _marker: PhantomData,
            },
            SAI: SAI {
                _marker: PhantomData,
            },
            TAMP: TAMP {
                _marker: PhantomData,
            },
            STK: STK {
                _marker: PhantomData,
            },
            FPU_CPACR: FPU_CPACR {
                _marker: PhantomData,
            },
            SCB_ACTRL: SCB_ACTRL {
                _marker: PhantomData,
            },
            FDCAN: FDCAN {
                _marker: PhantomData,
            },
            FDCAN1: FDCAN1 {
                _marker: PhantomData,
            },
            FDCAN2: FDCAN2 {
                _marker: PhantomData,
            },
            FDCAN3: FDCAN3 {
                _marker: PhantomData,
            },
            UCPD1: UCPD1 {
                _marker: PhantomData,
            },
            USB_FS_DEVICE: USB_FS_DEVICE {
                _marker: PhantomData,
            },
            CRS: CRS {
                _marker: PhantomData,
            },
            WWDG: WWDG {
                _marker: PhantomData,
            },
            IWDG: IWDG {
                _marker: PhantomData,
            },
        }
    }
}
