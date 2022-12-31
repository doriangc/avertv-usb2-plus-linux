use std::{time::Duration, thread};

use rusb::{DeviceHandle, UsbContext, Device, Context, Error, request_type};

const VID: u16 = 0x07ca;
const PID: u16 = 0x0036;

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

fn main() -> Result<(), Error> {
    let mut context = Context::new()?;
    let (mut device, mut handle) =
        open_device(&mut context, VID, PID).expect("Did not find USB device");

    print_device_info(&mut handle)?;

    // handle.set_active_configuration(config)

    // let endpoints = find_readable_endpoints(&mut device)?;

    // for endpoint in endpoints {
    //     let has_kernel_driver = match handle.kernel_driver_active(endpoint.iface) {
    //         Ok(true) => {
    //             handle.detach_kernel_driver(endpoint.iface)?;
    //             true
    //         }
    //         _ => false,
    //     };
    //     println!("Has kernel driver? {}", has_kernel_driver);

    //     // claim and configure device
    //     configure_endpoint(&mut handle, &endpoint)?;
    //     // en

    //     handle.release_interface(endpoint.iface)?;
    //     if has_kernel_driver {
    //         handle.attach_kernel_driver(endpoint.iface)?;
    //     }
    // }

    handle.claim_interface(0)?;
    // handle.claim_interface(0x80)?;
    handle.set_alternate_setting(0, 0).unwrap();

    handle.detach_kernel_driver(2)?;
    handle.claim_interface(2)?;
    handle.set_alternate_setting(2, 0).unwrap();

    let write_ctrl = |index: u16, value: u16| {
        let t:Vec<u8> = Vec::new();
        handle.write_control(0x40, 1, value, index, &t, Duration::from_millis(200)).unwrap();
    };

    let read_ctrl = |index: u16, capacity: usize| {
        let mut buf = [1];
        handle.read_control(0xc0, 0, 0x0, index, &mut buf, Duration::from_millis(200)).unwrap();
        buf
    };

    let read_ctrl_val = |index: u16, capacity: usize, val: u16| {
        let mut buf = [1];
        handle.read_control(0xc0, 0, val, index, &mut buf, Duration::from_millis(200)).unwrap();
        buf
    };

    // Enable GPIO 5 and 6
    write_ctrl(0x00, 0x60); 
    // Set GPIO 3, 5, 6, 7 direction to OUTPUT
    write_ctrl(0x02, 0xe8);
    // Disable EEPROM Interface (disable writes), set direction of GPIO 8, 9 to OUTPUT
    write_ctrl(0x03, 0x83);
    // Set GPIO 1, 2, 3, 5, 6, 7 direction to OUTPUT
    write_ctrl(0x02, 0xef);
    
    // Enable GPIO 2, 5, 6
    write_ctrl(0x00, 0x64);
    // Enable GPIO 2, 5, 6
    write_ctrl(0x00, 0x64);
    // Enable GPIO 1, 2, 5, 6
    write_ctrl(0x00, 0x66);
    // Enable GPIO 0, 1, 2, 5, 6
    write_ctrl(0x00, 0x67);
    // Enable GPIO 1, 2, 5, 6
    write_ctrl(0x00, 0x66);
    // Enable GPIO 2, 5, 6
    write_ctrl(0x00, 0x64);
    // Enable GPIO 0, 2, 5, 6
    write_ctrl(0x00, 0x65);
    // Enable GPIO 2, 5, 6
    write_ctrl(0x00, 0x64);
    // Enable GPIO 2, 5, 6
    write_ctrl(0x00, 0x64);
    // Enable GPIO 0, 2, 5, 6
    write_ctrl(0x00, 0x65);
    write_ctrl(0x00, 0x64);
    write_ctrl(0x00, 0x64);
    write_ctrl(0x00, 0x65);
    write_ctrl(0x00, 0x64);
    write_ctrl(0x00, 0x64);
    write_ctrl(0x00, 0x65);
    write_ctrl(0x00, 0x64);
    write_ctrl(0x00, 0x64);
    write_ctrl(0x00, 0x65);
    write_ctrl(0x00, 0x64);
    write_ctrl(0x00, 0x64);
    write_ctrl(0x00, 0x65);
    write_ctrl(0x00, 0x64);
    write_ctrl(0x00, 0x64);
    write_ctrl(0x00, 0x65);
    write_ctrl(0x00, 0x64);
    write_ctrl(0x00, 0x64);
    write_ctrl(0x00, 0x65);
    // Enable GPIO 0, 5, 6
    write_ctrl(0x00, 0x61);

    // Why are we reading values we've just written?
    // And then writing them back? These do correspond in WireShark to what was just
    // written. 
    let res0 = *read_ctrl(0, 1).first().unwrap();
    let res1 = *read_ctrl(2, 1).first().unwrap();
    write_ctrl(0, res0 as u16);
    write_ctrl(2, res1 as u16);

    // Set sensor address
    write_ctrl(0x203, 0xba);
    // Set GPIO 3, 5, 6, 7 direction to OUTPUT
    write_ctrl(0x02, 0xe8);
    // Disable ROM Interface again? (Don't know what other bits do, it's usualy set to 0x80)
    write_ctrl(0x03, 0x83);
    // Enable GPIO 5 and 6
    write_ctrl(0x00, 0x60);
    // More GPIO, I think (Enable 9)
    write_ctrl(0x01, 0x02);
    // Enable interrupts
    write_ctrl(0x05, 0x00);
    // Enable interrupts from GPIO9
    write_ctrl(0x07, 0x02);
    // Enable remote wakeup from GPIO9
    write_ctrl(0x0d, 0x00);
    write_ctrl(0x0f, 0x02);
    // ??? Might disable VBI mode (going off STK1160 datasheet)
    write_ctrl(0x103, 0x00);

    // configure CLKOUT 
    write_ctrl(0x300, 0x12);

    // Set positive edge clocked pulse high when pixel counter
    // =0 or =1 and low otherwise
    // Set negative edge clocked pulse high when pixel counter
    // =0 or =1 and low otherwise.
    write_ctrl(0x350, 0x2d);
    // Clock count = 4 for each pixel, no delay
    write_ctrl(0x351, 0x01);
    // More clock stuff
    write_ctrl(0x352, 0x00);
    write_ctrl(0x353, 0x00);

    // Enable timing generator
    write_ctrl(0x300, 0x80);

    // enable CLKOUT for sensor
    write_ctrl(0x18, 0x10);
    // Disable STOP clock
    write_ctrl(0x19, 0x00);
    // Set serial interface clock divider (30MHz/0x1e*16+2) = 62240 Hz
    write_ctrl(0x202, 0x1e);

    // Cropping?
    // Set capture start position X = 80, Y = 25
    write_ctrl(0x110, 0x50);
    write_ctrl(0x111, 0x00);
    write_ctrl(0x112, 0x19);
    write_ctrl(0x113, 0x00);
    // Set capture end position X = 1360, Y = 2305
    write_ctrl(0x114, 0x50);
    write_ctrl(0x115, 0x05);
    write_ctrl(0x116, 0x09);
    write_ctrl(0x117, 0x01);

    // The first time this is read, it's set to all zeroes.
    // This register controls some video settings (see datasheet).
    let res0 = *read_ctrl(0x100, 1).first().unwrap();
    // Set Hsync Positive, Vsync Positive, data is in ITU 656 format, 8-bit data
    write_ctrl(0x100, 0x33);
    
    // Set serial COMM address (0xBA is the TI chip)
    write_ctrl(0x203, 0xba);
    // Serial bus write address (0x7f)
    // This resets the TI chip (restart microprocessor)
    write_ctrl(0x204, 0x7f);
    // Serial bus write value
    write_ctrl(0x205, 0x00);
    // Begin write operation
    write_ctrl(0x200, 0x01);
    // Serial write ready (wait)
    let r0 = read_ctrl(0x201, 1).first().unwrap();
    // Ready = CONTINUE

    // Read address 0x80 from serial
    write_ctrl(0x208, 0x80);
    // Begin read operation (Read device ID MSB from TI)
    write_ctrl(0x200, 0x20);

    // Get a bunch of info on the serial interface (such as read success)
    let r0 = read_ctrl(0x201, 1).first().unwrap();
    // Get read data
    let r0 = read_ctrl(0x209, 1).first().unwrap();

    // Read address 0x81 from serial (Read device ID LSB from TI)
    write_ctrl(0x208, 0x81);
    // Begin read operation 
    write_ctrl(0x200, 0x20);

    // Get read success
    let r0 = read_ctrl(0x201, 1).first().unwrap();
    // Get read data
    let r0 = read_ctrl(0x209, 1).first().unwrap();

    // Set serial COMM address (a0 is some unknown chip)
    write_ctrl(0x203, 0xa0);
    // Read address 0x3c
    write_ctrl(0x208, 0x3c);
    // Begin read operation
    write_ctrl(0x200, 0x20);

    // Get read success
    let r0 = read_ctrl(0x201, 1).first().unwrap();
    // Get read data
    let r0 = read_ctrl(0x209, 1).first().unwrap();
    
    // Serial bus write address
    write_ctrl(0x204, 0x08);
    // Write value (Set luminance value to a reserved value?)
    write_ctrl(0x205, 0x08);
    // Begin write operation
    write_ctrl(0x200, 0x05);
    
    // Serial write ready (wait)
    let r0 = read_ctrl(0x201, 1).first().unwrap();

    // Serial bus write address
    write_ctrl(0x204, 0x28);
    // Write value (Set video standard to (B, G, H, I, N) PAL ITU-R BT.601)
    write_ctrl(0x205, 0x04);
    // Begin write operation
    write_ctrl(0x200, 0x05);

    // Serial write ready (wait)
    let r0 = read_ctrl(0x201, 1).first().unwrap();

    // Serial bus write address
    write_ctrl(0x204, 0x30);
    // Write value (Adheres to ITU-R BT.656.4 and BT.656.5 timing)
    write_ctrl(0x205, 0x00);
    // Begin write operation
    write_ctrl(0x200, 0x05);

    // Serial write ready (wait)
    let r0 = read_ctrl(0x201, 1).first().unwrap();

    // Serial bus write address
    write_ctrl(0x204, 0x0f);
    // Write value (configure what each pin does, which is vsync etc.)
    write_ctrl(0x205, 0x0a);
    // Begin write operation
    write_ctrl(0x200, 0x05);

    // Serial write ready (wait)
    let r0 = read_ctrl(0x201, 1).first().unwrap();
    

    // This is starting audio control
    // Enable AC97 interface
    // Reset AC97 interface
    // Control write phase
    write_ctrl(0x500, 0x94);
    // Enable AC97 interface
    // AC97 Operation
    // Control write phase
    write_ctrl(0x500, 0x8c);
    // Set to 16-bit audio
    write_ctrl(0x506, 0x01);
    // Write zeroes to a reserved register??
    write_ctrl(0x507, 0x00);

    // Read video settings (currently set to 0x33)
    let r0 = *read_ctrl(0x100, 1).first().unwrap();
    // I guess set the exact same value again???
    write_ctrl(0x100, 0x33);

    // Check which GPIOs are outputs
    let r0 = *read_ctrl(0x0, 1).first().unwrap();
    // Enable GPIO 2,3,5,6
    write_ctrl(0x0, 0x6c);

    // Serial bus write address
    write_ctrl(0x204, 0x00);
    // Write value (Set composite video input source to AIP1B)
    write_ctrl(0x205, 0x02);
    // Begin write operation
    write_ctrl(0x200, 0x05);
    // Serial write ready (wait)
    let r0 = *read_ctrl(0x201, 1).first().unwrap();

    // Serial bus write address
    write_ctrl(0x204, 0x03);
    // Write value (Set settings to non-high impedance but enable vblanks)
    write_ctrl(0x205, 0x6f);
    // Begin write operation
    write_ctrl(0x200, 0x05);
    // Serial write ready (wait)
    let r0 = *read_ctrl(0x201, 1).first().unwrap();

    // Yes, set this again!!
    // Set Hsync Positive, Vsync Positive, data is in ITU 656 format, 8-bit data
    write_ctrl(0x100, 0x33);
    // Set the command address to 0x10
    write_ctrl(0x504, 0x10);
    // Enable AC97, AC97 Operation, Control Read phase, In
    write_ctrl(0x500, 0x8b);

    // Read command data
    let r0 = *read_ctrl(0x502, 1).first().unwrap(); // 08
    let r1 = *read_ctrl(0x503, 1).first().unwrap(); // 88
    // Set the command address to 0x10 (RealTek audio chip)
    // This sets the LINE IN volume
    write_ctrl(0x504, 0x10);
    // This sets the LINE IN right volume to 0dB gain
    write_ctrl(0x502, 0x08);
    // This sets the LINE IN left volume to 0dB gain, and then mutes the whole thing
    write_ctrl(0x503, 0x88);
    // Enable AC97 interface, AC97 Operation, Control write phase
    write_ctrl(0x500, 0x8c);

    // CD Volume
    write_ctrl(0x504, 0x12);
    write_ctrl(0x500, 0x8b);

    let r0 = *read_ctrl(0x502, 1).first().unwrap();
    let r1 = *read_ctrl(0x503, 1).first().unwrap();
    // Set the command address to 0x12
    // This sets the CD volume
    write_ctrl(0x504, 0x12);
    // CD Right volume to 0dB gain
    write_ctrl(0x502, 0x08);
    // CD Left volume to 0dB gain
    write_ctrl(0x503, 0x08);
    write_ctrl(0x500, 0x8c);

    // Control MIC volume
    write_ctrl(0x504, 0x0e);
    // Enable AC97, AC97 Operation, Control Read phase, In
    write_ctrl(0x500, 0x8b);
    let r0 = *read_ctrl(0x502, 1).first().unwrap();
    let r1 = *read_ctrl(0x503, 1).first().unwrap();

    // Control MIC volume
    write_ctrl(0x504, 0x0e);
    // 0 dB gain on the MIC volume
    write_ctrl(0x502, 0x08);
    // No mute
    write_ctrl(0x503, 0x00);
    write_ctrl(0x500, 0x8c);

    // Control AUX volume
    write_ctrl(0x504, 0x16);
    write_ctrl(0x500, 0x8b);

    let r0 = *read_ctrl(0x502, 1).first().unwrap();
    let r1 = *read_ctrl(0x503, 1).first().unwrap();

    // Control AUX volume
    write_ctrl(0x504, 0x16);
    // 0 dB gain on the right volume
    write_ctrl(0x502, 0x08);
    // 0 dB gain on the left volume
    write_ctrl(0x503, 0x08);
    write_ctrl(0x500, 0x8c);

    // Record select
    write_ctrl(0x504, 0x1a);
    // Set right source to CD right
    write_ctrl(0x502, 0x01);
    // Set left source to CD left
    write_ctrl(0x503, 0x01);
    write_ctrl(0x500, 0x8c);

    // Record gain
    write_ctrl(0x504, 0x1c);
    // No gain right
    write_ctrl(0x502, 0x00);
    // No gain left
    write_ctrl(0x503, 0x00);
    write_ctrl(0x500, 0x8c);

    // // BIG PAUSE

    // START - Something with the timing generator
    write_ctrl(0x301, 0x0100);
    for _ in 1..4 {
        let r0 = *read_ctrl_val(0x301, 1, 0x0200).first().unwrap();
    }

    write_ctrl(0x301, 0x0200);
    
    let r0 = *read_ctrl_val(0x301, 1, 0x0402).first().unwrap();
    // END - Something with the timing generator

    for i in 0x3c..0x3f {
        // Set sensor address to some unknown device
        write_ctrl(0x203, 0xa0);
        // Read address `i`
        write_ctrl(0x208, i);
        // Begin read operation
        write_ctrl(0x200, 0x20);

        // Read something, I don't know what
        let r0 = *read_ctrl(0x201, 1).first().unwrap();
        let r1 = *read_ctrl(0x209, 1).first().unwrap();
    }

    for i in 0x3c..0x3f {
        // Set sensor address to some unknown device
        write_ctrl(0x203, 0xa0);
        // Read address `i`
        write_ctrl(0x208, i);
        // Begin read operation
        write_ctrl(0x200, 0x20);

        let r0 = *read_ctrl(0x201, 1).first().unwrap();
        let r1 = *read_ctrl(0x209, 1).first().unwrap();
        if i == 0x3d {
            // Something with the timing generator again
            let r2 = *read_ctrl_val(0x301, 1, 0x0100).first().unwrap();
        }
    }

    loop {
        println!("Call!");

        for i in [0x0d, 0x0b] {
            let t:Vec<u8> = Vec::new();

            // Set sensor address to some unknown device
            write_ctrl(0x203, 0x82);
            // Read address `0x0d`
            write_ctrl(0x208, 0x0d);
            // Initiate read
            write_ctrl(0x200, 0x20);

            // Read
            let r0 = *read_ctrl(0x201, 1).first().unwrap();
            let r1 = *read_ctrl(0x209, 1).first().unwrap();
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

    println!("Active configuration: {}", handle.active_configuration()?);

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