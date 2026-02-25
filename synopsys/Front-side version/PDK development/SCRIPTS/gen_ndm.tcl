###
# IC Compiler2 Library manager 
# Name: Junsik Yoon
# Date: 05/10/2021
###

# Set the run directory
set rundir $env(PWD)

set filename 3nm_NS_BSC
# set filename 3nm_FF_all_3d
# set filename 3nm_FF_all_macro3d
# set filename 3nm_FF_noBPR_3d
# set filename 3nm_FF_noBPR
# set filename 3nm_FF_BPR
# set filename ASAP7

set techfile ${rundir}/3nm_NS_BSC.tf

# Close all the lib before running commands
close_lib -all

# Create workspace
create_workspace -flow normal -technology $techfile -scale_factor 10000 $filename
read_lef ../Library/3nm_NS_BSC/2d_lef/3nm_NS_BSC.lef

# Read LEF files
# read_lef *filename -merge_action ignore|overwrite|update|add (default)
# read_lef ${filename}_macro.lef
#read_lef [glob *.lef]

### For syntex check only
# read_lef -syntax_only ${filename}_macro.lef
# read_gds ~~

### If FRAM exists, you can load FRAM instead of LEF (but do not mix them)

# Read db files
read_db ../Library/3nm_NS_BSC/2d_db/3nm_NS_BSC_nldm.db

# Summary workspace
report_workspace

# Validate contents of the library workspace
check_workspace

# Commit workspace (ndm file is made)
commit_workspace -output ${filename}.ndm

exit

