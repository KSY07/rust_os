use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)] // dead_code에 대한 warning을 허용한다. (띄우지 않음)
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // https://doc.rust-lang.org/rust-by-example/trait/derive.html Traits의 일종
#[repr(u8)] // C 스타일의 enum 을 사용한다. enum의 item 들은 u8 단위로 저장된다. // repr은 구조체나 열거형의 메모리 레이아웃을 명시적으로 제어하기 위해 사용된다. // 여기서는 기본 크기를 u8로 지정함.
// 이외에도 repr(packed) --> 구조체를 패딩 없이 정렬함. (위험성이 존재함 사용시 unsafe 블록에서만 메모리 접근 추천) repr(align(N)) --> 정렬을 명시적으로 지정함. (빌 경우 패딩으로 정렬) repr(Rust) >> 러스트의 기본 레이아웃 최적화를 따름.  
pub enum Color{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // transparent를 통해 struct에 정의한 u8의 형식을 그대로 따른다.
struct ColorCode(u8);

// VGA Buffer 구조
/*
    0 1 2 3 4 5 6 7   1byte     8 9 10 11                   12 13 14                        15   2byte
    [][][][][][][][]            [][][][]                    [][][]                          []
    0-7 bit ASCII code print    8-11 Foreground color       12-14 Background color          15 Blink
    
    
    

*/

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8)) // background color를 4비트 밀어서 12에 위치시킴 그리고 0000 부분에 foreground color를 넣음.
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // C 스타일 struct임을 보장함.
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25; // usize >> pointer size unsigned int
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    // Volatile struct로 감싸서 예외처리를 돕는다.
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT], // ScreenChar 타입의 값이 BUFFER_WIDTH 만큼 연속적 저장.
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT { // 각 버퍼 y 축을 순회하며, 기존 것을 위로 다시 쓰고 아래를 지운다.
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read(); // use volatile we can read from buffer
                self.buffer.chars[row - 1][col].write(character);
            }
            self.clear_row(BUFFER_HEIGHT - 1);
            self.column_position = 0;
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH { // BUFFER 넓이만큼 공백으로 채운다.
            self.buffer.chars[row][col].write(blank);
        }
    }
    
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(()) // return Ok
    }
}


pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer)},
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Bootloader!!");
    write!(writer, "The numbers are {} and {}", 42, 1.0 / 3.0).unwrap();
}

// Global에서 쓸 수 있는 static 구조체를 선언 한다.
// lazy_static은 immutable static을 mutable한 static으로 전환해 준다. 
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer)},
    });
}
