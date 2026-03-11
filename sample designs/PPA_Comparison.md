# SCCAD-3nm-PDK: Sample Designs and PPA Analysis

This repository contains sample designs and evaluation results for the **SCCAD 3nm Process Design Kit (PDK)**. The primary focus is on comparing physical implementation metrics between traditional **Frontside Power Delivery Networks (F-PDN)** and advanced **Backside Power Delivery Networks (B-PDN)**.

##  Overview
As scaling continues, PDN routing congestion on the frontside becomes a critical bottleneck. This project explores the quantitative benefits of moving the PDN to the backside of the wafer, specifically targeting high-performance and large-scale logic designs.

##  PPA Comparison: Frontside vs. Backside PDN
The following table summarizes the Power, Performance, and Area (PPA) results for two distinct design targets (8GHz and 2GHz).

| Metric | Frontside (8GHz) | Backside (8GHz) | Frontside (2GHz) | Backside (2GHz) |
| :--- | :---: | :---: | :---: | :---: |
| **Target Freq. (GHz)** | 8 | 8 | 2 | 2 |
| **Footprint ($\mu m \times \mu m$)** | 53 x 53 | 53 x 53 | 357 x 357 | 357 x 357 |
| **Cell area ($\mu m^2$)** | 2,106 | 1,997 | 13,558,012 | 12,773,707 |
| **Total Wirelength (m)** | 0.12 | 0.11 | 1.77 | 1.74 |
| **Total Power (mW)** | 136 | 108 | 207 | 163 |
| **WNS (ps)** | -12 | -32 | -35 | -66 |
| **Effective Freq. (GHz)** | 7.3 | 6.4 | 1.87 | 1.8 |

###  Key Observations
* **Power Efficiency:** The Backside-Version shows a significant reduction in **Total Power (mW)** (up to 20% in the 2GHz case) due to reduced IR drop and optimized routing.
* **Area Optimization:** Implementing B-PDN allows for higher cell density and a slight reduction in total cell area by alleviating routing congestion.
* **Performance Trade-offs:** While B-PDN offers superior power and area profiles, the Worst Negative Slack (WNS) and Effective Frequency should be carefully managed during the sign-off flow.
