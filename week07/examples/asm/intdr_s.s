.global intdr_s

.text

# a0 = pointer to int
# increment the value at the pointer
intdr_s:
    lw t0, 0(a0)        # load value
    addi t0, t0, 1      # increment
    sw t0, 0(a0)        # store back
    ret
