(module
  (func (export "is_prime") (param $num i32) (result i32) (local $i i32)
    ;; $num < 2 => not prime
    (if (i32.lt_s (local.get $num) (i32.const 2))
      (then
        (return (i32.const 0))
      )
    )

    ;; $num < 4 => prime
    (if (i32.lt_s (local.get $num) (i32.const 4))
      (then
        (return (i32.const 1))
      )
    )

    ;; $num % 2 == 0 => not prime
    (if (i32.eqz (i32.rem_s (local.get $num) (i32.const 2)))
      (then
        (return (i32.const 0))
      )
    )

    ;; i = 3
    (local.set $i (i32.const 3))

    ;; loop: while i*i < num
    (loop $while
      ;; if divisible => not prime
      (if (i32.eqz (i32.rem_s (local.get $num) (local.get $i)))
        (then
          (return (i32.const 0))
        )
      )

      ;; i += 2
      (local.set $i
        (i32.add (local.get $i) (i32.const 2))
      )

      ;; continue if i*i < num
      (br_if $while
        (i32.lt_s
          (i32.mul (local.get $i) (local.get $i))
          (local.get $num)
        )
      )
    )

    ;; prime
    (return (i32.const 1))
  )
)