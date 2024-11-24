#![cfg_attr(not(test), no_std)]

use device_driver::AsyncRegisterInterface;

pub mod common;

pub mod buck;
pub mod gpios;
pub mod leds;

const ADDR: u8 = 0x6B;

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum NPM1300Error<I2cError> {
    #[error("i2c error: {0:?}")]
    I2c(I2cError),
}

#[derive(Debug)]
pub struct DeviceInterface<I2c: embedded_hal_async::i2c::I2c> {
    pub i2c: I2c,
}

pub struct NPM1300<I2c: embedded_hal_async::i2c::I2c> {
    device: Device<DeviceInterface<I2c>>,
}

impl<I2c: embedded_hal_async::i2c::I2c> NPM1300<I2c> {
    pub fn new(i2c: I2c) -> Self {
        Self {
            device: Device::new(DeviceInterface { i2c }),
        }
    }
}

device_driver::create_device!(
    device_name: Device,
    manifest: "device.yaml"
);

impl<I2c: embedded_hal_async::i2c::I2c> device_driver::AsyncRegisterInterface
    for DeviceInterface<I2c>
{
    type AddressType = u16;

    type Error = NPM1300Error<I2c::Error>;

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let buf = [(address >> 8) as u8, address as u8, data[0]];
        self.i2c.write(ADDR, &buf).await.map_err(NPM1300Error::I2c)
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c
            .write_read(ADDR, &[(address >> 8) as u8, address as u8], data)
            .await
            .map_err(NPM1300Error::I2c)
    }
}

impl<I2c: embedded_hal_async::i2c::I2c> device_driver::AsyncCommandInterface
    for DeviceInterface<I2c>
{
    type AddressType = u16;

    type Error = NPM1300Error<I2c::Error>;

    async fn dispatch_command(
        &mut self,
        address: Self::AddressType,
        size_bits_in: u32,
        input: &[u8],
        _size_bits_out: u32,
        _output: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.write_register(address, size_bits_in, input).await
    }
}
