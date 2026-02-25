
# Set the run directory.
set rundir . 

# Create the directories Liberate will write to.
exec mkdir -p ${rundir}/LDB
exec mkdir -p ${rundir}/LIBRARY
exec mkdir -p ${rundir}/DATASHEET

### Define temperature and default voltage ###
set_operating_condition -voltage 0.7 -temp 25

## Load information for each cell ##
source ${rundir}/configure.tcl


## Load Spice models and subckts ##
set spicefiles $rundir/MODEL/include_TT.sp

set cells {
    AND2x1 AND3x1 AND4x1 OR2x1 OR3x1 OR4x1 \
    AOI21x1 AOI22x1 AOI31x1 AOI221x1 AOI222x1 AOI311x1 \
    DFFHQNx1 DFFHx1 DHLx1 MUX2x1  \
    OAI21x1 OAI22x1 OAI31x1 OAI221x1 OAI222x1 OAI311x1 \
    BUFx1 BUFx2 BUFx3 BUFx4 BUFx5 BUFx6 BUFx7 BUFx8 BUFx10 \
    INVx1 INVx2 INVx3 INVx4 INVx5 INVx6 INVx7 INVx8 INVx10 INVx12 INVx14 INVx16 \
    NAND2x1 NAND2x2 NAND3x1 NAND3x2 NAND4x1 NOR2x1 NOR2x2 NOR3x1 NOR3x2 NOR4x1 \
    XOR2x1 XOR3x1 XNOR2x1 XNOR3x1 \
}
#set cells {DFFHx1}


foreach cell $cells {
    lappend spicefiles ${rundir}/NETLIST/${cell}.spf
}


#read_verilog ${rundir}/DFFHx1.v

read_spice -format spectre $spicefiles




char_library -auto_index -cells ${cells}             


## Set the filename.
# set filename 3nm_FF_LP_BPR
# set filename 3nm_FF_SP_noBPR
# set filename 3nm_FF_SP_BPR
# set filename 3nm_NS_SP_BPR
# set filename 3nm_NS_SP_noBPR
set filename 3nm_NS_HP 

## Save characterization database for post-processing ##
write_ldb ${rundir}/LDB/$filename.ldb

## Generate a .lib with ccs, ecsm ###
write_library -overwrite ${rundir}/LIBRARY/${filename}_nldm.lib
# write_library -overwrite -ccs  ${rundir}/LIBRARY/${filename}_ccs.lib
# write_library -overwrite -ecsm ${rundir}/LIBRARY/${filename}_ecsm.lib

## Generate ascii datatsheet ###
write_datasheet -cells $cells ${rundir}/DATASHEET/${filename}
