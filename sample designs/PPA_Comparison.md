# SCCAD-3nm-PDK: Sample Designs and PPA Analysis

This repository contains sample designs and evaluation results for the **SCCAD 3nm Process Design Kit (PDK)**. 

##  PPA Comparison: Frontside vs. Backside PDN
The following table summarizes the Power, Performance, and Area (PPA) results for two distinct designs (ECG and OpenPiton).

| Benchmark | ECG |  | OpenPiton |  |
|---|---|---|---|---|
| PDK Version | Frontside-Version | Backside-Version | Frontside-Version | Backside-Version |
| Target Freq. (GHz) | 8 |  | 2 |  |
| Footprint (um x um) | 53 x 53 |  | 357 x 357 |  |
| Cell area (um x um) | 2106 | 1997 | 13558012 | 12773707 |
| Tot WL (m) | 0.12 | 0.11 | 1.77 | 1.74 |
| Tot P (mW) | 136 | 108 | 207 | 163 |
| WNS (ps) | -12 | -32 | -35 | -66 |
| Effective Freq. (GHz) | 7.3 | 6.4 | 1.87 | 1.8 |
