use stm32f4xx_hal::pac::{interrupt, CorePeripherals, NVIC};

const COMMON_INTERRUPTS: &[interrupt; 5] = &[interrupt::EXTI0, interrupt::EXTI1, interrupt::EXTI2, interrupt::EXTI3, interrupt::EXTI4];

empty_interrupt!(EXTI0);
empty_interrupt!(EXTI1);
empty_interrupt!(EXTI2);
empty_interrupt!(EXTI3);
empty_interrupt!(EXTI4);
empty_interrupt!(PVD);
empty_interrupt!(I2C3_EV);
empty_interrupt!(I2C3_ER);
empty_interrupt!(I2C2_EV);
empty_interrupt!(I2C2_ER);
empty_interrupt!(I2C1_EV);
empty_interrupt!(I2C1_ER);
empty_interrupt!(USART6);
empty_interrupt!(USART1);
empty_interrupt!(DMA2_STREAM7);
empty_interrupt!(DMA2_STREAM6);
empty_interrupt!(DMA2_STREAM5);
empty_interrupt!(DMA2_STREAM4);
empty_interrupt!(DMA2_STREAM3);
empty_interrupt!(DMA2_STREAM2);
empty_interrupt!(DMA2_STREAM1);
empty_interrupt!(DMA2_STREAM0);
empty_interrupt!(DMA1_STREAM7);
empty_interrupt!(DMA1_STREAM6);
empty_interrupt!(DMA1_STREAM5);
empty_interrupt!(DMA1_STREAM4);
empty_interrupt!(DMA1_STREAM3);
empty_interrupt!(DMA1_STREAM2);
empty_interrupt!(DMA1_STREAM1);
empty_interrupt!(DMA1_STREAM0);
empty_interrupt!(OTG_FS);
empty_interrupt!(TIM5);
empty_interrupt!(TIM4);
empty_interrupt!(TIM3);
empty_interrupt!(TIM2);
empty_interrupt!(TIM1_CC);
empty_interrupt!(TIM1_TRG_COM_TIM11);
empty_interrupt!(TIM1_UP_TIM10);
empty_interrupt!(TIM1_BRK_TIM9);
empty_interrupt!(EXTI9_5);
empty_interrupt!(SPI3);
empty_interrupt!(SPI2);
empty_interrupt!(SPI1);
empty_interrupt!(RTC_ALARM);
empty_interrupt!(RTC_WKUP);
empty_interrupt!(TAMP_STAMP);

pub unsafe fn setup_interrupts() {
    for int in 0..COMMON_INTERRUPTS.len() {
        NVIC::unmask(COMMON_INTERRUPTS[int]);
    }
}
