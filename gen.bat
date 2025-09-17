
del /Q %~dp0\src\*.*
del %~dp0\build.rs
del %~dp0\device.x

svd2rust --target cortex-m -i %~dp0\svd\sh30f2602_patch.svd

form -i %~dp0\lib.rs -o %~dp0\src

del %~dp0\lib.rs

cargo fmt

cargo build
