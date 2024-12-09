# Rust Example OS

- 아키텍처 : x86_64

### 1. 부트로더 (여기서는 BIOS 기반 부트로더를 구현한다.)
```
/* 러스트에서는 표준 라이브러리를 사용하지 않기 위해서는 다음과 같이 선언해 준다. */
#![no_std]

/* 프로그램의 첫 진입점이 main이 되지 않도록 다음과 같이 main 함수가 없음을 선언해 준다. */
#![no_main]

/* 기본적으로 panic을 다루기 위한 panic 핸들러가 필수이다. (no_std 환경) */
#[panic_handler]

/* 링커 인자로 진입점을 추가해 줘야 한다. _start 함수를 만들어 진입 함수를 만들고 컴파일 시 인자로 지정해준다. */
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

여기서 no_mangle은 해당 함수의 네임 망글링(Mangling)을 하지 않겠다는 선언이다. 즉, _start 함수는 컴파일러가 망글링 하지 않는다.
(Mangling: 컴파일러가 심볼, 또는 함수 변수 이름을 고유하게 변환하는 과정((ex) 함수 오버로딩등의 처리를 위함. 네임스페이스 충돌 방지))

extern 키워드는 Rust 코드가 함수 호출 규약을 지정하여, 특정 언어와 상호 운용될 수 있도록 인터페이스를 정의한다. (여기선 "C" 이므로 C에 대한 규칙에 맞춰 컴파일 된다.)

이후 윈도우에서 컴파일 하므로 다음 링크를 참조하여

https://learn.microsoft.com/en-us/cpp/build/reference/entry-entry-point-symbol?view=msvc-170
MSVC의 링킹 인자를 참조 다음과 같이 컴파일 한다.

cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
```

```
1. UEFI(Unified Extensible Firmware Interface)
UEFI (통합 확장 펌웨어 인터페이스)는 컴퓨터의 부팅 과정을 제어하는 최신 펌웨어 인터페이스.
전통적인 BIOS(Basic Input/Output System)을 대체하며, 부팅 프로세스, 운영 체제 로드, 하드웨어 초기화, 그리고 시스템 관리 기능을 제공한다.

<주요 특징>
1.1 부팅 시간 단축 : 병렬 하드웨어 초기화 및 최적화된 부팅 경로를 통해 부팅 속도를 크게 향상

1.2 대용량 디스크 지원
- GPT(GUID Partition Table)을 사용하여 2TB 이상의 대용량 디스크를 지원한다.
** GUID: Globally Unique Identifier
- MBR(Master Boot Record) 는 2TB 디스크 크기 제한이 있지만, 이 제한을 없앴다.

1.3 32bit/64bit 환경
- UEFI는 32비트와 64비트 환경을 모두 지원하며, 특히 64비트 모드에서 더 많은 메모리 공간과 성능을 제공.

1.4. GUI 지원
- BIOS의 텍스트 기반 화면과 달리, UEFI는 마우스와 키보드 입력을 모두 지원하는 그래픽 인터페이스를 제공

1.5 모듈식 설계
- 다양한 펌웨어와 하드웨어 제조업체에서 쉽게 확장 가능한 구조

1.6 보안 기능
- Secure Boot: 부팅 과정에서 디지털 서명을 확인하여 악성 소프트웨어로부터 시스템을 보호.
- 암호화 키 관리: 시스템 보안을 강화하기 위한 다양한 암호화 키 정책 관리.

1.7 네트워킹 지원
- UEFI는 네트워크를 통해 원격으로 시스템을 부팅하는 PXE(Preboot Execution Enviroment)를 지원한다. (이는 서버 관리와 클라우드 환경에서 유용하다.)

1.8 운영체제 독립성
- UEFI는 하드웨어와 소프트웨어 간의 추상화를 제공하여 특정 운영체제에 종속 되지 않는다.

<주요 구성 요소>
2.1 UEFI 펌웨어
- 시스템의 초기 하드웨어 및 소프트웨어 초기화를 담당.

2.2 EFI System Partition (ESP)
- 운영 체제와 부팅 로더가 사용하는 특별한 디스크 파티션.
- 부팅 관련 파일(Ubuntu의 grubx64.efi, Windows의 bootmgfw.efi등)을 저장한다.

2.3 UEFI 부팅 로더
- 운영 체제를 로드하기 위한 첫 번째 프로그램
- OS를 메모리에 적재하고 제어를 넘긴다.

2.4 NVRAM
- 비휘발성 메모리로, 부팅 설정 및 환경 변수를 저장한다.
```

```
2. BIOS(Basic Input/Output System)
BIOS는 컴퓨터의 하드웨어를 제어하고 운영체제가 로드되기 전에 하드웨어를 초기화 하는 역할을 하는 펌웨어 이다.
BIOS는 마더보드의 ROM(Read-Only Memory)칩에 저장되어 있으며, 컴퓨터를 켤 떄 가장 먼저 실행되는 소프트웨어이다.

<주요 역할>
1.1 전원 공급 및 초기화(POST)
- 컴퓨터를 켜면 BIOS는 Power-On Self-Test(POST)를 실행하여 하드웨어가 제대로 작동하는지 확인한다.
CPU, RAM, 저장 장치, 그래픽 카드 등 주요 하드웨어가 문제가 없는지 점검합니다.
- 하드웨어 이상이 발견되면 경고음(beep code)이나 화면 메시지로 알려준다.

1.2 부트로더 실행
- 하드웨어 점검이 완료되면 부팅 장치를 확인하고 운영체제를 로드한다.
- 부팅 장치는 BIOS 설정에서 우선 순위 설정이 가능하다.

1.3 하드웨어와 소프트웨어 간 인터페이스 제공
- BIOS는 기본적인 입출력 기능을 제공한다. 운영체제와 소프트웨어는 BIOS를 통해 하드웨어에 접근한다.

1.4 설정 저장 및 관리(CMOS)
- BIOS 설정은 CMOS RAM이라는 소형 메모리에 저장한다. CMOS는 컴퓨터가 꺼져 있을때도 설정을 유지하기 위해 배터리로 전원이 공급된다.
- BIOS 설정 화면에서 CMOS에 저장된 설정 데이터를 변경 할 수 있다.
```

```
BIOS 부팅 시에는 16-bit compability mode (real mode)로 진행된다. (운영체제가 로드되고 나서야 가상 메모리 환경이 적용 됨.)
64-bit 기준 모드 변경
16-bit real mode -> 32bit protected-mode -> 64-bit longmode
```

```
Compile 시에 다음과 같은 명령 필요 (Rust는 Nightly 버전이여야 -Z 커맨드를 사용 가능하다. )
cargo build -Z build-std=core,compiler_builtins -Z build-std-features=compiler-builtins-mem --target x86_64-HLeos.json
```

### 2. VGA 출력 다루기 (Visual Graphic Array)
- https://en.wikipedia.org/wiki/Video_Graphics_Array
- https://en.wikipedia.org/wiki/VGA_text_mode


- Simple VGA Driver 구현
```
#VGA Buffer의 시작 주소는 0xb8000 으로 정의되어 있다.

#static HELLO: &[u8] = b"Hello BootLoader!!";
#- &[u8] >> unsigned 8bit integer (&[u8]는 크기가 고정되지 않은 u8 배열에 대한 불변 참조를 나타낸다.)
#- b"Hello BootLoader!!" >> b"..."는 바이트 문자열 리터럴을 나타낸다. 

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

# vga_buffer: VGA Buffer 시작주소를 매핑
# HELLO 바이트 문자열 리터럴을 iter()로 순회하면서 enumerate()를 호출 각 요소와 인덱스를 반환받아.
# unsafe 블록 (Rust는 메모리를 직접 조작하는 작업을 안전하지 않은것으로 판단하므로, 포인터 연산을 수행하려면 unsafe 블록 내에서 작성해야 한다.)

# *vga_buffer.offset(i as isize * 2) = byte >> offset(n)은 포인터의 n 번째 위치를 가리킨다. VGA 텍스트 모드에서, 화면에 출력되는 각 문자는 2바이트 단위로 저장된다. (첫번째 바이트 ASCII 값, 두번째 바이트 문자 속성(색상 배경 등))
# i as isize * 2 는 문자마다 2바이트 간격으로 저장되는 VGA 메모리 레이아웃을 처리하기 위한 계산이다.
# *vga_buffer.offset(i as iszie * 2 + 1) = 0xb; >> 2바이트 중 두번째 바이트 들을 나타내며 0xb의 값은 전경색으로 Light Cyan을 지정함을 의미한다.
```

```
[dependencies] 에 bootloader = "0.9" 추가
cargo install bootimage 실행

>> bootimage 라이브러리는, ELF file로 만들어주고 커널 링크를 자동으로 실행해 준다.

이 후
cargo bootimage

실행하여 이미지 파일을 가져온다.
```

### 3. VGA Text Mode


- VGA Buffer의 구조는 다음과 같다.

|Byte|||||||||1byte|||||||15byte|
|---|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|
|Bit|0|1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|
|Value|A|S|C|I|I|C|O|D|FORE|GROUND|CO|LOR|BACK|GROUND|COLOR|Blink|

- 다음은 색상 값이다. (4비트, 3비트가 메인 색상, 1비트가  Bright Bit를 담당)
- background 에서는 Bright Bit가 Blink Bit로 대체된다.

|Number|Color|Number + Bright Bit|Bright Color|
|------|-----|-------------------|------------|
|0x0   |Black|0x8                |Dark Gray|
|0x1   |Black|0x9                |Light Blue|
|0x2   |Black|0xa                |Light Green|
|0x3   |Black|0xb                |Light Cyan|
|0x4   |Black|0xc                |Light Red|
|0x5   |Black|0xd                |Pink|
|0x6   |Black|0xe                |Yellow|
|0x7   |Black|0xf                 |White|


```
$ 추가적인 사실
- Rust 컴파일러의 상수 추론기는 참조를 포인터로 컴파일 타임에 변환하는것을 허용하지 않는다. 
(즉 static 스코프 내에서 const 인자를 생성하여 할당 할 수 없다.)
```