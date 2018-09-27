(module
  (type $t0 (func (param i32 i64 i32)))
  (type $t1 (func (param i32 i64 i64 i64)))
  (type $t2 (func (param i32 i64 i64)))
  (type $t3 (func (param i32 i64 i32 i32)))
  (type $t4 (func (param i32 i64 i64 i32 i32)))
  (type $t5 (func (param i32 i32 i32)))
  (type $t6 (func))
  (type $t7 (func (param i64)))
  (type $t8 (func (param i32 i32)))
  (type $t9 (func (param i64 i64 i64 i64) (result i32)))
  (type $t10 (func (result i64)))
  (type $t11 (func (param i64 i64 i64 i64 i32 i32) (result i32)))
  (type $t12 (func (param i32 i32 i32) (result i32)))
  (type $t13 (func (param i64) (result i32)))
  (type $t14 (func (param i32 i32) (result i32)))
  (type $t15 (func (result i32)))
  (type $t16 (func (param i32)))
  (type $t17 (func (param i32) (result i32)))
  (type $t18 (func (param i32 i32 i32 i32)))
  (type $t19 (func (param i32 i64 i32 i64)))
  (type $t20 (func (param i64 i64 i32 i32)))
  (type $t21 (func (param i32 i64 i32) (result i32)))
  (type $t22 (func (param i64 i64 i64)))
  (import "env" "require_auth" (func $env.require_auth (type $t7)))
  (import "env" "eosio_assert" (func $env.eosio_assert (type $t8)))
  (import "env" "db_find_i64" (func $env.db_find_i64 (type $t9)))
  (import "env" "current_receiver" (func $env.current_receiver (type $t10)))
  (import "env" "db_store_i64" (func $env.db_store_i64 (type $t11)))
  (import "env" "db_update_i64" (func $env.db_update_i64 (type $t3)))
  (import "env" "memcpy" (func $env.memcpy (type $t12)))
  (import "env" "is_account" (func $env.is_account (type $t13)))
  (import "env" "require_recipient" (func $env.require_recipient (type $t7)))
  (import "env" "has_auth" (func $env.has_auth (type $t13)))
  (import "env" "db_next_i64" (func $env.db_next_i64 (type $t14)))
  (import "env" "action_data_size" (func $env.action_data_size (type $t15)))
  (import "env" "read_action_data" (func $env.read_action_data (type $t14)))
  (import "env" "db_get_i64" (func $env.db_get_i64 (type $t12)))
  (import "env" "send_inline" (func $env.send_inline (type $t8)))
  (import "env" "db_remove_i64" (func $env.db_remove_i64 (type $t16)))
  (import "env" "abort" (func $env.abort (type $t6)))
  (func $f17 (type $t0) (param $p0 i32) (param $p1 i64) (param $p2 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i64) (local $l5 i64) (local $l6 i64) (local $l7 i64) (local $l8 i64)
    get_global $g0
    i32.const 128
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $p0
    i64.load
    call $env.require_auth
    get_local $p2
    i64.load offset=8
    tee_local $l7
    i64.const 8
    i64.shr_u
    tee_local $l6
    set_local $l4
    block $B0
      block $B1
        loop $L2
          get_local $l4
          i32.wrap/i64
          i32.const 24
          i32.shl
          i32.const -1073741825
          i32.add
          i32.const 452984830
          i32.gt_u
          br_if $B1
          get_local $l4
          i64.const 8
          i64.shr_u
          set_local $l5
          get_local $l4
          i64.const 65280
          i64.and
          i64.const 0
          i64.ne
          if $I3
            get_local $l5
            set_local $l4
            i32.const 1
            set_local $l2
            get_local $l1
            tee_local $l3
            i32.const 1
            i32.add
            set_local $l1
            get_local $l3
            i32.const 6
            i32.lt_s
            br_if $L2
            br $B0
          end
          get_local $l5
          set_local $l4
          loop $L4
            get_local $l4
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            br_if $B1
            get_local $l4
            i64.const 8
            i64.shr_u
            set_local $l4
            get_local $l1
            i32.const 6
            i32.lt_s
            get_local $l1
            i32.const 1
            i32.add
            tee_local $l3
            set_local $l1
            br_if $L4
          end
          i32.const 1
          set_local $l2
          get_local $l3
          i32.const 1
          i32.add
          set_local $l1
          get_local $l3
          i32.const 6
          i32.lt_s
          br_if $L2
        end
        br $B0
      end
      i32.const 0
      set_local $l2
    end
    get_local $l2
    i32.const 8192
    call $env.eosio_assert
    i32.const 0
    set_local $l2
    block $B5
      get_local $p2
      i64.load
      tee_local $l8
      i64.const 4611686018427387903
      i64.add
      i64.const 9223372036854775806
      i64.gt_u
      br_if $B5
      i32.const 0
      set_local $l1
      get_local $l6
      set_local $l4
      block $B6
        loop $L7
          get_local $l4
          i32.wrap/i64
          i32.const 24
          i32.shl
          i32.const -1073741825
          i32.add
          i32.const 452984830
          i32.gt_u
          br_if $B6
          get_local $l4
          i64.const 8
          i64.shr_u
          set_local $l5
          get_local $l4
          i64.const 65280
          i64.and
          i64.const 0
          i64.ne
          if $I8
            get_local $l5
            set_local $l4
            i32.const 1
            set_local $l2
            get_local $l1
            tee_local $l3
            i32.const 1
            i32.add
            set_local $l1
            get_local $l3
            i32.const 6
            i32.lt_s
            br_if $L7
            br $B5
          end
          get_local $l5
          set_local $l4
          loop $L9
            get_local $l4
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            br_if $B6
            get_local $l4
            i64.const 8
            i64.shr_u
            set_local $l4
            get_local $l1
            i32.const 6
            i32.lt_s
            get_local $l1
            i32.const 1
            i32.add
            tee_local $l3
            set_local $l1
            br_if $L9
          end
          i32.const 1
          set_local $l2
          get_local $l3
          i32.const 1
          i32.add
          set_local $l1
          get_local $l3
          i32.const 6
          i32.lt_s
          br_if $L7
        end
        br $B5
      end
      i32.const 0
      set_local $l2
    end
    get_local $l2
    i32.const 8212
    call $env.eosio_assert
    get_local $l8
    i64.const 0
    i64.gt_s
    i32.const 8313
    call $env.eosio_assert
    get_local $l0
    i32.const 40
    i32.add
    i32.const 0
    i32.store
    get_local $l0
    i64.const -1
    i64.store offset=24
    get_local $l0
    i64.const 0
    i64.store offset=32
    get_local $l0
    get_local $p0
    i64.load
    tee_local $l4
    i64.store offset=8
    get_local $l0
    get_local $l6
    i64.store offset=16
    block $B10 (result i32)
      get_local $l4
      get_local $l6
      i64.const -4157508551318700032
      get_local $l6
      call $env.db_find_i64
      tee_local $l1
      i32.const 0
      i32.ge_s
      if $I11
        get_local $l0
        i32.const 8
        i32.add
        get_local $l1
        call $f18
        i32.load offset=40
        get_local $l0
        i32.const 8
        i32.add
        i32.eq
        i32.const 8961
        call $env.eosio_assert
        i32.const 0
        br $B10
      end
      i32.const 1
    end
    tee_local $l1
    i32.const 8341
    call $env.eosio_assert
    get_local $p0
    i64.load
    set_local $l5
    get_local $l0
    i64.load offset=8
    call $env.current_receiver
    i64.eq
    i32.const 9089
    call $env.eosio_assert
    i32.const 56
    call $_Znwj
    tee_local $l1
    call $f19
    drop
    get_local $l1
    get_local $l0
    i32.const 8
    i32.add
    i32.store offset=40
    get_local $l1
    get_local $l7
    i64.store offset=8
    get_local $l1
    get_local $p1
    i64.store offset=32
    get_local $l1
    get_local $p2
    i64.load
    i64.store offset=16
    get_local $l1
    i32.const 24
    i32.add
    get_local $p2
    i32.const 8
    i32.add
    i64.load
    i64.store
    get_local $l0
    get_local $l0
    i32.const 88
    i32.add
    i32.store offset=96
    get_local $l0
    get_local $l0
    i32.const 48
    i32.add
    i32.store offset=92
    get_local $l0
    get_local $l0
    i32.const 48
    i32.add
    i32.store offset=88
    get_local $l0
    get_local $l0
    i32.const 88
    i32.add
    i32.store offset=104
    get_local $l0
    get_local $l1
    i32.const 16
    i32.add
    i32.store offset=116
    get_local $l0
    get_local $l1
    i32.store offset=112
    get_local $l0
    get_local $l1
    i32.const 32
    i32.add
    i32.store offset=120
    get_local $l0
    i32.const 112
    i32.add
    get_local $l0
    i32.const 104
    i32.add
    call $f20
    get_local $l1
    get_local $l0
    i32.const 16
    i32.add
    i64.load
    i64.const -4157508551318700032
    get_local $l5
    get_local $l1
    i64.load offset=8
    i64.const 8
    i64.shr_u
    tee_local $l4
    get_local $l0
    i32.const 48
    i32.add
    i32.const 40
    call $env.db_store_i64
    tee_local $l3
    i32.store offset=44
    get_local $l4
    get_local $l0
    i32.const 24
    i32.add
    tee_local $l2
    i64.load
    i64.ge_u
    if $I12
      get_local $l2
      get_local $l4
      i64.const 1
      i64.add
      i64.store
    end
    get_local $l0
    get_local $l1
    i32.store offset=112
    get_local $l0
    get_local $l1
    i32.const 8
    i32.add
    i64.load
    i64.const 8
    i64.shr_u
    tee_local $l4
    i64.store offset=48
    get_local $l0
    get_local $l3
    i32.store offset=88
    block $B13
      block $B14
        get_local $l0
        i32.const 36
        i32.add
        tee_local $p2
        i32.load
        tee_local $l2
        get_local $l0
        i32.const 40
        i32.add
        i32.load
        i32.lt_u
        if $I15
          get_local $l2
          get_local $l4
          i64.store offset=8
          get_local $l2
          get_local $l3
          i32.store offset=16
          get_local $l0
          i32.const 0
          i32.store offset=112
          get_local $l2
          get_local $l1
          i32.store
          get_local $p2
          get_local $l2
          i32.const 24
          i32.add
          i32.store
          get_local $l0
          i32.load offset=112
          set_local $l1
          get_local $l0
          i32.const 0
          i32.store offset=112
          get_local $l1
          br_if $B14
          br $B13
        end
        get_local $l0
        i32.const 32
        i32.add
        get_local $l0
        i32.const 112
        i32.add
        get_local $l0
        i32.const 48
        i32.add
        get_local $l0
        i32.const 88
        i32.add
        call $f21
        get_local $l0
        i32.load offset=112
        set_local $l1
        get_local $l0
        i32.const 0
        i32.store offset=112
        get_local $l1
        i32.eqz
        br_if $B13
      end
      get_local $l1
      call $_ZdlPv
    end
    get_local $l0
    i32.load offset=32
    tee_local $l3
    if $I16
      block $B17 (result i32)
        get_local $l0
        i32.const 36
        i32.add
        tee_local $p2
        i32.load
        tee_local $l1
        get_local $l3
        i32.ne
        if $I18
          loop $L19
            get_local $l1
            i32.const -24
            i32.add
            tee_local $l1
            i32.load
            set_local $l2
            get_local $l1
            i32.const 0
            i32.store
            get_local $l2
            if $I20
              get_local $l2
              call $_ZdlPv
            end
            get_local $l3
            get_local $l1
            i32.ne
            br_if $L19
          end
          get_local $l0
          i32.const 32
          i32.add
          i32.load
          br $B17
        end
        get_local $l3
      end
      get_local $p2
      get_local $l3
      i32.store
      call $_ZdlPv
    end
    get_local $l0
    i32.const 128
    i32.add
    set_global $g0)
  (func $f18 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i64)
    get_global $g0
    i32.const 48
    i32.sub
    tee_local $l3
    set_local $l1
    get_local $l3
    set_global $g0
    block $B0
      get_local $p0
      i32.load offset=24
      tee_local $l2
      get_local $p0
      i32.const 28
      i32.add
      i32.load
      tee_local $l0
      i32.eq
      br_if $B0
      block $B1
        loop $L2
          get_local $l0
          i32.const -8
          i32.add
          i32.load
          get_local $p1
          i32.eq
          br_if $B1
          get_local $l2
          get_local $l0
          i32.const -24
          i32.add
          tee_local $l0
          i32.ne
          br_if $L2
        end
        br $B0
      end
      get_local $l2
      get_local $l0
      i32.eq
      br_if $B0
      get_local $l0
      i32.const -24
      i32.add
      i32.load
      get_local $l1
      i32.const 48
      i32.add
      set_global $g0
      return
    end
    get_local $p1
    i32.const 0
    i32.const 0
    call $env.db_get_i64
    tee_local $l0
    i32.const 31
    i32.shr_u
    i32.const 1
    i32.xor
    i32.const 9012
    call $env.eosio_assert
    block $B3
      get_local $l0
      i32.const 513
      i32.ge_u
      if $I4
        get_local $l0
        call $f62
        set_local $l2
        br $B3
      end
      get_local $l3
      get_local $l0
      i32.const 15
      i32.add
      i32.const -16
      i32.and
      i32.sub
      tee_local $l2
      set_global $g0
    end
    get_local $p1
    get_local $l2
    get_local $l0
    call $env.db_get_i64
    drop
    get_local $l1
    get_local $l2
    i32.store offset=12
    get_local $l1
    get_local $l2
    i32.store offset=8
    get_local $l1
    get_local $l2
    get_local $l0
    i32.add
    i32.store offset=16
    get_local $l0
    i32.const 513
    i32.ge_u
    if $I5
      get_local $l2
      call $f65
    end
    i32.const 56
    call $_Znwj
    tee_local $l0
    call $f19
    drop
    get_local $l0
    get_local $p0
    i32.store offset=40
    get_local $l1
    get_local $l1
    i32.const 8
    i32.add
    i32.store offset=24
    get_local $l1
    get_local $l0
    i32.const 16
    i32.add
    i32.store offset=36
    get_local $l1
    get_local $l0
    i32.store offset=32
    get_local $l1
    get_local $l0
    i32.const 32
    i32.add
    i32.store offset=40
    get_local $l1
    i32.const 32
    i32.add
    get_local $l1
    i32.const 24
    i32.add
    call $f47
    get_local $l0
    get_local $p1
    i32.store offset=44
    get_local $l1
    get_local $l0
    i32.store offset=24
    get_local $l1
    get_local $l0
    i64.load offset=8
    i64.const 8
    i64.shr_u
    tee_local $l4
    i64.store offset=32
    get_local $l1
    get_local $p1
    i32.store offset=4
    block $B6
      block $B7
        get_local $p0
        i32.const 28
        i32.add
        tee_local $l3
        i32.load
        tee_local $l2
        get_local $p0
        i32.const 32
        i32.add
        i32.load
        i32.lt_u
        if $I8
          get_local $l2
          get_local $l4
          i64.store offset=8
          get_local $l2
          get_local $p1
          i32.store offset=16
          get_local $l1
          i32.const 0
          i32.store offset=24
          get_local $l2
          get_local $l0
          i32.store
          get_local $l3
          get_local $l2
          i32.const 24
          i32.add
          i32.store
          get_local $l1
          i32.load offset=24
          set_local $p1
          get_local $l1
          i32.const 0
          i32.store offset=24
          get_local $p1
          br_if $B7
          br $B6
        end
        get_local $p0
        i32.const 24
        i32.add
        get_local $l1
        i32.const 24
        i32.add
        get_local $l1
        i32.const 32
        i32.add
        get_local $l1
        i32.const 4
        i32.add
        call $f21
        get_local $l1
        i32.load offset=24
        set_local $p1
        get_local $l1
        i32.const 0
        i32.store offset=24
        get_local $p1
        i32.eqz
        br_if $B6
      end
      get_local $p1
      call $_ZdlPv
    end
    get_local $l1
    i32.const 48
    i32.add
    set_global $g0
    get_local $l0)
  (func $f19 (type $t17) (param $p0 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i64) (local $l4 i64)
    get_local $p0
    i64.const 1398362884
    i64.store offset=8
    get_local $p0
    i64.const 0
    i64.store
    i32.const 1
    i32.const 9035
    call $env.eosio_assert
    get_local $p0
    i64.load offset=8
    i64.const 8
    i64.shr_u
    set_local $l3
    block $B0
      block $B1
        loop $L2
          get_local $l3
          i32.wrap/i64
          i32.const 24
          i32.shl
          i32.const -1073741825
          i32.add
          i32.const 452984830
          i32.gt_u
          br_if $B1
          get_local $l3
          i64.const 8
          i64.shr_u
          set_local $l4
          get_local $l3
          i64.const 65280
          i64.and
          i64.const 0
          i64.ne
          if $I3
            get_local $l4
            set_local $l3
            i32.const 1
            set_local $l1
            get_local $l0
            tee_local $l2
            i32.const 1
            i32.add
            set_local $l0
            get_local $l2
            i32.const 6
            i32.lt_s
            br_if $L2
            br $B0
          end
          get_local $l4
          set_local $l3
          loop $L4
            get_local $l3
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            br_if $B1
            get_local $l3
            i64.const 8
            i64.shr_u
            set_local $l3
            get_local $l0
            i32.const 6
            i32.lt_s
            get_local $l0
            i32.const 1
            i32.add
            tee_local $l2
            set_local $l0
            br_if $L4
          end
          i32.const 1
          set_local $l1
          get_local $l2
          i32.const 1
          i32.add
          set_local $l0
          get_local $l2
          i32.const 6
          i32.lt_s
          br_if $L2
        end
        br $B0
      end
      i32.const 0
      set_local $l1
    end
    get_local $l1
    i32.const 8192
    call $env.eosio_assert
    get_local $p0
    i32.const 24
    i32.add
    tee_local $l0
    i64.const 1398362884
    i64.store
    get_local $p0
    i64.const 0
    i64.store offset=16
    i32.const 1
    i32.const 9035
    call $env.eosio_assert
    get_local $l0
    i64.load
    i64.const 8
    i64.shr_u
    set_local $l3
    i32.const 0
    set_local $l0
    block $B5
      block $B6
        loop $L7
          get_local $l3
          i32.wrap/i64
          i32.const 24
          i32.shl
          i32.const -1073741825
          i32.add
          i32.const 452984830
          i32.gt_u
          br_if $B5
          block $B8
            get_local $l3
            i64.const 8
            i64.shr_u
            set_local $l4
            get_local $l3
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            if $I9
              get_local $l4
              set_local $l3
              get_local $l0
              tee_local $l1
              i32.const 1
              i32.add
              set_local $l0
              get_local $l1
              i32.const 6
              i32.lt_s
              br_if $L7
              br $B8
            end
            get_local $l4
            set_local $l3
            loop $L10
              get_local $l3
              i64.const 65280
              i64.and
              i64.const 0
              i64.ne
              br_if $B6
              get_local $l3
              i64.const 8
              i64.shr_u
              set_local $l3
              get_local $l0
              i32.const 6
              i32.lt_s
              get_local $l0
              i32.const 1
              i32.add
              tee_local $l2
              set_local $l0
              br_if $L10
            end
            get_local $l2
            i32.const 1
            i32.add
            set_local $l0
            get_local $l2
            i32.const 6
            i32.lt_s
            br_if $L7
          end
        end
        i32.const 1
        i32.const 8192
        call $env.eosio_assert
        get_local $p0
        return
      end
      i32.const 0
      i32.const 8192
      call $env.eosio_assert
      get_local $p0
      return
    end
    i32.const 0
    i32.const 8192
    call $env.eosio_assert
    get_local $p0)
  (func $f20 (type $t8) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32)
    get_local $p0
    i32.load
    set_local $l1
    get_local $p1
    i32.load
    tee_local $l0
    i32.load offset=8
    get_local $l0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_s
    i32.const 9140
    call $env.eosio_assert
    get_local $l0
    i32.load offset=4
    get_local $l1
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    tee_local $l2
    i32.store offset=4
    get_local $l0
    i32.load offset=8
    get_local $l2
    i32.sub
    i32.const 7
    i32.gt_s
    i32.const 9140
    call $env.eosio_assert
    get_local $l0
    i32.load offset=4
    get_local $l1
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4
    get_local $p0
    i32.load offset=4
    set_local $l1
    get_local $p1
    i32.load
    tee_local $l0
    i32.load offset=8
    get_local $l0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_s
    i32.const 9140
    call $env.eosio_assert
    get_local $l0
    i32.load offset=4
    get_local $l1
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    tee_local $l2
    i32.store offset=4
    get_local $l0
    i32.load offset=8
    get_local $l2
    i32.sub
    i32.const 7
    i32.gt_s
    i32.const 9140
    call $env.eosio_assert
    get_local $l0
    i32.load offset=4
    get_local $l1
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4
    get_local $p0
    i32.load offset=8
    set_local $p0
    get_local $p1
    i32.load
    tee_local $l0
    i32.load offset=8
    get_local $l0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_s
    i32.const 9140
    call $env.eosio_assert
    get_local $l0
    i32.load offset=4
    get_local $p0
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4)
  (func $f21 (type $t18) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32)
    block $B0
      get_local $p0
      i32.load offset=4
      get_local $p0
      i32.load
      tee_local $l1
      i32.sub
      i32.const 24
      i32.div_s
      tee_local $l3
      i32.const 1
      i32.add
      tee_local $l2
      i32.const 178956971
      i32.lt_u
      if $I1
        i32.const 178956970
        set_local $l0
        block $B2
          get_local $p0
          i32.load offset=8
          get_local $l1
          i32.sub
          i32.const 24
          i32.div_s
          tee_local $l1
          i32.const 89478484
          i32.le_u
          if $I3
            get_local $l2
            get_local $l1
            i32.const 1
            i32.shl
            tee_local $l0
            get_local $l0
            get_local $l2
            i32.lt_u
            select
            tee_local $l0
            i32.eqz
            br_if $B2
          end
          get_local $l0
          i32.const 24
          i32.mul
          call $_Znwj
          set_local $l1
          br $B0
        end
        i32.const 0
        set_local $l0
        i32.const 0
        set_local $l1
        br $B0
      end
      get_local $p0
      call $f59
      unreachable
    end
    get_local $p1
    i32.load
    set_local $l2
    get_local $p1
    i32.const 0
    i32.store
    get_local $l1
    get_local $l3
    i32.const 24
    i32.mul
    tee_local $l4
    i32.add
    tee_local $p1
    get_local $l2
    i32.store
    get_local $p1
    get_local $p2
    i64.load
    i64.store offset=8
    get_local $p1
    get_local $p3
    i32.load
    i32.store offset=16
    get_local $l1
    get_local $l0
    i32.const 24
    i32.mul
    i32.add
    set_local $l3
    get_local $p1
    i32.const 24
    i32.add
    set_local $l2
    block $B4 (result i32)
      get_local $p0
      i32.const 4
      i32.add
      i32.load
      tee_local $p2
      get_local $p0
      i32.load
      tee_local $l0
      i32.ne
      if $I5
        get_local $l1
        get_local $l4
        i32.add
        i32.const -24
        i32.add
        set_local $p1
        loop $L6
          get_local $p2
          i32.const -24
          i32.add
          tee_local $l1
          i32.load
          set_local $p3
          get_local $l1
          i32.const 0
          i32.store
          get_local $p1
          get_local $p3
          i32.store
          get_local $p1
          i32.const 16
          i32.add
          get_local $p2
          i32.const -8
          i32.add
          i32.load
          i32.store
          get_local $p1
          i32.const 8
          i32.add
          get_local $p2
          i32.const -16
          i32.add
          i64.load
          i64.store
          get_local $p1
          i32.const -24
          i32.add
          set_local $p1
          get_local $l0
          get_local $l1
          tee_local $p2
          i32.ne
          br_if $L6
        end
        get_local $p1
        i32.const 24
        i32.add
        set_local $p1
        get_local $p0
        i32.const 4
        i32.add
        i32.load
        set_local $l0
        get_local $p0
        i32.load
        br $B4
      end
      get_local $l0
    end
    set_local $p2
    get_local $p0
    get_local $p1
    i32.store
    get_local $p0
    i32.const 4
    i32.add
    get_local $l2
    i32.store
    get_local $p0
    i32.const 8
    i32.add
    get_local $l3
    i32.store
    get_local $l0
    get_local $p2
    i32.ne
    if $I7
      loop $L8
        get_local $l0
        i32.const -24
        i32.add
        tee_local $l0
        i32.load
        set_local $p1
        get_local $l0
        i32.const 0
        i32.store
        get_local $p1
        if $I9
          get_local $p1
          call $_ZdlPv
        end
        get_local $p2
        get_local $l0
        i32.ne
        br_if $L8
      end
    end
    get_local $p2
    if $I10
      get_local $p2
      call $_ZdlPv
    end)
  (func $f22 (type $t3) (param $p0 i32) (param $p1 i64) (param $p2 i32) (param $p3 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i64) (local $l7 i64) (local $l8 i64) (local $l9 i64)
    get_global $g0
    i32.const 224
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $p2
    i64.load offset=8
    tee_local $l9
    i64.const 8
    i64.shr_u
    tee_local $l7
    set_local $l6
    block $B0
      block $B1
        loop $L2
          get_local $l6
          i32.wrap/i64
          i32.const 24
          i32.shl
          i32.const -1073741825
          i32.add
          i32.const 452984830
          i32.gt_u
          br_if $B1
          get_local $l6
          i64.const 8
          i64.shr_u
          set_local $l8
          get_local $l6
          i64.const 65280
          i64.and
          i64.const 0
          i64.ne
          if $I3
            get_local $l8
            set_local $l6
            i32.const 1
            set_local $l3
            get_local $l1
            tee_local $l2
            i32.const 1
            i32.add
            set_local $l1
            get_local $l2
            i32.const 6
            i32.lt_s
            br_if $L2
            br $B0
          end
          get_local $l8
          set_local $l6
          loop $L4
            get_local $l6
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            br_if $B1
            get_local $l6
            i64.const 8
            i64.shr_u
            set_local $l6
            get_local $l1
            i32.const 6
            i32.lt_s
            get_local $l1
            i32.const 1
            i32.add
            tee_local $l2
            set_local $l1
            br_if $L4
          end
          i32.const 1
          set_local $l3
          get_local $l2
          i32.const 1
          i32.add
          set_local $l1
          get_local $l2
          i32.const 6
          i32.lt_s
          br_if $L2
        end
        br $B0
      end
      i32.const 0
      set_local $l3
    end
    get_local $l3
    i32.const 8192
    call $env.eosio_assert
    block $B5 (result i32)
      get_local $p3
      i32.load8_u
      tee_local $l1
      i32.const 1
      i32.and
      i32.eqz
      if $I6
        get_local $l1
        i32.const 1
        i32.shr_u
        br $B5
      end
      get_local $p3
      i32.load offset=4
    end
    tee_local $l1
    i32.const 257
    i32.lt_u
    i32.const 8374
    call $env.eosio_assert
    i32.const 0
    set_local $l3
    get_local $l0
    i32.const 120
    i32.add
    i32.const 0
    i32.store
    get_local $l0
    i64.const -1
    i64.store offset=104
    get_local $l0
    i64.const 0
    i64.store offset=112
    get_local $l0
    get_local $p0
    i64.load
    tee_local $l6
    i64.store offset=88
    get_local $l0
    get_local $l7
    i64.store offset=96
    i32.const 0
    set_local $l2
    get_local $l6
    get_local $l7
    i64.const -4157508551318700032
    get_local $l7
    call $env.db_find_i64
    tee_local $l1
    i32.const 0
    i32.ge_s
    if $I7
      get_local $l0
      i32.const 88
      i32.add
      get_local $l1
      call $f18
      tee_local $l2
      i32.load offset=40
      get_local $l0
      i32.const 88
      i32.add
      i32.eq
      i32.const 8961
      call $env.eosio_assert
    end
    get_local $l2
    i32.const 0
    i32.ne
    i32.const 8403
    call $env.eosio_assert
    get_local $l2
    i64.load offset=32
    call $env.require_auth
    get_local $l2
    i32.const 32
    i32.add
    set_local $l5
    block $B8
      get_local $p2
      i64.load
      tee_local $l6
      i64.const 4611686018427387903
      i64.add
      i64.const 9223372036854775806
      i64.gt_u
      br_if $B8
      i32.const 0
      set_local $l1
      block $B9
        loop $L10
          get_local $l7
          i32.wrap/i64
          i32.const 24
          i32.shl
          i32.const -1073741825
          i32.add
          i32.const 452984830
          i32.gt_u
          br_if $B9
          get_local $l7
          i64.const 8
          i64.shr_u
          set_local $l8
          get_local $l7
          i64.const 65280
          i64.and
          i64.const 0
          i64.ne
          if $I11
            get_local $l8
            set_local $l7
            i32.const 1
            set_local $l3
            get_local $l1
            tee_local $l4
            i32.const 1
            i32.add
            set_local $l1
            get_local $l4
            i32.const 6
            i32.lt_s
            br_if $L10
            br $B8
          end
          get_local $l8
          set_local $l7
          loop $L12
            get_local $l7
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            br_if $B9
            get_local $l7
            i64.const 8
            i64.shr_u
            set_local $l7
            get_local $l1
            i32.const 6
            i32.lt_s
            get_local $l1
            i32.const 1
            i32.add
            tee_local $l4
            set_local $l1
            br_if $L12
          end
          i32.const 1
          set_local $l3
          get_local $l4
          i32.const 1
          i32.add
          set_local $l1
          get_local $l4
          i32.const 6
          i32.lt_s
          br_if $L10
        end
        br $B8
      end
      i32.const 0
      set_local $l3
    end
    get_local $l3
    i32.const 8463
    call $env.eosio_assert
    get_local $l6
    i64.const 0
    i64.gt_s
    i32.const 8480
    call $env.eosio_assert
    get_local $l9
    get_local $l2
    i64.load offset=8
    i64.eq
    i32.const 8509
    call $env.eosio_assert
    get_local $l6
    get_local $l2
    i64.load offset=16
    get_local $l2
    i64.load
    i64.sub
    i64.le_s
    i32.const 8535
    call $env.eosio_assert
    get_local $l2
    i32.load offset=40
    get_local $l0
    i32.const 88
    i32.add
    i32.eq
    i32.const 9146
    call $env.eosio_assert
    get_local $l0
    i64.load offset=88
    call $env.current_receiver
    i64.eq
    i32.const 9192
    call $env.eosio_assert
    get_local $l9
    get_local $l2
    i64.load offset=8
    tee_local $l7
    i64.eq
    i32.const 9302
    call $env.eosio_assert
    get_local $l2
    get_local $l2
    i64.load
    get_local $l6
    i64.add
    tee_local $l6
    i64.store
    get_local $l6
    i64.const -4611686018427387904
    i64.gt_s
    i32.const 9345
    call $env.eosio_assert
    get_local $l2
    i64.load
    i64.const 4611686018427387904
    i64.lt_s
    i32.const 9364
    call $env.eosio_assert
    get_local $l7
    i64.const 8
    i64.shr_u
    tee_local $l6
    get_local $l2
    i64.load offset=8
    i64.const 8
    i64.shr_u
    i64.eq
    i32.const 9243
    call $env.eosio_assert
    get_local $l0
    get_local $l0
    i32.const 168
    i32.add
    i32.store offset=192
    get_local $l0
    get_local $l0
    i32.const 128
    i32.add
    i32.store offset=188
    get_local $l0
    get_local $l0
    i32.const 128
    i32.add
    i32.store offset=184
    get_local $l0
    get_local $l0
    i32.const 184
    i32.add
    i32.store offset=200
    get_local $l0
    get_local $l2
    i32.const 16
    i32.add
    i32.store offset=212
    get_local $l0
    get_local $l2
    i32.store offset=208
    get_local $l0
    get_local $l5
    i32.store offset=216
    get_local $l0
    i32.const 208
    i32.add
    get_local $l0
    i32.const 200
    i32.add
    call $f20
    get_local $l2
    i32.load offset=44
    i64.const 0
    get_local $l0
    i32.const 128
    i32.add
    i32.const 40
    call $env.db_update_i64
    get_local $l6
    get_local $l0
    i32.const 104
    i32.add
    tee_local $l1
    i64.load
    i64.ge_u
    if $I13
      get_local $l1
      get_local $l6
      i64.const 1
      i64.add
      i64.store
    end
    get_local $l0
    i32.const 80
    i32.add
    get_local $p2
    i32.const 8
    i32.add
    i64.load
    tee_local $l8
    i64.store
    get_local $l5
    i64.load
    set_local $l6
    get_local $p2
    i64.load
    set_local $l7
    get_local $l0
    i32.const 16
    i32.add
    get_local $l8
    i64.store
    get_local $l0
    get_local $l7
    i64.store offset=72
    get_local $l0
    get_local $l7
    i64.store offset=8
    get_local $p0
    get_local $l6
    get_local $l0
    i32.const 8
    i32.add
    get_local $l6
    call $f23
    block $B14
      get_local $l5
      i64.load
      tee_local $l7
      get_local $p1
      i64.eq
      br_if $B14
      get_local $p0
      i64.load
      i64.const 6
      set_local $l6
      loop $L15
        get_local $l6
        i64.const 1
        i64.add
        tee_local $l6
        i64.const 13
        i64.ne
        br_if $L15
      end
      get_local $l0
      i32.const 48
      i32.add
      tee_local $l3
      get_local $p2
      i32.const 8
      i32.add
      i64.load
      i64.store
      get_local $l0
      get_local $p1
      i64.store offset=32
      get_local $l0
      get_local $l7
      i64.store offset=24
      get_local $l0
      get_local $p2
      i64.load
      i64.store offset=40
      get_local $l0
      i32.const 56
      i32.add
      get_local $p3
      call $f60
      drop
      i32.const 16
      call $_Znwj
      tee_local $l1
      get_local $l7
      i64.store
      get_local $l1
      i64.const 3617214756542218240
      i64.store offset=8
      get_local $l0
      i32.const 152
      i32.add
      get_local $l3
      i64.load
      i64.store
      get_local $l0
      i32.const 168
      i32.add
      tee_local $l2
      get_local $l0
      i32.const -64
      i32.sub
      tee_local $l3
      i32.load
      i32.store
      get_local $l3
      i32.const 0
      i32.store
      get_local $l0
      get_local $l1
      i32.store offset=208
      get_local $l0
      get_local $l1
      i32.const 16
      i32.add
      tee_local $l1
      i32.store offset=216
      get_local $l0
      get_local $l1
      i32.store offset=212
      get_local $l0
      get_local $l0
      i64.load offset=24
      i64.store offset=128
      get_local $l0
      get_local $l0
      i64.load offset=32
      i64.store offset=136
      get_local $l0
      get_local $l0
      i64.load offset=40
      i64.store offset=144
      get_local $l0
      get_local $l0
      i64.load offset=56
      i64.store offset=160
      get_local $l0
      i64.const 0
      i64.store offset=56
      i64.const -3617168760277827584
      get_local $l0
      i32.const 208
      i32.add
      get_local $l0
      i32.const 128
      i32.add
      call $f24
      get_local $l0
      i32.load8_u offset=160
      i32.const 1
      i32.and
      if $I16
        get_local $l2
        i32.load
        call $_ZdlPv
      end
      get_local $l0
      i32.load offset=208
      tee_local $l1
      if $I17
        get_local $l0
        get_local $l1
        i32.store offset=212
        get_local $l1
        call $_ZdlPv
      end
      get_local $l0
      i32.const 56
      i32.add
      i32.load8_u
      i32.const 1
      i32.and
      i32.eqz
      br_if $B14
      get_local $l0
      i32.const -64
      i32.sub
      i32.load
      call $_ZdlPv
    end
    get_local $l0
    i32.load offset=112
    tee_local $l2
    if $I18
      block $B19 (result i32)
        get_local $l0
        i32.const 116
        i32.add
        tee_local $l4
        i32.load
        tee_local $l1
        get_local $l2
        i32.ne
        if $I20
          loop $L21
            get_local $l1
            i32.const -24
            i32.add
            tee_local $l1
            i32.load
            set_local $l3
            get_local $l1
            i32.const 0
            i32.store
            get_local $l3
            if $I22
              get_local $l3
              call $_ZdlPv
            end
            get_local $l2
            get_local $l1
            i32.ne
            br_if $L21
          end
          get_local $l0
          i32.const 112
          i32.add
          i32.load
          br $B19
        end
        get_local $l2
      end
      get_local $l4
      get_local $l2
      i32.store
      call $_ZdlPv
    end
    get_local $l0
    i32.const 224
    i32.add
    set_global $g0)
  (func $f23 (type $t19) (param $p0 i32) (param $p1 i64) (param $p2 i32) (param $p3 i64)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i64) (local $l7 i64)
    get_global $g0
    i32.const 80
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $l0
    i32.const 40
    i32.add
    i32.const 0
    i32.store
    get_local $l0
    i64.const -1
    i64.store offset=24
    get_local $l0
    i64.const 0
    i64.store offset=32
    get_local $l0
    get_local $p0
    i64.load
    tee_local $l7
    i64.store offset=8
    get_local $p2
    i64.load offset=8
    set_local $l6
    get_local $l0
    get_local $p1
    i64.store offset=16
    block $B0
      block $B1
        block $B2
          get_local $l7
          get_local $p1
          i64.const 3607749779137757184
          get_local $l6
          i64.const 8
          i64.shr_u
          call $env.db_find_i64
          tee_local $p0
          i32.const 0
          i32.ge_s
          if $I3
            get_local $l0
            i32.const 8
            i32.add
            get_local $p0
            call $f25
            tee_local $l1
            i32.load offset=16
            get_local $l0
            i32.const 8
            i32.add
            i32.eq
            i32.const 8961
            call $env.eosio_assert
            i32.const 1
            i32.const 9473
            call $env.eosio_assert
            get_local $l1
            i32.load offset=16
            get_local $l0
            i32.const 8
            i32.add
            i32.eq
            i32.const 9146
            call $env.eosio_assert
            get_local $l0
            i64.load offset=8
            call $env.current_receiver
            i64.eq
            i32.const 9192
            call $env.eosio_assert
            get_local $l6
            get_local $l1
            i64.load offset=8
            tee_local $p1
            i64.eq
            i32.const 9302
            call $env.eosio_assert
            get_local $l1
            get_local $l1
            i64.load
            get_local $p2
            i64.load
            i64.add
            tee_local $l6
            i64.store
            get_local $l6
            i64.const -4611686018427387904
            i64.gt_s
            i32.const 9345
            call $env.eosio_assert
            get_local $l1
            i64.load
            i64.const 4611686018427387904
            i64.lt_s
            i32.const 9364
            call $env.eosio_assert
            get_local $p1
            i64.const 8
            i64.shr_u
            tee_local $p1
            get_local $l1
            i64.load offset=8
            i64.const 8
            i64.shr_u
            i64.eq
            i32.const 9243
            call $env.eosio_assert
            i32.const 1
            i32.const 9140
            call $env.eosio_assert
            get_local $l0
            i32.const -64
            i32.sub
            get_local $l1
            i32.const 8
            call $env.memcpy
            drop
            i32.const 1
            i32.const 9140
            call $env.eosio_assert
            get_local $l0
            i32.const -64
            i32.sub
            i32.const 8
            i32.or
            get_local $l1
            i32.const 8
            i32.add
            i32.const 8
            call $env.memcpy
            drop
            get_local $l1
            i32.load offset=20
            i64.const 0
            get_local $l0
            i32.const -64
            i32.sub
            i32.const 16
            call $env.db_update_i64
            get_local $p1
            get_local $l0
            i32.const 24
            i32.add
            tee_local $l1
            i64.load
            i64.lt_u
            br_if $B2
            get_local $l1
            get_local $p1
            i64.const 1
            i64.add
            i64.store
            get_local $l0
            i32.load offset=32
            tee_local $l2
            br_if $B1
            br $B0
          end
          get_local $l7
          call $env.current_receiver
          i64.eq
          i32.const 9089
          call $env.eosio_assert
          i32.const 32
          call $_Znwj
          tee_local $l3
          i64.const 1398362884
          i64.store offset=8
          get_local $l3
          i64.const 0
          i64.store
          i32.const 1
          i32.const 9035
          call $env.eosio_assert
          get_local $l3
          i32.const 8
          i32.add
          set_local $l5
          i64.const 5462355
          set_local $p1
          block $B4
            loop $L5
              i32.const 0
              set_local $l4
              get_local $p1
              i32.wrap/i64
              i32.const 24
              i32.shl
              i32.const -1073741825
              i32.add
              i32.const 452984830
              i32.gt_u
              br_if $B4
              get_local $p1
              i64.const 8
              i64.shr_u
              set_local $l6
              get_local $p1
              i64.const 65280
              i64.and
              i64.const 0
              i64.ne
              if $I6
                get_local $l6
                set_local $p1
                i32.const 1
                set_local $l4
                get_local $l1
                tee_local $p0
                i32.const 1
                i32.add
                set_local $l1
                get_local $p0
                i32.const 6
                i32.lt_s
                br_if $L5
                br $B4
              end
              get_local $l6
              set_local $p1
              loop $L7
                get_local $p1
                i64.const 65280
                i64.and
                i64.const 0
                i64.ne
                br_if $B4
                get_local $p1
                i64.const 8
                i64.shr_u
                set_local $p1
                get_local $l1
                i32.const 6
                i32.lt_s
                get_local $l1
                i32.const 1
                i32.add
                tee_local $l2
                set_local $l1
                br_if $L7
              end
              i32.const 1
              set_local $l4
              get_local $l2
              i32.const 1
              i32.add
              set_local $l1
              get_local $l2
              i32.const 6
              i32.lt_s
              br_if $L5
            end
          end
          get_local $l4
          i32.const 8192
          call $env.eosio_assert
          get_local $l3
          get_local $l0
          i32.const 8
          i32.add
          i32.store offset=16
          get_local $l3
          i32.const 8
          i32.add
          tee_local $l1
          get_local $p2
          i32.const 8
          i32.add
          i64.load
          i64.store
          get_local $l3
          get_local $p2
          i64.load
          i64.store
          i32.const 1
          i32.const 9140
          call $env.eosio_assert
          get_local $l0
          i32.const -64
          i32.sub
          get_local $l3
          i32.const 8
          call $env.memcpy
          drop
          i32.const 1
          i32.const 9140
          call $env.eosio_assert
          get_local $l0
          i32.const -64
          i32.sub
          i32.const 8
          i32.or
          get_local $l5
          i32.const 8
          call $env.memcpy
          drop
          get_local $l3
          get_local $l0
          i32.const 16
          i32.add
          i64.load
          i64.const 3607749779137757184
          get_local $p3
          get_local $l1
          i64.load
          i64.const 8
          i64.shr_u
          tee_local $p1
          get_local $l0
          i32.const -64
          i32.sub
          i32.const 16
          call $env.db_store_i64
          tee_local $p0
          i32.store offset=20
          get_local $p1
          get_local $l0
          i32.const 24
          i32.add
          tee_local $l2
          i64.load
          i64.ge_u
          if $I8
            get_local $l2
            get_local $p1
            i64.const 1
            i64.add
            i64.store
          end
          get_local $l0
          get_local $l3
          i32.store offset=56
          get_local $l0
          get_local $l1
          i64.load
          i64.const 8
          i64.shr_u
          tee_local $p1
          i64.store offset=64
          get_local $l0
          get_local $p0
          i32.store offset=52
          block $B9
            get_local $l0
            i32.const 36
            i32.add
            tee_local $l2
            i32.load
            tee_local $l1
            get_local $l0
            i32.const 40
            i32.add
            i32.load
            i32.lt_u
            if $I10
              get_local $l1
              get_local $p1
              i64.store offset=8
              get_local $l1
              get_local $p0
              i32.store offset=16
              get_local $l0
              i32.const 0
              i32.store offset=56
              get_local $l1
              get_local $l3
              i32.store
              get_local $l2
              get_local $l1
              i32.const 24
              i32.add
              i32.store
              get_local $l0
              i32.load offset=56
              set_local $l1
              get_local $l0
              i32.const 0
              i32.store offset=56
              get_local $l1
              br_if $B9
              br $B2
            end
            get_local $l0
            i32.const 32
            i32.add
            get_local $l0
            i32.const 56
            i32.add
            get_local $l0
            i32.const -64
            i32.sub
            get_local $l0
            i32.const 52
            i32.add
            call $f21
            get_local $l0
            i32.load offset=56
            set_local $l1
            get_local $l0
            i32.const 0
            i32.store offset=56
            get_local $l1
            i32.eqz
            br_if $B2
          end
          get_local $l1
          call $_ZdlPv
        end
        get_local $l0
        i32.load offset=32
        tee_local $l2
        i32.eqz
        br_if $B0
      end
      block $B11 (result i32)
        get_local $l0
        i32.const 36
        i32.add
        tee_local $l3
        i32.load
        tee_local $l1
        get_local $l2
        i32.ne
        if $I12
          loop $L13
            get_local $l1
            i32.const -24
            i32.add
            tee_local $l1
            i32.load
            set_local $p0
            get_local $l1
            i32.const 0
            i32.store
            get_local $p0
            if $I14
              get_local $p0
              call $_ZdlPv
            end
            get_local $l2
            get_local $l1
            i32.ne
            br_if $L13
          end
          get_local $l0
          i32.const 32
          i32.add
          i32.load
          br $B11
        end
        get_local $l2
      end
      get_local $l3
      get_local $l2
      i32.store
      call $_ZdlPv
    end
    get_local $l0
    i32.const 80
    i32.add
    set_global $g0)
  (func $f24 (type $t20) (param $p0 i64) (param $p1 i64) (param $p2 i32) (param $p3 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32)
    get_global $g0
    i32.const 96
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $l0
    i32.const 0
    i32.store offset=16
    get_local $l0
    i64.const 0
    i64.store offset=8
    block $B0
      block $B1
        get_local $p2
        i32.load offset=4
        get_local $p2
        i32.load
        i32.sub
        tee_local $l4
        i32.eqz
        br_if $B1
        get_local $l4
        i32.const 4
        i32.shr_s
        tee_local $l3
        i32.const 268435456
        i32.ge_u
        br_if $B0
        get_local $l0
        i32.const 16
        i32.add
        get_local $l4
        call $_Znwj
        tee_local $l1
        get_local $l3
        i32.const 4
        i32.shl
        i32.add
        tee_local $l3
        i32.store
        get_local $l0
        get_local $l1
        i32.store offset=8
        get_local $l0
        get_local $l1
        i32.store offset=12
        get_local $p2
        i32.const 4
        i32.add
        i32.load
        get_local $p2
        i32.load
        tee_local $l2
        i32.sub
        tee_local $p2
        i32.const 1
        i32.ge_s
        if $I2
          get_local $l1
          get_local $l2
          get_local $p2
          call $env.memcpy
          drop
          get_local $l0
          get_local $l1
          get_local $p2
          i32.add
          tee_local $l2
          i32.store offset=12
          br $B1
        end
        get_local $l1
        set_local $l2
      end
      get_local $l0
      i32.const 44
      i32.add
      get_local $l2
      i32.store
      get_local $l0
      i32.const 48
      i32.add
      get_local $l3
      i32.store
      get_local $l0
      i32.const 16
      i32.add
      i32.const 0
      i32.store
      get_local $l0
      i32.const 60
      i32.add
      i32.const 0
      i32.store
      get_local $l0
      get_local $p1
      i64.store offset=32
      get_local $l0
      get_local $p0
      i64.store offset=24
      get_local $l0
      get_local $l1
      i32.store offset=40
      get_local $l0
      i64.const 0
      i64.store offset=8
      get_local $l0
      i64.const 0
      i64.store offset=52 align=4
      get_local $p3
      i32.const 36
      i32.add
      i32.load
      get_local $p3
      i32.load8_u offset=32
      tee_local $l1
      i32.const 1
      i32.shr_u
      get_local $l1
      i32.const 1
      i32.and
      select
      tee_local $p2
      i32.const 32
      i32.add
      set_local $l1
      get_local $p2
      i64.extend_u/i32
      set_local $p0
      get_local $l0
      i32.const 52
      i32.add
      set_local $p2
      loop $L3
        get_local $l1
        i32.const 1
        i32.add
        set_local $l1
        get_local $p0
        i64.const 7
        i64.shr_u
        tee_local $p0
        i64.const 0
        i64.ne
        br_if $L3
      end
      block $B4 (result i32)
        get_local $l1
        if $I5
          get_local $p2
          get_local $l1
          call $f48
          get_local $l0
          i32.const 52
          i32.add
          i32.load
          set_local $l1
          get_local $l0
          i32.const 56
          i32.add
          i32.load
          br $B4
        end
        i32.const 0
        set_local $l1
        i32.const 0
      end
      set_local $p2
      get_local $l0
      get_local $l1
      i32.store offset=84
      get_local $l0
      get_local $l1
      i32.store offset=80
      get_local $l0
      get_local $p2
      i32.store offset=88
      get_local $l0
      get_local $l0
      i32.const 80
      i32.add
      i32.store offset=64
      get_local $l0
      get_local $p3
      i32.store offset=72
      get_local $l0
      i32.const 72
      i32.add
      get_local $l0
      i32.const -64
      i32.sub
      call $f49
      get_local $l0
      i32.const 80
      i32.add
      get_local $l0
      i32.const 24
      i32.add
      call $f50
      get_local $l0
      i32.load offset=80
      tee_local $l1
      get_local $l0
      i32.load offset=84
      get_local $l1
      i32.sub
      call $env.send_inline
      get_local $l0
      i32.load offset=80
      tee_local $l1
      if $I6
        get_local $l0
        get_local $l1
        i32.store offset=84
        get_local $l1
        call $_ZdlPv
      end
      get_local $l0
      i32.load offset=52
      tee_local $l1
      if $I7
        get_local $l0
        i32.const 56
        i32.add
        get_local $l1
        i32.store
        get_local $l1
        call $_ZdlPv
      end
      get_local $l0
      i32.load offset=40
      tee_local $l1
      if $I8
        get_local $l0
        i32.const 44
        i32.add
        get_local $l1
        i32.store
        get_local $l1
        call $_ZdlPv
      end
      get_local $l0
      i32.load offset=8
      tee_local $l1
      if $I9
        get_local $l0
        get_local $l1
        i32.store offset=12
        get_local $l1
        call $_ZdlPv
      end
      get_local $l0
      i32.const 96
      i32.add
      set_global $g0
      return
    end
    get_local $l0
    i32.const 8
    i32.add
    call $f59
    unreachable)
  (func $f25 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i64) (local $l10 i64)
    get_global $g0
    i32.const 32
    i32.sub
    tee_local $l4
    set_local $l1
    get_local $l4
    set_global $g0
    block $B0
      get_local $p0
      i32.load offset=24
      tee_local $l2
      get_local $p0
      i32.const 28
      i32.add
      i32.load
      tee_local $l0
      i32.eq
      br_if $B0
      block $B1
        loop $L2
          get_local $l0
          i32.const -8
          i32.add
          i32.load
          get_local $p1
          i32.eq
          br_if $B1
          get_local $l2
          get_local $l0
          i32.const -24
          i32.add
          tee_local $l0
          i32.ne
          br_if $L2
        end
        br $B0
      end
      get_local $l2
      get_local $l0
      i32.eq
      br_if $B0
      get_local $l0
      i32.const -24
      i32.add
      i32.load
      get_local $l1
      i32.const 32
      i32.add
      set_global $g0
      return
    end
    get_local $p1
    i32.const 0
    i32.const 0
    call $env.db_get_i64
    tee_local $l5
    i32.const 31
    i32.shr_u
    i32.const 1
    i32.xor
    i32.const 9012
    call $env.eosio_assert
    block $B3
      get_local $l5
      i32.const 512
      i32.gt_u
      if $I4
        get_local $p1
        get_local $l5
        call $f62
        tee_local $l6
        get_local $l5
        call $env.db_get_i64
        drop
        get_local $l6
        call $f65
        br $B3
      end
      get_local $l4
      get_local $l5
      i32.const 15
      i32.add
      i32.const -16
      i32.and
      i32.sub
      tee_local $l6
      set_global $g0
      get_local $p1
      get_local $l6
      get_local $l5
      call $env.db_get_i64
      drop
    end
    get_local $p0
    i32.const 24
    i32.add
    set_local $l7
    i32.const 32
    call $_Znwj
    tee_local $l3
    i64.const 1398362884
    i64.store offset=8
    get_local $l3
    i64.const 0
    i64.store
    i32.const 1
    i32.const 9035
    call $env.eosio_assert
    get_local $l3
    i32.const 8
    i32.add
    i64.const 5462355
    set_local $l9
    i32.const 0
    set_local $l0
    block $B5
      block $B6
        loop $L7
          get_local $l9
          i32.wrap/i64
          i32.const 24
          i32.shl
          i32.const -1073741825
          i32.add
          i32.const 452984830
          i32.gt_u
          br_if $B6
          get_local $l9
          i64.const 8
          i64.shr_u
          set_local $l10
          get_local $l9
          i64.const 65280
          i64.and
          i64.const 0
          i64.ne
          if $I8
            get_local $l10
            set_local $l9
            i32.const 1
            set_local $l2
            get_local $l0
            tee_local $l4
            i32.const 1
            i32.add
            set_local $l0
            get_local $l4
            i32.const 6
            i32.lt_s
            br_if $L7
            br $B5
          end
          get_local $l10
          set_local $l9
          loop $L9
            get_local $l9
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            br_if $B6
            get_local $l9
            i64.const 8
            i64.shr_u
            set_local $l9
            get_local $l0
            i32.const 6
            i32.lt_s
            get_local $l0
            i32.const 1
            i32.add
            tee_local $l4
            set_local $l0
            br_if $L9
          end
          i32.const 1
          set_local $l2
          get_local $l4
          i32.const 1
          i32.add
          set_local $l0
          get_local $l4
          i32.const 6
          i32.lt_s
          br_if $L7
        end
        br $B5
      end
      i32.const 0
      set_local $l2
    end
    get_local $l2
    i32.const 8192
    call $env.eosio_assert
    get_local $l3
    get_local $p0
    i32.store offset=16
    get_local $l5
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $l3
    get_local $l6
    i32.const 8
    call $env.memcpy
    drop
    get_local $l5
    i32.const -8
    i32.and
    i32.const 8
    i32.ne
    i32.const 9084
    call $env.eosio_assert
    get_local $l6
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $l3
    get_local $p1
    i32.store offset=20
    get_local $l1
    get_local $l3
    i32.store offset=24
    get_local $l1
    get_local $l3
    i32.const 8
    i32.add
    i64.load
    i64.const 8
    i64.shr_u
    tee_local $l9
    i64.store offset=16
    get_local $l1
    get_local $p1
    i32.store offset=12
    block $B10
      block $B11
        get_local $p0
        i32.const 28
        i32.add
        tee_local $l2
        i32.load
        tee_local $l0
        get_local $p0
        i32.const 32
        i32.add
        i32.load
        i32.lt_u
        if $I12
          get_local $l0
          get_local $l9
          i64.store offset=8
          get_local $l0
          get_local $p1
          i32.store offset=16
          get_local $l1
          i32.const 0
          i32.store offset=24
          get_local $l0
          get_local $l3
          i32.store
          get_local $l2
          get_local $l0
          i32.const 24
          i32.add
          i32.store
          get_local $l1
          i32.load offset=24
          set_local $l0
          get_local $l1
          i32.const 0
          i32.store offset=24
          get_local $l0
          br_if $B11
          br $B10
        end
        get_local $l7
        get_local $l1
        i32.const 24
        i32.add
        get_local $l1
        i32.const 16
        i32.add
        get_local $l1
        i32.const 12
        i32.add
        call $f21
        get_local $l1
        i32.load offset=24
        set_local $l0
        get_local $l1
        i32.const 0
        i32.store offset=24
        get_local $l0
        i32.eqz
        br_if $B10
      end
      get_local $l0
      call $_ZdlPv
    end
    get_local $l1
    i32.const 32
    i32.add
    set_global $g0
    get_local $l3)
  (func $f26 (type $t5) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i64) (local $l6 i64) (local $l7 i64) (local $l8 i64)
    get_global $g0
    i32.const 160
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $p1
    i64.load offset=8
    tee_local $l8
    i64.const 8
    i64.shr_u
    tee_local $l6
    set_local $l5
    block $B0
      block $B1
        loop $L2
          get_local $l5
          i32.wrap/i64
          i32.const 24
          i32.shl
          i32.const -1073741825
          i32.add
          i32.const 452984830
          i32.gt_u
          br_if $B1
          get_local $l5
          i64.const 8
          i64.shr_u
          set_local $l7
          get_local $l5
          i64.const 65280
          i64.and
          i64.const 0
          i64.ne
          if $I3
            get_local $l7
            set_local $l5
            i32.const 1
            set_local $l3
            get_local $l1
            tee_local $l2
            i32.const 1
            i32.add
            set_local $l1
            get_local $l2
            i32.const 6
            i32.lt_s
            br_if $L2
            br $B0
          end
          get_local $l7
          set_local $l5
          loop $L4
            get_local $l5
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            br_if $B1
            get_local $l5
            i64.const 8
            i64.shr_u
            set_local $l5
            get_local $l1
            i32.const 6
            i32.lt_s
            get_local $l1
            i32.const 1
            i32.add
            tee_local $l2
            set_local $l1
            br_if $L4
          end
          i32.const 1
          set_local $l3
          get_local $l2
          i32.const 1
          i32.add
          set_local $l1
          get_local $l2
          i32.const 6
          i32.lt_s
          br_if $L2
        end
        br $B0
      end
      i32.const 0
      set_local $l3
    end
    get_local $l3
    i32.const 8192
    call $env.eosio_assert
    block $B5 (result i32)
      get_local $p2
      i32.load8_u
      tee_local $l1
      i32.const 1
      i32.and
      i32.eqz
      if $I6
        get_local $l1
        i32.const 1
        i32.shr_u
        br $B5
      end
      get_local $p2
      i32.load offset=4
    end
    tee_local $l1
    i32.const 257
    i32.lt_u
    i32.const 8374
    call $env.eosio_assert
    i32.const 0
    set_local $l3
    get_local $l0
    i32.const 72
    i32.add
    i32.const 0
    i32.store
    get_local $l0
    i64.const -1
    i64.store offset=56
    get_local $l0
    i64.const 0
    i64.store offset=64
    get_local $l0
    get_local $p0
    i64.load
    tee_local $l5
    i64.store offset=40
    get_local $l0
    get_local $l6
    i64.store offset=48
    i32.const 0
    set_local $l2
    get_local $l5
    get_local $l6
    i64.const -4157508551318700032
    get_local $l6
    call $env.db_find_i64
    tee_local $l1
    i32.const 0
    i32.ge_s
    if $I7
      get_local $l0
      i32.const 40
      i32.add
      get_local $l1
      call $f18
      tee_local $l2
      i32.load offset=40
      get_local $l0
      i32.const 40
      i32.add
      i32.eq
      i32.const 8961
      call $env.eosio_assert
    end
    get_local $l2
    i32.const 0
    i32.ne
    i32.const 8569
    call $env.eosio_assert
    get_local $l2
    i64.load offset=32
    call $env.require_auth
    get_local $l2
    i32.const 32
    i32.add
    set_local $l4
    block $B8
      get_local $p1
      i64.load
      tee_local $l7
      i64.const 4611686018427387903
      i64.add
      i64.const 9223372036854775806
      i64.gt_u
      br_if $B8
      i32.const 0
      set_local $l1
      block $B9
        loop $L10
          get_local $l6
          i32.wrap/i64
          i32.const 24
          i32.shl
          i32.const -1073741825
          i32.add
          i32.const 452984830
          i32.gt_u
          br_if $B9
          get_local $l6
          i64.const 8
          i64.shr_u
          set_local $l5
          get_local $l6
          i64.const 65280
          i64.and
          i64.const 0
          i64.ne
          if $I11
            get_local $l5
            set_local $l6
            i32.const 1
            set_local $l3
            get_local $l1
            tee_local $p2
            i32.const 1
            i32.add
            set_local $l1
            get_local $p2
            i32.const 6
            i32.lt_s
            br_if $L10
            br $B8
          end
          get_local $l5
          set_local $l6
          loop $L12
            get_local $l6
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            br_if $B9
            get_local $l6
            i64.const 8
            i64.shr_u
            set_local $l6
            get_local $l1
            i32.const 6
            i32.lt_s
            get_local $l1
            i32.const 1
            i32.add
            tee_local $p2
            set_local $l1
            br_if $L12
          end
          i32.const 1
          set_local $l3
          get_local $p2
          i32.const 1
          i32.add
          set_local $l1
          get_local $p2
          i32.const 6
          i32.lt_s
          br_if $L10
        end
        br $B8
      end
      i32.const 0
      set_local $l3
    end
    get_local $l3
    i32.const 8463
    call $env.eosio_assert
    get_local $l7
    i64.const 0
    i64.gt_s
    i32.const 8602
    call $env.eosio_assert
    get_local $l8
    get_local $l2
    i64.load offset=8
    i64.eq
    i32.const 8509
    call $env.eosio_assert
    get_local $l2
    i32.load offset=40
    get_local $l0
    i32.const 40
    i32.add
    i32.eq
    i32.const 9146
    call $env.eosio_assert
    get_local $l0
    i64.load offset=40
    call $env.current_receiver
    i64.eq
    i32.const 9192
    call $env.eosio_assert
    get_local $l8
    get_local $l2
    i64.load offset=8
    tee_local $l5
    i64.eq
    i32.const 9382
    call $env.eosio_assert
    get_local $l2
    get_local $l2
    i64.load
    get_local $l7
    i64.sub
    tee_local $l6
    i64.store
    get_local $l6
    i64.const -4611686018427387904
    i64.gt_s
    i32.const 9430
    call $env.eosio_assert
    get_local $l2
    i64.load
    i64.const 4611686018427387904
    i64.lt_s
    i32.const 9452
    call $env.eosio_assert
    get_local $l5
    i64.const 8
    i64.shr_u
    tee_local $l5
    get_local $l2
    i64.load offset=8
    i64.const 8
    i64.shr_u
    i64.eq
    i32.const 9243
    call $env.eosio_assert
    get_local $l0
    get_local $l0
    i32.const 120
    i32.add
    i32.store offset=128
    get_local $l0
    get_local $l0
    i32.const 80
    i32.add
    i32.store offset=124
    get_local $l0
    get_local $l0
    i32.const 80
    i32.add
    i32.store offset=120
    get_local $l0
    get_local $l0
    i32.const 120
    i32.add
    i32.store offset=136
    get_local $l0
    get_local $l2
    i32.const 16
    i32.add
    i32.store offset=148
    get_local $l0
    get_local $l2
    i32.store offset=144
    get_local $l0
    get_local $l4
    i32.store offset=152
    get_local $l0
    i32.const 144
    i32.add
    get_local $l0
    i32.const 136
    i32.add
    call $f20
    get_local $l2
    i32.load offset=44
    i64.const 0
    get_local $l0
    i32.const 80
    i32.add
    i32.const 40
    call $env.db_update_i64
    get_local $l5
    get_local $l0
    i32.const 56
    i32.add
    tee_local $l1
    i64.load
    i64.ge_u
    if $I13
      get_local $l1
      get_local $l5
      i64.const 1
      i64.add
      i64.store
    end
    get_local $l0
    i32.const 32
    i32.add
    get_local $p1
    i32.const 8
    i32.add
    i64.load
    tee_local $l6
    i64.store
    get_local $l4
    i64.load
    set_local $l7
    get_local $p1
    i64.load
    set_local $l5
    get_local $l0
    i32.const 16
    i32.add
    get_local $l6
    i64.store
    get_local $l0
    get_local $l5
    i64.store offset=24
    get_local $l0
    get_local $l5
    i64.store offset=8
    get_local $p0
    get_local $l7
    get_local $l0
    i32.const 8
    i32.add
    call $f27
    get_local $l0
    i32.load offset=64
    tee_local $l2
    if $I14
      block $B15 (result i32)
        get_local $l0
        i32.const 68
        i32.add
        tee_local $p2
        i32.load
        tee_local $l1
        get_local $l2
        i32.ne
        if $I16
          loop $L17
            get_local $l1
            i32.const -24
            i32.add
            tee_local $l1
            i32.load
            set_local $l3
            get_local $l1
            i32.const 0
            i32.store
            get_local $l3
            if $I18
              get_local $l3
              call $_ZdlPv
            end
            get_local $l2
            get_local $l1
            i32.ne
            br_if $L17
          end
          get_local $l0
          i32.const -64
          i32.sub
          i32.load
          br $B15
        end
        get_local $l2
      end
      get_local $p2
      get_local $l2
      i32.store
      call $_ZdlPv
    end
    get_local $l0
    i32.const 160
    i32.add
    set_global $g0)
  (func $f27 (type $t0) (param $p0 i32) (param $p1 i64) (param $p2 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i64) (local $l4 i64) (local $l5 i64)
    get_global $g0
    i32.const -64
    i32.add
    tee_local $l0
    set_global $g0
    get_local $l0
    i32.const 40
    i32.add
    i32.const 0
    i32.store
    get_local $l0
    get_local $p1
    i64.store offset=16
    get_local $l0
    i64.const -1
    i64.store offset=24
    get_local $l0
    i64.const 0
    i64.store offset=32
    get_local $l0
    get_local $p0
    i64.load
    i64.store offset=8
    get_local $l0
    i32.const 8
    i32.add
    get_local $p2
    i64.load offset=8
    tee_local $l3
    i64.const 8
    i64.shr_u
    i32.const 8733
    call $f28
    tee_local $p0
    i64.load
    get_local $p2
    i64.load
    tee_local $l4
    i64.ge_s
    i32.const 8757
    call $env.eosio_assert
    get_local $p0
    i32.load offset=16
    get_local $l0
    i32.const 8
    i32.add
    i32.eq
    i32.const 9146
    call $env.eosio_assert
    get_local $l0
    i64.load offset=8
    call $env.current_receiver
    i64.eq
    i32.const 9192
    call $env.eosio_assert
    get_local $l3
    get_local $p0
    i64.load offset=8
    tee_local $l5
    i64.eq
    i32.const 9382
    call $env.eosio_assert
    get_local $p0
    get_local $p0
    i64.load
    get_local $l4
    i64.sub
    tee_local $l3
    i64.store
    get_local $l3
    i64.const -4611686018427387904
    i64.gt_s
    i32.const 9430
    call $env.eosio_assert
    get_local $p0
    i64.load
    i64.const 4611686018427387904
    i64.lt_s
    i32.const 9452
    call $env.eosio_assert
    get_local $l5
    i64.const 8
    i64.shr_u
    tee_local $l3
    get_local $p0
    i64.load offset=8
    i64.const 8
    i64.shr_u
    i64.eq
    i32.const 9243
    call $env.eosio_assert
    i32.const 1
    i32.const 9140
    call $env.eosio_assert
    get_local $l0
    i32.const 48
    i32.add
    get_local $p0
    i32.const 8
    call $env.memcpy
    drop
    i32.const 1
    i32.const 9140
    call $env.eosio_assert
    get_local $l0
    i32.const 48
    i32.add
    i32.const 8
    i32.or
    get_local $p0
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $p0
    i32.load offset=20
    get_local $p1
    get_local $l0
    i32.const 48
    i32.add
    i32.const 16
    call $env.db_update_i64
    get_local $l3
    get_local $l0
    i64.load offset=24
    i64.ge_u
    if $I0
      get_local $l0
      i32.const 24
      i32.add
      get_local $l3
      i64.const 1
      i64.add
      i64.store
    end
    get_local $l0
    i32.load offset=32
    tee_local $l1
    if $I1
      block $B2 (result i32)
        get_local $l0
        i32.const 36
        i32.add
        tee_local $l2
        i32.load
        tee_local $p0
        get_local $l1
        i32.ne
        if $I3
          loop $L4
            get_local $p0
            i32.const -24
            i32.add
            tee_local $p0
            i32.load
            set_local $p2
            get_local $p0
            i32.const 0
            i32.store
            get_local $p2
            if $I5
              get_local $p2
              call $_ZdlPv
            end
            get_local $l1
            get_local $p0
            i32.ne
            br_if $L4
          end
          get_local $l0
          i32.const 32
          i32.add
          i32.load
          br $B2
        end
        get_local $l1
      end
      get_local $l2
      get_local $l1
      i32.store
      call $_ZdlPv
    end
    get_local $l0
    i32.const -64
    i32.sub
    set_global $g0)
  (func $f28 (type $t21) (param $p0 i32) (param $p1 i64) (param $p2 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32)
    block $B0
      get_local $p0
      i32.load offset=24
      tee_local $l3
      get_local $p0
      i32.const 28
      i32.add
      i32.load
      tee_local $l0
      i32.eq
      br_if $B0
      block $B1
        loop $L2
          get_local $l0
          i32.const -24
          i32.add
          tee_local $l1
          i32.load
          tee_local $l2
          i64.load offset=8
          i64.const 8
          i64.shr_u
          get_local $p1
          i64.eq
          br_if $B1
          get_local $l3
          get_local $l1
          tee_local $l0
          i32.ne
          br_if $L2
        end
        br $B0
      end
      get_local $l3
      get_local $l0
      i32.eq
      br_if $B0
      get_local $l2
      i32.load offset=16
      get_local $p0
      i32.eq
      i32.const 8961
      call $env.eosio_assert
      get_local $l2
      i32.const 0
      i32.ne
      get_local $p2
      call $env.eosio_assert
      get_local $l2
      return
    end
    i32.const 0
    set_local $l1
    get_local $p0
    i64.load
    get_local $p0
    i64.load offset=8
    i64.const 3607749779137757184
    get_local $p1
    call $env.db_find_i64
    tee_local $l0
    i32.const 0
    i32.ge_s
    if $I3
      get_local $p0
      get_local $l0
      call $f25
      tee_local $l1
      i32.load offset=16
      get_local $p0
      i32.eq
      i32.const 8961
      call $env.eosio_assert
    end
    get_local $l1
    i32.const 0
    i32.ne
    get_local $p2
    call $env.eosio_assert
    get_local $l1)
  (func $f29 (type $t4) (param $p0 i32) (param $p1 i64) (param $p2 i64) (param $p3 i32) (param $p4 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i64) (local $l6 i64) (local $l7 i64) (local $l8 i64)
    get_global $g0
    i32.const 112
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $p1
    get_local $p2
    i64.ne
    i32.const 8632
    call $env.eosio_assert
    get_local $p1
    call $env.require_auth
    get_local $p2
    call $env.is_account
    i32.const 8656
    call $env.eosio_assert
    get_local $p3
    i64.load offset=8
    set_local $l7
    get_local $l0
    i32.const 104
    i32.add
    i32.const 0
    i32.store
    get_local $l0
    get_local $l7
    i64.const 8
    i64.shr_u
    tee_local $l5
    i64.store offset=80
    get_local $l0
    i64.const -1
    i64.store offset=88
    get_local $l0
    i64.const 0
    i64.store offset=96
    get_local $l0
    get_local $p0
    i64.load
    i64.store offset=72
    get_local $l0
    i32.const 72
    i32.add
    get_local $l5
    i32.const 8682
    call $f30
    set_local $l4
    get_local $p1
    call $env.require_recipient
    get_local $p2
    call $env.require_recipient
    block $B0
      get_local $p3
      i64.load
      tee_local $l8
      i64.const 4611686018427387903
      i64.add
      i64.const 9223372036854775806
      i64.gt_u
      br_if $B0
      block $B1
        loop $L2
          get_local $l5
          i32.wrap/i64
          i32.const 24
          i32.shl
          i32.const -1073741825
          i32.add
          i32.const 452984830
          i32.gt_u
          br_if $B1
          get_local $l5
          i64.const 8
          i64.shr_u
          set_local $l6
          get_local $l5
          i64.const 65280
          i64.and
          i64.const 0
          i64.ne
          if $I3
            get_local $l6
            set_local $l5
            i32.const 1
            set_local $l2
            get_local $l1
            tee_local $l3
            i32.const 1
            i32.add
            set_local $l1
            get_local $l3
            i32.const 6
            i32.lt_s
            br_if $L2
            br $B0
          end
          get_local $l6
          set_local $l5
          loop $L4
            get_local $l5
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            br_if $B1
            get_local $l5
            i64.const 8
            i64.shr_u
            set_local $l5
            get_local $l1
            i32.const 6
            i32.lt_s
            get_local $l1
            i32.const 1
            i32.add
            tee_local $l3
            set_local $l1
            br_if $L4
          end
          i32.const 1
          set_local $l2
          get_local $l3
          i32.const 1
          i32.add
          set_local $l1
          get_local $l3
          i32.const 6
          i32.lt_s
          br_if $L2
        end
        br $B0
      end
      i32.const 0
      set_local $l2
    end
    get_local $l2
    i32.const 8463
    call $env.eosio_assert
    get_local $l8
    i64.const 0
    i64.gt_s
    i32.const 8701
    call $env.eosio_assert
    get_local $l7
    get_local $l4
    i64.load offset=8
    i64.eq
    i32.const 8509
    call $env.eosio_assert
    block $B5 (result i32)
      get_local $p4
      i32.load8_u
      tee_local $l1
      i32.const 1
      i32.and
      i32.eqz
      if $I6
        get_local $l1
        i32.const 1
        i32.shr_u
        br $B5
      end
      get_local $p4
      i32.load offset=4
    end
    tee_local $l1
    i32.const 257
    i32.lt_u
    i32.const 8374
    call $env.eosio_assert
    get_local $p2
    call $env.has_auth
    set_local $l1
    get_local $l0
    i32.const -64
    i32.sub
    get_local $p3
    i32.const 8
    i32.add
    tee_local $l2
    i64.load
    tee_local $l6
    i64.store
    get_local $p3
    i64.load
    set_local $l5
    get_local $l0
    i32.const 32
    i32.add
    get_local $l6
    i64.store
    get_local $l0
    get_local $l5
    i64.store offset=24
    get_local $l0
    get_local $l5
    i64.store offset=56
    get_local $p0
    get_local $p1
    get_local $l0
    i32.const 24
    i32.add
    call $f27
    get_local $l0
    i32.const 48
    i32.add
    get_local $l2
    i64.load
    tee_local $l6
    i64.store
    get_local $p3
    i64.load
    set_local $l5
    get_local $l0
    i32.const 16
    i32.add
    get_local $l6
    i64.store
    get_local $l0
    get_local $l5
    i64.store offset=8
    get_local $l0
    get_local $l5
    i64.store offset=40
    get_local $p0
    get_local $p2
    get_local $l0
    i32.const 8
    i32.add
    get_local $p2
    get_local $p1
    get_local $l1
    select
    call $f23
    get_local $l0
    i32.load offset=96
    tee_local $l3
    if $I7
      block $B8 (result i32)
        get_local $l0
        i32.const 100
        i32.add
        tee_local $p3
        i32.load
        tee_local $l1
        get_local $l3
        i32.ne
        if $I9
          loop $L10
            get_local $l1
            i32.const -24
            i32.add
            tee_local $l1
            i32.load
            set_local $l2
            get_local $l1
            i32.const 0
            i32.store
            get_local $l2
            if $I11
              get_local $l2
              call $_ZdlPv
            end
            get_local $l3
            get_local $l1
            i32.ne
            br_if $L10
          end
          get_local $l0
          i32.const 96
          i32.add
          i32.load
          br $B8
        end
        get_local $l3
      end
      get_local $p3
      get_local $l3
      i32.store
      call $_ZdlPv
    end
    get_local $l0
    i32.const 112
    i32.add
    set_global $g0)
  (func $f30 (type $t21) (param $p0 i32) (param $p1 i64) (param $p2 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32)
    block $B0
      get_local $p0
      i32.load offset=24
      tee_local $l3
      get_local $p0
      i32.const 28
      i32.add
      i32.load
      tee_local $l0
      i32.eq
      br_if $B0
      block $B1
        loop $L2
          get_local $l0
          i32.const -24
          i32.add
          tee_local $l1
          i32.load
          tee_local $l2
          i64.load offset=8
          i64.const 8
          i64.shr_u
          get_local $p1
          i64.eq
          br_if $B1
          get_local $l3
          get_local $l1
          tee_local $l0
          i32.ne
          br_if $L2
        end
        br $B0
      end
      get_local $l3
      get_local $l0
      i32.eq
      br_if $B0
      get_local $l2
      i32.load offset=40
      get_local $p0
      i32.eq
      i32.const 8961
      call $env.eosio_assert
      get_local $l2
      i32.const 0
      i32.ne
      get_local $p2
      call $env.eosio_assert
      get_local $l2
      return
    end
    i32.const 0
    set_local $l1
    get_local $p0
    i64.load
    get_local $p0
    i64.load offset=8
    i64.const -4157508551318700032
    get_local $p1
    call $env.db_find_i64
    tee_local $l0
    i32.const 0
    i32.ge_s
    if $I3
      get_local $p0
      get_local $l0
      call $f18
      tee_local $l1
      i32.load offset=40
      get_local $p0
      i32.eq
      i32.const 8961
      call $env.eosio_assert
    end
    get_local $l1
    i32.const 0
    i32.ne
    get_local $p2
    call $env.eosio_assert
    get_local $l1)
  (func $f31 (type $t1) (param $p0 i32) (param $p1 i64) (param $p2 i64) (param $p3 i64)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i64)
    get_global $g0
    i32.const 96
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $l0
    get_local $p2
    i64.store offset=48
    get_local $p3
    call $env.require_auth
    get_local $l0
    i32.const 40
    i32.add
    i32.const 0
    i32.store
    get_local $l0
    i64.const -1
    i64.store offset=24
    get_local $l0
    i64.const 0
    i64.store offset=32
    get_local $l0
    get_local $p0
    i64.load
    tee_local $l5
    i64.store offset=8
    get_local $l0
    get_local $p1
    i64.store offset=16
    block $B0
      block $B1
        get_local $l5
        get_local $p1
        i64.const 3607749779137757184
        get_local $p2
        i64.const 8
        i64.shr_u
        call $env.db_find_i64
        tee_local $p0
        i32.const 0
        i32.ge_s
        if $I2
          get_local $l0
          i32.const 8
          i32.add
          get_local $p0
          call $f25
          i32.load offset=16
          get_local $l0
          i32.const 8
          i32.add
          i32.eq
          i32.const 8961
          call $env.eosio_assert
          get_local $l0
          i32.load offset=32
          tee_local $l2
          br_if $B1
          br $B0
        end
        get_local $l0
        get_local $l0
        i32.const 48
        i32.add
        i32.store
        get_local $l0
        get_local $p3
        i64.store offset=88
        get_local $l5
        call $env.current_receiver
        i64.eq
        i32.const 9089
        call $env.eosio_assert
        get_local $l0
        get_local $l0
        i32.store offset=68
        get_local $l0
        get_local $l0
        i32.const 8
        i32.add
        i32.store offset=64
        get_local $l0
        get_local $l0
        i32.const 88
        i32.add
        i32.store offset=72
        i32.const 32
        call $_Znwj
        tee_local $l3
        i64.const 1398362884
        i64.store offset=8
        get_local $l3
        i64.const 0
        i64.store
        i32.const 1
        i32.const 9035
        call $env.eosio_assert
        i64.const 5462355
        set_local $p1
        block $B3
          loop $L4
            i32.const 0
            set_local $l4
            get_local $p1
            i32.wrap/i64
            i32.const 24
            i32.shl
            i32.const -1073741825
            i32.add
            i32.const 452984830
            i32.gt_u
            br_if $B3
            get_local $p1
            i64.const 8
            i64.shr_u
            set_local $p2
            get_local $p1
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            if $I5
              get_local $p2
              set_local $p1
              i32.const 1
              set_local $l4
              get_local $l1
              tee_local $p0
              i32.const 1
              i32.add
              set_local $l1
              get_local $p0
              i32.const 6
              i32.lt_s
              br_if $L4
              br $B3
            end
            get_local $p2
            set_local $p1
            loop $L6
              get_local $p1
              i64.const 65280
              i64.and
              i64.const 0
              i64.ne
              br_if $B3
              get_local $p1
              i64.const 8
              i64.shr_u
              set_local $p1
              get_local $l1
              i32.const 6
              i32.lt_s
              get_local $l1
              i32.const 1
              i32.add
              tee_local $l2
              set_local $l1
              br_if $L6
            end
            i32.const 1
            set_local $l4
            get_local $l2
            i32.const 1
            i32.add
            set_local $l1
            get_local $l2
            i32.const 6
            i32.lt_s
            br_if $L4
          end
        end
        get_local $l4
        i32.const 8192
        call $env.eosio_assert
        get_local $l3
        get_local $l0
        i32.const 8
        i32.add
        i32.store offset=16
        get_local $l0
        i32.const -64
        i32.sub
        get_local $l3
        call $f32
        get_local $l0
        get_local $l3
        i32.store offset=80
        get_local $l0
        get_local $l3
        i32.const 8
        i32.add
        i64.load
        i64.const 8
        i64.shr_u
        tee_local $p1
        i64.store offset=64
        get_local $l0
        get_local $l3
        i32.load offset=20
        tee_local $p0
        i32.store offset=60
        block $B7
          block $B8
            get_local $l0
            i32.const 36
            i32.add
            tee_local $l2
            i32.load
            tee_local $l1
            get_local $l0
            i32.const 40
            i32.add
            i32.load
            i32.lt_u
            if $I9
              get_local $l1
              get_local $p1
              i64.store offset=8
              get_local $l1
              get_local $p0
              i32.store offset=16
              get_local $l0
              i32.const 0
              i32.store offset=80
              get_local $l1
              get_local $l3
              i32.store
              get_local $l2
              get_local $l1
              i32.const 24
              i32.add
              i32.store
              get_local $l0
              i32.load offset=80
              set_local $l1
              get_local $l0
              i32.const 0
              i32.store offset=80
              get_local $l1
              br_if $B8
              br $B7
            end
            get_local $l0
            i32.const 32
            i32.add
            get_local $l0
            i32.const 80
            i32.add
            get_local $l0
            i32.const -64
            i32.sub
            get_local $l0
            i32.const 60
            i32.add
            call $f21
            get_local $l0
            i32.load offset=80
            set_local $l1
            get_local $l0
            i32.const 0
            i32.store offset=80
            get_local $l1
            i32.eqz
            br_if $B7
          end
          get_local $l1
          call $_ZdlPv
        end
        get_local $l0
        i32.load offset=32
        tee_local $l2
        i32.eqz
        br_if $B0
      end
      block $B10 (result i32)
        get_local $l0
        i32.const 36
        i32.add
        tee_local $l4
        i32.load
        tee_local $l1
        get_local $l2
        i32.ne
        if $I11
          loop $L12
            get_local $l1
            i32.const -24
            i32.add
            tee_local $l1
            i32.load
            set_local $p0
            get_local $l1
            i32.const 0
            i32.store
            get_local $p0
            if $I13
              get_local $p0
              call $_ZdlPv
            end
            get_local $l2
            get_local $l1
            i32.ne
            br_if $L12
          end
          get_local $l0
          i32.const 32
          i32.add
          i32.load
          br $B10
        end
        get_local $l2
      end
      get_local $l4
      get_local $l2
      i32.store
      call $_ZdlPv
    end
    get_local $l0
    i32.const 96
    i32.add
    set_global $g0)
  (func $f32 (type $t8) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i64) (local $l7 i64) (local $l8 i64)
    get_global $g0
    tee_local $l4
    get_local $p0
    i32.load
    set_local $l3
    get_local $p0
    i32.load offset=4
    i32.load
    i64.load
    set_local $l7
    i32.const 1
    i32.const 9035
    call $env.eosio_assert
    get_local $l7
    i64.const 8
    i64.shr_u
    set_local $l6
    block $B0
      block $B1
        loop $L2
          get_local $l6
          i32.wrap/i64
          i32.const 24
          i32.shl
          i32.const -1073741825
          i32.add
          i32.const 452984830
          i32.gt_u
          br_if $B1
          get_local $l6
          i64.const 8
          i64.shr_u
          set_local $l8
          get_local $l6
          i64.const 65280
          i64.and
          i64.const 0
          i64.ne
          if $I3
            get_local $l8
            set_local $l6
            i32.const 1
            set_local $l1
            get_local $l0
            tee_local $l2
            i32.const 1
            i32.add
            set_local $l0
            get_local $l2
            i32.const 6
            i32.lt_s
            br_if $L2
            br $B0
          end
          get_local $l8
          set_local $l6
          loop $L4
            get_local $l6
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            br_if $B1
            get_local $l6
            i64.const 8
            i64.shr_u
            set_local $l6
            get_local $l0
            i32.const 6
            i32.lt_s
            get_local $l0
            i32.const 1
            i32.add
            tee_local $l2
            set_local $l0
            br_if $L4
          end
          i32.const 1
          set_local $l1
          get_local $l2
          i32.const 1
          i32.add
          set_local $l0
          get_local $l2
          i32.const 6
          i32.lt_s
          br_if $L2
        end
        br $B0
      end
      i32.const 0
      set_local $l1
    end
    get_local $l1
    i32.const 8192
    call $env.eosio_assert
    get_local $p1
    get_local $l7
    i64.store offset=8
    get_local $p1
    i64.const 0
    i64.store
    get_local $l4
    tee_local $l1
    i32.const -16
    i32.add
    tee_local $l0
    set_global $g0
    i32.const 1
    i32.const 9140
    call $env.eosio_assert
    get_local $l0
    get_local $p1
    i32.const 8
    call $env.memcpy
    drop
    i32.const 1
    i32.const 9140
    call $env.eosio_assert
    get_local $l1
    i32.const -8
    i32.add
    get_local $p1
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $p1
    get_local $l3
    i64.load offset=8
    i64.const 3607749779137757184
    get_local $p0
    i32.load offset=8
    i64.load
    get_local $p1
    i64.load offset=8
    i64.const 8
    i64.shr_u
    tee_local $l6
    get_local $l0
    i32.const 16
    call $env.db_store_i64
    i32.store offset=20
    get_local $l6
    get_local $l3
    i64.load offset=16
    i64.ge_u
    if $I5
      get_local $l3
      i32.const 16
      i32.add
      get_local $l6
      i64.const 1
      i64.add
      i64.store
    end
    set_global $g0)
  (func $f33 (type $t2) (param $p0 i32) (param $p1 i64) (param $p2 i64)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i64)
    get_global $g0
    i32.const 48
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $p1
    call $env.require_auth
    get_local $l0
    i32.const 32
    i32.add
    i32.const 0
    i32.store
    get_local $l0
    get_local $p1
    i64.store offset=8
    get_local $l0
    i64.const -1
    i64.store offset=16
    get_local $l0
    i64.const 0
    i64.store offset=24
    get_local $l0
    get_local $p0
    i64.load
    tee_local $l4
    i64.store
    i32.const 0
    set_local $p0
    get_local $l4
    get_local $p1
    i64.const 3607749779137757184
    get_local $p2
    i64.const 8
    i64.shr_u
    call $env.db_find_i64
    tee_local $l1
    i32.const 0
    i32.ge_s
    if $I0
      get_local $l0
      get_local $l1
      call $f25
      tee_local $p0
      i32.load offset=16
      get_local $l0
      i32.eq
      i32.const 8961
      call $env.eosio_assert
    end
    get_local $p0
    i32.const 0
    i32.ne
    tee_local $l1
    i32.const 8775
    call $env.eosio_assert
    get_local $p0
    i64.load
    i64.eqz
    i32.const 8851
    call $env.eosio_assert
    get_local $l1
    i32.const 9508
    call $env.eosio_assert
    get_local $l1
    i32.const 9542
    call $env.eosio_assert
    get_local $p0
    i32.load offset=20
    get_local $l0
    i32.const 40
    i32.add
    call $env.db_next_i64
    tee_local $l1
    i32.const 0
    i32.ge_s
    if $I1
      get_local $l0
      get_local $l1
      call $f25
      drop
    end
    get_local $l0
    get_local $p0
    call $f34
    get_local $l0
    i32.load offset=24
    tee_local $l2
    if $I2
      block $B3 (result i32)
        get_local $l0
        i32.const 28
        i32.add
        tee_local $l3
        i32.load
        tee_local $p0
        get_local $l2
        i32.ne
        if $I4
          loop $L5
            get_local $p0
            i32.const -24
            i32.add
            tee_local $p0
            i32.load
            set_local $l1
            get_local $p0
            i32.const 0
            i32.store
            get_local $l1
            if $I6
              get_local $l1
              call $_ZdlPv
            end
            get_local $l2
            get_local $p0
            i32.ne
            br_if $L5
          end
          get_local $l0
          i32.const 24
          i32.add
          i32.load
          br $B3
        end
        get_local $l2
      end
      get_local $l3
      get_local $l2
      i32.store
      call $_ZdlPv
    end
    get_local $l0
    i32.const 48
    i32.add
    set_global $g0)
  (func $f34 (type $t8) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i64)
    get_local $p1
    i32.load offset=16
    get_local $p0
    i32.eq
    i32.const 9572
    call $env.eosio_assert
    get_local $p0
    i64.load
    call $env.current_receiver
    i64.eq
    i32.const 9617
    call $env.eosio_assert
    get_local $p0
    i32.load offset=24
    tee_local $l3
    set_local $l1
    block $B0
      get_local $l3
      get_local $p0
      i32.const 28
      i32.add
      tee_local $l5
      i32.load
      tee_local $l0
      i32.eq
      br_if $B0
      get_local $l0
      i32.const -24
      i32.add
      i32.load
      i64.load offset=8
      get_local $p1
      i64.load offset=8
      tee_local $l6
      i64.xor
      i64.const 256
      i64.lt_u
      if $I1
        get_local $l0
        set_local $l1
        br $B0
      end
      get_local $l3
      i32.const 24
      i32.add
      set_local $l4
      block $B2
        loop $L3
          get_local $l4
          get_local $l0
          i32.eq
          br_if $B2
          get_local $l0
          i32.const -48
          i32.add
          get_local $l0
          i32.const -24
          i32.add
          tee_local $l1
          set_local $l0
          i32.load
          i64.load offset=8
          get_local $l6
          i64.xor
          i64.const 256
          i64.ge_u
          br_if $L3
        end
        br $B0
      end
      get_local $l3
      set_local $l1
    end
    get_local $l1
    get_local $l3
    i32.ne
    i32.const 9667
    call $env.eosio_assert
    block $B4
      block $B5 (result i32)
        get_local $l1
        get_local $l5
        i32.load
        tee_local $l3
        i32.ne
        if $I6
          get_local $l1
          set_local $l0
          loop $L7
            get_local $l0
            i32.load
            set_local $l2
            get_local $l0
            i32.const 0
            i32.store
            get_local $l0
            i32.const -24
            i32.add
            tee_local $l4
            i32.load
            set_local $l1
            get_local $l4
            get_local $l2
            i32.store
            get_local $l1
            if $I8
              get_local $l1
              call $_ZdlPv
            end
            get_local $l0
            i32.const -8
            i32.add
            get_local $l0
            i32.const 16
            i32.add
            i32.load
            i32.store
            get_local $l0
            i32.const -16
            i32.add
            get_local $l0
            i32.const 8
            i32.add
            i64.load
            i64.store
            get_local $l3
            get_local $l0
            i32.const 24
            i32.add
            tee_local $l0
            i32.ne
            br_if $L7
          end
          get_local $l0
          i32.const -24
          i32.add
          tee_local $l2
          get_local $p0
          i32.const 28
          i32.add
          i32.load
          tee_local $l1
          i32.const 24
          i32.add
          get_local $l0
          i32.ne
          br_if $B5
          drop
          br $B4
        end
        get_local $l1
        i32.const -24
        i32.add
      end
      set_local $l2
      loop $L9
        get_local $l1
        i32.const -24
        i32.add
        tee_local $l1
        i32.load
        set_local $l0
        get_local $l1
        i32.const 0
        i32.store
        get_local $l0
        if $I10
          get_local $l0
          call $_ZdlPv
        end
        get_local $l2
        get_local $l1
        i32.ne
        br_if $L9
      end
    end
    get_local $p0
    i32.const 28
    i32.add
    get_local $l2
    i32.store
    get_local $p1
    i32.load offset=20
    call $env.db_remove_i64)
  (func $apply (type $t22) (param $p0 i64) (param $p1 i64) (param $p2 i64)
    (local $l0 i32) (local $l1 i64)
    get_global $g0
    i32.const 112
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
      i32.const 8897
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
      i64.store offset=104
      block $B6
        block $B7
          block $B8
            block $B9
              get_local $p2
              i64.const 4929617502180212735
              i64.le_s
              if $I10
                get_local $p2
                i64.const -6533262907872903168
                i64.eq
                br_if $B9
                get_local $p2
                i64.const -4993669930013425664
                i64.eq
                br_if $B8
                get_local $p2
                i64.const -3617168760277827584
                i64.ne
                br_if $B3
                get_local $l0
                i32.const 0
                i32.store offset=84
                get_local $l0
                i32.const 1
                i32.store offset=80
                get_local $l0
                get_local $l0
                i64.load offset=80
                i64.store offset=24
                get_local $l0
                i32.const 104
                i32.add
                get_local $l0
                i32.const 24
                i32.add
                call $f36
                drop
                br $B3
              end
              get_local $p2
              i64.const 4929617502180212736
              i64.eq
              br_if $B7
              get_local $p2
              i64.const 8516769789752901632
              i64.eq
              br_if $B6
              get_local $p2
              i64.const 5031766152489992192
              i64.ne
              br_if $B3
              get_local $l0
              i32.const 0
              i32.store offset=100
              get_local $l0
              i32.const 2
              i32.store offset=96
              get_local $l0
              get_local $l0
              i64.load offset=96
              i64.store offset=8
              get_local $l0
              i32.const 104
              i32.add
              get_local $l0
              i32.const 8
              i32.add
              call $f37
              drop
              br $B3
            end
            get_local $l0
            i32.const 0
            i32.store offset=76
            get_local $l0
            i32.const 3
            i32.store offset=72
            get_local $l0
            get_local $l0
            i64.load offset=72
            i64.store offset=32
            get_local $l0
            i32.const 104
            i32.add
            get_local $l0
            i32.const 32
            i32.add
            call $f38
            drop
            br $B3
          end
          get_local $l0
          i32.const 0
          i32.store offset=60
          get_local $l0
          i32.const 4
          i32.store offset=56
          get_local $l0
          get_local $l0
          i64.load offset=56
          i64.store offset=48
          get_local $l0
          i32.const 104
          i32.add
          get_local $l0
          i32.const 48
          i32.add
          call $f39
          drop
          br $B3
        end
        get_local $l0
        i32.const 0
        i32.store offset=68
        get_local $l0
        i32.const 5
        i32.store offset=64
        get_local $l0
        get_local $l0
        i64.load offset=64
        i64.store offset=40
        get_local $l0
        i32.const 104
        i32.add
        get_local $l0
        i32.const 40
        i32.add
        call $f40
        drop
        br $B3
      end
      get_local $l0
      i32.const 0
      i32.store offset=92
      get_local $l0
      i32.const 6
      i32.store offset=88
      get_local $l0
      get_local $l0
      i64.load offset=88
      i64.store offset=16
      get_local $l0
      i32.const 104
      i32.add
      get_local $l0
      i32.const 16
      i32.add
      call $f41
      drop
    end
    get_local $l0
    i32.const 112
    i32.add
    set_global $g0)
  (func $f36 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i64) (local $l5 i64)
    get_global $g0
    i32.const 96
    i32.sub
    tee_local $l1
    set_global $g0
    get_local $l1
    tee_local $l0
    get_local $p0
    i32.store offset=60
    get_local $l0
    get_local $p1
    i64.load align=4
    i64.store offset=48
    i32.const 0
    set_local $p1
    call $env.action_data_size
    tee_local $l3
    if $I0
      block $B1
        get_local $l3
        i32.const 513
        i32.ge_u
        if $I2
          get_local $l3
          call $f62
          set_local $l2
          br $B1
        end
        get_local $l0
        get_local $l3
        i32.const 15
        i32.add
        i32.const -16
        i32.and
        i32.sub
        tee_local $l2
        set_global $g0
      end
      get_local $l2
      get_local $l3
      call $env.read_action_data
      drop
    end
    get_local $l0
    i32.const 24
    i32.add
    i64.const 1398362884
    i64.store
    get_local $l0
    i64.const 0
    i64.store offset=8
    get_local $l0
    i64.const 0
    i64.store
    get_local $l0
    i64.const 0
    i64.store offset=16
    i32.const 1
    i32.const 9035
    call $env.eosio_assert
    i64.const 5462355
    set_local $l4
    block $B3
      block $B4
        loop $L5
          get_local $l4
          i32.wrap/i64
          i32.const 24
          i32.shl
          i32.const -1073741825
          i32.add
          i32.const 452984830
          i32.gt_u
          br_if $B4
          get_local $l4
          i64.const 8
          i64.shr_u
          set_local $l5
          get_local $l4
          i64.const 65280
          i64.and
          i64.const 0
          i64.ne
          if $I6
            get_local $l5
            set_local $l4
            i32.const 1
            set_local $l1
            get_local $p1
            tee_local $p0
            i32.const 1
            i32.add
            set_local $p1
            get_local $p0
            i32.const 6
            i32.lt_s
            br_if $L5
            br $B3
          end
          get_local $l5
          set_local $l4
          loop $L7
            get_local $l4
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            br_if $B4
            get_local $l4
            i64.const 8
            i64.shr_u
            set_local $l4
            get_local $p1
            i32.const 6
            i32.lt_s
            get_local $p1
            i32.const 1
            i32.add
            tee_local $p0
            set_local $p1
            br_if $L7
          end
          i32.const 1
          set_local $l1
          get_local $p0
          i32.const 1
          i32.add
          set_local $p1
          get_local $p0
          i32.const 6
          i32.lt_s
          br_if $L5
        end
        br $B3
      end
      i32.const 0
      set_local $l1
    end
    get_local $l1
    i32.const 8192
    call $env.eosio_assert
    get_local $l0
    i32.const 40
    i32.add
    i32.const 0
    i32.store
    get_local $l0
    i64.const 0
    i64.store offset=32
    get_local $l0
    get_local $l2
    i32.store offset=68
    get_local $l0
    get_local $l2
    i32.store offset=64
    get_local $l0
    get_local $l2
    get_local $l3
    i32.add
    i32.store offset=72
    get_local $l0
    get_local $l0
    i32.const -64
    i32.sub
    i32.store offset=80
    get_local $l0
    get_local $l0
    i32.store offset=88
    get_local $l0
    i32.const 88
    i32.add
    get_local $l0
    i32.const 80
    i32.add
    call $f44
    get_local $l3
    i32.const 513
    i32.ge_u
    if $I8
      get_local $l2
      call $f65
    end
    get_local $l0
    get_local $l0
    i32.const 48
    i32.add
    i32.store offset=68
    get_local $l0
    get_local $l0
    i32.const 60
    i32.add
    i32.store offset=64
    get_local $l0
    i32.const -64
    i32.sub
    get_local $l0
    call $f45
    get_local $l0
    i32.load8_u offset=32
    i32.const 1
    i32.and
    if $I9
      get_local $l0
      i32.const 40
      i32.add
      i32.load
      call $_ZdlPv
    end
    get_local $l0
    i32.const 96
    i32.add
    set_global $g0
    i32.const 1)
  (func $f37 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i64) (local $l9 i64)
    get_global $g0
    i32.const 96
    i32.sub
    tee_local $l1
    set_local $l0
    get_local $l1
    set_global $g0
    get_local $p1
    i32.load offset=4
    set_local $l6
    get_local $p1
    i32.load
    set_local $l4
    i32.const 0
    set_local $p1
    call $env.action_data_size
    tee_local $l3
    if $I0
      block $B1
        get_local $l3
        i32.const 513
        i32.ge_u
        if $I2
          get_local $l3
          call $f62
          set_local $l2
          br $B1
        end
        get_local $l1
        get_local $l3
        i32.const 15
        i32.add
        i32.const -16
        i32.and
        i32.sub
        tee_local $l2
        set_global $g0
      end
      get_local $l2
      get_local $l3
      call $env.read_action_data
      drop
    end
    get_local $l0
    i32.const 40
    i32.add
    i64.const 1398362884
    i64.store
    get_local $l0
    i64.const 0
    i64.store offset=32
    get_local $l0
    i64.const 0
    i64.store offset=24
    i32.const 1
    i32.const 9035
    call $env.eosio_assert
    i64.const 5462355
    set_local $l8
    block $B3
      loop $L4
        i32.const 0
        set_local $l5
        get_local $l8
        i32.wrap/i64
        i32.const 24
        i32.shl
        i32.const -1073741825
        i32.add
        i32.const 452984830
        i32.gt_u
        br_if $B3
        get_local $l8
        i64.const 8
        i64.shr_u
        set_local $l9
        get_local $l8
        i64.const 65280
        i64.and
        i64.const 0
        i64.ne
        if $I5
          get_local $l9
          set_local $l8
          i32.const 1
          set_local $l5
          get_local $p1
          tee_local $l1
          i32.const 1
          i32.add
          set_local $p1
          get_local $l1
          i32.const 6
          i32.lt_s
          br_if $L4
          br $B3
        end
        get_local $l9
        set_local $l8
        loop $L6
          get_local $l8
          i64.const 65280
          i64.and
          i64.const 0
          i64.ne
          br_if $B3
          get_local $l8
          i64.const 8
          i64.shr_u
          set_local $l8
          get_local $p1
          i32.const 6
          i32.lt_s
          get_local $p1
          i32.const 1
          i32.add
          tee_local $l7
          set_local $p1
          br_if $L6
        end
        i32.const 1
        set_local $l5
        get_local $l7
        i32.const 1
        i32.add
        set_local $p1
        get_local $l7
        i32.const 6
        i32.lt_s
        br_if $L4
      end
    end
    get_local $l5
    i32.const 8192
    call $env.eosio_assert
    get_local $l3
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $l0
    i32.const 24
    i32.add
    get_local $l2
    i32.const 8
    call $env.memcpy
    drop
    get_local $l3
    i32.const -8
    i32.and
    tee_local $l1
    i32.const 8
    i32.ne
    i32.const 9084
    call $env.eosio_assert
    get_local $l0
    i32.const 32
    i32.add
    tee_local $p1
    get_local $l2
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $l1
    i32.const 16
    i32.ne
    i32.const 9084
    call $env.eosio_assert
    get_local $l0
    i32.const 40
    i32.add
    get_local $l2
    i32.const 16
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $l3
    i32.const 513
    i32.ge_u
    if $I7
      get_local $l2
      call $f65
    end
    get_local $l0
    i32.const 56
    i32.add
    tee_local $l1
    get_local $p1
    i32.const 8
    i32.add
    i64.load
    i64.store
    get_local $l0
    get_local $p1
    i64.load
    i64.store offset=48
    get_local $l0
    i64.load offset=24
    set_local $l8
    get_local $l0
    i32.const 72
    i32.add
    get_local $l1
    i64.load
    i64.store
    get_local $l0
    get_local $l0
    i64.load offset=48
    i64.store offset=64
    get_local $p0
    get_local $l6
    i32.const 1
    i32.shr_s
    i32.add
    set_local $p1
    get_local $l6
    i32.const 1
    i32.and
    if $I8
      get_local $p1
      i32.load
      get_local $l4
      i32.add
      i32.load
      set_local $l4
    end
    get_local $l0
    i32.const 88
    i32.add
    get_local $l0
    i32.const 72
    i32.add
    i64.load
    tee_local $l9
    i64.store
    get_local $l0
    i32.const 16
    i32.add
    get_local $l9
    i64.store
    get_local $l0
    get_local $l0
    i64.load offset=64
    tee_local $l9
    i64.store offset=8
    get_local $l0
    get_local $l9
    i64.store offset=80
    get_local $p1
    get_local $l8
    get_local $l0
    i32.const 8
    i32.add
    get_local $l4
    call_indirect (type $t0)
    get_local $l0
    i32.const 96
    i32.add
    set_global $g0
    i32.const 1)
  (func $f38 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i64) (local $l6 i64) (local $l7 i64)
    get_global $g0
    i32.const 32
    i32.sub
    tee_local $l0
    set_local $l1
    get_local $l0
    set_global $g0
    get_local $p1
    i32.load offset=4
    set_local $l4
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
            get_local $p1
            call $f62
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
    i64.store offset=24
    get_local $l1
    i64.const 0
    i64.store offset=8
    get_local $p1
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $l1
    i32.const 8
    i32.add
    get_local $l0
    i32.const 8
    call $env.memcpy
    drop
    get_local $p1
    i32.const -8
    i32.and
    tee_local $l3
    i32.const 8
    i32.ne
    i32.const 9084
    call $env.eosio_assert
    get_local $l1
    i32.const 16
    i32.add
    get_local $l0
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $l3
    i32.const 16
    i32.ne
    i32.const 9084
    call $env.eosio_assert
    get_local $l1
    i32.const 24
    i32.add
    tee_local $l3
    get_local $l0
    i32.const 16
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $p1
    i32.const 513
    i32.ge_u
    if $I4
      get_local $l0
      call $f65
    end
    get_local $p0
    get_local $l4
    i32.const 1
    i32.shr_s
    i32.add
    set_local $p1
    get_local $l3
    i64.load
    set_local $l5
    get_local $l1
    i64.load offset=16
    set_local $l6
    get_local $l1
    i64.load offset=8
    set_local $l7
    get_local $l4
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
    get_local $l7
    get_local $l6
    get_local $l5
    get_local $l2
    call_indirect (type $t1)
    get_local $l1
    i32.const 32
    i32.add
    set_global $g0
    i32.const 1)
  (func $f39 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i64) (local $l5 i64)
    get_global $g0
    i32.const -64
    i32.add
    tee_local $l3
    set_global $g0
    get_local $l3
    tee_local $l0
    get_local $p0
    i32.store offset=44
    get_local $l0
    get_local $p1
    i64.load align=4
    i64.store offset=32
    i32.const 0
    set_local $p1
    call $env.action_data_size
    tee_local $l2
    if $I0
      block $B1
        get_local $l2
        i32.const 513
        i32.ge_u
        if $I2
          get_local $l2
          call $f62
          set_local $l1
          br $B1
        end
        get_local $l0
        get_local $l2
        i32.const 15
        i32.add
        i32.const -16
        i32.and
        i32.sub
        tee_local $l1
        set_global $g0
      end
      get_local $l1
      get_local $l2
      call $env.read_action_data
      drop
    end
    get_local $l0
    i64.const 1398362884
    i64.store offset=8
    get_local $l0
    i64.const 0
    i64.store
    i32.const 1
    i32.const 9035
    call $env.eosio_assert
    i64.const 5462355
    set_local $l4
    block $B3
      block $B4
        loop $L5
          get_local $l4
          i32.wrap/i64
          i32.const 24
          i32.shl
          i32.const -1073741825
          i32.add
          i32.const 452984830
          i32.gt_u
          br_if $B4
          get_local $l4
          i64.const 8
          i64.shr_u
          set_local $l5
          get_local $l4
          i64.const 65280
          i64.and
          i64.const 0
          i64.ne
          if $I6
            get_local $l5
            set_local $l4
            i32.const 1
            set_local $l3
            get_local $p1
            tee_local $p0
            i32.const 1
            i32.add
            set_local $p1
            get_local $p0
            i32.const 6
            i32.lt_s
            br_if $L5
            br $B3
          end
          get_local $l5
          set_local $l4
          loop $L7
            get_local $l4
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            br_if $B4
            get_local $l4
            i64.const 8
            i64.shr_u
            set_local $l4
            get_local $p1
            i32.const 6
            i32.lt_s
            get_local $p1
            i32.const 1
            i32.add
            tee_local $p0
            set_local $p1
            br_if $L7
          end
          i32.const 1
          set_local $l3
          get_local $p0
          i32.const 1
          i32.add
          set_local $p1
          get_local $p0
          i32.const 6
          i32.lt_s
          br_if $L5
        end
        br $B3
      end
      i32.const 0
      set_local $l3
    end
    get_local $l3
    i32.const 8192
    call $env.eosio_assert
    get_local $l0
    i32.const 24
    i32.add
    i32.const 0
    i32.store
    get_local $l0
    i64.const 0
    i64.store offset=16
    get_local $l0
    get_local $l1
    get_local $l2
    i32.add
    i32.store offset=56
    get_local $l0
    get_local $l1
    i32.store offset=48
    get_local $l2
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $l0
    get_local $l1
    i32.const 8
    call $env.memcpy
    drop
    get_local $l2
    i32.const -8
    i32.and
    i32.const 8
    i32.ne
    i32.const 9084
    call $env.eosio_assert
    get_local $l0
    i32.const 8
    i32.add
    get_local $l1
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l1
    i32.const 16
    i32.add
    i32.store offset=52
    get_local $l0
    i32.const 48
    i32.add
    get_local $l0
    i32.const 16
    i32.add
    call $f42
    drop
    get_local $l2
    i32.const 513
    i32.ge_u
    if $I8
      get_local $l1
      call $f65
    end
    get_local $l0
    get_local $l0
    i32.const 32
    i32.add
    i32.store offset=52
    get_local $l0
    get_local $l0
    i32.const 44
    i32.add
    i32.store offset=48
    get_local $l0
    i32.const 48
    i32.add
    get_local $l0
    call $f46
    get_local $l0
    i32.load8_u offset=16
    i32.const 1
    i32.and
    if $I9
      get_local $l0
      i32.const 24
      i32.add
      i32.load
      call $_ZdlPv
    end
    get_local $l0
    i32.const -64
    i32.sub
    set_global $g0
    i32.const 1)
  (func $f40 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i64) (local $l5 i64)
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
            get_local $p1
            call $f62
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
    i64.store
    get_local $p1
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $l1
    get_local $l0
    i32.const 8
    call $env.memcpy
    drop
    get_local $p1
    i32.const -8
    i32.and
    i32.const 8
    i32.ne
    i32.const 9084
    call $env.eosio_assert
    get_local $l1
    i32.const 8
    i32.add
    get_local $l0
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $p1
    i32.const 513
    i32.ge_u
    if $I4
      get_local $l0
      call $f65
    end
    get_local $p0
    get_local $l3
    i32.const 1
    i32.shr_s
    i32.add
    set_local $p1
    get_local $l1
    i64.load offset=8
    set_local $l4
    get_local $l1
    i64.load
    set_local $l5
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
    get_local $l5
    get_local $l4
    get_local $l2
    call_indirect (type $t2)
    get_local $l1
    i32.const 16
    i32.add
    set_global $g0
    i32.const 1)
  (func $f41 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i64) (local $l5 i64)
    get_global $g0
    i32.const 80
    i32.sub
    tee_local $l3
    set_global $g0
    get_local $l3
    tee_local $l0
    get_local $p0
    i32.store offset=60
    get_local $l0
    get_local $p1
    i64.load align=4
    i64.store offset=48
    i32.const 0
    set_local $p1
    call $env.action_data_size
    tee_local $l2
    if $I0
      block $B1
        get_local $l2
        i32.const 513
        i32.ge_u
        if $I2
          get_local $l2
          call $f62
          set_local $l1
          br $B1
        end
        get_local $l0
        get_local $l2
        i32.const 15
        i32.add
        i32.const -16
        i32.and
        i32.sub
        tee_local $l1
        set_global $g0
      end
      get_local $l1
      get_local $l2
      call $env.read_action_data
      drop
    end
    get_local $l0
    i32.const 24
    i32.add
    i64.const 1398362884
    i64.store
    get_local $l0
    i64.const 0
    i64.store offset=16
    get_local $l0
    i64.const 0
    i64.store offset=8
    i32.const 1
    i32.const 9035
    call $env.eosio_assert
    i64.const 5462355
    set_local $l4
    block $B3
      block $B4
        loop $L5
          get_local $l4
          i32.wrap/i64
          i32.const 24
          i32.shl
          i32.const -1073741825
          i32.add
          i32.const 452984830
          i32.gt_u
          br_if $B4
          get_local $l4
          i64.const 8
          i64.shr_u
          set_local $l5
          get_local $l4
          i64.const 65280
          i64.and
          i64.const 0
          i64.ne
          if $I6
            get_local $l5
            set_local $l4
            i32.const 1
            set_local $l3
            get_local $p1
            tee_local $p0
            i32.const 1
            i32.add
            set_local $p1
            get_local $p0
            i32.const 6
            i32.lt_s
            br_if $L5
            br $B3
          end
          get_local $l5
          set_local $l4
          loop $L7
            get_local $l4
            i64.const 65280
            i64.and
            i64.const 0
            i64.ne
            br_if $B4
            get_local $l4
            i64.const 8
            i64.shr_u
            set_local $l4
            get_local $p1
            i32.const 6
            i32.lt_s
            get_local $p1
            i32.const 1
            i32.add
            tee_local $p0
            set_local $p1
            br_if $L7
          end
          i32.const 1
          set_local $l3
          get_local $p0
          i32.const 1
          i32.add
          set_local $p1
          get_local $p0
          i32.const 6
          i32.lt_s
          br_if $L5
        end
        br $B3
      end
      i32.const 0
      set_local $l3
    end
    get_local $l3
    i32.const 8192
    call $env.eosio_assert
    get_local $l0
    i32.const 40
    i32.add
    i32.const 0
    i32.store
    get_local $l0
    i64.const 0
    i64.store offset=32
    get_local $l0
    get_local $l1
    get_local $l2
    i32.add
    i32.store offset=72
    get_local $l0
    get_local $l1
    i32.store offset=64
    get_local $l2
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $l0
    i32.const 8
    i32.add
    get_local $l1
    i32.const 8
    call $env.memcpy
    drop
    get_local $l2
    i32.const -8
    i32.and
    tee_local $p1
    i32.const 8
    i32.ne
    i32.const 9084
    call $env.eosio_assert
    get_local $l0
    i32.const 16
    i32.add
    get_local $l1
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $p1
    i32.const 16
    i32.ne
    i32.const 9084
    call $env.eosio_assert
    get_local $l0
    i32.const 24
    i32.add
    get_local $l1
    i32.const 16
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l1
    i32.const 24
    i32.add
    i32.store offset=68
    get_local $l0
    i32.const -64
    i32.sub
    get_local $l0
    i32.const 32
    i32.add
    call $f42
    drop
    get_local $l2
    i32.const 513
    i32.ge_u
    if $I8
      get_local $l1
      call $f65
    end
    get_local $l0
    get_local $l0
    i32.const 48
    i32.add
    i32.store offset=68
    get_local $l0
    get_local $l0
    i32.const 60
    i32.add
    i32.store offset=64
    get_local $l0
    i32.const -64
    i32.sub
    get_local $l0
    i32.const 8
    i32.add
    call $f43
    get_local $l0
    i32.load8_u offset=32
    i32.const 1
    i32.and
    if $I9
      get_local $l0
      i32.const 40
      i32.add
      i32.load
      call $_ZdlPv
    end
    get_local $l0
    i32.const 80
    i32.add
    set_global $g0
    i32.const 1)
  (func $f42 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
    get_global $g0
    i32.const 32
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $l0
    i32.const 0
    i32.store offset=24
    get_local $l0
    i64.const 0
    i64.store offset=16
    get_local $p0
    get_local $l0
    i32.const 16
    i32.add
    call $f54
    drop
    block $B0
      block $B1
        block $B2
          block $B3 (result i32)
            block $B4
              block $B5
                block $B6
                  get_local $l0
                  i32.load offset=20
                  get_local $l0
                  i32.load offset=16
                  tee_local $l1
                  i32.sub
                  tee_local $l2
                  if $I7
                    get_local $l0
                    i32.const 8
                    i32.add
                    i32.const 0
                    i32.store
                    get_local $l0
                    i64.const 0
                    i64.store
                    get_local $l2
                    i32.const -16
                    i32.ge_u
                    br_if $B2
                    get_local $l2
                    i32.const 10
                    i32.gt_u
                    br_if $B6
                    get_local $l0
                    get_local $l2
                    i32.const 1
                    i32.shl
                    i32.store8
                    get_local $l0
                    i32.const 1
                    i32.or
                    set_local $l4
                    br $B5
                  end
                  get_local $p1
                  i32.load8_u
                  i32.const 1
                  i32.and
                  br_if $B4
                  get_local $p1
                  i32.const 0
                  i32.store16
                  get_local $p1
                  i32.const 8
                  i32.add
                  br $B3
                end
                get_local $l2
                i32.const 16
                i32.add
                i32.const -16
                i32.and
                tee_local $l3
                call $_Znwj
                set_local $l4
                get_local $l0
                get_local $l3
                i32.const 1
                i32.or
                i32.store
                get_local $l0
                get_local $l4
                i32.store offset=8
                get_local $l0
                get_local $l2
                i32.store offset=4
              end
              get_local $l2
              set_local $l5
              get_local $l4
              set_local $l3
              loop $L8
                get_local $l3
                get_local $l1
                i32.load8_u
                i32.store8
                get_local $l3
                i32.const 1
                i32.add
                set_local $l3
                get_local $l1
                i32.const 1
                i32.add
                set_local $l1
                get_local $l5
                i32.const -1
                i32.add
                tee_local $l5
                br_if $L8
              end
              get_local $l4
              get_local $l2
              i32.add
              i32.const 0
              i32.store8
              block $B9
                get_local $p1
                i32.load8_u
                i32.const 1
                i32.and
                i32.eqz
                if $I10
                  get_local $p1
                  i32.const 0
                  i32.store16
                  br $B9
                end
                get_local $p1
                i32.load offset=8
                i32.const 0
                i32.store8
                get_local $p1
                i32.const 0
                i32.store offset=4
              end
              get_local $p1
              i32.const 0
              call $f61
              get_local $p1
              i32.const 8
              i32.add
              get_local $l0
              i32.const 8
              i32.add
              i32.load
              i32.store
              get_local $p1
              get_local $l0
              i64.load
              i64.store align=4
              get_local $l0
              i32.load offset=16
              tee_local $l1
              i32.eqz
              br_if $B0
              br $B1
            end
            get_local $p1
            i32.load offset=8
            i32.const 0
            i32.store8
            get_local $p1
            i32.const 0
            i32.store offset=4
            get_local $p1
            i32.const 8
            i32.add
          end
          get_local $p1
          i32.const 0
          call $f61
          i32.const 0
          i32.store
          get_local $p1
          i64.const 0
          i64.store align=4
          get_local $l0
          i32.load offset=16
          tee_local $l1
          br_if $B1
          br $B0
        end
        get_local $l0
        call $f59
        unreachable
      end
      get_local $l0
      get_local $l1
      i32.store offset=20
      get_local $l1
      call $_ZdlPv
    end
    get_local $l0
    i32.const 32
    i32.add
    set_global $g0
    get_local $p0)
  (func $f43 (type $t8) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i64)
    get_global $g0
    i32.const 96
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $l0
    i32.const 40
    i32.add
    tee_local $l1
    get_local $p1
    i32.const 16
    i32.add
    i64.load
    i64.store
    get_local $l0
    get_local $p1
    i64.load offset=8
    i64.store offset=32
    get_local $p1
    i64.load
    set_local $l4
    get_local $l0
    i32.const 16
    i32.add
    get_local $p1
    i32.const 24
    i32.add
    call $f60
    set_local $p1
    get_local $l0
    i32.const 56
    i32.add
    get_local $l1
    i64.load
    i64.store
    get_local $l0
    get_local $l0
    i64.load offset=32
    i64.store offset=48
    get_local $p0
    i32.load
    i32.load
    get_local $p0
    i32.load offset=4
    tee_local $p0
    i32.load offset=4
    tee_local $l2
    i32.const 1
    i32.shr_s
    i32.add
    set_local $l1
    get_local $p0
    i32.load
    set_local $p0
    get_local $l2
    i32.const 1
    i32.and
    if $I0
      get_local $l1
      i32.load
      get_local $p0
      i32.add
      i32.load
      set_local $p0
    end
    get_local $l0
    i32.const 88
    i32.add
    tee_local $l3
    get_local $l0
    i32.const 56
    i32.add
    i64.load
    i64.store
    get_local $l0
    get_local $l0
    i64.load offset=48
    i64.store offset=80
    get_local $l0
    i32.const -64
    i32.sub
    get_local $p1
    call $f60
    set_local $l2
    get_local $l0
    i32.const 8
    i32.add
    get_local $l3
    i64.load
    i64.store
    get_local $l0
    get_local $l0
    i64.load offset=80
    i64.store
    get_local $l1
    get_local $l4
    get_local $l0
    get_local $l2
    get_local $p0
    call_indirect (type $t3)
    block $B1
      block $B2
        get_local $l0
        i32.load8_u offset=64
        i32.const 1
        i32.and
        i32.eqz
        if $I3
          get_local $p1
          i32.load8_u
          i32.const 1
          i32.and
          br_if $B2
          br $B1
        end
        get_local $l2
        i32.load offset=8
        call $_ZdlPv
        get_local $p1
        i32.load8_u
        i32.const 1
        i32.and
        i32.eqz
        br_if $B1
      end
      get_local $p1
      i32.load offset=8
      call $_ZdlPv
      get_local $l0
      i32.const 96
      i32.add
      set_global $g0
      return
    end
    get_local $l0
    i32.const 96
    i32.add
    set_global $g0)
  (func $f44 (type $t8) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32)
    get_local $p0
    i32.load
    get_local $p1
    i32.load
    tee_local $l0
    i32.load offset=8
    get_local $l0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $l0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4
    get_local $p0
    i32.load
    set_local $p0
    get_local $p1
    i32.load
    tee_local $l0
    i32.load offset=8
    get_local $l0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $p0
    i32.const 8
    i32.add
    get_local $l0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4
    get_local $p1
    i32.load
    tee_local $l0
    i32.load offset=8
    get_local $l0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $p0
    i32.const 16
    i32.add
    get_local $l0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    tee_local $l1
    i32.store offset=4
    get_local $l0
    i32.load offset=8
    get_local $l1
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $p0
    i32.const 24
    i32.add
    get_local $l0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4
    get_local $p1
    i32.load
    get_local $p0
    i32.const 32
    i32.add
    call $f42
    drop)
  (func $f45 (type $t8) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i64) (local $l5 i64)
    get_global $g0
    i32.const 96
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $l0
    i32.const 40
    i32.add
    tee_local $l1
    get_local $p1
    i32.const 24
    i32.add
    i64.load
    i64.store
    get_local $l0
    get_local $p1
    i64.load offset=16
    i64.store offset=32
    get_local $p1
    i64.load offset=8
    set_local $l4
    get_local $p1
    i64.load
    set_local $l5
    get_local $l0
    i32.const 16
    i32.add
    get_local $p1
    i32.const 32
    i32.add
    call $f60
    set_local $p1
    get_local $l0
    i32.const 56
    i32.add
    get_local $l1
    i64.load
    i64.store
    get_local $l0
    get_local $l0
    i64.load offset=32
    i64.store offset=48
    get_local $p0
    i32.load
    i32.load
    get_local $p0
    i32.load offset=4
    tee_local $p0
    i32.load offset=4
    tee_local $l2
    i32.const 1
    i32.shr_s
    i32.add
    set_local $l1
    get_local $p0
    i32.load
    set_local $p0
    get_local $l2
    i32.const 1
    i32.and
    if $I0
      get_local $l1
      i32.load
      get_local $p0
      i32.add
      i32.load
      set_local $p0
    end
    get_local $l0
    i32.const 88
    i32.add
    tee_local $l3
    get_local $l0
    i32.const 56
    i32.add
    i64.load
    i64.store
    get_local $l0
    get_local $l0
    i64.load offset=48
    i64.store offset=80
    get_local $l0
    i32.const -64
    i32.sub
    get_local $p1
    call $f60
    set_local $l2
    get_local $l0
    i32.const 8
    i32.add
    get_local $l3
    i64.load
    i64.store
    get_local $l0
    get_local $l0
    i64.load offset=80
    i64.store
    get_local $l1
    get_local $l5
    get_local $l4
    get_local $l0
    get_local $l2
    get_local $p0
    call_indirect (type $t4)
    block $B1
      block $B2
        get_local $l0
        i32.load8_u offset=64
        i32.const 1
        i32.and
        i32.eqz
        if $I3
          get_local $p1
          i32.load8_u
          i32.const 1
          i32.and
          br_if $B2
          br $B1
        end
        get_local $l2
        i32.load offset=8
        call $_ZdlPv
        get_local $p1
        i32.load8_u
        i32.const 1
        i32.and
        i32.eqz
        br_if $B1
      end
      get_local $p1
      i32.load offset=8
      call $_ZdlPv
      get_local $l0
      i32.const 96
      i32.add
      set_global $g0
      return
    end
    get_local $l0
    i32.const 96
    i32.add
    set_global $g0)
  (func $f46 (type $t8) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32)
    get_global $g0
    i32.const 96
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $l0
    i32.const 40
    i32.add
    tee_local $l1
    get_local $p1
    i32.const 8
    i32.add
    i64.load
    i64.store
    get_local $l0
    get_local $p1
    i64.load
    i64.store offset=32
    get_local $l0
    i32.const 16
    i32.add
    get_local $p1
    i32.const 16
    i32.add
    call $f60
    set_local $p1
    get_local $l0
    i32.const 56
    i32.add
    get_local $l1
    i64.load
    i64.store
    get_local $l0
    get_local $l0
    i64.load offset=32
    i64.store offset=48
    get_local $p0
    i32.load
    i32.load
    get_local $p0
    i32.load offset=4
    tee_local $p0
    i32.load offset=4
    tee_local $l2
    i32.const 1
    i32.shr_s
    i32.add
    set_local $l1
    get_local $p0
    i32.load
    set_local $p0
    get_local $l2
    i32.const 1
    i32.and
    if $I0
      get_local $l1
      i32.load
      get_local $p0
      i32.add
      i32.load
      set_local $p0
    end
    get_local $l0
    i32.const 88
    i32.add
    tee_local $l3
    get_local $l0
    i32.const 56
    i32.add
    i64.load
    i64.store
    get_local $l0
    get_local $l0
    i64.load offset=48
    i64.store offset=80
    get_local $l0
    i32.const -64
    i32.sub
    get_local $p1
    call $f60
    set_local $l2
    get_local $l0
    i32.const 8
    i32.add
    get_local $l3
    i64.load
    i64.store
    get_local $l0
    get_local $l0
    i64.load offset=80
    i64.store
    get_local $l1
    get_local $l0
    get_local $l2
    get_local $p0
    call_indirect (type $t5)
    block $B1
      block $B2
        get_local $l0
        i32.load8_u offset=64
        i32.const 1
        i32.and
        i32.eqz
        if $I3
          get_local $p1
          i32.load8_u
          i32.const 1
          i32.and
          br_if $B2
          br $B1
        end
        get_local $l2
        i32.load offset=8
        call $_ZdlPv
        get_local $p1
        i32.load8_u
        i32.const 1
        i32.and
        i32.eqz
        br_if $B1
      end
      get_local $p1
      i32.load offset=8
      call $_ZdlPv
      get_local $l0
      i32.const 96
      i32.add
      set_global $g0
      return
    end
    get_local $l0
    i32.const 96
    i32.add
    set_global $g0)
  (func $f47 (type $t8) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32)
    get_local $p0
    i32.load
    set_local $l1
    get_local $p1
    i32.load
    tee_local $l0
    i32.load offset=8
    get_local $l0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $l1
    get_local $l0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    tee_local $l2
    i32.store offset=4
    get_local $l0
    i32.load offset=8
    get_local $l2
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $l1
    i32.const 8
    i32.add
    get_local $l0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4
    get_local $p0
    i32.load offset=4
    set_local $l1
    get_local $p1
    i32.load
    tee_local $l0
    i32.load offset=8
    get_local $l0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $l1
    get_local $l0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    tee_local $l2
    i32.store offset=4
    get_local $l0
    i32.load offset=8
    get_local $l2
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $l1
    i32.const 8
    i32.add
    get_local $l0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4
    get_local $p0
    i32.load offset=8
    get_local $p1
    i32.load
    tee_local $l0
    i32.load offset=8
    get_local $l0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 9084
    call $env.eosio_assert
    get_local $l0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4)
  (func $f48 (type $t8) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
    block $B0
      block $B1
        block $B2
          block $B3
            get_local $p0
            i32.load offset=8
            tee_local $l0
            get_local $p0
            i32.load offset=4
            tee_local $l1
            i32.sub
            get_local $p1
            i32.lt_u
            if $I4
              get_local $l1
              get_local $p0
              i32.load
              tee_local $l2
              i32.sub
              tee_local $l5
              get_local $p1
              i32.add
              tee_local $l4
              i32.const -1
              i32.le_s
              br_if $B2
              i32.const 2147483647
              set_local $l3
              get_local $l0
              get_local $l2
              i32.sub
              tee_local $l0
              i32.const 1073741822
              i32.le_u
              if $I5
                get_local $l4
                get_local $l0
                i32.const 1
                i32.shl
                tee_local $l0
                get_local $l0
                get_local $l4
                i32.lt_u
                select
                tee_local $l3
                i32.eqz
                br_if $B3
              end
              get_local $l3
              call $_Znwj
              set_local $l0
              br $B1
            end
            get_local $p0
            i32.const 4
            i32.add
            set_local $p0
            loop $L6
              get_local $l1
              i32.const 0
              i32.store8
              get_local $p0
              get_local $p0
              i32.load
              i32.const 1
              i32.add
              tee_local $l1
              i32.store
              get_local $p1
              i32.const -1
              i32.add
              tee_local $p1
              br_if $L6
            end
            br $B0
          end
          i32.const 0
          set_local $l3
          i32.const 0
          set_local $l0
          br $B1
        end
        get_local $p0
        call $f59
        unreachable
      end
      get_local $l0
      get_local $l3
      i32.add
      set_local $l3
      get_local $l1
      get_local $p1
      i32.add
      get_local $l2
      i32.sub
      set_local $l2
      get_local $l0
      get_local $l5
      i32.add
      tee_local $l5
      set_local $l1
      loop $L7
        get_local $l1
        i32.const 0
        i32.store8
        get_local $l1
        i32.const 1
        i32.add
        set_local $l1
        get_local $p1
        i32.const -1
        i32.add
        tee_local $p1
        br_if $L7
      end
      get_local $l0
      get_local $l2
      i32.add
      set_local $l2
      get_local $l5
      get_local $p0
      i32.const 4
      i32.add
      tee_local $l4
      i32.load
      get_local $p0
      i32.load
      tee_local $p1
      i32.sub
      tee_local $l1
      i32.sub
      set_local $l0
      get_local $l1
      i32.const 1
      i32.ge_s
      if $I8
        get_local $l0
        get_local $p1
        get_local $l1
        call $env.memcpy
        drop
        get_local $p0
        i32.load
        set_local $p1
      end
      get_local $p0
      get_local $l0
      i32.store
      get_local $l4
      get_local $l2
      i32.store
      get_local $p0
      i32.const 8
      i32.add
      get_local $l3
      i32.store
      get_local $p1
      i32.eqz
      br_if $B0
      get_local $p1
      call $_ZdlPv
    end)
  (func $f49 (type $t8) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32)
    get_local $p0
    i32.load
    set_local $l1
    get_local $p1
    i32.load
    tee_local $l0
    i32.load offset=8
    get_local $l0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_s
    i32.const 9140
    call $env.eosio_assert
    get_local $l0
    i32.load offset=4
    get_local $l1
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4
    get_local $p0
    i32.load
    set_local $p0
    get_local $p1
    i32.load
    tee_local $l0
    i32.load offset=8
    get_local $l0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_s
    i32.const 9140
    call $env.eosio_assert
    get_local $l0
    i32.load offset=4
    get_local $p0
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4
    get_local $p1
    i32.load
    tee_local $l0
    i32.load offset=8
    get_local $l0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_s
    i32.const 9140
    call $env.eosio_assert
    get_local $l0
    i32.load offset=4
    get_local $p0
    i32.const 16
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    tee_local $l1
    i32.store offset=4
    get_local $l0
    i32.load offset=8
    get_local $l1
    i32.sub
    i32.const 7
    i32.gt_s
    i32.const 9140
    call $env.eosio_assert
    get_local $l0
    i32.load offset=4
    get_local $p0
    i32.const 24
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4
    get_local $p1
    i32.load
    get_local $p0
    i32.const 32
    i32.add
    call $f51
    drop)
  (func $f50 (type $t8) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i64)
    get_global $g0
    i32.const 16
    i32.sub
    tee_local $l2
    set_global $g0
    get_local $p0
    i32.const 0
    i32.store offset=8
    get_local $p0
    i64.const 0
    i64.store align=4
    i32.const 16
    set_local $l0
    get_local $p1
    i32.const 20
    i32.add
    i32.load
    tee_local $l1
    get_local $p1
    i32.load offset=16
    tee_local $l3
    i32.sub
    tee_local $l4
    i32.const 4
    i32.shr_s
    i64.extend_u/i32
    set_local $l6
    loop $L0
      get_local $l0
      i32.const 1
      i32.add
      set_local $l0
      get_local $l6
      i64.const 7
      i64.shr_u
      tee_local $l6
      i64.const 0
      i64.ne
      br_if $L0
    end
    get_local $l3
    get_local $l1
    i32.ne
    if $I1
      get_local $l4
      i32.const -16
      i32.and
      get_local $l0
      i32.add
      set_local $l0
    end
    get_local $p1
    i32.const 16
    i32.add
    set_local $l5
    get_local $p1
    i32.load offset=28
    tee_local $l1
    get_local $l0
    i32.sub
    get_local $p1
    i32.const 32
    i32.add
    i32.load
    tee_local $l3
    i32.sub
    set_local $l0
    get_local $p1
    i32.const 28
    i32.add
    set_local $l4
    get_local $l3
    get_local $l1
    i32.sub
    i64.extend_u/i32
    set_local $l6
    loop $L2
      get_local $l0
      i32.const -1
      i32.add
      set_local $l0
      get_local $l6
      i64.const 7
      i64.shr_u
      tee_local $l6
      i64.const 0
      i64.ne
      br_if $L2
    end
    i32.const 0
    set_local $l1
    get_local $l2
    block $B3 (result i32)
      get_local $l0
      if $I4
        get_local $p0
        i32.const 0
        get_local $l0
        i32.sub
        call $f48
        get_local $p0
        i32.const 4
        i32.add
        i32.load
        set_local $l1
        get_local $p0
        i32.load
        br $B3
      end
      i32.const 0
    end
    tee_local $l0
    i32.store
    get_local $l2
    get_local $l1
    i32.store offset=8
    get_local $l1
    get_local $l0
    i32.sub
    tee_local $p0
    i32.const 7
    i32.gt_s
    i32.const 9140
    call $env.eosio_assert
    get_local $l0
    get_local $p1
    i32.const 8
    call $env.memcpy
    drop
    get_local $p0
    i32.const -8
    i32.add
    i32.const 7
    i32.gt_s
    i32.const 9140
    call $env.eosio_assert
    get_local $l0
    i32.const 8
    i32.add
    get_local $p1
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $l2
    get_local $l0
    i32.const 16
    i32.add
    i32.store offset=4
    get_local $l2
    get_local $l5
    call $f52
    get_local $l4
    call $f53
    drop
    get_local $l2
    i32.const 16
    i32.add
    set_global $g0)
  (func $f51 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i64)
    get_global $g0
    i32.const 16
    i32.sub
    tee_local $l2
    set_global $g0
    get_local $p1
    i32.load offset=4
    get_local $p1
    i32.load8_u
    tee_local $l0
    i32.const 1
    i32.shr_u
    get_local $l0
    i32.const 1
    i32.and
    select
    i64.extend_u/i32
    set_local $l6
    get_local $p0
    i32.load offset=4
    set_local $l1
    get_local $p0
    i32.const 8
    i32.add
    set_local $l5
    get_local $p0
    i32.const 4
    i32.add
    set_local $l0
    loop $L0
      get_local $l6
      i32.wrap/i64
      set_local $l3
      get_local $l2
      get_local $l6
      i64.const 7
      i64.shr_u
      tee_local $l6
      i64.const 0
      i64.ne
      tee_local $l4
      i32.const 7
      i32.shl
      get_local $l3
      i32.const 127
      i32.and
      i32.or
      i32.store8 offset=15
      get_local $l5
      i32.load
      get_local $l1
      i32.sub
      i32.const 0
      i32.gt_s
      i32.const 9140
      call $env.eosio_assert
      get_local $l0
      i32.load
      get_local $l2
      i32.const 15
      i32.add
      i32.const 1
      call $env.memcpy
      drop
      get_local $l0
      get_local $l0
      i32.load
      i32.const 1
      i32.add
      tee_local $l1
      i32.store
      get_local $l4
      br_if $L0
    end
    get_local $p1
    i32.const 4
    i32.add
    i32.load
    get_local $p1
    i32.load8_u
    tee_local $l0
    i32.const 1
    i32.shr_u
    get_local $l0
    i32.const 1
    i32.and
    tee_local $l3
    select
    tee_local $l0
    if $I1
      get_local $p1
      i32.load offset=8
      set_local $l4
      get_local $p0
      i32.const 8
      i32.add
      i32.load
      get_local $l1
      i32.sub
      get_local $l0
      i32.ge_s
      i32.const 9140
      call $env.eosio_assert
      get_local $p0
      i32.const 4
      i32.add
      tee_local $l1
      i32.load
      get_local $l4
      get_local $p1
      i32.const 1
      i32.add
      get_local $l3
      select
      get_local $l0
      call $env.memcpy
      drop
      get_local $l1
      get_local $l1
      i32.load
      get_local $l0
      i32.add
      i32.store
    end
    get_local $l2
    i32.const 16
    i32.add
    set_global $g0
    get_local $p0)
  (func $f52 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i64)
    get_global $g0
    i32.const 16
    i32.sub
    tee_local $l3
    set_global $g0
    get_local $p1
    i32.load offset=4
    get_local $p1
    i32.load
    i32.sub
    i32.const 4
    i32.shr_s
    i64.extend_u/i32
    set_local $l5
    get_local $p0
    i32.load offset=4
    set_local $l1
    get_local $p0
    i32.const 8
    i32.add
    set_local $l4
    loop $L0
      get_local $l5
      i32.wrap/i64
      set_local $l0
      get_local $l3
      get_local $l5
      i64.const 7
      i64.shr_u
      tee_local $l5
      i64.const 0
      i64.ne
      tee_local $l2
      i32.const 7
      i32.shl
      get_local $l0
      i32.const 127
      i32.and
      i32.or
      i32.store8 offset=15
      get_local $l4
      i32.load
      get_local $l1
      i32.sub
      i32.const 0
      i32.gt_s
      i32.const 9140
      call $env.eosio_assert
      get_local $p0
      i32.const 4
      i32.add
      tee_local $l0
      i32.load
      get_local $l3
      i32.const 15
      i32.add
      i32.const 1
      call $env.memcpy
      drop
      get_local $l0
      get_local $l0
      i32.load
      i32.const 1
      i32.add
      tee_local $l1
      i32.store
      get_local $l2
      br_if $L0
    end
    get_local $p1
    i32.load
    tee_local $l2
    get_local $p1
    i32.const 4
    i32.add
    i32.load
    tee_local $p1
    i32.ne
    if $I1
      get_local $p0
      i32.const 4
      i32.add
      set_local $l0
      loop $L2
        get_local $p0
        i32.const 8
        i32.add
        tee_local $l4
        i32.load
        get_local $l1
        i32.sub
        i32.const 7
        i32.gt_s
        i32.const 9140
        call $env.eosio_assert
        get_local $l0
        i32.load
        get_local $l2
        i32.const 8
        call $env.memcpy
        drop
        get_local $l0
        get_local $l0
        i32.load
        i32.const 8
        i32.add
        tee_local $l1
        i32.store
        get_local $l4
        i32.load
        get_local $l1
        i32.sub
        i32.const 7
        i32.gt_s
        i32.const 9140
        call $env.eosio_assert
        get_local $l0
        i32.load
        get_local $l2
        i32.const 8
        i32.add
        i32.const 8
        call $env.memcpy
        drop
        get_local $l0
        get_local $l0
        i32.load
        i32.const 8
        i32.add
        tee_local $l1
        i32.store
        get_local $l2
        i32.const 16
        i32.add
        tee_local $l2
        get_local $p1
        i32.ne
        br_if $L2
      end
    end
    get_local $l3
    i32.const 16
    i32.add
    set_global $g0
    get_local $p0)
  (func $f53 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i64)
    get_global $g0
    i32.const 16
    i32.sub
    tee_local $l2
    set_global $g0
    get_local $p1
    i32.load offset=4
    get_local $p1
    i32.load
    i32.sub
    i64.extend_u/i32
    set_local $l6
    get_local $p0
    i32.load offset=4
    set_local $l0
    get_local $p0
    i32.const 8
    i32.add
    set_local $l4
    get_local $p0
    i32.const 4
    i32.add
    set_local $l1
    loop $L0
      get_local $l6
      i32.wrap/i64
      set_local $l3
      get_local $l2
      get_local $l6
      i64.const 7
      i64.shr_u
      tee_local $l6
      i64.const 0
      i64.ne
      tee_local $l5
      i32.const 7
      i32.shl
      get_local $l3
      i32.const 127
      i32.and
      i32.or
      i32.store8 offset=15
      get_local $l4
      i32.load
      get_local $l0
      i32.sub
      i32.const 0
      i32.gt_s
      i32.const 9140
      call $env.eosio_assert
      get_local $l1
      i32.load
      get_local $l2
      i32.const 15
      i32.add
      i32.const 1
      call $env.memcpy
      drop
      get_local $l1
      get_local $l1
      i32.load
      i32.const 1
      i32.add
      tee_local $l0
      i32.store
      get_local $l5
      br_if $L0
    end
    get_local $p0
    i32.const 8
    i32.add
    i32.load
    get_local $l0
    i32.sub
    get_local $p1
    i32.const 4
    i32.add
    i32.load
    get_local $p1
    i32.load
    tee_local $l3
    i32.sub
    tee_local $l1
    i32.ge_s
    i32.const 9140
    call $env.eosio_assert
    get_local $p0
    i32.const 4
    i32.add
    tee_local $l0
    i32.load
    get_local $l3
    get_local $l1
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load
    get_local $l1
    i32.add
    i32.store
    get_local $l2
    i32.const 16
    i32.add
    set_global $g0
    get_local $p0)
  (func $f54 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i64)
    get_local $p0
    i32.load offset=4
    set_local $l1
    get_local $p0
    i32.const 8
    i32.add
    set_local $l4
    get_local $p0
    i32.const 4
    i32.add
    set_local $l3
    loop $L0
      get_local $l1
      get_local $l4
      i32.load
      i32.lt_u
      i32.const 9720
      call $env.eosio_assert
      get_local $l3
      i32.load
      tee_local $l1
      i32.load8_u
      set_local $l2
      get_local $l3
      get_local $l1
      i32.const 1
      i32.add
      tee_local $l1
      i32.store
      get_local $l5
      get_local $l2
      i32.const 127
      i32.and
      get_local $l0
      i32.const 255
      i32.and
      tee_local $l0
      i32.shl
      i64.extend_u/i32
      i64.or
      set_local $l5
      get_local $l0
      i32.const 7
      i32.add
      set_local $l0
      get_local $l2
      i32.const 128
      i32.and
      br_if $L0
    end
    block $B1
      get_local $p1
      i32.load offset=4
      tee_local $l0
      get_local $p1
      i32.load
      tee_local $l2
      i32.sub
      tee_local $l4
      get_local $l5
      i32.wrap/i64
      tee_local $l3
      i32.lt_u
      if $I2
        get_local $p1
        get_local $l3
        get_local $l4
        i32.sub
        call $f48
        get_local $p0
        i32.const 4
        i32.add
        i32.load
        set_local $l1
        get_local $p1
        i32.const 4
        i32.add
        i32.load
        set_local $l0
        get_local $p1
        i32.load
        set_local $l2
        br $B1
      end
      get_local $l4
      get_local $l3
      i32.le_u
      br_if $B1
      get_local $p1
      i32.const 4
      i32.add
      get_local $l2
      get_local $l3
      i32.add
      tee_local $l0
      i32.store
    end
    get_local $p0
    i32.const 8
    i32.add
    i32.load
    get_local $l1
    i32.sub
    get_local $l0
    get_local $l2
    i32.sub
    tee_local $l1
    i32.ge_u
    i32.const 9084
    call $env.eosio_assert
    get_local $l2
    get_local $p0
    i32.const 4
    i32.add
    tee_local $l0
    i32.load
    get_local $l1
    call $env.memcpy
    drop
    get_local $l0
    get_local $l0
    i32.load
    get_local $l1
    i32.add
    i32.store
    get_local $p0)
  (func $_Znwj (type $t17) (param $p0 i32) (result i32)
    (local $l0 i32) (local $l1 i32)
    block $B0
      get_local $p0
      i32.const 1
      get_local $p0
      select
      tee_local $l0
      call $f62
      tee_local $p0
      br_if $B0
      loop $L1
        i32.const 0
        set_local $p0
        i32.const 9724
        i32.load
        tee_local $l1
        i32.eqz
        br_if $B0
        get_local $l1
        call_indirect (type $t6)
        get_local $l0
        call $f62
        tee_local $p0
        i32.eqz
        br_if $L1
      end
    end
    get_local $p0)
  (func $_Znaj (type $t17) (param $p0 i32) (result i32)
    get_local $p0
    call $_Znwj)
  (func $_ZdlPv (type $t16) (param $p0 i32)
    get_local $p0
    if $I0
      get_local $p0
      call $f65
    end)
  (func $_ZdaPv (type $t16) (param $p0 i32)
    get_local $p0
    call $_ZdlPv)
  (func $f59 (type $t16) (param $p0 i32)
    call $env.abort
    unreachable)
  (func $f60 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32)
    block $B0
      get_local $p0
      i64.const 0
      i64.store align=4
      get_local $p0
      i32.const 8
      i32.add
      tee_local $l0
      i32.const 0
      i32.store
      get_local $p1
      i32.load8_u
      i32.const 1
      i32.and
      i32.eqz
      if $I1
        get_local $p0
        get_local $p1
        i64.load align=4
        i64.store align=4
        get_local $l0
        get_local $p1
        i32.const 8
        i32.add
        i32.load
        i32.store
        get_local $p0
        return
      end
      get_local $p1
      i32.load offset=4
      tee_local $l0
      i32.const -16
      i32.lt_u
      if $I2
        get_local $p1
        i32.load offset=8
        set_local $l1
        block $B3
          get_local $l0
          i32.const 11
          i32.lt_u
          if $I4
            get_local $p0
            get_local $l0
            i32.const 1
            i32.shl
            i32.store8
            get_local $p0
            i32.const 1
            i32.add
            set_local $p1
            get_local $l0
            br_if $B3
            br $B0
          end
          get_local $l0
          i32.const 16
          i32.add
          i32.const -16
          i32.and
          tee_local $l2
          call $_Znwj
          set_local $p1
          get_local $p0
          get_local $l2
          i32.const 1
          i32.or
          i32.store
          get_local $p0
          get_local $p1
          i32.store offset=8
          get_local $p0
          get_local $l0
          i32.store offset=4
        end
        get_local $p1
        get_local $l1
        get_local $l0
        call $env.memcpy
        drop
        br $B0
      end
      call $env.abort
      unreachable
    end
    get_local $p1
    get_local $l0
    i32.add
    i32.const 0
    i32.store8
    get_local $p0)
  (func $f61 (type $t8) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
    block $B0
      block $B1
        block $B2
          get_local $p1
          i32.const -16
          i32.lt_u
          if $I3
            block $B4 (result i32)
              get_local $p0
              i32.load8_u
              tee_local $l0
              i32.const 1
              i32.and
              i32.eqz
              if $I5
                get_local $l0
                i32.const 1
                i32.shr_u
                set_local $l3
                i32.const 10
                br $B4
              end
              get_local $p0
              i32.load offset=4
              set_local $l3
              get_local $p0
              i32.load
              tee_local $l0
              i32.const -2
              i32.and
              i32.const -1
              i32.add
            end
            set_local $l1
            i32.const 10
            set_local $l2
            get_local $l3
            get_local $p1
            get_local $l3
            get_local $p1
            i32.gt_u
            select
            tee_local $p1
            i32.const 11
            i32.ge_u
            if $I6
              get_local $p1
              i32.const 16
              i32.add
              i32.const -16
              i32.and
              i32.const -1
              i32.add
              set_local $l2
            end
            block $B7
              block $B8
                get_local $l2
                get_local $l1
                i32.ne
                if $I9
                  get_local $l2
                  i32.const 10
                  i32.eq
                  if $I10
                    i32.const 1
                    set_local $l4
                    get_local $p0
                    i32.const 1
                    i32.add
                    set_local $p1
                    get_local $p0
                    i32.load offset=8
                    set_local $l1
                    get_local $l0
                    i32.const 1
                    i32.and
                    br_if $B7
                    br $B2
                  end
                  get_local $l2
                  get_local $l1
                  i32.gt_u
                  get_local $l2
                  i32.const 1
                  i32.add
                  call $_Znwj
                  tee_local $p1
                  i32.or
                  br_if $B8
                end
                return
              end
              get_local $p0
              i32.load8_u
              tee_local $l0
              i32.const 1
              i32.and
              i32.eqz
              if $I11
                i32.const 1
                set_local $l5
                get_local $p0
                i32.const 1
                i32.add
                set_local $l1
                get_local $l0
                i32.const 1
                i32.and
                i32.eqz
                br_if $B2
                br $B7
              end
              get_local $p0
              i32.load offset=8
              set_local $l1
              i32.const 1
              set_local $l4
              i32.const 1
              set_local $l5
              get_local $l0
              i32.const 1
              i32.and
              i32.eqz
              br_if $B2
            end
            get_local $p0
            i32.load offset=4
            i32.const 1
            i32.add
            tee_local $l0
            i32.eqz
            br_if $B0
            br $B1
          end
          call $env.abort
          unreachable
        end
        get_local $l0
        i32.const 254
        i32.and
        i32.const 1
        i32.shr_u
        i32.const 1
        i32.add
        tee_local $l0
        i32.eqz
        br_if $B0
      end
      get_local $p1
      get_local $l1
      get_local $l0
      call $env.memcpy
      drop
    end
    get_local $l4
    if $I12
      get_local $l1
      call $_ZdlPv
    end
    get_local $l5
    if $I13
      get_local $p0
      get_local $l3
      i32.store offset=4
      get_local $p0
      get_local $p1
      i32.store offset=8
      get_local $p0
      get_local $l2
      i32.const 1
      i32.add
      i32.const 1
      i32.or
      i32.store
      return
    end
    get_local $p0
    get_local $l3
    i32.const 1
    i32.shl
    i32.store8)
  (func $f62 (type $t17) (param $p0 i32) (result i32)
    i32.const 9736
    get_local $p0
    call $f63)
  (func $f63 (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
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
              call $f64
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
            i32.const 8227
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
  (func $f64 (type $t17) (param $p0 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32)
    get_local $p0
    i32.load offset=8388
    set_local $l4
    block $B0
      i32.const 9728
      i32.load8_u
      if $I1
        i32.const 9732
        i32.load
        set_local $l0
        br $B0
      end
      memory.size
      set_local $l0
      i32.const 9728
      i32.const 1
      i32.store8
      i32.const 9732
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
            i32.const 9732
            i32.load
            set_local $l1
          end
          i32.const 0
          set_local $l2
          i32.const 9732
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
          i32.const 9728
          i32.load8_u
          i32.eqz
          if $I8
            memory.size
            set_local $l1
            i32.const 9728
            i32.const 1
            i32.store8
            i32.const 9732
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
            i32.const 9732
            i32.load
            set_local $l5
          end
          i32.const 9732
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
  (func $f65 (type $t16) (param $p0 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32)
    block $B0
      block $B1
        get_local $p0
        i32.eqz
        br_if $B1
        i32.const 18120
        i32.load
        tee_local $l1
        i32.const 1
        i32.lt_s
        br_if $B1
        i32.const 17928
        set_local $l0
        get_local $l1
        i32.const 12
        i32.mul
        i32.const 17928
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
  (table $T0 7 7 anyfunc)
  (memory $memory 1)
  (global $g0 (mut i32) (i32.const 8192))
  (global $__heap_base i32 (i32.const 18132))
  (global $__data_end i32 (i32.const 18132))
  (export "memory" (memory 0))
  (export "__heap_base" (global 1))
  (export "__data_end" (global 2))
  (export "apply" (func $apply))
  (export "_Znwj" (func $_Znwj))
  (export "_ZdlPv" (func $_ZdlPv))
  (export "_Znaj" (func $_Znaj))
  (export "_ZdaPv" (func $_ZdaPv))
  (elem (i32.const 1) $f29 $f17 $f31 $f26 $f33 $f22)
  (data (i32.const 8192) "invalid symbol name")
  (data (i32.const 8212) "invalid supply\00malloc_from_freed was designed to only be called after _heap was completely allocated")
  (data (i32.const 8313) "max-supply must be positive")
  (data (i32.const 8341) "token with symbol already exists")
  (data (i32.const 8374) "memo has more than 256 bytes")
  (data (i32.const 8403) "token with symbol does not exist, create token before issue")
  (data (i32.const 8463) "invalid quantity")
  (data (i32.const 8480) "must issue positive quantity")
  (data (i32.const 8509) "symbol precision mismatch")
  (data (i32.const 8535) "quantity exceeds available supply")
  (data (i32.const 8569) "token with symbol does not exist")
  (data (i32.const 8602) "must retire positive quantity")
  (data (i32.const 8632) "cannot transfer to self")
  (data (i32.const 8656) "to account does not exist")
  (data (i32.const 8682) "unable to find key")
  (data (i32.const 8701) "must transfer positive quantity")
  (data (i32.const 8733) "no balance object found")
  (data (i32.const 8757) "overdrawn balance")
  (data (i32.const 8775) "Balance row already deleted or never existed. Action won't have any effect.")
  (data (i32.const 8851) "Cannot close because the balance is not zero.")
  (data (i32.const 8897) "onerror action's are only valid from the \22eosio\22 system account")
  (data (i32.const 8961) "object passed to iterator_to is not in multi_index")
  (data (i32.const 9012) "error reading iterator")
  (data (i32.const 9035) "magnitude of asset amount must be less than 2^62")
  (data (i32.const 9084) "read")
  (data (i32.const 9089) "cannot create objects in table of another contract")
  (data (i32.const 9140) "write")
  (data (i32.const 9146) "object passed to modify is not in multi_index")
  (data (i32.const 9192) "cannot modify objects in table of another contract")
  (data (i32.const 9243) "updater cannot change primary key when modifying an object")
  (data (i32.const 9302) "attempt to add asset with different symbol")
  (data (i32.const 9345) "addition underflow")
  (data (i32.const 9364) "addition overflow")
  (data (i32.const 9382) "attempt to subtract asset with different symbol")
  (data (i32.const 9430) "subtraction underflow")
  (data (i32.const 9452) "subtraction overflow")
  (data (i32.const 9473) "cannot pass end iterator to modify")
  (data (i32.const 9508) "cannot pass end iterator to erase")
  (data (i32.const 9542) "cannot increment end iterator")
  (data (i32.const 9572) "object passed to erase is not in multi_index")
  (data (i32.const 9617) "cannot erase objects in table of another contract")
  (data (i32.const 9667) "attempt to remove object that was not in multi_index")
  (data (i32.const 9720) "get"))
