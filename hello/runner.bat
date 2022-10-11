@echo off

set "f=%1"
call set "f=%%f:\=/%%"
call set "f=%%f:C:=%%"

set "d=%CARGO_MANIFEST_DIR%"
call set "d=%%d:\=/%%"
call set "d=%%d:C:=%%"

docker run --rm -v $CARGO_MANIFEST_DIR:$CARGO_MANIFEST_DIR \
    qemu ^
    sh -c ^"cd /%d%; ^
    qemu-system-gnuarmeclipse ^
        -cpu cortex-m4 \
        -machine STM32F4-Discovery ^
        -nographic ^
        -semihosting-config enable=on,target=native ^
        -kernel $1"