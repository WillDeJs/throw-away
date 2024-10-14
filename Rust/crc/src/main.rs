fn main() {
    let poly4 = 0b10110000;
    let poly16 = 0x8005; // 00011000000000000101
    let bytes = &[0b00011110, 0b1110111 ];
    
    let crc = crc4(bytes, poly4);
    println!("CRC4 of {:?} = {:0b}", bytes, crc);

    let crc = crc4(bytes, poly4);
    println!("CRC4_2 of {:?} = {:0b}", bytes, crc);


    let crc = crc16(&[1], poly16);
    println!("CRC16 of {:?} = {:#0x}", &[1], crc);

    let crc = crc16_2(&[1], poly16 );
    println!("CRC16_2 of {:?} = {:#0x}", &[1], crc);

    let crc = crc16_3(&[1], poly16 );
    println!("CRC16_3 of {:?} = {:#0x}", &[1], crc);
}

fn crc4(bytes: &[u8], poly: u8) -> u8 {
    let mut crc = 0;
    crc = 0;
    for byte in bytes {
        crc ^= byte;
        for i in 0..8 {
            if (crc & 0x80) == 0x80 {
                crc = ((crc)^ poly) << 1;
            } else {
                crc <<= 1;
            }
        }
    }
    crc >> 5
}

fn crc4_2(bytes: &[u8], poly: u8) -> u8 {
    let mut crc = 0;
    crc = 0;
    for byte in bytes {
        crc ^= byte;
        for i in 0..8 {
            if (crc & 0x01) == 0x01 {
                crc = ((crc >> 1)^ 0x0c);
            } else {
                crc >>= 1;
            }
        }
    }
    crc >> 5
}


fn crc16(bytes: &[u8], poly: u16) -> u16 {
    let mut crc = 0;

    for byte in bytes {
        crc ^= (*byte as u16)  << 8;
        for _ in 0..8 {
            // println!("{:016b}", crc);
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ poly;
            } else {
                crc <<= 1;
            }
        }
    }
    crc 
}



fn crc16_2reversed(bytes: &[u8], poly: u16) -> u16 {
    let mut crc = 0;

    for byte in bytes {
        crc ^= (*byte as  u16);
        for _ in 0..8 {
            // println!("{:016b}", crc);
            if (crc & 0x0001) != 0 {
                crc = (crc >>1) ^ 0xa001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}



fn crc16_3(bytes: &[u8], poly: u16) -> u16 {
    let mut crc = 0;

    for byte in bytes {
        crc ^= (*byte as  u16);
        for _ in 0..8 {
            // println!("{:016b}", crc);
            if (crc & 0x0001) != 0 {
                crc = (crc >>1) ^ 0x8005;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}
