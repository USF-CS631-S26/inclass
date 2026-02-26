# The .global directive allows a label to be seen by other code
.global first_s

# first_s adds 3 + 99 and returns the result
first_s:
    li t0, 3        # t0 = 3
    li t1, 99       # t1 = 99
    add a0, t0, t1  # a0 = t0 + t1 (return value goes in a0)
    ret
