# temporal-fmt

[![Crates.io](https://img.shields.io/crates/v/temporal-fmt.svg)](https://crates.io/crates/temporal-fmt)
[![Docs.rs](https://docs.rs/temporal-fmt/badge.svg)](https://docs.rs/temporal-fmt)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Unlicense-blue.svg)](LICENSE)

A high-performance, zero-dependency UNIX timestamp formatter for Rust. 
`temporal-fmt` calculates civil, astronomical, and meteorological variables directly from a UNIX timestamp without relying on heavy external dependencies like `chrono` or `time`.

## Features

- **Zero external dependencies**
- **No heap allocations in the parsing loop** (byte-slice sliding cursor)
- Custom format patterns including unique tags for Quarters, Astronomical Seasons, and Meteorological Seasons
- Safe execution avoiding UTF-8 boundary slice panics

## Usage

```rust
use std::time::SystemTime;
use temporal_fmt::Temporal;

fn main() {
    let now = SystemTime::now();

    // 1. Full high-precision timestamp with sub-second tokens
    let formatted_precise = Temporal::format_system_time(
        now,
        "YYYY-MM-MD hh:mm:ss.zzz (tt:qq)"
    );
    println!("Precise: {}", formatted_precise);
    // Output: Precise: 2026-09-21 14:08:45.546 (32:45)

    // 2. Custom seasonal and astronomical formatting
    let formatted_seasons = Temporal::format_system_time(
        now,
        "YYYY WWW D (AAA / SSS) Q"
    );
    println!("Seasons: {}", formatted_seasons);
    // Output: Seasons: 2026 W39 MON (AUT / AUT) 3

    // 3. Formatting from raw Unix timestamp in seconds
    let formatted_secs = Temporal::format(1700000000, "YYYY-MM-MD hh:mm");
    println!("From secs: {}", formatted_secs);
}
```

## Available Tokens

### Year & Month

* `YYYY`: 4-digit Year (e.g. 2026)
* `WYYY`: 4-digit ISO Year
* `YY`: 2-digit Year (e.g. 26)
* `WY`: 2-digit ISO Year
* `MMM`: 3-letter Month Abbreviation (e.g. JAN, FEB)
* `MM`: 2-digit Month (01-12)

### Day & Week

* `YDD`: 3-digit Day of the Year (001-366)
* `DDD`: 3-letter Weekday Abbreviation (MON-SUN)
* `MD`: 2-digit Day of the Month (01-31)
* `D`: 1-digit Weekday Number (1 = MON, 7 = SUN)
* `WW`: 2-digit ISO Week Number (01-53)

### Quarters & Seasons

* `Q`: Quarter of the year (1-4)
* `AAA`: Astronomical Season 3-letter abbreviation (SPR, SUM, AUT, WIN)
* `SSS`: Meteorological Season 3-letter abbreviation (SPR, SUM, AUT, WIN)
* `A`: Astronomical Season Number (1-4)
* `S`: Meteorological Season Number (1-4)

### Time

* `hhhh`: 12-hour format with am/pm suffix (e.g. am10, pm03)
* `hh`: 2-digit Hour (00-23)
* `mm`: 2-digit Minute (00-59)
* `ss`: 2-digit Second (00-59)
* `zzz` : 3-digit Millisecond (000-999)
* `tt`: 2-digit Tierce (00-59)
* `qq`: 2-digit Quadra (00-59)

---