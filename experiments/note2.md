❯ macchina
                                                                                
        a8888b.           Host        -  chronos@localhost                      
       d888888b.          Machine     -  Default string GML Y116                
       8P"YP"Y88          Kernel      -  5.15.108-fyde                          
       8|o||o|88          Distro      -  FydeOS 114                             
       8'    .88          Packages    -  902 (portage), 1748 (cargo)            
       8`._.' Y8.         Terminal    -  crosh                                  
      d/      `8b.        Shell       -  bash                                   
     dP        Y8b.       Uptime      -  2h 21m                                 
    d8:       ::88b.      CPU         -  Intel® Celeron® N4000 CPU @ 1.10GHz (2)
   d8"         'Y88b      Resolution  -  1366x768                               
  :8P           :888      Brightness  -  54%                                    
   8a.         _a88P      CPU Load    -  44%                                    
 ._/"Yaa     .| 88P|      Memory      -  2.7 GB/5.9 GB                          
 \    YP"    `|     `.                                                          
 /     \.___.d|    .'                                                           
 `--..__)     `._.'                                                             

ecgen-cpp/experiments on  master [!?] 
❯ time g++ -O3 -fno-exceptions diffset.cpp -o diffset

real    0m35.822s
user    0m35.276s
sys     0m0.302s


ecgen-cpp/experiments on  master [!?] took 35s 
❯ lsd -l
.rwxr-xr-x. chronos chronos  97 KB Fri Jan 12 15:56:29 2024  diffset
.rw-r--r--. chronos chronos 5.3 KB Fri Jan 12 15:53:09 2024  diffset.cpp
.rwxr-xr-x. chronos chronos  80 KB Fri Jan 12 10:56:19 2024  diffset2
.rw-r--r--. chronos chronos 5.3 KB Fri Jan 12 14:15:08 2024  diffset2.cpp

ecgen-cpp/experiments on  master [!?] 
❯ time ./diffset 91 10 1
36 38 50 57 65 68 81 85 90 91 

real    0m0.453s
user    0m0.443s
sys     0m0.006s

ecgen-cpp/experiments on  master [!?] 
❯ time ./diffset 90 10 1
No solution is found.

real    0m2.398s
user    0m2.349s
sys     0m0.017s

ecgen-cpp/experiments on  master [!?] took 2s 
❯ time ./diffset 90 11 1
38 45 50 54 58 63 73 84 85 87 90 

real    1m18.138s
user    1m17.301s
sys     0m0.264s

ecgen-cpp/experiments on  master [!?] took 1m18s 
❯ time ./diffset 89 10 1
No solution is found.

real    0m5.912s
user    0m5.746s
sys     0m0.077s

----------------------------------------------------------------------
❯ time rustc -C opt-level=3 diffset.rs

real    0m0.816s
user    0m0.697s
sys     0m0.109s

ecgen-rs/experiments on  master [✘!?] via 🦀 v1.74.0 took 25s 
❯ lsd -l
.rwxr-xr-x. chronos chronos 4.5 MB Fri Jan 12 16:01:36 2024  diffset
.rw-r--r--. chronos chronos 3.3 KB Fri Jan 12 15:40:02 2024  diffset.rs
.rwxr-xr-x. chronos chronos 4.5 MB Fri Jan 12 10:38:52 2024  diffset2
.rw-r--r--. chronos chronos 3.3 KB Fri Jan 12 14:36:56 2024  diffset2.rs
.rw-r--r--. chronos chronos 5.3 KB Fri Jan 12 09:43:18 2024  diffset3.rs

ecgen-rs/experiments on  master [✘!?] via 🦀 v1.74.0 
❯ time ./diffset 91 10 1
36 38 50 57 65 68 81 85 90 91 

real    0m0.674s
user    0m0.633s
sys     0m0.020s

ecgen-rs/experiments on  master [✘!?] via 🦀 v1.74.0 
❯ time ./diffset 90 10 1
No solution is found.

real    0m3.034s
user    0m2.981s
sys     0m0.018s

ecgen-rs/experiments on  master [✘!?] via 🦀 v1.74.0 took 3s 
❯ time ./diffset 90 11 1
38 45 50 54 58 63 73 84 85 87 90 

real    1m49.717s
user    1m48.606s
sys     0m0.313s

ecgen-rs/experiments on  master [✘!?] via 🦀 v1.74.0 took 1m49s 
❯ time ./diffset 89 10 1
No solution is found.

real    0m8.304s
user    0m8.150s
sys     0m0.024s


