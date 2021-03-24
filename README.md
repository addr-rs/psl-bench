# PSL Bench

## Setup

```bash
git clone https://github.com/addr-rs/psl-bench
cd psl-bench/
docker build -t pslbench .
```

## Sample result

```bash
$ docker run --rm pslbench

Running Rust (psl) benchmark
2.42user 1.05system 0:03.49elapsed 99%CPU (0avgtext+0avgdata 2452maxresident)k
2392inputs+0outputs (8major+136minor)pagefaults 0swaps
10000000

Running Rust (nom-psl) benchmark
69.54user 3.64system 1:14.06elapsed 98%CPU (0avgtext+0avgdata 1431240maxresident)k
2320inputs+0outputs (4major+357397minor)pagefaults 0swaps
10000000

Running Rust (publicsuffix) benchmark
2.58user 0.53system 0:03.12elapsed 99%CPU (0avgtext+0avgdata 3864maxresident)k
1808inputs+0outputs (3major+578minor)pagefaults 0swaps
10000000

Running C (libpsl) benchmark
17.60user 0.83system 0:18.47elapsed 99%CPU (0avgtext+0avgdata 3020maxresident)k
2176inputs+0outputs (9major+332minor)pagefaults 0swaps
10000000
```

## Server CPU info

```bash
~/bench]# lscpu
Architecture:        x86_64
CPU op-mode(s):      32-bit, 64-bit
Byte Order:          Little Endian
CPU(s):              4
On-line CPU(s) list: 0-3
Thread(s) per core:  1
Core(s) per socket:  1
Socket(s):           4
NUMA node(s):        1
Vendor ID:           GenuineIntel
CPU family:          6
Model:               63
Model name:          Intel(R) Xeon(R) CPU E5-2680 v3 @ 2.50GHz
Stepping:            2
CPU MHz:             2499.998
BogoMIPS:            4999.99
Hypervisor vendor:   KVM
Virtualization type: full
L1d cache:           32K
L1i cache:           32K
L2 cache:            4096K
L3 cache:            16384K
NUMA node0 CPU(s):   0-3
Flags:               fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss syscall nx pdpe1gb rdtscp lm constant_tsc arch_perfmon rep_good nopl xtopology cpuid pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand hypervisor lahf_lm abm invpcid_single pti fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid xsaveopt arat
```
