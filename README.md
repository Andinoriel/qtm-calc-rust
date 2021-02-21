# qtm-calc-rust

This is Rust reimplementation of [qtm-calc](https://github.com/Andinoriel/qtm-calc) tool and provided as a dynamic library for embedding into Python language (using pyo3).

## Build

1. Clone this project and then cd to the project folder;

2. On Windows and Linux, you can build normally with:
```
    cargo build --release
```

* On macOS, you need to set additional linker arguments. One option is to compile with:
```
cargo rustc --release -- -C link-arg=-undefined -C link-arg=dynamic_lookup
```

3. Aftet build you must rename the shared library from the target folder: 
* on MacOS, rename libqtmcalc.dylib to libqtmcalc.sо;
* on Windows libqtmcalc.dll to libqtmcalc.pyd;
* on Linux leave it unchanged.

## Usage

Example usage in python code:
```python
import libqtmcalc

x = libqtmcalc.Qtm(10, 1, 1.5, 0.7, 0, -1)
x.calc_final_states() # necessary for further action
print(x.final_states())

# calc operational characteristics
print(libqtmcalc.QtmData.calc_avg_queue(x))
print(libqtmcalc.QtmData.calc_ete(x))
print(libqtmcalc.QtmData.calc_avg_time_queue(x))
print(libqtmcalc.QtmData.calc_perc_served_req(x))
print(libqtmcalc.QtmData.calc_avg_count_served_req(x))
print(libqtmcalc.QtmData.calc_avg_count_req(x))

# get current values of system internals
print(x.channel_count)
print(x.queue_size)
print(x.la)
print(x.mu)
print(x.nu)
print(x.n)

# set new values of system internals
# keep in mind that after setting new values
# you must perform x.calc_final_states() again for using actual fs
x.channel_count = 11
x.queue_size = 2
x.la = 2.5
x.mu = 1.7
x.nu = 1
x.n = 1
```

## License

This project is licensed under the [MIT License](LICENSE).

## Credits

My thanks to the developers of the [pyo3](https://github.com/PyO3/pyo3),  [nalgebra](https://github.com/dimforge/nalgebra) and [float-ord](https://github.com/notriddle/rust-float-ord).
