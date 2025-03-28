编译生成core0 bin的命令：
cargo objcopy --bin core0 -- -O binary core0.bin

编译生成core0 hex的命令：
cargo objcopy --bin core0 -- -O ihex core0.hex

编译生成core1 bin的命令：
cargo objcopy --bin core1 -- -O binary core1.bin

编译生成core1 hex的命令：
cargo objcopy --bin core1 -- -O ihex core1.hex



hex的合并指令：
srec_cat ./core_m0/core0.hex -intel ./core_m4/core1.hex -intel -o firmware.hex -intel
