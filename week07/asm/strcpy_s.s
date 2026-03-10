.global strcpy_s

.text

# a0 = dest, a1 = src
# copy string from src to dest, return dest
strcpy_s:
    mv t0, a0           # save dest
strcpy_s_loop:
    lb t1, 0(a1)        # load byte from src
    sb t1, 0(a0)        # store byte to dest
    beqz t1, strcpy_s_done  # if null terminator, done
    addi a0, a0, 1      # advance dest
    addi a1, a1, 1      # advance src
    j strcpy_s_loop
strcpy_s_done:
    mv a0, t0           # return original dest
    ret
