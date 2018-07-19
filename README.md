# PSL Bench

## Setup

```bash
git clone https://github.com/addr-rs/pslbench
cd pslbench/docker
docker build -t pslbench .
```

## Sample result

```bash
~/bench]# docker run --rm -it pslbench
Running PyPy benchmark
20.79user 0.84system 0:21.78elapsed 99%CPU (0avgtext+0avgdata 142040maxresident)k
141224inputs+0outputs (230major+24985minor)pagefaults 0swaps
10485760

Running Rust benchmark
4.16user 12.51system 0:16.75elapsed 99%CPU (0avgtext+0avgdata 2784maxresident)k
2464inputs+0outputs (10major+134minor)pagefaults 0swaps
10366973

Running C benchmark
16.99user 0.73system 0:17.79elapsed 99%CPU (0avgtext+0avgdata 2672maxresident)k
560inputs+0outputs (3major+263minor)pagefaults 0swaps
10485759
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
