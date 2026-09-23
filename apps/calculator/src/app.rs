use crate::engine::Calculator;
use super::abi;
use super::abi::KeyEvent;

pub fn run() -> ! {
    let mut calc = Calculator::new();
    let title = b"Aether Calculator\0";
    let mut value = [0u8; 32];

    loop {
        abi::fill_rect(40, 40, 360, 300, 0x00C0C0C0);
        abi::draw_text(56, 56, title.as_ptr(), 0x00000000);
        render_value(&mut value, calc.value(), calc.is_error());
        abi::draw_text(56, 80, value.as_ptr(), 0x00000000);

        let mut ev = KeyEvent { key: 0, pressed: 0, hid_code: 0, modifiers: 0 };
        if abi::poll_key(&mut ev) != 0 && ev.pressed != 0 {
            match ev.key {
                b'0'..=b'9' => calc.push_digit(ev.key - b'0'),
                b'+' => calc.add(),
                b'-' => calc.subtract(),
                b'=' | b'\n' => calc.equals(),
                b'*' => calc.multiply(),
                b'/' => calc.divide(),
                0x08 | b'c' | b'C' => calc.clear(),
                0x1b => abi::exit(0),
                _ => {}
            }
        } else {
            abi::yield_now();
        }
    }
}

fn render_value(buf: &mut [u8; 32], value: Option<i64>, error: bool) {
    let mut i = 0usize;
    while i < buf.len() { buf[i] = 0; i += 1; }

    if error {
        copy(b"ERROR", buf);
        return;
    }

    let mut n = value.unwrap_or(0);
    let mut pos = 31usize;
    if n == 0 {
        buf[30] = b'0';
        buf[31] = 0;
        return;
    }

    let neg = n < 0;
    if neg { n = n.wrapping_neg(); }

    while n > 0 && pos > 0 {
        pos -= 1;
        buf[pos] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    if neg && pos > 0 {
        pos -= 1;
        buf[pos] = b'-';
    }

    let mut j = 0usize;
    while pos + j < 31 {
        buf[j] = buf[pos + j];
        j += 1;
    }
    buf[j] = 0;
}

fn copy(src: &[u8], dst: &mut [u8; 32]) {
    let mut i = 0usize;
    while i < src.len() && i + 1 < dst.len() {
        dst[i] = src[i];
        i += 1;
    }
    dst[i] = 0;
}
