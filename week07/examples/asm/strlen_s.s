.global strlen_s

.text

# a0 = pointer to string
# return length of string
strlen_s:
    li t0, 0            # length = 0
strlen_s_loop:
    lb t1, 0(a0)        # load byte
    beqz t1, strlen_s_done  # if null terminator, done
    addi t0, t0, 1      # length++
    addi a0, a0, 1      # advance pointer
    j strlen_s_loop
strlen_s_done:
    mv a0, t0           # return length
    ret
