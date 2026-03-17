.global get_bitseq_s

.text

# a0 = num, a1 = start, a2 = end
# extract bits [start:end] from num
get_bitseq_s:
    srl a0, a0, a1      # shift right by start
    sub t0, a2, a1      # end - start
    addi t0, t0, 1      # end - start + 1
    li t1, 1
    sll t1, t1, t0      # 1 << (end - start + 1)
    addi t1, t1, -1     # mask = (1 << (end - start + 1)) - 1
    and a0, a0, t1      # num & mask
    ret
