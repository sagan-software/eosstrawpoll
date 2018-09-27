(module
  (type $t0 (func (param i64)))
  (type $t1 (func (param i32 i32)))
  (type $t2 (func (param i32)))
  (type $t3 (func (param i64 i64 i64)))
  (import "env" "printn" (func $env.printn (type $t0)))
  (import "env" "eosio_assert" (func $env.eosio_assert (type $t1)))
  (import "env" "printui" (func $env.printui (type $t0)))
  (import "env" "prints" (func $env.prints (type $t2)))
  (func $apply (type $t3) (param $p0 i64) (param $p1 i64) (param $p2 i64)
    block $B0
      block $B1
        get_local $p2
        i64.const 4581286720942637056
        i64.ne
        if $I2
          get_local $p2
          i64.const 7746191359077253120
          i64.ne
          br_if $B1
          i64.const 7749723791755837440
          call $env.printn
          br $B0
        end
        i64.const 4581397109463842816
        call $env.printn
        br $B0
      end
      i32.const 0
      i32.const 48000
      call $env.eosio_assert
    end
    i64.const -2043764404237504544
    call $env.printn
    i64.const 1
    call $env.printui
    i64.const 0
    call $env.printui
    i32.const 48014
    call $env.prints)
  (table $__indirect_function_table 1 1 anyfunc)
  (memory $memory 1)
  (global $__heap_base i32 (i32.const 48043))
  (global $__data_end i32 (i32.const 48043))
  (export "memory" (memory 0))
  (export "__indirect_function_table" (table 0))
  (export "__heap_base" (global 0))
  (export "__data_end" (global 1))
  (export "apply" (func $apply))
  (data (i32.const 48000) "unknown action")
  (data (i32.const 48014) "hello world what is your name"))
