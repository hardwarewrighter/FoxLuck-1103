# Luckfox Pico RV1103 hardware

Documentation checked 2026-10-06. The owner's board was identified as the
standard **Luckfox Pico**, not Pico Plus, Mini, Pro, or Max. The directory
name `Foxluck-1103` is retained; the manufacturer spells the name Luckfox.
Published specifications below are distinct from observations of this unit.

## Board and processor

| Component | Published specification |
| --- | --- |
| SoC | Rockchip RV1103 |
| Linux CPU | Single Arm Cortex-A7 at up to 1.2 GHz; 32-bit ARMv7-A, NEON and FPU |
| Auxiliary processor | RISC-V MCU; not the processor running the observed Linux system |
| Memory | 64 MB DDR2 |
| Boot/storage | microSD on the standard Pico |
| USB | USB 2.0 host/device through Type-C |
| Camera | Two-lane MIPI CSI; ISP input up to 4 MP at 30 fps |
| Video engine | Hardware H.264/H.265 encoding |
| Expansion | GPIO, UART, SPI and I2C; standard Pico specifies 24 GPIO pins |

These specifications come from the [Luckfox RV1103 board comparison](https://wiki.luckfox.com/Luckfox-Pico/Luckfox-Pico-RV1103/)
and [standard Pico product page](https://www.luckfox.com/Luckfox-Pico/EN-Luckfox-Pico).
Do not substitute the RV1106 or larger-memory Pico Pro/Max specifications.
The standard Pico does not provide the Plus model's Ethernet port; this
unit's working network connection was through USB.

## Neural processing unit

The RV1103 has Rockchip's fourth-generation NPU, advertised at **up to
0.5 TOPS**, with **INT4, INT8 and INT16 mixed-precision support**. Luckfox's
specification does not give a separate throughput figure for each precision;
do not assume all three operate at 0.5 TOPS. These are vendor capabilities,
not measured results on this board. [Luckfox specifications](https://wiki.luckfox.com/Luckfox-Pico/Luckfox-Pico-RV1103/)

Luckfox places this hardware in the **RKNPU2 software ecosystem** despite
the fourth-generation hardware name. Deploy with **RKNN-Toolkit2** and a
compatible runtime/driver. The documented flow converts a trained model on
the host to an `.rknn` artifact, then executes it through the board runtime.
The guide includes C inference examples. Prefer a working RV1103 example
before assuming a generic Python deployment package supports this small
32-bit board. [Luckfox RKNN guide](https://wiki.luckfox.com/Luckfox-Pico-Plus-Mini/RKNN/)

The NPU is a separate accelerator, not an Arm instruction-set extension.
Compiling C/Rust/k for Cortex-A7 does not automatically use it. An application
must supply a supported compiled model and call the runtime; ISP and video
encoding are separate engines too. A future k interface would need model
loading, tensor conversion, inference calls, and result conversion.

Practical assessment: use this board for small vision/classification models
first. Model weights, intermediate tensors, runtime buffers, camera buffers,
and the Linux application all compete for limited memory. TOPS alone does
not predict latency, frames per second, or model fit. Check operator support,
quantization accuracy, actual allocation requirements, and driver/runtime
versions before selecting a model. Do not infer FP32/BF16 NPU support from
the CPU's FPU or from other Rockchip chips.

## Observations of this unit

The previous bring-up notes record these results from 2026-09-28:

| Item | Observed result |
| --- | --- |
| Linux architecture | `armv7l`, Cortex-A7 CPU part `0xc07` |
| CPU features | NEON, VFPv3/VFPv4 and integer division |
| OS | Buildroot 2023.02.6; Linux 5.10.160 |
| Linux-visible RAM | Approximately 33 MiB total; about 19 MiB available then |
| Root filesystem | Approximately 5.8 GiB on the prepared microSD image |
| USB interfaces | RNDIS networking and ADB; USB identity `2207:0019` |
| Network | Board `172.32.0.93`, host `172.32.0.100/24` |
| Applications | SSH/file transfer and CPU-based applications verified |
| NPU | No inference benchmark or runtime verification recorded |

64 MB is the published physical memory. The smaller Linux total is not an
independent measurement of physical capacity, and the reservation breakdown
was not inspected. Do not assume all missing memory is allocated to the NPU.
Historical available-memory values and USB interface names are not current
measurements.

Local evidence: `docs/bringup.md` in this board repository. No fresh board inspection was performed for this document.

## First NPU validation

1. Record the exact board/image, `uname -a`, `/proc/cpuinfo`, and `/proc/meminfo`.
2. Identify the installed NPU driver and runtime; inspect boot logs and
   available device nodes/libraries rather than assuming a device pathname.
3. Build a vendor RV1103 C example and run a small, known-supported model.
4. Record compiler/toolkit/runtime versions, model hash, input shape and
   quantization, output correctness, memory use, and warmed inference latency.
5. Add camera preprocessing only after inference from fixed input works.

## Sources

- [Luckfox standard Pico product specifications](https://www.luckfox.com/Luckfox-Pico/EN-Luckfox-Pico)
- [Luckfox RV1103 variants](https://wiki.luckfox.com/Luckfox-Pico/Luckfox-Pico-RV1103/)
- [Luckfox RKNN conversion and deployment](https://wiki.luckfox.com/Luckfox-Pico-Plus-Mini/RKNN/)
