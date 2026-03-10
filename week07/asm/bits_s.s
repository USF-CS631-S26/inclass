.global and_s
.global or_s
.global xor_s
.global not_s
.global sll_w
.global srl_w
.global sra_w

.text

# a0 = a, a1 = b
# return a & b
and_s:
    and a0, a0, a1
    ret

# a0 = a, a1 = b
# return a | b
or_s:
    or a0, a0, a1
    ret

# a0 = a, a1 = b
# return a ^ b
xor_s:
    xor a0, a0, a1
    ret

# a0 = a
# return ~a
not_s:
    not a0, a0
    ret

# a0 = a, a1 = n
# return a << n
sll_w:
    sllw a0, a0, a1
    ret

# a0 = a, a1 = n
# return a >> n (logical)
srl_w:
    srlw a0, a0, a1
    ret

# a0 = a, a1 = n
# return a >> n (arithmetic)
sra_w:
    sraw a0, a0, a1
    ret
