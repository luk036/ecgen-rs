❯ macchina
                                                                                
        a8888b.           Host        -  chronos@localhost                      
       d888888b.          Machine     -  FUJITSU FMVNQ8P9                       
       8P"YP"Y88          Kernel      -  5.4.207-15-0+                          
       8|o||o|88          Distro      -  FydeOS 102                             
       8'    .88          Packages    -  832 (portage), 2044 (cargo)            
       8`._.' Y8.         Terminal    -  crosh                                  
      d/      `8b.        Shell       -  bash                                   
     dP        Y8b.       Uptime      -  2d 1h 21m                              
    d8:       ::88b.      CPU         -  Intel® Core™ i5-3427U CPU @ 1.80GHz (4)
   d8"         'Y88b      Resolution  -  1920x1200, 1366x768                    
  :8P           :888      Brightness  -  62%                                    
   8a.         _a88P      CPU Load    -  6%                                     
 ._/"Yaa     .| 88P|      Memory      -  1.2 GB/3.9 GB                          
 \    YP"    `|     `.                                                          
 /     \.___.d|    .'                                                           
 `--..__)     `._.'                                                             

ecgen-rs/experiments on  master [!?] via 🦀 v1.74.0 
❯ rustc -C opt-level=3 diffset.rs

ecgen-rs/experiments on  master [!?] via 🦀 v1.74.0 
❯ time ./diffset 72 9 1
No solution is found.

real    0m0.221s
user    0m0.217s
sys     0m0.003s

ecgen-rs/experiments on  master [!?] via 🦀 v1.74.0 
❯ time ./diffset 72 10 1
36 37 41 45 52 59 66 69 71 72 

real    0m1.047s
user    0m1.025s
sys     0m0.004s

ecgen-cpp/source on  master [?] via C v13.2.0-gcc 
❯ gcc -O3 diffset2.c -o test2

ecgen-cpp/source on  master [?] via C v13.2.0-gcc 
❯ time ./test2 72 9 1
No solution is found.

real    0m0.220s
user    0m0.216s
sys     0m0.003s

ecgen-cpp/source on  master [?] via C v13.2.0-gcc 
❯ time ./test2 72 10 1
36 37 41 45 52 59 66 69 71 72 

real    0m1.011s
user    0m0.988s
sys     0m0.006s

ecgen-cpp/experiments on  master [!?] 
❯ g++ -O3 diffset.cpp -o diffset

❯ time ./diffset 72 9 1
No solution is found.

real    0m0.200s
user    0m0.195s
sys     0m0.003s

ecgen-cpp/experiments on  master [!?] 
❯ time ./diffset 72 10 1
36 37 41 45 52 59 66 69 71 72 

real    0m0.944s
user    0m0.936s
sys     0m0.003s

