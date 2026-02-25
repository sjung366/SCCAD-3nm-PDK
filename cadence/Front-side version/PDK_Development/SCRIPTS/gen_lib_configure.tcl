
# Units (default current: 1ma, default pulling_resistance: 1kohm)
set_units -timing 1ps -leakage_power 1pw -capacitance 1ff
# rise/fall slew
set_var slew_lower_rise 0.1
set_var slew_upper_rise 0.9
set_var slew_lower_fall 0.1
set_var slew_upper_fall 0.9

set_var measure_slew_lower_rise 0.1
set_var measure_slew_upper_rise 0.9
set_var measure_slew_lower_fall 0.1
set_var measure_slew_upper_fall 0.9

## 02.09.2026 - Revised by Sungwoo
set_var constraint_search_bound 5e-9 ;



# Set the maximum output transition time allowed
#set_var max_transition 0.32e-09
set_var min_transition 0.0025e-09
set_var max_transition 0.16e-09

set_var min_output_cap 3.6e-16
set_var max_output_cap 2.304e-14

# Set input & output voltages
set_var input_output_voltage 1

set_var alspice_option "sim_method=trap sim_gmin=1e-14 sim_step=1e-14"
#set_var alspice_option "sim_method=gear sim_gmin=1e-15 sim_step=1e-15"

#set_var ext_sample_duration 2.0e-9  ;
#set_var ext_sample_step 1.0e-14    ;  


# Define cells to be characterized
set cells {
    AND2x1 AND3x1 AND4x1 OR2x1 OR3x1 OR4x1 \
    AOI21x1 AOI22x1 AOI31x1 AOI221x1 AOI222x1 AOI311x1 \
    DFFHQNx1 DFFHx1 DHLx1 MUX2x1 DHLx1 \
    OAI21x1 OAI22x1 OAI31x1 OAI221x1 OAI222x1 OAI311x1 \
    BUFx1 BUFx2 BUFx3 BUFx4 BUFx5 BUFx6 BUFx7 BUFx8 BUFx10 \
    INVx1 INVx2 INVx3 INVx4 INVx5 INVx6 INVx7 INVx8 INVx10 INVx12 INVx14 INVx16 \
    NAND2x1 NAND2x2 NAND3x1 NAND3x2 NAND4x1 NOR2x1 NOR2x2 NOR3x1 NOR3x2 NOR4x1 \
    XOR2x1 XOR3x1 XNOR2x1 XNOR3x1 \
}
#set cells {DFFHx1}



# Templates for delay, power, constraint (units: ns, pF)
# 7nm node set
# define_template -type delay \
#        -index_1        {0.005 0.010 0.020 0.040 0.080 0.160 0.320} \
#        -index_2        {0.00072 0.00144 0.00288 0.00576 0.01152 0.02304 0.04608} \
#        delay_template_7x7_x1

# define_template -type power \
#        -index_1        {0.005 0.010 0.020 0.040 0.080 0.160 0.320} \
#        -index_2        {0.00072 0.00144 0.00288 0.00576 0.01152 0.02304 0.04608} \
#        power_template_7x7_x1

# define_template -type constraint \
#        -index_1  {0.005 0.010 0.020 0.040 0.080 0.160 0.320} \
#        -index_2  {0.005 0.010 0.020 0.040 0.080 0.160 0.320} \
#        constraint_template_7x7

# 3nm node set
define_template -type delay \
        -index_1        {0.0025 0.005 0.010 0.020 0.040 0.080 0.160} \
        -index_2        {0.00036 0.00072 0.00144 0.00288 0.00576 0.01152 0.02304} \
        delay_template_7x7_x1

define_template -type power \
        -index_1        {0.0025 0.005 0.010 0.020 0.040 0.080 0.160} \
        -index_2        {0.00036 0.00072 0.00144 0.00288 0.00576 0.01152 0.02304} \
        power_template_7x7_x1

define_template -type constraint \
        -index_1        {0.0025 0.005 0.010 0.020 0.040 0.080 0.160} \
        -index_2        {0.0025 0.005 0.010 0.020 0.040 0.080 0.160} \
        constraint_template_7x7


set inputs  {D}
set outputs {QN}
set clocks  {CK}
set asyncs  {RESET SET}

define_cell \
        -input $inputs  -output $outputs  -clock $clocks  -async $asyncs \
        -constraint  constraint_template_7x7    \
        -delay       delay_template_7x7_x1 \
        -power       power_template_7x7_x1 \
        $cells

define_arc \
  -type removal \
  -pin RESET \
  -pin_dir F \
  -related_pin CK \
  -related_pin_dir R \
  -when "(!D * !SET)" \
  -attribute { timing_type removal_rising } \
  -probe {MH:10} \
  -probe_dir F \
  DFFHx1

define_arc \
  -type removal \
  -pin SET \
  -pin_dir F \
  -related_pin CK \
  -related_pin_dir R \
  -when "(D * !RESET)" \
  -attribute { timing_type removal_rising } \
  -probe {MH:10} \
  -probe_dir F \
  DFFHx1


define_arc \
  -type recovery \
  -pin RESET \
  -pin_dir R \
  -related_pin CK \
  -related_pin_dir R \
  -when "(!D * !SET)" \
  -attribute { timing_type recovery_rising } \
  -probe {MH:10} \
  -probe_dir R \
  DFFHx1


define_arc \
  -type recovery \
  -pin SET \
  -pin_dir R \
  -related_pin CK \
  -related_pin_dir R \
  -when "(D * !RESET)" \
  -attribute { timing_type recovery_rising } \
  -probe {MH:10} \
  -probe_dir R \
  DFFHx1

