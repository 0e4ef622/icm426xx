#![no_std]
#![cfg_attr(not(doctest), doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md")))]

pub mod config;
pub mod fifo;
pub mod ll;
pub mod ready;
pub mod register_bank;
pub mod uninitialized;

pub use config::Config;

#[derive(Debug)]
pub struct Uninitialized;

/// Indicates that the `ICM42688` instance is ready to be used
#[derive(Debug)]
pub struct Ready;

/// ICM42688 top-level driver
///
/// Usage:
///
/// ```rust,ignore
/// # use async_std::prelude::*; // Just for the runtime
/// # use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};
/// # use embedded_hal_mock::eh1::digital::Mock as PinMock;
/// # use embedded_hal_mock::eh1::digital::{State as PinState, Transaction as PinTransaction};
/// # use embedded_hal_mock::eh1::delay::NoopDelay as Delay;
/// # #[async_std::main]
/// async fn main() {
///     let spi = SpiMock::new(&[]);
///     let mut pin = PinMock::new(&[PinTransaction::set(PinState::High)]);
///     let spidev =
///         embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, pin.clone()).unwrap();
///     let mut icm = icm426xx::ICM42688::new(spidev);
///     let mut icm = icm.initialize(Delay).await.unwrap();
///     let mut bank = icm.ll().bank::<{ icm426xx::register_bank::BANK0 }>();
///
///     // print WHO_AM_I register
///     let who_am_i = bank.who_am_i().async_read().await;
///     loop {
///         let fifo_count = icm.read_fifo_count().await;
///         let mut fifo_buffer = [0u32; 128];
///         let num_read = icm.read_fifo(&mut fifo_buffer).await.unwrap();
///     }
/// }
/// ```
pub struct ICM42688<SPI, State> {
    ll: crate::ll::ICM42688<SPI>,
    _state: State,
}

#[cfg(test)]
mod test {
    use embedded_hal_mock::eh1::{delay::NoopDelay, spi};
    extern crate std;
    use std::vec;

    #[async_std::test]
    async fn test_init() {
        let mut spi = spi::Mock::new(&[
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![145, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![17, 1]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![245, 0], vec![0, 0x47]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![148, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![20, 6]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![150, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![22, 192]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![204, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![76, 51]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![205, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![77, 64]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![207, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![79, 6]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![208, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![80, 6]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![209, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![81, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![210, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![82, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![211, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![83, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![212, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![84, 29]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![223, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![95, 55]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![224, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![96, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![225, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![97, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![227, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![99, 8]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![228, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![100, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![229, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![101, 4]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![118, 1]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![139, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![11, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![140, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![12, 13]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![141, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![13, 170]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![142, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![14, 128]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![251, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![123, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![118, 2]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![131, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![3, 26]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![132, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![4, 170]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![133, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![5, 128]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![118, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::transfer_in_place(vec![206, 0], vec![0, 0]),
            spi::Transaction::transaction_end(),
            spi::Transaction::transaction_start(),
            spi::Transaction::write_vec(vec![78, 15]),
            spi::Transaction::transaction_end(),
        ]);
        let mut icm = super::ICM42688::new(&mut spi);
        let mut icm = icm.initialize(NoopDelay, Default::default()).await.unwrap();
        drop(icm);
        spi.done();
    }
}
