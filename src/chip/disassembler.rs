/* This file is also pretty bad */
pub fn rom(rom: Vec<u8>) -> Vec<[String; 3]> {
    let mut current_byte = 0x00;
    let mut disassembly = Vec::new();

    loop {
        if current_byte + 1 >= rom.len() {
            break;
        }

        let opcode = (rom[current_byte] as u16) << 8 | rom[current_byte + 1] as u16;
        let in_mem = current_byte + 0x200;

        let byte = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8
        );

        let nnn = (opcode & 0x0FFF) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let n = byte.3 as usize;
        let x = byte.1 as usize;
        let y = byte.2 as usize;

        let op = match byte {
            (0x00, 0x00, 0x0E, 0x00) =>  [String::from("(00E0)"), String::from("CLS")],
            (0x00, 0x00, 0x0E, 0x0E) => [String::from("(00EE)"), String::from("RET")],
            (0x01, _, _, _) => [format!("(1{:03X})", nnn), format!("JP {:#05X}", nnn)],
            (0x02, _, _, _) => [format!("(2{:03X})", nnn), format!("CALL {:#05X}", nnn)],
            (0x03, _, _, _) => [format!("(3{:01X}{:02X})", x, nn), format!("SE V{:01X}, {:#04X}", x, nn)],
            (0x04, _, _, _) => [format!("(4{:01X}{:02X})", x, nn), format!("SNE V{:01X}, {:#04X}", x, nn)],
            (0x05, _, _, 0x00) => [format!("5{:01X}{:01X}0", x, y), format!("SE V{:01X}, {:01X}", x, y)],
            (0x06, _, _, _) => [format!("(6{:01X}{:02X})", x, nn), format!("LD V{:01X}, {:#04X}", x, nn)],
            (0x07, _, _, _) => [format!("(7{:01X}{:02X})", x, nn), format!("ADD V{:01X}, {:#04X}", x, nn)],
            (0x08, _, _, 0x00) => [format!("(8{:01X}{:01X}0)", x, y), format!("LD V{:01X}, V{:01X}", x, y)],
            (0x08, _, _, 0x01) => [format!("(8{:01X}{:01X}1)", x, y), format!("OR V{:01X}, V{:01X}", x, y)],
            (0x08, _, _, 0x02) => [format!("(8{:01X}{:01X}2)", x, y), format!("AND V{:01X}, V{:01X}", x, y)],
            (0x08, _, _, 0x03) => [format!("(8{:01X}{:01X}3)", x, y), format!("XOR V{:01X}, V{:01X}", x, y)],
            (0x08, _, _, 0x04) => [format!("(8{:01X}{:01X}4)", x, y), format!("ADD V{:01X}, V{:01X}", x, y)],
            (0x08, _, _, 0x05) => [format!("(8{:01X}{:01X}5)", x, y), format!("SUB V{:01X}, V{:01X}", x, y)],
            (0x08, _, _, 0x06) => [format!("(8{:01X}{:01X}6)", x, y), format!("SHR V{:01X}, V{:01X}", x, y)],
            (0x08, _, _, 0x07) => [format!("(8{:01X}{:01X}7)", x, y), format!("SUBN V{:01X}, V{:01X}", x, y)],
            (0x08, _, _, 0x0E) => [format!("8{:01X}{:01X}E", x, y), format!("SHL V{:01X}, V{:01X}", x, y)],
            (0x09, _, _, 0x00) => [format!("(9{:01X}{:01X}0)", x, y), format!("SNE V:{:01X}, V{:01X}", x, y)],
            (0x0A, _, _, _) => [format!("(A{:03X})", nnn), format!("LD I, {:#05X}", nnn)],
            (0x0B, _, _, _) => [format!("(B{:03X})", nnn), format!("JP V0, {:#05X}", nnn)],
            (0x0C, _, _, _) => [format!("(C{:01X}{:02X})", x, nn), format!("RND V{:01X}, {:#04X}", x, nn)],
            (0x0D, _, _, _) => [format!("(D{:01X}{:01X}{:01X})", x, y, n), format!("DRW V{:01X}, V{:01X}, {:#0X}", x, y, n)],
            (0x0E, _, 0x09, 0x0E) => [format!("(E{:01X}9E)", x), format!("SKP V{:01X}", x)],
            (0x0E, _, 0x0A, 0x01) => [format!("(E{:01X}A1)", x), format!("SKNP V{:01X}", x)],
            (0x0F, _, 0x00, 0x07) => [format!("(F{:01X}07)", x), format!("LD V{:01X}, DT", x)],
            (0x0F, _, 0x00, 0x0A) => [format!("(F{:01X}0A)", x), format!("LD V{:01X}, K", x)],
            (0x0F, _, 0x01, 0x05) => [format!("(F{:01X}15)", x), format!("LD DT, V{:01X}", x)],
            (0x0F, _, 0x01, 0x08) => [format!("(F{:01X}18)", x), format!("LD ST, V{:01X}", x)],
            (0x0F, _, 0x01, 0x0E) => [format!("(F{:01X}1E)", x), format!("ADD I, V{:01X}", x)],
            (0x0F, _, 0x02, 0x09) => [format!("(F{:01X}29)", x), format!("LD F, V{:01X}", x)],
            (0x0F, _, 0x03, 0x03) => [format!("(F{:01X}33)", x), format!("LD B, V{:01X}", x)],
            (0x0F, _, 0x05, 0x05) => [format!("(F{:01X}55)", x), format!("LD I, V{:01X}", x)],
            (0x0F, _, 0x06, 0x05) => [format!("(F{:01X}65)", x), format!("LD V{:01X}, I", x)],
             _ => { [String::from("ignore"), String::from("ignore")] }
        };

        if op[0] != "ignore" {
            disassembly.push([format!("{:#06X}", in_mem), op[0].clone(), op[1].clone()]);
        }
        current_byte += 2;
    }

    return disassembly;
}
