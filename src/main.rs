use std::{time::Duration, thread};

use rusb::{DeviceHandle, UsbContext, Device, Context, Error, request_type};

const VID: u16 = 0x07ca;
const PID: u16 = 0x0036;

const ENABLE_GPIO: u16 = 0x00;
const GPIO_DIRECTION: u16 = 0x02;

fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            println!("Found AverMedia! Attempting to open.");
            match device.open() {
                Ok(handle) => return Some((device, handle)),
                Err(a) => {
                    println!("Failed to open! {}", a);
                },
            }
        }
    }

    None
}

fn write_ctrl(index: u16, value: u16, handle: &DeviceHandle<Context>) {
    let t:Vec<u8> = Vec::new();
    handle.write_control(0x40, 1, value, index, &t, Duration::from_millis(200)).unwrap();
}

fn read_ctrl_val(index: u16, handle: &DeviceHandle<Context>, val: u16) -> Option<u8> {
    let mut buf = [1];
    handle.read_control(0xc0, 0, val, index, &mut buf, Duration::from_millis(200)).unwrap();
    buf.first().copied()
}

fn read_ctrl(index: u16, handle: &DeviceHandle<Context>) -> Option<u8> {
    read_ctrl_val(index, handle, 0)
}

fn set_bits_u8(bits: &[u8]) -> u8 {
    let mut rslt = 0x0;
    
    for bit in bits {
        rslt |= 1 << bit;
    }
    rslt
}

fn enable_gpio(pins: &[u8], handle: &DeviceHandle<Context>) {
    write_ctrl(ENABLE_GPIO, set_bits_u8(pins) as u16, handle);
}

fn set_lower_gpio_output(pins: &[u8], handle: &DeviceHandle<Context>) {
    write_ctrl(GPIO_DIRECTION, 0xe8, handle);
}

fn main() -> Result<(), Error> {
    let mut context = Context::new()?;
    let (mut device, mut handle) =
        open_device(&mut context, VID, PID).expect("Did not find USB device");

    print_device_info(&mut handle)?;

    handle.claim_interface(0)?;
    // handle.claim_interface(0x80)?;
    handle.set_alternate_setting(0, 0).unwrap();

    handle.detach_kernel_driver(2)?;
    handle.claim_interface(2)?;
    handle.set_alternate_setting(2, 0).unwrap();

    // Enable GPIO 5 and 6
    enable_gpio(&[5, 6], &handle);
    // Set GPIO 3, 5, 6, 7 direction to OUTPUT
    set_lower_gpio_output(&[3, 5, 6, 7], &handle);
    // Disable EEPROM Interface (disable writes), set direction of GPIO 8, 9 to OUTPUT
    write_ctrl(0x03, 0x83, &handle);
    // Set GPIO 1, 2, 3, 5, 6, 7 direction to OUTPUT
    write_ctrl(0x02, 0xef, &handle);
    
    // Enable GPIO 2, 5, 6
    write_ctrl(0x00, 0x64, &handle);
    // Enable GPIO 2, 5, 6
    write_ctrl(0x00, 0x64, &handle);
    // Enable GPIO 1, 2, 5, 6
    write_ctrl(0x00, 0x66, &handle);
    // Enable GPIO 0, 1, 2, 5, 6
    write_ctrl(0x00, 0x67, &handle);
    // Enable GPIO 1, 2, 5, 6
    write_ctrl(0x00, 0x66, &handle);
    // Enable GPIO 2, 5, 6
    write_ctrl(0x00, 0x64, &handle);
    // Enable GPIO 0, 2, 5, 6
    write_ctrl(0x00, 0x65, &handle);
    // Enable GPIO 2, 5, 6
    write_ctrl(0x00, 0x64, &handle);
    // Enable GPIO 2, 5, 6
    write_ctrl(0x00, 0x64, &handle);
    // Enable GPIO 0, 2, 5, 6
    write_ctrl(0x00, 0x65, &handle);
    write_ctrl(0x00, 0x64, &handle);
    write_ctrl(0x00, 0x64, &handle);
    write_ctrl(0x00, 0x65, &handle);
    write_ctrl(0x00, 0x64, &handle);
    write_ctrl(0x00, 0x64, &handle);
    write_ctrl(0x00, 0x65, &handle);
    write_ctrl(0x00, 0x64, &handle);
    write_ctrl(0x00, 0x64, &handle);
    write_ctrl(0x00, 0x65, &handle);
    write_ctrl(0x00, 0x64, &handle);
    write_ctrl(0x00, 0x64, &handle);
    write_ctrl(0x00, 0x65, &handle);
    write_ctrl(0x00, 0x64, &handle);
    write_ctrl(0x00, 0x64, &handle);
    write_ctrl(0x00, 0x65, &handle);
    write_ctrl(0x00, 0x64, &handle);
    write_ctrl(0x00, 0x64, &handle);
    write_ctrl(0x00, 0x65, &handle);
    // Enable GPIO 0, 5, 6
    write_ctrl(0x00, 0x61, &handle);

    // Why are we reading values we've just written?
    // And then writing them back? These do correspond in WireShark to what was just
    // written. 
    let res0 = read_ctrl(0, &handle).unwrap();
    let res1 = read_ctrl(2, &handle).unwrap();
    write_ctrl(0, res0 as u16, &handle);
    write_ctrl(2, res1 as u16, &handle);

    // Set sensor address
    write_ctrl(0x203, 0xba, &handle);
    // Set GPIO 3, 5, 6, 7 direction to OUTPUT
    write_ctrl(0x02, 0xe8, &handle);
    // Disable ROM Interface again? (Don't know what other bits do, it's usualy set to 0x80)
    write_ctrl(0x03, 0x83, &handle);
    // Enable GPIO 5 and 6
    write_ctrl(0x00, 0x60, &handle);
    // More GPIO, I think (Enable 9)
    write_ctrl(0x01, 0x02, &handle);
    // Enable interrupts
    write_ctrl(0x05, 0x00, &handle);
    // Enable interrupts from GPIO9
    write_ctrl(0x07, 0x02, &handle);
    // Enable remote wakeup from GPIO9
    write_ctrl(0x0d, 0x00, &handle);
    write_ctrl(0x0f, 0x02, &handle);
    // ??? Might disable VBI mode (going off STK1160 datasheet)
    write_ctrl(0x103, 0x00, &handle);

    // configure CLKOUT 
    write_ctrl(0x300, 0x12, &handle);

    // Set positive edge clocked pulse high when pixel counter
    // =0 or =1 and low otherwise
    // Set negative edge clocked pulse high when pixel counter
    // =0 or =1 and low otherwise.
    write_ctrl(0x350, 0x2d, &handle);
    // Clock count = 4 for each pixel, no delay
    write_ctrl(0x351, 0x01, &handle);
    // More clock stuff
    write_ctrl(0x352, 0x00, &handle);
    write_ctrl(0x353, 0x00, &handle);

    // Enable timing generator
    write_ctrl(0x300, 0x80, &handle);

    // enable CLKOUT for sensor
    write_ctrl(0x18, 0x10, &handle);
    // Disable STOP clock
    write_ctrl(0x19, 0x00, &handle);
    // Set serial interface clock divider (30MHz/0x1e*16+2) = 62240 Hz
    write_ctrl(0x202, 0x1e, &handle);

    // Cropping?
    // Set capture start position X = 80, Y = 25
    write_ctrl(0x110, 0x50, &handle);
    write_ctrl(0x111, 0x00, &handle);
    write_ctrl(0x112, 0x19, &handle);
    write_ctrl(0x113, 0x00, &handle);
    // Set capture end position X = 1360, Y = 2305
    write_ctrl(0x114, 0x50, &handle);
    write_ctrl(0x115, 0x05, &handle);
    write_ctrl(0x116, 0x09, &handle);
    write_ctrl(0x117, 0x01, &handle);

    // The first time this is read, it's set to all zeroes.
    // This register controls some video settings (see datasheet).
    let res0 = read_ctrl(0x100, &handle).unwrap();
    // Set Hsync Positive, Vsync Positive, data is in ITU 656 format, 8-bit data
    write_ctrl(0x100, 0x33, &handle);
    
    // Set serial COMM address (0xBA is the TI chip)
    write_ctrl(0x203, 0xba, &handle);
    // Serial bus write address (0x7f)
    // This resets the TI chip (restart microprocessor)
    write_ctrl(0x204, 0x7f, &handle);
    // Serial bus write value
    write_ctrl(0x205, 0x00, &handle);
    // Begin write operation
    write_ctrl(0x200, 0x01, &handle);
    // Serial write ready (wait)
    let r0 = read_ctrl(0x201, &handle).unwrap();
    // Ready = CONTINUE

    // Read address 0x80 from serial
    write_ctrl(0x208, 0x80, &handle);
    // Begin read operation (Read device ID MSB from TI)
    write_ctrl(0x200, 0x20, &handle);

    // Get a bunch of info on the serial interface (such as read success)
    let r0 = read_ctrl(0x20, &handle).unwrap();
    // Get read data
    let r0 = read_ctrl(0x209, &handle).unwrap();

    // Read address 0x81 from serial (Read device ID LSB from TI)
    write_ctrl(0x208, 0x81, &handle);
    // Begin read operation 
    write_ctrl(0x200, 0x20, &handle);

    // Get read success
    let r0 = read_ctrl(0x201, &handle).unwrap();
    // Get read data
    let r0 = read_ctrl(0x209, &handle).unwrap();

    // Set serial COMM address (a0 is some unknown chip)
    write_ctrl(0x203, 0xa0, &handle);
    // Read address 0x3c
    write_ctrl(0x208, 0x3c, &handle);
    // Begin read operation
    write_ctrl(0x200, 0x20, &handle);

    // Get read success
    let r0 = read_ctrl(0x201, &handle).unwrap();
    // Get read data
    let r0 = read_ctrl(0x209, &handle).unwrap();
    
    // Serial bus write address
    write_ctrl(0x204, 0x08, &handle);
    // Write value (Set luminance value to a reserved value?)
    write_ctrl(0x205, 0x08, &handle);
    // Begin write operation
    write_ctrl(0x200, 0x05, &handle);
    
    // Serial write ready (wait)
    let r0 = read_ctrl(0x201, &handle).unwrap();

    // Serial bus write address
    write_ctrl(0x204, 0x28, &handle);
    // Write value (Set video standard to (B, G, H, I, N) PAL ITU-R BT.601)
    write_ctrl(0x205, 0x04, &handle);
    // Begin write operation
    write_ctrl(0x200, 0x05, &handle);

    // Serial write ready (wait)
    let r0 = read_ctrl(0x201, &handle).unwrap();

    // Serial bus write address
    write_ctrl(0x204, 0x30, &handle);
    // Write value (Adheres to ITU-R BT.656.4 and BT.656.5 timing)
    write_ctrl(0x205, 0x00, &handle);
    // Begin write operation
    write_ctrl(0x200, 0x05, &handle);

    // Serial write ready (wait)
    let r0 = read_ctrl(0x201, &handle).unwrap();

    // Serial bus write address
    write_ctrl(0x204, 0x0f, &handle);
    // Write value (configure what each pin does, which is vsync etc.)
    write_ctrl(0x205, 0x0a, &handle);
    // Begin write operation
    write_ctrl(0x200, 0x05, &handle);

    // Serial write ready (wait)
    let r0 = read_ctrl(0x201, &handle).unwrap();
    

    // This is starting audio control
    // Enable AC97 interface
    // Reset AC97 interface
    // Control write phase
    write_ctrl(0x500, 0x94, &handle);
    // Enable AC97 interface
    // AC97 Operation
    // Control write phase
    write_ctrl(0x500, 0x8c, &handle);
    // Set to 16-bit audio
    write_ctrl(0x506, 0x01, &handle);
    // Write zeroes to a reserved register??
    write_ctrl(0x507, 0x00, &handle);

    // Read video settings (currently set to 0x33)
    let r0 = read_ctrl(0x100, &handle).unwrap();
    // I guess set the exact same value again???
    write_ctrl(0x100, 0x33, &handle);

    // Check which GPIOs are outputs
    let r0 = read_ctrl(0x0, &handle).unwrap();
    // Enable GPIO 2,3,5,6
    write_ctrl(0x0, 0x6c, &handle);

    // Serial bus write address
    write_ctrl(0x204, 0x00, &handle);
    // Write value (Set composite video input source to AIP1B)
    write_ctrl(0x205, 0x02, &handle);
    // Begin write operation
    write_ctrl(0x200, 0x05, &handle);
    // Serial write ready (wait)
    let r0 = read_ctrl(0x201, &handle).unwrap();

    // Serial bus write address
    write_ctrl(0x204, 0x03, &handle);
    // Write value (Set settings to non-high impedance but enable vblanks)
    write_ctrl(0x205, 0x6f, &handle);
    // Begin write operation
    write_ctrl(0x200, 0x05, &handle);
    // Serial write ready (wait)
    let r0 = read_ctrl(0x201, &handle).unwrap();

    // Yes, set this again!!
    // Set Hsync Positive, Vsync Positive, data is in ITU 656 format, 8-bit data
    write_ctrl(0x100, 0x33, &handle);
    // Set the command address to 0x10
    write_ctrl(0x504, 0x10, &handle);
    // Enable AC97, AC97 Operation, Control Read phase, In
    write_ctrl(0x500, 0x8b, &handle);

    // Read command data
    let r0 = read_ctrl(0x502, &handle).unwrap(); // 08
    let r1 = read_ctrl(0x503, &handle).unwrap(); // 88
    // Set the command address to 0x10 (RealTek audio chip)
    // This sets the LINE IN volume
    write_ctrl(0x504, 0x10, &handle);
    // This sets the LINE IN right volume to 0dB gain
    write_ctrl(0x502, 0x08, &handle);
    // This sets the LINE IN left volume to 0dB gain, and then mutes the whole thing
    write_ctrl(0x503, 0x88, &handle);
    // Enable AC97 interface, AC97 Operation, Control write phase
    write_ctrl(0x500, 0x8c, &handle);

    // CD Volume
    write_ctrl(0x504, 0x12, &handle);
    write_ctrl(0x500, 0x8b, &handle);

    let r0 = read_ctrl(0x502, &handle).unwrap();
    let r1 = read_ctrl(0x503, &handle).unwrap();
    // Set the command address to 0x12
    // This sets the CD volume
    write_ctrl(0x504, 0x12, &handle);
    // CD Right volume to 0dB gain
    write_ctrl(0x502, 0x08, &handle);
    // CD Left volume to 0dB gain
    write_ctrl(0x503, 0x08, &handle);
    write_ctrl(0x500, 0x8c, &handle);

    // Control MIC volume
    write_ctrl(0x504, 0x0e, &handle);
    // Enable AC97, AC97 Operation, Control Read phase, In
    write_ctrl(0x500, 0x8b, &handle);
    let r0 = read_ctrl(0x502, &handle).unwrap();
    let r1 = read_ctrl(0x503, &handle).unwrap();

    // Control MIC volume
    write_ctrl(0x504, 0x0e, &handle);
    // 0 dB gain on the MIC volume
    write_ctrl(0x502, 0x08, &handle);
    // No mute
    write_ctrl(0x503, 0x00, &handle);
    write_ctrl(0x500, 0x8c, &handle);

    // Control AUX volume
    write_ctrl(0x504, 0x16, &handle);
    write_ctrl(0x500, 0x8b, &handle);

    let r0 = read_ctrl(0x502, &handle).unwrap();
    let r1 = read_ctrl(0x503, &handle).unwrap();

    // Control AUX volume
    write_ctrl(0x504, 0x16, &handle);
    // 0 dB gain on the right volume
    write_ctrl(0x502, 0x08, &handle);
    // 0 dB gain on the left volume
    write_ctrl(0x503, 0x08, &handle);
    write_ctrl(0x500, 0x8c, &handle);

    // Record select
    write_ctrl(0x504, 0x1a, &handle);
    // Set right source to CD right
    write_ctrl(0x502, 0x01, &handle);
    // Set left source to CD left
    write_ctrl(0x503, 0x01, &handle);
    write_ctrl(0x500, 0x8c, &handle);

    // Record gain
    write_ctrl(0x504, 0x1c, &handle);
    // No gain right
    write_ctrl(0x502, 0x00, &handle);
    // No gain left
    write_ctrl(0x503, 0x00, &handle);
    write_ctrl(0x500, 0x8c, &handle);

    // // BIG PAUSE

    // START - Something with the timing generator
    write_ctrl(0x301, 0x0100, &handle);
    for _ in 1..4 {
        let r0 = read_ctrl_val(0x301, &handle, 0x0200).unwrap();
    }

    write_ctrl(0x301, 0x0200, &handle);
    
    let r0 = read_ctrl_val(0x301, &handle, 0x0402).unwrap();
    // END - Something with the timing generator

    for i in 0x3c..0x3f {
        // Set sensor address to some unknown device
        write_ctrl(0x203, 0xa0, &handle);
        // Read address `i`
        write_ctrl(0x208, i, &handle);
        // Begin read operation
        write_ctrl(0x200, 0x20, &handle);

        // Read something, I don't know what
        let r0 = read_ctrl(0x201, &handle).unwrap();
        let r1 = read_ctrl(0x209, &handle).unwrap();
    }

    for i in 0x3c..0x3f {
        // Set sensor address to some unknown device
        write_ctrl(0x203, 0xa0, &handle);
        // Read address `i`
        write_ctrl(0x208, i, &handle);
        // Begin read operation
        write_ctrl(0x200, 0x20, &handle);

        let r0 = read_ctrl(0x201, &handle).unwrap();
        let r1 = read_ctrl(0x209, &handle).unwrap();
        if i == 0x3d {
            // Something with the timing generator again
            let r2 = read_ctrl_val(0x301, &handle, 0x0100).unwrap();
        }
    }

    loop {
        println!("Call!");

        for i in [0x0d, 0x0b] {
            let t:Vec<u8> = Vec::new();

            // Set sensor address to some unknown device
            write_ctrl(0x203, 0x82, &handle);
            // Read address `0x0d`
            write_ctrl(0x208, 0x0d, &handle);
            // Initiate read
            write_ctrl(0x200, 0x20, &handle);

            // Read
            let r0 = read_ctrl(0x201, &handle).unwrap();
            let r1 = read_ctrl(0x209, &handle).unwrap();
        }
 
        thread::sleep(Duration::from_millis(333));
    }
    
    Ok(())
}


fn configure_endpoint<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
    endpoint: &Endpoint,
) -> Result<(), Error> {
    // handle.set_active_configuration(endpoint.config)?;
    handle.claim_interface(endpoint.iface)?;
    handle.set_alternate_setting(endpoint.iface, endpoint.setting)
}


fn print_device_info<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<(), Error> {
    let device_desc = handle.device().device_descriptor()?;
    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout)?;

    println!("Active configuration: {}", &handle.active_configuration()?);

    if !languages.is_empty() {
        let language = languages[0];
        println!("Language: {:?}", language);

        println!(
            "Manufacturer: {}",
            handle
                .read_manufacturer_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Product: {}",
            handle
                .read_product_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Serial Number: {}",
            handle
                .read_serial_number_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
    }
    Ok(())
}

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

// returns all readable endpoints for given usb device and descriptor
fn find_readable_endpoints<T: UsbContext>(device: &mut Device<T>) -> Result<Vec<Endpoint>, Error> {
    let device_desc = device.device_descriptor()?;
    let mut endpoints = vec![];
    for n in 0..device_desc.num_configurations() {
        let config_desc = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue,
        };
        // println!("{:#?}", config_desc);
        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                // println!("{:#?}", interface_desc);
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    // println!("{:#?}", endpoint_desc);
                    endpoints.push(Endpoint {
                        config: config_desc.number(),
                        iface: interface_desc.interface_number(),
                        setting: interface_desc.setting_number(),
                        address: endpoint_desc.address(),
                    });
                }
            }
        }
    }

    Ok(endpoints)
}