# SCCAD-3nm-PDK

**SCCAD-3nm-PDK** is an open-source **3 nm Process Design Kit (PDK)** developed by the **SCCAD Group** at the University of Southern California (USC).  
This PDK is intended for academic research and educational use, enabling exploration of advanced-node physical design, device–interconnect interactions, and design-technology co-optimization (DTCO).

---

## Overview

The SCCAD-3nm PDK provides a complete technology stack for digital design studies at the 3 nm node, including technology definitions, standard-cell libraries, timing models, parasitic extraction data, and design rule checking (DRC) support.

The PDK is designed to support:
- Standard-cell based digital implementation
- Physical design and place-and-route (PnR) experiments
- RC extraction and timing analysis
- Advanced-node research (e.g., DTCO, BEOL scaling, M3D, BS-PDN)

---

## Contents

The repository includes the following major components:

- **Standard Cells**
  - Logic gates, buffers, multiplexers, and sequential elements
  - Multiple drive strengths and threshold-voltage flavors
- **Technology Files**
  - Technology LEF and routing constraints
  - Interconnect technology descriptions for RC extraction
- **Timing Libraries**
  - Liberty (`.lib`) files across three PVT corners
- **Verification**
  - Design Rule Check (DRC) runsets
  - Layout-versus-Schematic (LVS) collateral
- **Extraction**
  - RC models compatible with signoff-grade extraction tools

---

## Supported Design Flow

The SCCAD-3nm PDK is compatible with a standard digital implementation flow, including:

1. Logic synthesis
2. Placement and routing
3. Parasitic extraction
4. Static timing analysis
5. Physical verification (DRC/LVS)

The PDK is primarily oriented toward **research and prototyping**, rather than manufacturing signoff.

---

## Intended Use

This PDK is intended for:
- Academic research
- Graduate-level education
- Methodology development
- Tool evaluation and benchmarking

It is **not intended for commercial tape-out or fabrication**.

---

## License

This project is released as an open-source academic PDK.  
Please refer to the `LICENSE` file for detailed terms of use.

---

## Citation

If you use SCCAD-3nm-PDK in your research, please cite the corresponding SCCAD publications.

---

## Contact

For questions or contributions, please contact the SCCAD Group at USC.
