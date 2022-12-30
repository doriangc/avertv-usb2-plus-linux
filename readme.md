# AverMedia AverTV USB2.0 Plus

## Board Overview
### DC 1120-E
- This is the main USB chip (controls pretty much everything)
- Datasheet for extremely similar STK1160 here: https://datasheetspdf.com/datasheet/STK1160.html

### Texas Instruments 5150AM1
- Video Decoder
- https://www.ti.com/lit/ds/symlink/tvp5150am1.pdf

### ST A21SP16
- Audio Amplifier

### 93CC46
- EEPROM
- Probably what the serial reads go to?
- https://datasheetspdf.com/pdf-file/1144999/ETC/S-93CC46/1

### AIC1525 
- USB Power
- https://pdf1.alldatasheet.com/datasheet-pdf/view/54872/AIC/AIC1525.html

### Sonix SN8P2501
- Some kind of 8-bit microcontroller
- Not sure what it's used for

### ALC 655

### 74HCT4052D

### LG InnoTek TALN-M205T
- TV Tuner
- Contains EPCOS  K3965D


## Some I2C sinks
### 0xba
This is the TI chip
### 0xc2, 0x86, 0x1c, 0xc2
These seem to control TV tuning
Packets 32706 to 34272 in chnl.pcap change channel from 374.75 to 537.50


