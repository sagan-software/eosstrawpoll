(module
  (type $t0 (func (param i32 i64)))
  (type $t1 (func (param i32 i32)))
  (type $t2 (func (result i32)))
  (type $t3 (func (param i32 i32) (result i32)))
  (type $t4 (func (param i32 i32 i32) (result i32)))
  (type $t5 (func (param i32)))
  (type $t6 (func (param i64)))
  (type $t7 (func (param i64 i64 i64)))
  (type $t8 (func (param i32) (result i32)))
  (import "env" "eosio_assert" (func $env.eosio_assert (type $t1)))
  (import "env" "action_data_size" (func $env.action_data_size (type $t2)))
  (import "env" "read_action_data" (func $env.read_action_data (type $t3)))
  (import "env" "memcpy" (func $env.memcpy (type $t4)))
  (import "env" "prints" (func $env.prints (type $t5)))
  (import "env" "printn" (func $env.printn (type $t6)))
  (func $apply (type $t7) (param $p0 i64) (param $p1 i64) (param $p2 i64)
    (local $l0 i32) (local $l1 i64)
    get_global $g0
    i32.const 32
    i32.sub
    tee_local $l0
    set_global $g0
    i64.const 7
    set_local $l1
    loop $L0
      get_local $l1
      i64.const 1
      i64.add
      tee_local $l1
      i64.const 13
      i64.ne
      br_if $L0
    end
    get_local $p2
    i64.const -6569208335818555392
    i64.eq
    if $I1
      i64.const 5
      set_local $l1
      loop $L2
        get_local $l1
        i64.const 1
        i64.add
        tee_local $l1
        i64.const 13
        i64.ne
        br_if $L2
      end
      get_local $p1
      i64.const 6138663577826885632
      i64.eq
      i32.const 8192
      call $env.eosio_assert
    end
    block $B3
      get_local $p1
      get_local $p0
      i64.ne
      if $I4
        i64.const 7
        set_local $l1
        loop $L5
          get_local $l1
          i64.const 1
          i64.add
          tee_local $l1
          i64.const 13
          i64.ne
          br_if $L5
        end
        get_local $p2
        i64.const -6569208335818555392
        i64.ne
        br_if $B3
      end
      get_local $l0
      get_local $p0
      i64.store offset=24
      get_local $p2
      i64.const 7746191359077253120
      i64.ne
      br_if $B3
      get_local $l0
      i32.const 0
      i32.store offset=20
      get_local $l0
      i32.const 1
      i32.store offset=16
      get_local $l0
      get_local $l0
      i64.load offset=16
      i64.store offset=8
      get_local $l0
      i32.const 24
      i32.add
      get_local $l0
      i32.const 8
      i32.add
      call $f8
      drop
    end
    get_local $l0
    i32.const 32
    i32.add
    set_global $g0)
  (func $f7 (type $t0) (param $p0 i32) (param $p1 i64)
    i32.const 8256
    call $env.prints
    get_local $p1
    call $env.printn)
  (func $f8 (type $t3) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i64)
    get_global $g0
    i32.const 16
    i32.sub
    tee_local $l0
    set_local $l1
    get_local $l0
    set_global $g0
    get_local $p1
    i32.load offset=4
    set_local $l3
    get_local $p1
    i32.load
    set_local $l2
    block $B0
      block $B1
        block $B2
          call $env.action_data_size
          tee_local $p1
          if $I3
            get_local $p1
            i32.const 513
            i32.lt_u
            br_if $B2
            i32.const 8280
            get_local $p1
            call $f9
            set_local $l0
            br $B1
          end
          i32.const 0
          set_local $l0
          br $B0
        end
        get_local $l0
        get_local $p1
        i32.const 15
        i32.add
        i32.const -16
        i32.and
        i32.sub
        tee_local $l0
        set_global $g0
      end
      get_local $l0
      get_local $p1
      call $env.read_action_data
      drop
    end
    get_local $l1
    i64.const 0
    i64.store offset=8
    get_local $p1
    i32.const 7
    i32.gt_u
    i32.const 8264
    call $env.eosio_assert
    get_local $l1
    i32.const 8
    i32.add
    get_local $l0
    i32.const 8
    call $env.memcpy
    drop
    get_local $l1
    i64.load offset=8
    set_local $l4
    get_local $p1
    i32.const 513
    i32.ge_u
    if $I4
      get_local $l0
      call $f11
    end
    get_local $p0
    get_local $l3
    i32.const 1
    i32.shr_s
    i32.add
    set_local $p1
    get_local $l3
    i32.const 1
    i32.and
    if $I5
      get_local $p1
      i32.load
      get_local $l2
      i32.add
      i32.load
      set_local $l2
    end
    get_local $p1
    get_local $l4
    get_local $l2
    call_indirect (type $t0)
    get_local $l1
    i32.const 16
    i32.add
    set_global $g0
    i32.const 1)
  (func $f9 (type $t3) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32)
    get_local $p1
    if $I0
      get_local $p0
      i32.load offset=8384
      tee_local $l0
      i32.eqz
      if $I1
        i32.const 16
        set_local $l0
        get_local $p0
        i32.const 8384
        i32.add
        i32.const 16
        i32.store
      end
      get_local $p1
      i32.const 8
      i32.add
      get_local $p1
      i32.const 4
      i32.add
      i32.const 7
      i32.and
      tee_local $l1
      i32.sub
      get_local $p1
      get_local $l1
      select
      set_local $l1
      block $B2
        block $B3
          get_local $p0
          i32.load offset=8388
          tee_local $l2
          get_local $l0
          i32.lt_u
          if $I4
            get_local $p0
            get_local $l2
            i32.const 12
            i32.mul
            i32.add
            i32.const -8192
            i32.sub
            set_local $p1
            block $B5
              get_local $l2
              br_if $B5
              get_local $p0
              i32.const 8196
              i32.add
              tee_local $l0
              i32.load
              br_if $B5
              get_local $p1
              i32.const 8192
              i32.store
              get_local $l0
              get_local $p0
              i32.store
            end
            get_local $l1
            i32.const 4
            i32.add
            set_local $l2
            loop $L6
              get_local $p1
              i32.load offset=8
              tee_local $l0
              get_local $l2
              i32.add
              get_local $p1
              i32.load
              i32.le_u
              if $I7
                get_local $p1
                i32.load offset=4
                get_local $l0
                i32.add
                tee_local $l0
                get_local $l0
                i32.load
                i32.const -2147483648
                i32.and
                get_local $l1
                i32.or
                i32.store
                get_local $p1
                i32.const 8
                i32.add
                tee_local $p1
                get_local $p1
                i32.load
                get_local $l2
                i32.add
                i32.store
                get_local $l0
                get_local $l0
                i32.load
                i32.const -2147483648
                i32.or
                i32.store
                get_local $l0
                i32.const 4
                i32.add
                tee_local $p1
                br_if $B3
              end
              get_local $p0
              call $f10
              tee_local $p1
              br_if $L6
            end
          end
          i32.const 2147483644
          get_local $l1
          i32.sub
          set_local $l7
          get_local $p0
          i32.const 8392
          i32.add
          set_local $l4
          get_local $p0
          i32.const 8384
          i32.add
          set_local $l8
          get_local $p0
          i32.load offset=8392
          tee_local $l9
          set_local $l0
          loop $L8
            get_local $p0
            get_local $l0
            i32.const 12
            i32.mul
            i32.add
            tee_local $p1
            i32.const 8200
            i32.add
            i32.load
            get_local $p1
            i32.const -8192
            i32.sub
            tee_local $l10
            i32.load
            i32.eq
            i32.const 16676
            call $env.eosio_assert
            get_local $p1
            i32.const 8196
            i32.add
            i32.load
            tee_local $l11
            i32.const 4
            i32.add
            set_local $l0
            loop $L9
              get_local $l11
              get_local $l10
              i32.load
              i32.add
              set_local $l5
              get_local $l0
              i32.const -4
              i32.add
              tee_local $l3
              i32.load
              tee_local $l6
              i32.const 2147483647
              i32.and
              set_local $p1
              get_local $l6
              i32.const 0
              i32.ge_s
              if $I10
                block $B11
                  get_local $p1
                  get_local $l1
                  i32.ge_u
                  br_if $B11
                  loop $L12
                    get_local $l0
                    get_local $p1
                    i32.add
                    tee_local $l2
                    get_local $l5
                    i32.ge_u
                    br_if $B11
                    get_local $l2
                    i32.load
                    tee_local $l2
                    i32.const 0
                    i32.lt_s
                    br_if $B11
                    get_local $p1
                    get_local $l2
                    i32.const 2147483647
                    i32.and
                    i32.add
                    i32.const 4
                    i32.add
                    tee_local $p1
                    get_local $l1
                    i32.lt_u
                    br_if $L12
                  end
                end
                get_local $l3
                get_local $p1
                get_local $l1
                get_local $p1
                get_local $l1
                i32.lt_u
                select
                get_local $l6
                i32.const -2147483648
                i32.and
                i32.or
                i32.store
                get_local $p1
                get_local $l1
                i32.gt_u
                if $I13
                  get_local $l0
                  get_local $l1
                  i32.add
                  get_local $l7
                  get_local $p1
                  i32.add
                  i32.const 2147483647
                  i32.and
                  i32.store
                end
                get_local $p1
                get_local $l1
                i32.ge_u
                br_if $B2
              end
              get_local $l0
              get_local $p1
              i32.add
              i32.const 4
              i32.add
              tee_local $l0
              get_local $l5
              i32.lt_u
              br_if $L9
            end
            i32.const 0
            set_local $p1
            get_local $l4
            i32.const 0
            get_local $l4
            i32.load
            i32.const 1
            i32.add
            tee_local $l0
            get_local $l0
            get_local $l8
            i32.load
            i32.eq
            select
            tee_local $l0
            i32.store
            get_local $l0
            get_local $l9
            i32.ne
            br_if $L8
          end
        end
        get_local $p1
        return
      end
      get_local $l3
      get_local $l3
      i32.load
      i32.const -2147483648
      i32.or
      i32.store
      get_local $l0
      return
    end
    i32.const 0)
  (func $f10 (type $t8) (param $p0 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32)
    get_local $p0
    i32.load offset=8388
    set_local $l4
    block $B0
      i32.const 8272
      i32.load8_u
      if $I1
        i32.const 8276
        i32.load
        set_local $l0
        br $B0
      end
      memory.size
      set_local $l0
      i32.const 8272
      i32.const 1
      i32.store8
      i32.const 8276
      get_local $l0
      i32.const 16
      i32.shl
      tee_local $l0
      i32.store
    end
    get_local $l0
    set_local $l1
    block $B2
      block $B3
        block $B4
          get_local $l0
          i32.const 65535
          i32.add
          i32.const 16
          i32.shr_u
          tee_local $l3
          memory.size
          tee_local $l2
          i32.gt_u
          if $I5
            get_local $l3
            get_local $l2
            i32.sub
            memory.grow
            drop
            i32.const 0
            set_local $l2
            get_local $l3
            memory.size
            i32.ne
            br_if $B4
            i32.const 8276
            i32.load
            set_local $l1
          end
          i32.const 0
          set_local $l2
          i32.const 8276
          get_local $l1
          i32.store
          get_local $l0
          i32.const 0
          i32.lt_s
          br_if $B4
          get_local $p0
          get_local $l4
          i32.const 12
          i32.mul
          tee_local $l3
          i32.add
          block $B6 (result i32)
            get_local $l0
            i32.const 65535
            i32.and
            tee_local $l2
            i32.const 64512
            i32.le_u
            if $I7
              get_local $l0
              i32.const 65536
              i32.add
              get_local $l2
              i32.sub
              br $B6
            end
            get_local $l0
            i32.const 131072
            i32.add
            get_local $l0
            i32.const 131071
            i32.and
            i32.sub
          end
          tee_local $l2
          get_local $l0
          i32.sub
          set_local $l0
          i32.const 8272
          i32.load8_u
          i32.eqz
          if $I8
            memory.size
            set_local $l1
            i32.const 8272
            i32.const 1
            i32.store8
            i32.const 8276
            get_local $l1
            i32.const 16
            i32.shl
            tee_local $l1
            i32.store
          end
          i32.const -8192
          i32.sub
          set_local $l3
          get_local $l0
          i32.const 0
          i32.lt_s
          br_if $B3
          get_local $l1
          set_local $l5
          get_local $l0
          i32.const 7
          i32.add
          i32.const -8
          i32.and
          tee_local $l6
          get_local $l1
          i32.add
          i32.const 65535
          i32.add
          i32.const 16
          i32.shr_u
          tee_local $l2
          memory.size
          tee_local $l7
          i32.gt_u
          if $I9
            get_local $l2
            get_local $l7
            i32.sub
            memory.grow
            drop
            get_local $l2
            memory.size
            i32.ne
            br_if $B3
            i32.const 8276
            i32.load
            set_local $l5
          end
          i32.const 8276
          get_local $l5
          get_local $l6
          i32.add
          i32.store
          get_local $l1
          i32.const -1
          i32.eq
          br_if $B3
          get_local $p0
          get_local $l4
          i32.const 12
          i32.mul
          i32.add
          tee_local $l4
          i32.const 8196
          i32.add
          i32.load
          tee_local $l5
          get_local $l3
          i32.load
          tee_local $l2
          i32.add
          get_local $l1
          i32.eq
          br_if $B2
          get_local $l2
          get_local $l4
          i32.const 8200
          i32.add
          tee_local $l6
          i32.load
          tee_local $l4
          i32.ne
          if $I10
            get_local $l5
            get_local $l4
            i32.add
            tee_local $l5
            get_local $l5
            i32.load
            i32.const -2147483648
            i32.and
            i32.const -4
            get_local $l4
            i32.sub
            get_local $l2
            i32.add
            i32.or
            i32.store
            get_local $l6
            get_local $l3
            i32.load
            i32.store
            get_local $l5
            get_local $l5
            i32.load
            i32.const 2147483647
            i32.and
            i32.store
          end
          get_local $p0
          i32.const 8388
          i32.add
          tee_local $l3
          get_local $l3
          i32.load
          i32.const 1
          i32.add
          tee_local $l3
          i32.store
          get_local $p0
          get_local $l3
          i32.const 12
          i32.mul
          i32.add
          tee_local $p0
          i32.const -8192
          i32.sub
          tee_local $l2
          get_local $l0
          i32.store
          get_local $p0
          i32.const 8196
          i32.add
          get_local $l1
          i32.store
        end
        get_local $l2
        return
      end
      get_local $l3
      i32.load
      tee_local $l2
      get_local $p0
      get_local $l4
      i32.const 12
      i32.mul
      i32.add
      tee_local $l1
      i32.const 8200
      i32.add
      tee_local $l4
      i32.load
      tee_local $l0
      i32.ne
      if $I11
        get_local $l1
        i32.const 8196
        i32.add
        i32.load
        get_local $l0
        i32.add
        tee_local $l1
        get_local $l1
        i32.load
        i32.const -2147483648
        i32.and
        i32.const -4
        get_local $l0
        i32.sub
        get_local $l2
        i32.add
        i32.or
        i32.store
        get_local $l4
        get_local $l3
        i32.load
        i32.store
        get_local $l1
        get_local $l1
        i32.load
        i32.const 2147483647
        i32.and
        i32.store
      end
      get_local $p0
      get_local $p0
      i32.const 8388
      i32.add
      tee_local $l0
      i32.load
      i32.const 1
      i32.add
      tee_local $l1
      i32.store offset=8384
      get_local $l0
      get_local $l1
      i32.store
      i32.const 0
      return
    end
    get_local $l3
    get_local $l2
    get_local $l0
    i32.add
    i32.store
    get_local $l3)
  (func $f11 (type $t5) (param $p0 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32)
    block $B0
      block $B1
        get_local $p0
        i32.eqz
        br_if $B1
        i32.const 16664
        i32.load
        tee_local $l1
        i32.const 1
        i32.lt_s
        br_if $B1
        i32.const 16472
        set_local $l0
        get_local $l1
        i32.const 12
        i32.mul
        i32.const 16472
        i32.add
        set_local $l2
        loop $L2
          get_local $l0
          i32.const 4
          i32.add
          i32.load
          tee_local $l1
          i32.eqz
          br_if $B1
          get_local $l1
          i32.const 4
          i32.add
          get_local $p0
          i32.le_u
          if $I3
            get_local $l1
            get_local $l0
            i32.load
            i32.add
            get_local $p0
            i32.gt_u
            br_if $B0
          end
          get_local $l0
          i32.const 12
          i32.add
          tee_local $l0
          get_local $l2
          i32.lt_u
          br_if $L2
        end
      end
      return
    end
    get_local $p0
    i32.const -4
    i32.add
    tee_local $l0
    get_local $l0
    i32.load
    i32.const 2147483647
    i32.and
    i32.store)
  (table $T0 2 2 anyfunc)
  (memory $memory 1)
  (global $g0 (mut i32) (i32.const 8192))
  (global $__heap_base i32 (i32.const 16762))
  (global $__data_end i32 (i32.const 16762))
  (export "memory" (memory 0))
  (export "__heap_base" (global 1))
  (export "__data_end" (global 2))
  (export "apply" (func $apply))
  (elem (i32.const 1) $f7)
  (data (i32.const 8192) "onerror action's are only valid from the \22eosio\22 system account")
  (data (i32.const 8256) "Hello, ")
  (data (i32.const 8264) "read")
  (data (i32.const 16676) "malloc_from_freed was designed to only be called after _heap was completely allocated"))
