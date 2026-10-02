# 来源和许可

本项目原创 Rust 代码采用 MIT OR Apache-2.0。未复制 embassy-stm32
驱动实现；架构参考 Embassy 官方源码（当前分层链路详见 README）：
https://docs.rs/crate/embassy-stm32/0.6.0/source/build.rs
https://docs.rs/crate/embassy-stm32/0.6.0/source/src/lib.rs
上游 Embassy 采用 MIT OR Apache-2.0。

寄存器事实来自 WHXY CW32L012 SDK V1.0.5、CMSIS header V1.2 (2026-06-24)、
MDK DFP 1.0.2 SVD V1.2、CW32L012 User Manual CN V1.4。
厂家 CMSIS 头部标记 Copyright (c) 2009-2018 ARM Limited、Apache-2.0。
厂家 C 外设库另有自己的代码许可/免责条款；本项目不重新许可或分发整份 SDK。
vendor/ 保留原始 CMSIS/SVD 与单独 GPIO SDK 头文件内容供离线审核，未分发完整 SDK。
GPIO 头文件保留原始武汉芯源代码许可和免责信息，不纳入本项目原创 Rust 的重新许可。使用者须遵守
下载文件各自条款。元数据记录的是硬件事实，不覆盖厂商商标或支持承诺。

Cargo.lock 在本地解析生成，不进入源码交付；Cargo.toml 固定直接依赖版本。各依赖依其自身 crate 许可证使用。
参考来源记录日期：2026-10-02。未推送、发布或声明 Embassy 官方支持 CW32。

## User-supplied BLDC example application

The numbered `examples/` application is a Rust adaptation of the user-supplied
`10 XUNLIANYING 260726 LAST.zip`, especially `BLDC CONTROL/MOTOR CONTORL`
and `BLDC CONTROL/User/main.c`. The uploaded archive is not redistributed.
No independent license grant for that application was found in the supplied
application files. The repository licenses do not purport to relicense any
third-party rights in the original application; confirm rights before external
redistribution or commercial use. Included vendor headers remain governed by
their existing notices. Original names, pins, constants and behavior are
identified in `examples/l012-bldc/README.md` for traceability, not hardware certification.
