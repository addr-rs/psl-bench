# PSL Bench

## Setup

```bash
git clone https://github.com/addr-rs/pslbench
cd pslbench
docker build -t pslbench .
```

## Sample result

```bash
~/bench]# docker run --rm -it pslbench
Running PyPy benchmark
2.20user 0.12system 0:02.38elapsed 97%CPU (0avgtext+0avgdata 116076maxresident)k
143392inputs+0outputs (238major+18498minor)pagefaults 0swaps
1048576

Running Rust benchmark
0.40user 0.95system 0:01.36elapsed 99%CPU (0avgtext+0avgdata 2796maxresident)k
2400inputs+0outputs (10major+133minor)pagefaults 0swaps
1048576

Running C benchmark
1.70user 0.06system 0:01.77elapsed 99%CPU (0avgtext+0avgdata 2884maxresident)k
1752inputs+0outputs (8major+266minor)pagefaults 0swaps
1048576
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
