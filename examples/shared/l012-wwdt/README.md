# WWDT warning example

Select one exact feature with defaults disabled: `cw32a030c8t7`, `cw32f002f3p7`,
`cw32f003f4p7`, `cw32f020c6u7`, `cw32f030c8t7`, `cw32l012c8t6`, `cw32l012c8u6`,
`cw32l031c8t6`, `cw32l052c8t6`, `cw32l083rct6` or `cw32r031c8u6`.
The non-L012 selections have not been compiled or tested. Local builds/tests
remain prohibited under the current work policy.

The example intentionally does not feed: execution causes watchdog reset.
Warning-task delivery before reset is not guaranteed. No pins are needed;
keep the default system clocks and WWDT gate running. Any independently enabled
IWDT IRQ needs its own shared-vector handler.
See [interface and source boundaries](../../../docs/wwdt-warning.md).
