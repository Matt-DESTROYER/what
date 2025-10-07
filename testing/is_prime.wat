(module
  (func (export "is_prime") (param $num i32) (result i32) (local $i i32)
    ;; $num < 2 => not prime
    local.get $num
    i32.const 2
    i32.lt_s
    if
      i32.const 0
      return
    end

    ;; $num < 4 => prime
    local.get $num
    i32.const 4
    i32.lt_s
    if
      i32.const 1
      return
    end

    ;; $num % 2 == 0 => not prime
    local.get $num
    i32.const 2
    i32.rem_s
    i32.eqz
    if
      i32.const 0
      return
    end

    ;; i = 3
    i32.const 3
    local.set $i

    ;; loop: while i*i < num
    loop $while
      ;; if divisible => not prime
      local.get $num
      local.get $i
      i32.rem_s
      i32.eqz
      if
        i32.const 0
        return
      end

      ;; i += 2
      local.get $i
      i32.const 2
      i32.add
      local.set $i

      ;; continue if i*i < num
      local.get $i
      local.get $i
      i32.mul
      local.get $num
      i32.lt_s
      br_if $while
    end

    ;; prime
    i32.const 1
    return
  )
)
