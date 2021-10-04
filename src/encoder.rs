use crate::shared::{ BASE64_TABLE, SIX_BIT_MASK, PADDING };
use std::io::{ Read, Write };

pub fn encode(input: &mut dyn Read, out: &mut dyn Write) -> Result<(), String> {
    let mut buffer = vec![0 as u8; 3];

    loop {
        let line_read = match input.read(&mut buffer[..]) {
            Ok(o) => o,
            Err(e) => return Err(format!("error reading input {}", e))   
        };

        if line_read <= 0 {
            break;
        }

        let seg_data = &buffer[..line_read];
        
        let mut segment_count = 0;
        let mut dec = 0;
        for i in seg_data {
            let l_shift: u64 = 16 - segment_count * 8;
            let i = *i as u64;
            dec |= i << l_shift;

            segment_count = segment_count + 1;
        }
        
        for i in 0..segment_count+1 {
            let r_shift: u64 =  18 - i * 6;
            let b = ((dec >> r_shift) & SIX_BIT_MASK) as u8;
            let r = BASE64_TABLE[b as usize];
            if let Err(e) = out.write(r.as_bytes()) {
                return Err(format!("error write buffer out {}", e));
            }
        }

        if segment_count == 1 {
            if let Err(e) = out.write(&[PADDING, PADDING]) {
                return Err(format!("error write buffer out {}", e));
            }
        } else if segment_count == 2 {
            if let Err(e) = out.write(&[PADDING]) {
                return Err(format!("error write buffer out {}", e));
            }
        }
        
    }

    Ok(())
}