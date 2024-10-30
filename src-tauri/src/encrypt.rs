use base64::prelude::*;


const S1: [[u8; 16]; 4] = [
    [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
    [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
    [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
    [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
];

const S2: [[u8; 16]; 4] = [
    [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10],
    [3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5],
    [0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15],
    [13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9],
];

const S3: [[u8; 16]; 4] = [
    [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8],
    [13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1],
    [13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7],
    [1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12],
];

const S4: [[u8; 16]; 4] = [
    [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15],
    [13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9],
    [10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4],
    [3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14],
];

const S5: [[u8; 16]; 4] = [
    [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9],
    [14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6],
    [4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14],
    [11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3],
];

const S6: [[u8; 16]; 4] = [
    [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11],
    [10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8],
    [9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6],
    [4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13],
];

const S7: [[u8; 16]; 4] = [
    [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1],
    [13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6],
    [1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2],
    [6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12],
];

const S8: [[u8; 16]; 4] = [
    [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],
    [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
    [7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8],
    [2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11],
];

fn generate_keys(key_byte: &[u8]) -> Vec<Vec<u8>> {
    let mut key = vec![0u8; 56];
    let mut keys = vec![vec![0u8; 48]; 16];
    let loop_shifts = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];

    // Initialize the key array
    for i in 0..7 {
        for j in 0..8 {
            key[i * 8 + j] = key_byte[(7 - j) * 8 + i];
        }
    }

    // Generate the keys
    for i in 0..16 {
        for _ in 0..loop_shifts[i] {
            let temp_left = key[0];
            let temp_right = key[28];
            for k in 0..27 {
                key[k] = key[k + 1];
                key[28 + k] = key[29 + k];
            }
            key[27] = temp_left;
            key[55] = temp_right;
        }

        let temp_key = vec![
            key[13], key[16], key[10], key[23], key[0], key[4], key[2], key[27],
            key[14], key[5], key[20], key[9], key[22], key[18], key[11], key[3],
            key[25], key[7], key[15], key[6], key[26], key[19], key[12], key[1],
            key[40], key[51], key[30], key[36], key[46], key[54], key[29], key[39],
            key[50], key[44], key[32], key[47], key[43], key[48], key[38], key[55],
            key[33], key[52], key[45], key[41], key[49], key[35], key[28], key[31],
        ];

        keys[i].copy_from_slice(&temp_key);
    }

    keys
}

fn str_to_bt(s: &str) -> Vec<u8> {
    let leng = s.len();
    let mut bt = vec![0u8; 64];

    if leng < 4 {
        for (i, c) in s.chars().enumerate() {
            let k = c as u32;
            for j in 0..16 {
                let pow = (1 << (15 - j)) as u32;
                bt[16 * i + j] = ((k / pow) % 2) as u8;
            }
        }
        for p in leng..4 {
            let k = 0u32;
            for q in 0..16 {
                let pow = (1 << (15 - q)) as u32;
                bt[16 * p + q] = ((k / pow) % 2) as u8;
            }
        }
    } else {
        for (i, c) in s.chars().enumerate().take(4) {
            let k = c as u32;
            for j in 0..16 {
                let pow = (1 << (15 - j)) as u32;
                bt[16 * i + j] = ((k / pow) % 2) as u8;
            }
        }
    }
    bt
}

fn get_key_bytes(key: &str) -> Vec<Vec<u8>> {
    let leng = key.len();
    let iterator = leng / 4;
    let remainder = leng % 4;
    let mut key_bytes = Vec::with_capacity(iterator + (remainder > 0) as usize);

    for i in 0..iterator {
        key_bytes.push(str_to_bt(&key[i * 4..i * 4 + 4]));
    }
    if remainder > 0 {
        key_bytes.push(str_to_bt(&key[iterator * 4..]));
    }
    key_bytes
}

fn init_permute(original_data: &[u8]) -> Vec<u8> {
    let mut ip_byte = vec![0u8; 64];

    for i in 0..4 {
        for j in 0..8 {
            ip_byte[i * 8 + j] = original_data[(7 - j) * 8 + 2 * i + 1];
            ip_byte[i * 8 + j + 32] = original_data[(7 - j) * 8 + 2 * i];
        }
    }

    ip_byte
}

fn expand_permute(right_data: &[u8]) -> Vec<u8> {
    let mut ep_byte = vec![0u8; 48];

    for i in 0..8 {
        ep_byte[i * 6] = if i == 0 { right_data[31] } else { right_data[i * 4 - 1] };
        ep_byte[i * 6 + 1] = right_data[i * 4];
        ep_byte[i * 6 + 2] = right_data[i * 4 + 1];
        ep_byte[i * 6 + 3] = right_data[i * 4 + 2];
        ep_byte[i * 6 + 4] = right_data[i * 4 + 3];
        ep_byte[i * 6 + 5] = if i == 7 { right_data[0] } else { right_data[i * 4 + 4] };
    }

    ep_byte
}

fn xor(byte_one: &[u8], byte_two: &[u8]) -> Vec<u8> {
    byte_one.iter().zip(byte_two.iter()).map(|(&a, &b)| a ^ b).collect()
}

fn get_box_binary(i: u8) -> &'static str {
    const BINARY_VALUES: [&str; 16] = [
        "0000", "0001", "0010", "0011",
        "0100", "0101", "0110", "0111",
        "1000", "1001", "1010", "1011",
        "1100", "1101", "1110", "1111",
    ];
    BINARY_VALUES[i as usize]
}

fn s_box_permute(expand_byte: &[u8]) -> Vec<u8> {
    let mut s_box_byte = Vec::with_capacity(32);
    let s_boxes = [S1, S2, S3, S4, S5, S6, S7, S8];

    for m in 0..8 {
        let i = expand_byte[m * 6] * 2 + expand_byte[m * 6 + 5];
        let j = expand_byte[m * 6 + 1] * 8 + expand_byte[m * 6 + 2] * 4 + expand_byte[m * 6 + 3] * 2 + expand_byte[m * 6 + 4];
        let binary = get_box_binary(s_boxes[m][i as usize][j as usize]);
        // for k in 0..4 {
        //     s_box_byte[m * 4 + k] = binary.chars().nth(0).unwrap().to_digit(10).unwrap() as u8;
        // }
        for char in binary.chars() {
            s_box_byte.push(if char == '0' { 0 } else { 1 });
        }
    }

    s_box_byte
}

fn p_permute(s_box_byte: &[u8]) -> Vec<u8> {
    const P_BOX_PERMUTE: [usize; 32] = [
        15, 6, 19, 20, 28, 11, 27, 16,
        0, 14, 22, 25, 4, 17, 30, 9,
        1, 7, 23, 13, 31, 26, 2, 8,
        18, 12, 29, 5, 21, 10, 3, 24,
    ];

    let mut p_box_permute = vec![0u8; 32];
    for (i, &index) in P_BOX_PERMUTE.iter().enumerate() {
        p_box_permute[i] = s_box_byte[index];
    }
    p_box_permute
}

fn finally_permute(end_byte: &[u8]) -> Vec<u8> {
    const FP_PERMUTE: [usize; 64] = [
        39, 7, 47, 15, 55, 23, 63, 31,
        38, 6, 46, 14, 54, 22, 62, 30,
        37, 5, 45, 13, 53, 21, 61, 29,
        36, 4, 44, 12, 52, 20, 60, 28,
        35, 3, 43, 11, 51, 19, 59, 27,
        34, 2, 42, 10, 50, 18, 58, 26,
        33, 1, 41, 9, 49, 17, 57, 25,
        32, 0, 40, 8, 48, 16, 56, 24,
    ];

    let mut fp_byte = vec![0u8; 64];
    for (i, &index) in FP_PERMUTE.iter().enumerate() {
        fp_byte[i] = end_byte[index];
    }
    fp_byte
}

fn enc(data_byte: &[u8], key_byte: &[u8]) -> Vec<u8> {
    // println!("{data_byte:?}{key_byte:?}");
    let keys = generate_keys(key_byte);
    let mut ip_byte = init_permute(data_byte);
    // println!("{:#?} {:#?}",&keys,&ip_byte);


    let mut ip_left = ip_byte[..32].to_vec();
    let mut ip_right = ip_byte[32..].to_vec();

    for i in 0..16 {
        let temp_left = ip_left.clone();
        ip_left = ip_right.clone();
        let key = &keys[i];
        let temp_right = xor(&p_permute(&s_box_permute(&xor(&expand_permute(&ip_right), key))), &temp_left);
        ip_right = temp_right;
    }

    let mut final_data = vec![0u8; 64];
    final_data[..32].copy_from_slice(&ip_right);
    final_data[32..].copy_from_slice(&ip_left);

    finally_permute(&final_data)
}

fn bt4_to_hex(binary: &str) -> char {
    const HEX_VALUES: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7',
        '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let index = u8::from_str_radix(binary, 2).expect(&format!("{binary}")) as usize;
    HEX_VALUES[index]
}

fn bt64_to_hex(byte_data: &[u8]) -> String {
    let mut hex = String::new();
    for i in 0..16 {
        let mut bt = String::new();
        for j in 0..4 {
            bt += &byte_data[i * 4 + j].to_string();
        }
        hex.push(bt4_to_hex(&bt));
    }
    hex
    // byte_data.iter().map(|byte| format!("{:02x}", byte)).collect()
}

pub fn str_enc(data: &str, first_key: &str, second_key: &str, third_key: &str) -> String {
    let leng = data.len();
    let mut enc_data = String::new();
    let first_key_bt = if !first_key.is_empty() { get_key_bytes(first_key) } else { Vec::new() };
    let second_key_bt = if !second_key.is_empty() { get_key_bytes(second_key) } else { Vec::new() };
    let third_key_bt = if !third_key.is_empty() { get_key_bytes(third_key) } else { Vec::new() };
    let first_length = first_key_bt.len();
    let second_length = second_key_bt.len();
    let third_length = third_key_bt.len();

    if leng == 0 {
        return enc_data;
    }

    let enc_block = |block: &str| -> Vec<u8> {
        let mut temp_bt = str_to_bt(block);
        if !first_key.is_empty() {
            for x in 0..first_length {
                temp_bt = enc(&temp_bt, &first_key_bt[x]);
            }
        }
        if !second_key.is_empty() {
            for y in 0..second_length {
                temp_bt = enc(&temp_bt, &second_key_bt[y]);
            }
        }
        if !third_key.is_empty() {
            for z in 0..third_length {
                temp_bt = enc(&temp_bt, &third_key_bt[z]);
            }
        }
        temp_bt
    };

    if leng < 4 {
        enc_data = bt64_to_hex(&enc_block(data));
    } else {
        let iterator = leng / 4;
        let remainder = leng % 4;

        for i in 0..iterator {
            let temp_data = &data[i * 4..i * 4 + 4];
            enc_data.push_str(&bt64_to_hex(&enc_block(temp_data)));
        }

        if remainder > 0 {
            let remainder_data = &data[iterator * 4..];
            enc_data.push_str(&bt64_to_hex(&enc_block(remainder_data)));
        }
    }

    enc_data
}

pub fn b64_encode(data: &str) -> String {
    BASE64_STANDARD.encode(data)
}

pub fn md5(s: &str) -> String {
    let digest = md5::compute(s);
    format!("{:x}", digest)
}

pub fn fast_enc(data: &str, first_key: &str) -> String {
    b64_encode(&str_enc(data, first_key, "", ""))
}

pub fn enc_params(deskey: &str, nowtime: &str, params: &str) -> String {
    let token = md5(&(md5(params) + &md5(nowtime)));
    let p = fast_enc(params, deskey);
    format!("params={p}&token={token}&timestamp={nowtime}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enc() {
        let start = std::time::Instant::now();
        let s = b64_encode(&str_enc("202111150036", "bnu", "", ""));
        println!("{s} {:?}", start.elapsed());
    }
    #[test]
    fn test_param() {
        let des = "59284172985747211583399";
        let now_time = "2024-10-25 19:57:52";
        let params = "xktype=2&xn=2024&xq=0&xh=202161286408&nj=2021&zydm=BQ108&kcdm=2310178582&kclb1=05&kclb2=A1&kclb3=01&khfs=01&skbjdm=2310178582-01&skbzdm=&xf=1.0&is_checkTime=1&kknj=&kkzydm=&txt_skbjdm=&xk_points=0&is_buy_book=0&is_cx=0&is_yxtj=1&menucode_current=JW130403&kcfw=zxbnj";
        let res = "params=QzIwNEVEQTQ2RUVGODFBNzFEN0E5M0M1MjBGNEM0RTM3NkUwNDhGOTUxQzQ3RkJBOEVGRDFFNTMyMjQ0MkUyNDE4ODNDNkU0ODMyOTA4RTU1OTJERjY1ODQzODJBNEY4QkFGMDFENDZEMkI1NTI4OEM3RjU0OTA5QTU3Mjk3Nzc1OUVCMDYzMTNEMjBDMjJFNDcyMkFBREM4QUVCRkMyQ0JBRjAxRDQ2RDJCNTUyODhGMjgxMjY4MzlFQ0E2M0EzMkMyMzMyQUQ0QkQ4MDg4QkRCNzI1OTQ0QzdBQjFDNjI0NEUxM0JCNTEwOEZBMEJFQkMyNUE4MTIxMDI1MTNCMkREM0QzQjNDRThDRTBCRTQ4NTg1MkMwMjRFNzI2QTg0QUI2RjgyRDAxNjk4OUMyQzI2Qjg5MTgxOTFBRUE3OTBFMDI3QjQ5NjJBMURFNzBDNERENEE1MTY3MjAxQUQ1MEFENjc3MzlEMzFBRThCMzA4NTE3NEU0RUQ3RUU1OEFFNTg5MzM0QkY2NENGODM5REQ2MzhDQjhDN0Q0RTdGNTU1ODkzMzRCRjY0Q0Y4MzlEMEJBOTJFMEY1ODZGNkM5NUEwRDA1NUY4RjJBRkE1NEM2RThCQkRDMkYzRjE3QzhFN0Y1MEY3REYwMDY0MjRCNUFGQTBGMEFCQTJENUI1NTM3MUE1MUUzNEY4MEIwQjUyRTc1MjlFNzhDMkM1NjI0ODU1RTcyODcwNDQ2OTg0OUNGOTg1RTlGMUNGQjZFRjEwOUZDQjUyRTFEOTA3MzJBOTk3MkNGQzY0RTc5RDIzM0E0RjcwQTQ5RTQ0M0RFMjM0MDE3MTlERDY2NTk0REU3RDhERUIzQzVBMDQ2MkIzRERDMENGNzcxNUM2OUE4NDY1NjhBN0FDQzU2Q0FDODAyM0QxRkI4NzRDMjU3Q0NFRDdCNEIwNThGNzQzNDkzNDY3N0ZBQTRGNzNEQTY2NjcyRjc4RUYwNDgzQThGMUJCM0IyOTQ2OTJCM0UxMTBBMjFFNzlFREJEMzFFQzU0Qjg5RjEwNTI5MkNBNEM2Q0FEQUMyOTIzMDI4NDJGRDk5RDlDODJENkU0MjA4RkM4NDk4MDlDNDZGNkNENEJDRDFCMkFCOTU0NThFNzY4RDQwNEVDOUM0NkY2Q0Q0QkNEMUIyQTYzNUQ3OEEyOTgyOEVDMDhDQzdDRDg4RjQ0QzBFQzUxMkExRjNCMjEwNDNDMjk1MkMyMzREQ0IxOUZBNkNBOTAxOEFGQ0E0QkRFRTIwNEY3Q0VGNDYzQjFCRUJDMkIwMTRDNjY4OTA5QkZGQUIwNUE2RDk4MUU2NjhFOERDREVGQzM1NTYxNzRFNUI1QTMzODkxQUE0RTUwNkY1MTBCNDNDRjg4MTBCNTE2MDNDMkI1Mzc1NEJEOTA2MjYxMDI4RQ==&token=a189d29995cfe6517db41d46b8b10667&timestamp=2024-10-25 19:57:52";

        eprintln!("{}", enc_params(des, now_time, params));
        assert_eq!(res,enc_params(des, now_time, params))
    }
}
