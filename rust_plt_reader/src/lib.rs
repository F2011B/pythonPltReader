use std::convert::TryInto;

pub fn i32u(data: &[u8]) -> u32 {
    let arr: [u8; 4] = data[0..4].try_into().unwrap();
    u32::from_le_bytes(arr)
}

pub fn i32s(data: &[u8]) -> i32 {
    let arr: [u8; 4] = data[0..4].try_into().unwrap();
    i32::from_le_bytes(arr)
}

pub fn i16s(data: &[u8]) -> i16 {
    let arr: [u8; 2] = data[0..2].try_into().unwrap();
    i16::from_le_bytes(arr)
}

pub fn f32_le(data: &[u8]) -> f32 {
    let arr: [u8; 4] = data[0..4].try_into().unwrap();
    f32::from_le_bytes(arr)
}

pub fn f64_le(data: &[u8]) -> f64 {
    let arr: [u8; 8] = data[0..8].try_into().unwrap();
    f64::from_le_bytes(arr)
}

#[derive(Debug, PartialEq)]
pub struct TecStrRes {
    pub correct: bool,
    pub ch: Option<char>,
    pub end: bool,
}

pub fn read_tec_str(data: &[u8]) -> TecStrRes {
    if data.len() != 4 {
        return TecStrRes { correct: false, ch: None, end: false };
    }
    let check = i32u(data);
    if check != 0 {
        TecStrRes { correct: true, ch: Some(data[0] as char), end: false }
    } else {
        TecStrRes { correct: true, ch: None, end: true }
    }
}

#[derive(Debug, PartialEq)]
pub struct QWordRes {
    pub correct: bool,
    pub qword: u64,
    pub i32ul: i32,
    pub uni_chars: String,
    pub tec_str: String,
}

pub fn construct_qword(bytes: &[u8]) -> QWordRes {
    if bytes.len() < 8 {
        return QWordRes { correct: false, qword: 0, i32ul: 0, uni_chars: String::new(), tec_str: String::new() };
    }
    let first = read_tec_str(&bytes[0..4]);
    let second = read_tec_str(&bytes[4..8]);
    let mut tec = String::new();
    if first.correct {
        if let Some(c) = first.ch {
            tec.push(c);
        }
    }
    if second.correct {
        if let Some(c) = second.ch {
            tec.push(c);
        }
    }
    let mut qword: u64 = 0;
    let mut uni = String::new();
    for (i, b) in bytes[0..8].iter().enumerate() {
        let shift = (7 - i) * 8;
        qword += (*b as u64) << shift;
        uni.push(*b as char);
    }
    let i32ul = i32s(&bytes[0..4]);
    QWordRes { correct: true, qword, i32ul, uni_chars: uni, tec_str: tec }
}

pub fn read_magic_number(bytes: &[u8]) -> QWordRes {
    if bytes.len() < 8 {
        return QWordRes { correct: false, qword: 0, i32ul: 0, uni_chars: String::new(), tec_str: String::new() };
    }
    construct_qword(&bytes[0..8])
}

#[derive(Debug, PartialEq)]
pub struct TitleRes {
    pub correct: bool,
    pub title: String,
    pub next_byte: usize,
}

pub fn get_title(bytes: &[u8]) -> TitleRes {
    let mut title = String::new();
    let mut title_end = false;
    let mut counter = 0usize;
    let mut next_rel_byte = 0usize;
    while !title_end {
        let first_rel_byte = counter * 8;
        let second_rel_byte = first_rel_byte + 4;
        if second_rel_byte + 4 > bytes.len() {
            return TitleRes { correct: false, title: title, next_byte: next_rel_byte };
        }
        let first = read_tec_str(&bytes[first_rel_byte..first_rel_byte + 4]);
        let second = read_tec_str(&bytes[second_rel_byte..second_rel_byte + 4]);
        if !first.correct || !second.correct {
            return TitleRes { correct: false, title: title, next_byte: next_rel_byte };
        }
        if first.end {
            title_end = true;
            next_rel_byte = first_rel_byte + 4;
            continue;
        }
        if let Some(c) = first.ch { title.push(c); }
        if second.end {
            title_end = true;
            next_rel_byte = second_rel_byte + 4;
            continue;
        }
        if let Some(c) = second.ch { title.push(c); }
        counter += 1;
    }
    TitleRes { correct: true, title, next_byte: next_rel_byte }
}

pub fn read_var_names(bytes: &[u8], num_vars: i32) -> (Vec<String>, usize) {
    let mut names = Vec::new();
    let mut next = 0usize;
    for _ in 0..num_vars {
        let res = get_title(&bytes[next..]);
        if !res.correct {
            break;
        }
        names.push(res.title);
        next += res.next_byte;
    }
    (names, next)
}

pub fn find_end_of_header(bytes: &[u8]) -> usize {
    let mut counter = 0usize;
    while counter * 4 + 4 <= bytes.len() {
        let value = f32_le(&bytes[counter*4..counter*4+4]);
        if (value - 357.0).abs() < f32::EPSILON {
            return counter*4 + 4;
        }
        counter += 1;
    }
    bytes.len()
}

pub fn find_zones(bytes: &[u8], eo_header: usize) -> Vec<usize> {
    let mut result = Vec::new();
    let mut counter = 0usize;
    while counter * 4 + 4 <= eo_header {
        let value = f32_le(&bytes[counter*4..counter*4+4]);
        if (value - 299.0).abs() < f32::EPSILON {
            result.push(counter*4);
        }
        counter += 1;
    }
    result
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub correct: bool,
    pub magic_num: QWordRes,
    pub byte_order: i16,
    pub file_type: String,
    pub title: String,
    pub num_vars: i32,
    pub var_names: Vec<String>,
    pub eof_header: usize,
    pub zone_markers: Vec<usize>,
}

pub fn read_header(bytes: &[u8]) -> Header {
    let file_type_name = ["FULL", "GRID", "SOLUTION"];
    let magic_num = read_magic_number(&bytes[0..8]);
    if !magic_num.correct {
        return Header {
            correct: false,
            magic_num,
            byte_order: 0,
            file_type: String::new(),
            title: String::new(),
            num_vars: 0,
            var_names: Vec::new(),
            eof_header: 0,
            zone_markers: Vec::new(),
        };
    }
    let byte_order = i16s(&bytes[8..10]);
    let file_type_idx = i16s(&bytes[12..14]);

    let title_res = get_title(&bytes[16..]);
    let title = if title_res.correct { title_res.title.clone() } else { String::new() };
    let num_vars = i32s(&bytes[title_res.next_byte + 16..title_res.next_byte + 20]);
    let start = title_res.next_byte + 20;
    let (var_names, next_byte) = read_var_names(&bytes[start..], num_vars);
    let start_after_vars = start + next_byte;
    let end_of_header = find_end_of_header(&bytes[start_after_vars..]);
    let eof_abs = start_after_vars + end_of_header;
    let zone_markers = find_zones(&bytes[start_after_vars..], end_of_header);

    Header {
        correct: true,
        magic_num,
        byte_order,
        file_type: file_type_name[file_type_idx as usize].to_string(),
        title,
        num_vars,
        var_names,
        eof_header: eof_abs,
        zone_markers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_qword() {
        let bytes = b"\x01\x00\x00\x00\x00\x00\x00\x01";
        let res = construct_qword(bytes);
        assert!(res.correct);
        assert_eq!(res.qword, 72057594037927937u64);
    }

    #[test]
    fn test_construct_qword_tecstr() {
        let bytes = b"\x2e\x00\x00\x00\x2e\x00\x00\x00";
        let res = construct_qword(bytes);
        assert!(res.correct);
        assert_eq!(res.tec_str, "..");
    }

    #[test]
    fn test_read_magic_number() {
        let bytes = b"\x23\x21\x54\x44\x56\x31\x31\x32";
        let res = read_magic_number(bytes);
        assert!(res.correct);
        assert_eq!(res.uni_chars, "#!TDV112");
    }

    #[test]
    fn test_read_header() {
        let data = b"\x23\x21\x54\x44\x56\x31\x31\x32\x01\x00\x00\x00\x00\x00\x00\x00\x2e\x00\x00\x00\x2e\x00\x00\x00\x2e\x00\x00\x00\x00\x00\x00\x00\x2f\x00\x00\x00\x50\x00\x00\x00\x69\x00\x00\x00\x63\x00\x00\x00\x74\x00\x00\x00\x75\x00\x00\x00\x72\x00\x00\x00\x65\x00\x00\x00\x00\x00\x00\x00\x78\x00\x00\x00";
        let hdr = read_header(data);
        assert!(hdr.correct);
        assert_eq!(hdr.magic_num.uni_chars, "#!TDV112");
        assert_eq!(hdr.byte_order, 1);
        assert_eq!(hdr.file_type, "FULL");
        assert_eq!(hdr.num_vars, 47);
        assert_eq!(hdr.title, "...");
    }
}
