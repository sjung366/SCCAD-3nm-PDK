# SCCAD-3nm-PDK: Sample Designs and PPA Analysis

This repository contains sample designs and evaluation results for the **SCCAD 3nm Process Design Kit (PDK)**. 

##  PPA Comparison: Frontside vs. Backside PDN
The following table summarizes the Power, Performance, and Area (PPA) results for two distinct designs (ECG and OpenPiton).

| Benchmark | \- | **ECG** | \- | **OpenPiton** | 
| :--- | :--- | :---: | :---: | :---: | :---: |
| **PDK Version** |  **Frontside** | **Backside** | **Frontside** | **Backside** |
| **Target Freq. (GHz)** |  8 | 8 | 2 | 2 |
| **Footprint ($\mu m \times \mu m$)** |  53 x 53 | 53 x 53 | 357 x 357 | 357 x 357 |
| **Cell area ($\mu m^2$)** |  2,106 | 1,997 | 13,558,012 | 12,773,707 |
| **Total Wirelength (m)** |  0.12 | 0.11 | 1.77 | 1.74 |
| **Total Power (mW)** |  136 | 108 | 207 | 163 |
| **WNS (ps)** |  -12 | -32 | -35 | -66 |
| **Effective Freq. (GHz)** |  7.3 | 6.4 | 1.87 | 1.8 |
