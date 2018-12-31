.text
.global wait_for_interrupt

wait_for_interrupt:
    push {lr}
    wfi
    pop {pc}

