(module
  (type $t0 (func (param i32 i32 i32 i32)))
  (type $t1 (func (param i32 i32 i32)))
  (type $t2 (func (param i32 i32 i32 i32 i32 i32)))
  (type $t3 (func))
  (type $t4 (func (param i32 i32 i32) (result i32)))
  (type $t5 (func (param i64)))
  (type $t6 (func (param i32 i32)))
  (type $t7 (func (param i64 i64 i64 i64) (result i32)))
  (type $t8 (func (result i64)))
  (type $t9 (func (param i32 i32) (result i32)))
  (type $t10 (func (result i32)))
  (type $t11 (func (param i64 i64 i64 i64 i32 i32) (result i32)))
  (type $t12 (func (param i32 i64 i32 i32)))
  (type $t13 (func (param i32)))
  (type $t14 (func (param i32) (result i64)))
  (type $t15 (func (param i32 i32 i32 i32) (result i32)))
  (type $t16 (func (param i32) (result i32)))
  (type $t17 (func (param i32 i32 i64)))
  (type $t18 (func (param i32 i32 i64 i32)))
  (type $t19 (func (param i64 i64 i64)))
  (import "env" "abort" (func $env.abort (type $t3)))
  (import "env" "memmove" (func $env.memmove (type $t4)))
  (import "env" "memcpy" (func $env.memcpy (type $t4)))
  (import "env" "require_auth" (func $env.require_auth (type $t5)))
  (import "env" "eosio_assert" (func $env.eosio_assert (type $t6)))
  (import "env" "db_find_i64" (func $env.db_find_i64 (type $t7)))
  (import "env" "current_receiver" (func $env.current_receiver (type $t8)))
  (import "env" "db_next_i64" (func $env.db_next_i64 (type $t9)))
  (import "env" "action_data_size" (func $env.action_data_size (type $t10)))
  (import "env" "read_action_data" (func $env.read_action_data (type $t9)))
  (import "env" "db_get_i64" (func $env.db_get_i64 (type $t4)))
  (import "env" "db_store_i64" (func $env.db_store_i64 (type $t11)))
  (import "env" "db_update_i64" (func $env.db_update_i64 (type $t12)))
  (import "env" "db_remove_i64" (func $env.db_remove_i64 (type $t13)))
  (func $f14 (type $t14) (param $p0 i32) (result i64)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32) (local $l14 i32) (local $l15 i64)
    get_global $g0
    i32.const 16
    i32.sub
    tee_local $l2
    set_global $g0
    i32.const 3
    set_local $l7
    i32.const 12
    call $_Znwj
    tee_local $l3
    i32.const 3
    i32.store offset=8
    get_local $l3
    i64.const 12884901891
    i64.store align=4
    i32.const 12
    call $_Znwj
    tee_local $l4
    i32.const 3
    i32.store offset=8
    get_local $l4
    i64.const 12884901891
    i64.store align=4
    i32.const 1
    set_local $l8
    i32.const 3
    set_local $l9
    get_local $p0
    i32.const 36
    i32.add
    i32.load
    tee_local $l0
    get_local $p0
    i32.load offset=32
    tee_local $l12
    i32.ne
    if $I0
      get_local $l0
      get_local $l12
      i32.sub
      set_local $l13
      i32.const 0
      set_local $l0
      get_local $l4
      set_local $l6
      loop $L1
        get_local $l6
        get_local $l0
        i32.const 3
        i32.div_u
        tee_local $l5
        i32.const -12
        i32.mul
        i32.add
        tee_local $l1
        get_local $l1
        i32.load
        get_local $l12
        get_local $l0
        i32.add
        tee_local $l10
        i32.load8_u
        tee_local $l11
        i32.and
        i32.store
        get_local $l3
        get_local $l5
        i32.const 65535
        i32.and
        tee_local $l1
        i32.const 2
        i32.shl
        i32.add
        tee_local $l14
        get_local $l14
        i32.load
        get_local $l10
        i32.load8_u
        i32.and
        i32.store
        get_local $l0
        get_local $l5
        i32.const 3
        i32.mul
        i32.sub
        tee_local $l5
        get_local $l1
        i32.eq
        if $I2
          get_local $l9
          get_local $l10
          i32.load8_u
          i32.and
          set_local $l9
        end
        get_local $l11
        i32.eqz
        set_local $l11
        get_local $l1
        get_local $l5
        i32.add
        i32.const 2
        i32.eq
        if $I3
          get_local $l7
          get_local $l10
          i32.load8_u
          i32.and
          set_local $l7
        end
        get_local $l8
        get_local $l11
        i32.and
        set_local $l8
        get_local $l6
        i32.const 4
        i32.add
        set_local $l6
        get_local $l0
        i32.const 1
        i32.add
        tee_local $l0
        get_local $l13
        i32.lt_u
        br_if $L1
      end
    end
    i32.const 8
    call $_Znwj
    tee_local $l0
    get_local $l9
    i32.store
    get_local $l0
    get_local $l7
    i32.store offset=4
    get_local $l2
    get_local $l0
    i32.store
    get_local $l2
    get_local $l0
    i32.const 8
    i32.add
    tee_local $l0
    i32.store offset=8
    get_local $l2
    get_local $l0
    i32.store offset=4
    get_local $l2
    get_local $l0
    get_local $l3
    get_local $l3
    i32.const 12
    i32.add
    call $f15
    drop
    get_local $l2
    get_local $l2
    i32.load offset=4
    get_local $l4
    get_local $l4
    i32.const 12
    i32.add
    call $f15
    drop
    block $B4
      block $B5
        block $B6
          block $B7
            get_local $l2
            i32.load
            tee_local $l1
            get_local $l2
            i32.load offset=4
            tee_local $l5
            i32.ne
            if $I8
              get_local $l1
              set_local $l0
              loop $L9
                get_local $l0
                i32.load
                tee_local $l6
                i32.const 2
                i32.eq
                br_if $B7
                get_local $l6
                i32.const 1
                i32.eq
                br_if $B6
                get_local $l5
                get_local $l0
                i32.const 4
                i32.add
                tee_local $l0
                i32.ne
                br_if $L9
              end
            end
            i64.const 4
            set_local $l15
            loop $L10
              get_local $l15
              i64.const 1
              i64.add
              tee_local $l15
              i64.const 13
              i64.ne
              br_if $L10
            end
            i64.const 4
            set_local $l15
            loop $L11
              get_local $l15
              i64.const 1
              i64.add
              tee_local $l15
              i64.const 13
              i64.ne
              br_if $L11
            end
            i64.const 5606348217378668544
            i64.const -7122829838779416576
            get_local $l8
            select
            set_local $l15
            get_local $l1
            i32.eqz
            br_if $B4
            br $B5
          end
          get_local $p0
          i64.load
          set_local $l15
          get_local $l1
          br_if $B5
          br $B4
        end
        get_local $p0
        i64.load offset=8
        set_local $l15
        get_local $l1
        i32.eqz
        br_if $B4
      end
      get_local $l2
      get_local $l1
      i32.store offset=4
      get_local $l1
      call $_ZdlPv
    end
    get_local $l4
    if $I12
      get_local $l4
      call $_ZdlPv
    end
    get_local $l3
    if $I13
      get_local $l3
      call $_ZdlPv
    end
    get_local $l2
    i32.const 16
    i32.add
    set_global $g0
    get_local $l15)
  (func $f15 (type $t15) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
    block $B0
      get_local $p3
      get_local $p2
      i32.sub
      tee_local $l0
      i32.const 1
      i32.lt_s
      br_if $B0
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                get_local $l0
                i32.const 2
                i32.shr_s
                tee_local $l3
                get_local $p0
                i32.load offset=8
                tee_local $l0
                get_local $p0
                i32.load offset=4
                tee_local $l1
                i32.sub
                i32.const 2
                i32.shr_s
                i32.gt_s
                if $I6
                  get_local $l1
                  get_local $p0
                  i32.load
                  tee_local $l2
                  i32.sub
                  i32.const 2
                  i32.shr_s
                  get_local $l3
                  i32.add
                  tee_local $l1
                  i32.const 1073741824
                  i32.ge_u
                  br_if $B2
                  get_local $l0
                  get_local $l2
                  i32.sub
                  tee_local $l3
                  i32.const 2
                  i32.shr_s
                  i32.const 536870911
                  i32.ge_u
                  br_if $B5
                  get_local $p1
                  get_local $l2
                  i32.sub
                  i32.const 2
                  i32.shr_s
                  set_local $l0
                  get_local $l1
                  get_local $l3
                  i32.const 1
                  i32.shr_s
                  tee_local $l2
                  get_local $l2
                  get_local $l1
                  i32.lt_u
                  select
                  tee_local $l1
                  i32.eqz
                  br_if $B3
                  get_local $l1
                  i32.const 1073741824
                  i32.lt_u
                  br_if $B4
                  call $env.abort
                  unreachable
                end
                block $B7
                  get_local $l3
                  get_local $l1
                  get_local $p1
                  i32.sub
                  tee_local $l5
                  i32.const 2
                  i32.shr_s
                  tee_local $l0
                  i32.gt_s
                  if $I8
                    get_local $l1
                    set_local $l2
                    get_local $p2
                    get_local $l0
                    i32.const 2
                    i32.shl
                    i32.add
                    tee_local $l4
                    get_local $p3
                    i32.ne
                    if $I9
                      get_local $p3
                      i32.const -4
                      i32.add
                      get_local $l4
                      i32.sub
                      set_local $l6
                      get_local $l1
                      set_local $l0
                      get_local $l4
                      set_local $l2
                      loop $L10
                        get_local $l0
                        get_local $l2
                        i32.load
                        i32.store
                        get_local $l0
                        i32.const 4
                        i32.add
                        set_local $l0
                        get_local $p3
                        get_local $l2
                        i32.const 4
                        i32.add
                        tee_local $l2
                        i32.ne
                        br_if $L10
                      end
                      get_local $p0
                      i32.const 4
                      i32.add
                      get_local $l1
                      get_local $l6
                      i32.const -4
                      i32.and
                      i32.add
                      i32.const 4
                      i32.add
                      tee_local $l2
                      i32.store
                    end
                    get_local $l5
                    i32.const 1
                    i32.ge_s
                    br_if $B7
                    br $B0
                  end
                  get_local $l1
                  set_local $l2
                  get_local $p3
                  set_local $l4
                end
                get_local $l2
                get_local $p1
                get_local $l3
                i32.const 2
                i32.shl
                tee_local $l0
                i32.add
                i32.sub
                tee_local $l3
                i32.const 2
                i32.shr_s
                set_local $l5
                get_local $l2
                get_local $l0
                i32.sub
                tee_local $l0
                get_local $l1
                i32.lt_u
                if $I11
                  get_local $l1
                  get_local $l5
                  i32.const 2
                  i32.shl
                  i32.sub
                  get_local $p1
                  i32.const -1
                  i32.xor
                  i32.add
                  i32.const 2
                  i32.shr_u
                  set_local $l6
                  get_local $l2
                  set_local $p3
                  loop $L12
                    get_local $p3
                    get_local $l0
                    i32.load
                    i32.store
                    get_local $p3
                    i32.const 4
                    i32.add
                    set_local $p3
                    get_local $l0
                    i32.const 4
                    i32.add
                    tee_local $l0
                    get_local $l1
                    i32.lt_u
                    br_if $L12
                  end
                  get_local $p0
                  i32.const 4
                  i32.add
                  get_local $l2
                  get_local $l6
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.const 4
                  i32.add
                  i32.store
                end
                get_local $l3
                if $I13
                  get_local $l2
                  get_local $l5
                  i32.const 2
                  i32.shl
                  i32.sub
                  get_local $p1
                  get_local $l3
                  call $env.memmove
                  drop
                end
                get_local $l4
                get_local $p2
                i32.sub
                tee_local $l0
                i32.eqz
                br_if $B0
                get_local $p1
                get_local $p2
                get_local $l0
                call $env.memmove
                drop
                get_local $p1
                return
              end
              get_local $p1
              get_local $l2
              i32.sub
              i32.const 2
              i32.shr_s
              set_local $l0
              i32.const 1073741823
              set_local $l1
            end
            get_local $l1
            i32.const 2
            i32.shl
            call $_Znwj
            set_local $l3
            br $B1
          end
          i32.const 0
          set_local $l1
          i32.const 0
          set_local $l3
          br $B1
        end
        get_local $p0
        call $f42
        unreachable
      end
      get_local $l3
      get_local $l0
      i32.const 2
      i32.shl
      i32.add
      tee_local $l2
      set_local $l0
      get_local $p2
      get_local $p3
      i32.ne
      if $I14
        get_local $p3
        i32.const -4
        i32.add
        get_local $p2
        i32.sub
        i32.const 2
        i32.shr_u
        set_local $l4
        get_local $l2
        set_local $l0
        loop $L15
          get_local $l0
          get_local $p2
          i32.load
          i32.store
          get_local $l0
          i32.const 4
          i32.add
          set_local $l0
          get_local $p3
          get_local $p2
          i32.const 4
          i32.add
          tee_local $p2
          i32.ne
          br_if $L15
        end
        get_local $l2
        get_local $l4
        i32.const 2
        i32.shl
        i32.add
        i32.const 4
        i32.add
        set_local $l0
      end
      get_local $l1
      i32.const 2
      i32.shl
      set_local $l1
      get_local $l2
      get_local $p1
      get_local $p0
      i32.load
      tee_local $l4
      i32.sub
      tee_local $p2
      i32.sub
      set_local $p3
      get_local $p2
      i32.const 1
      i32.ge_s
      if $I16
        get_local $p3
        get_local $l4
        get_local $p2
        call $env.memcpy
        drop
      end
      get_local $l3
      get_local $l1
      i32.add
      set_local $l1
      get_local $p0
      i32.const 4
      i32.add
      tee_local $l3
      i32.load
      get_local $p1
      i32.sub
      tee_local $p2
      i32.const 1
      i32.ge_s
      if $I17
        get_local $l0
        get_local $p1
        get_local $p2
        call $env.memcpy
        drop
        get_local $l0
        get_local $p2
        i32.add
        set_local $l0
      end
      get_local $l3
      get_local $l0
      i32.store
      get_local $p0
      i32.load
      set_local $p2
      get_local $p0
      get_local $p3
      i32.store
      get_local $p0
      i32.const 8
      i32.add
      get_local $l1
      i32.store
      get_local $p2
      if $I18
        get_local $p2
        call $_ZdlPv
      end
      get_local $l2
      set_local $p1
    end
    get_local $p1)
  (func $f16 (type $t1) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i64) (local $l4 i64)
    get_global $g0
    i32.const 96
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $p2
    i64.load
    call $env.require_auth
    get_local $p1
    i64.load
    get_local $p2
    i64.load
    i64.ne
    i32.const 8192
    call $env.eosio_assert
    get_local $l0
    i32.const 48
    i32.add
    tee_local $l2
    i32.const 0
    i32.store
    get_local $l0
    i64.const -1
    i64.store offset=32
    get_local $l0
    i64.const 0
    i64.store offset=40
    get_local $l0
    get_local $p0
    i64.load
    tee_local $l3
    i64.store offset=16
    get_local $l0
    get_local $p2
    i64.load
    tee_local $l4
    i64.store offset=24
    block $B0
      get_local $l3
      get_local $l4
      i64.const 7035937633859534848
      get_local $p1
      i64.load
      call $env.db_find_i64
      tee_local $p0
      i32.const 0
      i32.ge_s
      if $I1
        get_local $l0
        i32.const 16
        i32.add
        get_local $p0
        call $f17
        i32.load offset=44
        get_local $l0
        i32.const 16
        i32.add
        i32.eq
        i32.const 8426
        call $env.eosio_assert
        br $B0
      end
      i32.const 1
      set_local $l1
    end
    get_local $l1
    i32.const 8233
    call $env.eosio_assert
    get_local $p2
    i64.load
    set_local $l3
    get_local $l0
    get_local $p2
    i32.store offset=12
    get_local $l0
    get_local $p1
    i32.store offset=8
    get_local $l0
    get_local $l3
    i64.store offset=88
    get_local $l0
    i64.load offset=16
    call $env.current_receiver
    i64.eq
    i32.const 8509
    call $env.eosio_assert
    get_local $l0
    get_local $l0
    i32.const 8
    i32.add
    i32.store offset=68
    get_local $l0
    get_local $l0
    i32.const 16
    i32.add
    i32.store offset=64
    get_local $l0
    get_local $l0
    i32.const 88
    i32.add
    i32.store offset=72
    i32.const 56
    call $_Znwj
    tee_local $p2
    call $f18
    drop
    get_local $p2
    get_local $l0
    i32.const 16
    i32.add
    i32.store offset=44
    get_local $l0
    i32.const -64
    i32.sub
    get_local $p2
    call $f19
    get_local $l0
    get_local $p2
    i32.store offset=80
    get_local $l0
    get_local $p2
    i64.load
    tee_local $l3
    i64.store offset=64
    get_local $l0
    get_local $p2
    i32.load offset=48
    tee_local $l1
    i32.store offset=60
    block $B2
      block $B3
        block $B4
          get_local $l0
          i32.const 44
          i32.add
          tee_local $p0
          i32.load
          tee_local $p1
          get_local $l2
          i32.load
          i32.lt_u
          if $I5
            get_local $p1
            get_local $l3
            i64.store offset=8
            get_local $p1
            get_local $l1
            i32.store offset=16
            get_local $l0
            i32.const 0
            i32.store offset=80
            get_local $p1
            get_local $p2
            i32.store
            get_local $p0
            get_local $p1
            i32.const 24
            i32.add
            i32.store
            get_local $l0
            i32.load offset=80
            set_local $p2
            get_local $l0
            i32.const 0
            i32.store offset=80
            get_local $p2
            i32.eqz
            br_if $B3
            br $B4
          end
          get_local $l0
          i32.const 40
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
          call $f20
          get_local $l0
          i32.load offset=80
          set_local $p2
          get_local $l0
          i32.const 0
          i32.store offset=80
          get_local $p2
          br_if $B4
          br $B3
        end
        get_local $p2
        i32.load offset=32
        tee_local $p1
        if $I6
          get_local $p2
          i32.const 36
          i32.add
          get_local $p1
          i32.store
          get_local $p1
          call $_ZdlPv
        end
        get_local $p2
        call $_ZdlPv
      end
      get_local $l0
      i32.load offset=40
      tee_local $p0
      i32.eqz
      br_if $B2
      block $B7 (result i32)
        get_local $l0
        i32.const 44
        i32.add
        tee_local $l2
        i32.load
        tee_local $p2
        get_local $p0
        i32.ne
        if $I8
          loop $L9
            get_local $p2
            i32.const -24
            i32.add
            tee_local $p2
            i32.load
            set_local $p1
            get_local $p2
            i32.const 0
            i32.store
            get_local $p1
            if $I10
              get_local $p1
              i32.load offset=32
              tee_local $l1
              if $I11
                get_local $p1
                i32.const 36
                i32.add
                get_local $l1
                i32.store
                get_local $l1
                call $_ZdlPv
              end
              get_local $p1
              call $_ZdlPv
            end
            get_local $p0
            get_local $p2
            i32.ne
            br_if $L9
          end
          get_local $l0
          i32.const 40
          i32.add
          i32.load
          br $B7
        end
        get_local $p0
      end
      get_local $l2
      get_local $p0
      i32.store
      call $_ZdlPv
    end
    get_local $l0
    i32.const 96
    i32.add
    set_global $g0)
  (func $f17 (type $t9) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i64)
    block $B0
      get_global $g0
      i32.const 48
      i32.sub
      tee_local $l3
      set_local $l2
      get_local $l3
      set_global $g0
      block $B1
        get_local $p0
        i32.load offset=24
        tee_local $l1
        get_local $p0
        i32.const 28
        i32.add
        i32.load
        tee_local $l0
        i32.eq
        br_if $B1
        block $B2
          loop $L3
            get_local $l0
            i32.const -8
            i32.add
            i32.load
            get_local $p1
            i32.eq
            br_if $B2
            get_local $l1
            get_local $l0
            i32.const -24
            i32.add
            tee_local $l0
            i32.ne
            br_if $L3
          end
          br $B1
        end
        get_local $l1
        get_local $l0
        i32.eq
        br_if $B1
        get_local $l0
        i32.const -24
        i32.add
        i32.load
        set_local $l0
        br $B0
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
      i32.const 8477
      call $env.eosio_assert
      block $B4
        get_local $l0
        i32.const 513
        i32.ge_u
        if $I5
          get_local $l0
          call $f43
          set_local $l1
          br $B4
        end
        get_local $l3
        get_local $l0
        i32.const 15
        i32.add
        i32.const -16
        i32.and
        i32.sub
        tee_local $l1
        set_global $g0
      end
      get_local $p1
      get_local $l1
      get_local $l0
      call $env.db_get_i64
      drop
      get_local $l2
      get_local $l1
      i32.store offset=36
      get_local $l2
      get_local $l1
      i32.store offset=32
      get_local $l2
      get_local $l1
      get_local $l0
      i32.add
      i32.store offset=40
      get_local $l0
      i32.const 513
      i32.ge_u
      if $I6
        get_local $l1
        call $f46
      end
      i32.const 56
      call $_Znwj
      tee_local $l0
      call $f18
      set_local $l1
      get_local $l0
      get_local $p0
      i32.store offset=44
      get_local $l2
      i32.const 32
      i32.add
      get_local $l1
      call $f32
      drop
      get_local $l0
      get_local $p1
      i32.store offset=48
      get_local $l2
      get_local $l0
      i32.store offset=24
      get_local $l2
      get_local $l0
      i64.load
      tee_local $l4
      i64.store offset=16
      get_local $l2
      get_local $p1
      i32.store offset=12
      block $B7
        get_local $p0
        i32.const 28
        i32.add
        tee_local $l3
        i32.load
        tee_local $l1
        get_local $p0
        i32.const 32
        i32.add
        i32.load
        i32.lt_u
        if $I8
          get_local $l1
          get_local $l4
          i64.store offset=8
          get_local $l1
          get_local $p1
          i32.store offset=16
          get_local $l2
          i32.const 0
          i32.store offset=24
          get_local $l1
          get_local $l0
          i32.store
          get_local $l3
          get_local $l1
          i32.const 24
          i32.add
          i32.store
          get_local $l2
          i32.load offset=24
          set_local $p1
          get_local $l2
          i32.const 0
          i32.store offset=24
          get_local $p1
          i32.eqz
          br_if $B0
          br $B7
        end
        get_local $p0
        i32.const 24
        i32.add
        get_local $l2
        i32.const 24
        i32.add
        get_local $l2
        i32.const 16
        i32.add
        get_local $l2
        i32.const 12
        i32.add
        call $f20
        get_local $l2
        i32.load offset=24
        set_local $p1
        get_local $l2
        i32.const 0
        i32.store offset=24
        get_local $p1
        br_if $B7
        br $B0
      end
      get_local $p1
      i32.load offset=32
      tee_local $l1
      if $I9
        get_local $p1
        i32.const 36
        i32.add
        get_local $l1
        i32.store
        get_local $l1
        call $_ZdlPv
      end
      get_local $p1
      call $_ZdlPv
      get_local $l2
      i32.const 48
      i32.add
      set_global $g0
      get_local $l0
      return
    end
    get_local $l2
    i32.const 48
    i32.add
    set_global $g0
    get_local $l0)
  (func $f18 (type $t16) (param $p0 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i64)
    i64.const 4
    set_local $l3
    loop $L0
      get_local $l3
      i64.const 1
      i64.add
      tee_local $l3
      i64.const 13
      i64.ne
      br_if $L0
    end
    get_local $p0
    i64.const 0
    i64.store offset=32 align=4
    get_local $p0
    i32.const 24
    i32.add
    i64.const -7122829838779416576
    i64.store
    get_local $p0
    i32.const 40
    i32.add
    tee_local $l2
    i32.const 0
    i32.store
    i32.const 9
    call $_Znwj
    tee_local $l1
    i32.const 0
    i32.store8 offset=8
    get_local $l1
    i64.const 0
    i64.store align=1
    get_local $p0
    i32.load offset=32
    tee_local $l0
    if $I1
      get_local $p0
      i32.const 36
      i32.add
      get_local $l0
      i32.store
      get_local $l0
      call $_ZdlPv
      get_local $l2
      i32.const 0
      i32.store
      get_local $p0
      i32.const 32
      i32.add
      i64.const 0
      i64.store align=4
    end
    get_local $p0
    i32.const 36
    i32.add
    get_local $l1
    i32.const 9
    i32.add
    tee_local $l0
    i32.store
    get_local $p0
    i32.const 32
    i32.add
    get_local $l1
    i32.store
    get_local $l2
    get_local $l0
    i32.store
    get_local $p0)
  (func $f19 (type $t6) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i64)
    get_global $g0
    i32.const 16
    i32.sub
    tee_local $l4
    set_local $l2
    get_local $l4
    set_global $g0
    get_local $p1
    get_local $p0
    i32.load offset=4
    tee_local $l0
    i32.load
    i64.load
    i64.store
    get_local $p1
    get_local $l0
    i32.const 4
    i32.add
    i32.load
    tee_local $l0
    i64.load
    i64.store offset=8
    get_local $p1
    get_local $l0
    i64.load
    i64.store offset=16
    get_local $p1
    i32.const 36
    i32.add
    i32.load
    tee_local $l1
    get_local $p1
    i32.load offset=32
    tee_local $l5
    i32.sub
    tee_local $l6
    i64.extend_u/i32
    set_local $l7
    get_local $p0
    i32.load
    set_local $l3
    i32.const 32
    set_local $l0
    loop $L0
      get_local $l0
      i32.const 1
      i32.add
      set_local $l0
      get_local $l7
      i64.const 7
      i64.shr_u
      tee_local $l7
      i64.const 0
      i64.ne
      br_if $L0
    end
    block $B1
      get_local $l0
      get_local $l6
      get_local $l0
      i32.add
      get_local $l5
      get_local $l1
      i32.eq
      select
      tee_local $l1
      i32.const 513
      i32.ge_u
      if $I2
        get_local $l1
        call $f43
        set_local $l0
        br $B1
      end
      get_local $l4
      get_local $l1
      i32.const 15
      i32.add
      i32.const -16
      i32.and
      i32.sub
      tee_local $l0
      set_global $g0
    end
    get_local $l2
    get_local $l0
    i32.store offset=4
    get_local $l2
    get_local $l0
    i32.store
    get_local $l2
    get_local $l0
    get_local $l1
    i32.add
    i32.store offset=8
    get_local $l2
    get_local $p1
    call $f35
    drop
    get_local $p1
    get_local $l3
    i64.load offset=8
    i64.const 7035937633859534848
    get_local $p0
    i32.load offset=8
    i64.load
    get_local $p1
    i64.load
    tee_local $l7
    get_local $l0
    get_local $l1
    call $env.db_store_i64
    i32.store offset=48
    block $B3
      block $B4
        get_local $l1
        i32.const 513
        i32.lt_u
        if $I5
          get_local $l7
          get_local $l3
          i64.load offset=16
          i64.ge_u
          br_if $B4
          br $B3
        end
        get_local $l0
        call $f46
        get_local $l7
        get_local $l3
        i64.load offset=16
        i64.lt_u
        br_if $B3
      end
      get_local $l3
      i32.const 16
      i32.add
      i64.const -2
      get_local $l7
      i64.const 1
      i64.add
      get_local $l7
      i64.const -3
      i64.gt_u
      select
      i64.store
      get_local $l2
      i32.const 16
      i32.add
      set_global $g0
      return
    end
    get_local $l2
    i32.const 16
    i32.add
    set_global $g0)
  (func $f20 (type $t0) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32)
    block $B0
      get_local $p0
      i32.load offset=4
      get_local $p0
      i32.load
      tee_local $l0
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
        set_local $l1
        block $B2
          get_local $p0
          i32.load offset=8
          get_local $l0
          i32.sub
          i32.const 24
          i32.div_s
          tee_local $l0
          i32.const 89478484
          i32.le_u
          if $I3
            get_local $l2
            get_local $l0
            i32.const 1
            i32.shl
            tee_local $l1
            get_local $l1
            get_local $l2
            i32.lt_u
            select
            tee_local $l1
            i32.eqz
            br_if $B2
          end
          get_local $l1
          i32.const 24
          i32.mul
          call $_Znwj
          set_local $l0
          br $B0
        end
        i32.const 0
        set_local $l1
        i32.const 0
        set_local $l0
        br $B0
      end
      get_local $p0
      call $f42
      unreachable
    end
    get_local $p1
    i32.load
    set_local $l2
    get_local $p1
    i32.const 0
    i32.store
    get_local $l0
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
    get_local $l0
    get_local $l1
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
      tee_local $l1
      i32.ne
      if $I5
        get_local $l0
        get_local $l4
        i32.add
        i32.const -24
        i32.add
        set_local $p1
        loop $L6
          get_local $p2
          i32.const -24
          i32.add
          tee_local $l0
          i32.load
          set_local $p3
          get_local $l0
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
          get_local $l1
          get_local $l0
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
        set_local $l1
        get_local $p0
        i32.load
        br $B4
      end
      get_local $l1
    end
    set_local $l0
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
    get_local $l1
    get_local $l0
    i32.ne
    if $I7
      loop $L8
        get_local $l1
        i32.const -24
        i32.add
        tee_local $l1
        i32.load
        set_local $p1
        get_local $l1
        i32.const 0
        i32.store
        get_local $p1
        if $I9
          get_local $p1
          i32.load offset=32
          tee_local $p2
          if $I10
            get_local $p1
            i32.const 36
            i32.add
            get_local $p2
            i32.store
            get_local $p2
            call $_ZdlPv
          end
          get_local $p1
          call $_ZdlPv
        end
        get_local $l0
        get_local $l1
        i32.ne
        br_if $L8
      end
    end
    get_local $l0
    if $I11
      get_local $l0
      call $_ZdlPv
    end)
  (func $f21 (type $t0) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i64) (local $l3 i64)
    get_global $g0
    i32.const 48
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $p3
    i64.load
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
    tee_local $l2
    i64.store offset=8
    get_local $l0
    get_local $p2
    i64.load
    tee_local $l3
    i64.store offset=16
    i32.const 0
    set_local $p2
    get_local $l2
    get_local $l3
    i64.const 7035937633859534848
    get_local $p1
    i64.load
    call $env.db_find_i64
    tee_local $p1
    i32.const 0
    i32.ge_s
    if $I0
      get_local $l0
      i32.const 8
      i32.add
      get_local $p1
      call $f17
      tee_local $p2
      i32.load offset=44
      get_local $l0
      i32.const 8
      i32.add
      i32.eq
      i32.const 8426
      call $env.eosio_assert
    end
    get_local $p2
    i32.const 0
    i32.ne
    tee_local $p0
    i32.const 8253
    call $env.eosio_assert
    i32.const 1
    set_local $p1
    get_local $p3
    i64.load
    tee_local $l2
    get_local $p2
    i64.load offset=8
    i64.ne
    if $I1
      get_local $l2
      get_local $p2
      i64.load
      i64.eq
      set_local $p1
    end
    get_local $p1
    i32.const 8273
    call $env.eosio_assert
    get_local $p2
    i32.const 8
    i32.add
    i64.load
    set_local $l2
    get_local $p0
    i32.const 8566
    call $env.eosio_assert
    get_local $l0
    i32.const 8
    i32.add
    get_local $p2
    get_local $l2
    call $f22
    get_local $l0
    i32.load offset=32
    tee_local $p0
    if $I2
      block $B3 (result i32)
        get_local $l0
        i32.const 36
        i32.add
        tee_local $l1
        i32.load
        tee_local $p2
        get_local $p0
        i32.ne
        if $I4
          loop $L5
            get_local $p2
            i32.const -24
            i32.add
            tee_local $p2
            i32.load
            set_local $p3
            get_local $p2
            i32.const 0
            i32.store
            get_local $p3
            if $I6
              get_local $p3
              i32.load offset=32
              tee_local $p1
              if $I7
                get_local $p3
                i32.const 36
                i32.add
                get_local $p1
                i32.store
                get_local $p1
                call $_ZdlPv
              end
              get_local $p3
              call $_ZdlPv
            end
            get_local $p0
            get_local $p2
            i32.ne
            br_if $L5
          end
          get_local $l0
          i32.const 32
          i32.add
          i32.load
          br $B3
        end
        get_local $p0
      end
      get_local $l1
      get_local $p0
      i32.store
      call $_ZdlPv
    end
    get_local $l0
    i32.const 48
    i32.add
    set_global $g0)
  (func $f22 (type $t17) (param $p0 i32) (param $p1 i32) (param $p2 i64)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i64) (local $l7 i64)
    get_global $g0
    i32.const 16
    i32.sub
    tee_local $l3
    set_local $l2
    get_local $l3
    set_global $g0
    get_local $p1
    i32.load offset=44
    get_local $p0
    i32.eq
    i32.const 8601
    call $env.eosio_assert
    get_local $p0
    i64.load
    call $env.current_receiver
    i64.eq
    i32.const 8647
    call $env.eosio_assert
    get_local $p1
    i64.load
    set_local $l6
    get_local $p1
    call $f37
    get_local $l6
    get_local $p1
    i64.load
    i64.eq
    i32.const 8698
    call $env.eosio_assert
    get_local $p1
    i32.const 36
    i32.add
    i32.load
    tee_local $l1
    get_local $p1
    i32.load offset=32
    tee_local $l4
    i32.sub
    tee_local $l5
    i64.extend_u/i32
    set_local $l7
    i32.const 32
    set_local $l0
    loop $L0
      get_local $l0
      i32.const 1
      i32.add
      set_local $l0
      get_local $l7
      i64.const 7
      i64.shr_u
      tee_local $l7
      i64.const 0
      i64.ne
      br_if $L0
    end
    block $B1
      get_local $l0
      get_local $l5
      get_local $l0
      i32.add
      get_local $l4
      get_local $l1
      i32.eq
      select
      tee_local $l1
      i32.const 513
      i32.ge_u
      if $I2
        get_local $l1
        call $f43
        set_local $l0
        br $B1
      end
      get_local $l3
      get_local $l1
      i32.const 15
      i32.add
      i32.const -16
      i32.and
      i32.sub
      tee_local $l0
      set_global $g0
    end
    get_local $l2
    get_local $l0
    i32.store offset=4
    get_local $l2
    get_local $l0
    i32.store
    get_local $l2
    get_local $l0
    get_local $l1
    i32.add
    i32.store offset=8
    get_local $l2
    get_local $p1
    call $f35
    drop
    get_local $p1
    i32.load offset=48
    get_local $p2
    get_local $l0
    get_local $l1
    call $env.db_update_i64
    block $B3
      block $B4
        get_local $l1
        i32.const 513
        i32.lt_u
        if $I5
          get_local $l6
          get_local $p0
          i64.load offset=16
          i64.ge_u
          br_if $B4
          br $B3
        end
        get_local $l0
        call $f46
        get_local $l6
        get_local $p0
        i64.load offset=16
        i64.lt_u
        br_if $B3
      end
      get_local $p0
      i32.const 16
      i32.add
      i64.const -2
      get_local $l6
      i64.const 1
      i64.add
      get_local $l6
      i64.const -3
      i64.gt_u
      select
      i64.store
      get_local $l2
      i32.const 16
      i32.add
      set_global $g0
      return
    end
    get_local $l2
    i32.const 16
    i32.add
    set_global $g0)
  (func $f23 (type $t1) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i64) (local $l4 i64)
    get_global $g0
    i32.const 48
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $p2
    i64.load
    call $env.require_auth
    get_local $l0
    i32.const 32
    i32.add
    i32.const 0
    i32.store
    get_local $l0
    i64.const -1
    i64.store offset=16
    get_local $l0
    i64.const 0
    i64.store offset=24
    get_local $l0
    get_local $p0
    i64.load
    tee_local $l3
    i64.store
    get_local $l0
    get_local $p2
    i64.load
    tee_local $l4
    i64.store offset=8
    i32.const 0
    set_local $p2
    get_local $l3
    get_local $l4
    i64.const 7035937633859534848
    get_local $p1
    i64.load
    call $env.db_find_i64
    tee_local $p1
    i32.const 0
    i32.ge_s
    if $I0
      get_local $l0
      get_local $p1
      call $f17
      tee_local $p2
      i32.load offset=44
      get_local $l0
      i32.eq
      i32.const 8426
      call $env.eosio_assert
    end
    get_local $p2
    i32.const 0
    i32.ne
    tee_local $p1
    i32.const 8253
    call $env.eosio_assert
    get_local $p1
    i32.const 8757
    call $env.eosio_assert
    get_local $p1
    i32.const 8791
    call $env.eosio_assert
    get_local $p2
    i32.load offset=48
    get_local $l0
    i32.const 40
    i32.add
    call $env.db_next_i64
    tee_local $p1
    i32.const 0
    i32.ge_s
    if $I1
      get_local $l0
      get_local $p1
      call $f17
      drop
    end
    get_local $l0
    get_local $p2
    call $f24
    get_local $l0
    i32.load offset=24
    tee_local $l1
    if $I2
      block $B3 (result i32)
        get_local $l0
        i32.const 28
        i32.add
        tee_local $l2
        i32.load
        tee_local $p2
        get_local $l1
        i32.ne
        if $I4
          loop $L5
            get_local $p2
            i32.const -24
            i32.add
            tee_local $p2
            i32.load
            set_local $p1
            get_local $p2
            i32.const 0
            i32.store
            get_local $p1
            if $I6
              get_local $p1
              i32.load offset=32
              tee_local $p0
              if $I7
                get_local $p1
                i32.const 36
                i32.add
                get_local $p0
                i32.store
                get_local $p0
                call $_ZdlPv
              end
              get_local $p1
              call $_ZdlPv
            end
            get_local $l1
            get_local $p2
            i32.ne
            br_if $L5
          end
          get_local $l0
          i32.const 24
          i32.add
          i32.load
          br $B3
        end
        get_local $l1
      end
      get_local $l2
      get_local $l1
      i32.store
      call $_ZdlPv
    end
    get_local $l0
    i32.const 48
    i32.add
    set_global $g0)
  (func $f24 (type $t6) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i64)
    get_local $p1
    i32.load offset=44
    get_local $p0
    i32.eq
    i32.const 8821
    call $env.eosio_assert
    get_local $p0
    i64.load
    call $env.current_receiver
    i64.eq
    i32.const 8866
    call $env.eosio_assert
    get_local $p0
    i32.load offset=24
    tee_local $l4
    set_local $l1
    block $B0
      get_local $l4
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
      i64.load
      get_local $p1
      i64.load
      tee_local $l6
      i64.eq
      if $I1
        get_local $l0
        set_local $l1
        br $B0
      end
      get_local $l4
      i32.const 24
      i32.add
      set_local $l3
      block $B2
        loop $L3
          get_local $l3
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
          i64.load
          get_local $l6
          i64.ne
          br_if $L3
        end
        br $B0
      end
      get_local $l4
      set_local $l1
    end
    get_local $l1
    get_local $l4
    i32.ne
    i32.const 8916
    call $env.eosio_assert
    block $B4
      block $B5 (result i32)
        get_local $l1
        get_local $l5
        i32.load
        tee_local $l4
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
            tee_local $l3
            i32.load
            set_local $l1
            get_local $l3
            get_local $l2
            i32.store
            get_local $l1
            if $I8
              get_local $l1
              i32.load offset=32
              tee_local $l2
              if $I9
                get_local $l1
                i32.const 36
                i32.add
                get_local $l2
                i32.store
                get_local $l2
                call $_ZdlPv
              end
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
            get_local $l4
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
          tee_local $l3
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
      set_local $l3
      loop $L10
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
        if $I11
          get_local $l0
          i32.load offset=32
          tee_local $l2
          if $I12
            get_local $l0
            i32.const 36
            i32.add
            get_local $l2
            i32.store
            get_local $l2
            call $_ZdlPv
          end
          get_local $l0
          call $_ZdlPv
        end
        get_local $l3
        get_local $l1
        i32.ne
        br_if $L10
      end
    end
    get_local $p0
    i32.const 28
    i32.add
    get_local $l3
    i32.store
    get_local $p1
    i32.load offset=48
    call $env.db_remove_i64)
  (func $f25 (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32) (param $p5 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i64) (local $l3 i64)
    get_global $g0
    i32.const 80
    i32.sub
    tee_local $l0
    set_global $g0
    get_local $p3
    i64.load
    call $env.require_auth
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
    tee_local $l2
    i64.store offset=40
    get_local $l0
    get_local $p2
    i64.load
    tee_local $l3
    i64.store offset=48
    i32.const 0
    set_local $p2
    get_local $l2
    get_local $l3
    i64.const 7035937633859534848
    get_local $p1
    i64.load
    call $env.db_find_i64
    tee_local $p1
    i32.const 0
    i32.ge_s
    if $I0
      get_local $l0
      i32.const 40
      i32.add
      get_local $p1
      call $f17
      tee_local $p2
      i32.load offset=44
      get_local $l0
      i32.const 40
      i32.add
      i32.eq
      i32.const 8426
      call $env.eosio_assert
    end
    get_local $p2
    i32.const 0
    i32.ne
    tee_local $p0
    i32.const 8253
    call $env.eosio_assert
    get_local $p2
    i64.load offset=24
    i64.const 4
    set_local $l2
    loop $L1
      get_local $l2
      i64.const 1
      i64.add
      tee_local $l2
      i64.const 13
      i64.ne
      br_if $L1
    end
    i64.const -7122829838779416576
    i64.eq
    i32.const 8296
    call $env.eosio_assert
    i32.const 1
    set_local $p1
    get_local $p3
    i64.load
    tee_local $l2
    get_local $p2
    i64.load offset=8
    i64.ne
    if $I2
      get_local $l2
      get_local $p2
      i64.load
      i64.eq
      set_local $p1
    end
    get_local $p1
    i32.const 8273
    call $env.eosio_assert
    get_local $p3
    i64.load
    get_local $p2
    i64.load offset=16
    i64.eq
    i32.const 8316
    call $env.eosio_assert
    block $B3 (result i32)
      i32.const 0
      tee_local $p3
      get_local $p4
      i32.load16_u
      tee_local $p1
      i32.const 2
      i32.gt_u
      br_if $B3
      drop
      i32.const 0
      get_local $p5
      i32.load16_u
      tee_local $l1
      i32.const 2
      i32.gt_u
      br_if $B3
      drop
      get_local $p2
      i32.load offset=32
      get_local $p1
      i32.const 3
      i32.mul
      get_local $l1
      i32.add
      i32.add
      i32.load8_u
      i32.eqz
    end
    tee_local $p3
    i32.const 8340
    call $env.eosio_assert
    get_local $l0
    i32.const 1
    i32.const 2
    get_local $p2
    i32.const 16
    i32.add
    i64.load
    get_local $p2
    i32.const 8
    i32.add
    i64.load
    tee_local $l2
    i64.eq
    tee_local $p3
    select
    i32.store8 offset=39
    get_local $l0
    get_local $p2
    i32.const 0
    i32.const 8
    get_local $p3
    select
    i32.add
    i64.load
    i64.store offset=24
    get_local $l0
    get_local $p5
    i32.store offset=12
    get_local $l0
    get_local $p4
    i32.store offset=8
    get_local $l0
    get_local $l0
    i32.const 39
    i32.add
    i32.store offset=16
    get_local $l0
    get_local $l0
    i32.const 24
    i32.add
    i32.store offset=20
    get_local $p0
    i32.const 8566
    call $env.eosio_assert
    get_local $l0
    i32.const 40
    i32.add
    get_local $p2
    get_local $l2
    get_local $l0
    i32.const 8
    i32.add
    call $f26
    get_local $l0
    i32.load offset=64
    tee_local $p5
    if $I4
      block $B5 (result i32)
        get_local $l0
        i32.const 68
        i32.add
        tee_local $p1
        i32.load
        tee_local $p2
        get_local $p5
        i32.ne
        if $I6
          loop $L7
            get_local $p2
            i32.const -24
            i32.add
            tee_local $p2
            i32.load
            set_local $p3
            get_local $p2
            i32.const 0
            i32.store
            get_local $p3
            if $I8
              get_local $p3
              i32.load offset=32
              tee_local $p4
              if $I9
                get_local $p3
                i32.const 36
                i32.add
                get_local $p4
                i32.store
                get_local $p4
                call $_ZdlPv
              end
              get_local $p3
              call $_ZdlPv
            end
            get_local $p5
            get_local $p2
            i32.ne
            br_if $L7
          end
          get_local $l0
          i32.const -64
          i32.sub
          i32.load
          br $B5
        end
        get_local $p5
      end
      get_local $p1
      get_local $p5
      i32.store
      call $_ZdlPv
    end
    get_local $l0
    i32.const 80
    i32.add
    set_global $g0)
  (func $f26 (type $t18) (param $p0 i32) (param $p1 i32) (param $p2 i64) (param $p3 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i64) (local $l6 i64)
    get_global $g0
    i32.const 16
    i32.sub
    tee_local $l2
    set_local $l1
    get_local $l2
    set_global $g0
    get_local $p1
    i32.load offset=44
    get_local $p0
    i32.eq
    i32.const 8601
    call $env.eosio_assert
    get_local $p0
    i64.load
    call $env.current_receiver
    i64.eq
    i32.const 8647
    call $env.eosio_assert
    get_local $p1
    i64.load
    set_local $l5
    get_local $p1
    i32.load offset=32
    get_local $p3
    i32.load
    i32.load16_u
    i32.const 3
    i32.mul
    get_local $p3
    i32.load offset=4
    i32.load16_u
    i32.add
    i32.add
    get_local $p3
    i32.load offset=8
    i32.load8_u
    i32.store8
    get_local $p1
    get_local $p3
    i32.load offset=12
    i64.load
    i64.store offset=16
    get_local $p1
    get_local $p1
    call $f14
    i64.store offset=24
    get_local $l5
    get_local $p1
    i64.load
    i64.eq
    i32.const 8698
    call $env.eosio_assert
    get_local $p1
    i32.const 36
    i32.add
    i32.load
    tee_local $l0
    get_local $p1
    i32.load offset=32
    tee_local $l3
    i32.sub
    tee_local $l4
    i64.extend_u/i32
    set_local $l6
    i32.const 32
    set_local $p3
    loop $L0
      get_local $p3
      i32.const 1
      i32.add
      set_local $p3
      get_local $l6
      i64.const 7
      i64.shr_u
      tee_local $l6
      i64.const 0
      i64.ne
      br_if $L0
    end
    block $B1
      get_local $p3
      get_local $l4
      get_local $p3
      i32.add
      get_local $l3
      get_local $l0
      i32.eq
      select
      tee_local $l0
      i32.const 513
      i32.ge_u
      if $I2
        get_local $l0
        call $f43
        set_local $p3
        br $B1
      end
      get_local $l2
      get_local $l0
      i32.const 15
      i32.add
      i32.const -16
      i32.and
      i32.sub
      tee_local $p3
      set_global $g0
    end
    get_local $l1
    get_local $p3
    i32.store offset=4
    get_local $l1
    get_local $p3
    i32.store
    get_local $l1
    get_local $p3
    get_local $l0
    i32.add
    i32.store offset=8
    get_local $l1
    get_local $p1
    call $f35
    drop
    get_local $p1
    i32.load offset=48
    get_local $p2
    get_local $p3
    get_local $l0
    call $env.db_update_i64
    block $B3
      block $B4
        get_local $l0
        i32.const 513
        i32.lt_u
        if $I5
          get_local $l5
          get_local $p0
          i64.load offset=16
          i64.ge_u
          br_if $B4
          br $B3
        end
        get_local $p3
        call $f46
        get_local $l5
        get_local $p0
        i64.load offset=16
        i64.lt_u
        br_if $B3
      end
      get_local $p0
      i32.const 16
      i32.add
      i64.const -2
      get_local $l5
      i64.const 1
      i64.add
      get_local $l5
      i64.const -3
      i64.gt_u
      select
      i64.store
      get_local $l1
      i32.const 16
      i32.add
      set_global $g0
      return
    end
    get_local $l1
    i32.const 16
    i32.add
    set_global $g0)
  (func $apply (type $t19) (param $p0 i64) (param $p1 i64) (param $p2 i64)
    (local $l0 i32) (local $l1 i64)
    get_global $g0
    i32.const 80
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
      i32.const 8362
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
      i64.store offset=72
      block $B6
        block $B7
          get_local $p2
          i64.const 4929617502180212735
          i64.le_s
          if $I8
            get_local $p2
            i64.const -7694786991455469568
            i64.eq
            br_if $B7
            get_local $p2
            i64.const -4994048475009122304
            i64.ne
            br_if $B3
            get_local $l0
            i32.const 0
            i32.store offset=60
            get_local $l0
            i32.const 1
            i32.store offset=56
            get_local $l0
            get_local $l0
            i64.load offset=56
            i64.store offset=16
            get_local $l0
            i32.const 72
            i32.add
            get_local $l0
            i32.const 16
            i32.add
            call $f28
            drop
            br $B3
          end
          get_local $p2
          i64.const 4929617502180212736
          i64.eq
          br_if $B6
          get_local $p2
          i64.const 5031766152489992192
          i64.ne
          br_if $B3
          get_local $l0
          i32.const 0
          i32.store offset=68
          get_local $l0
          i32.const 2
          i32.store offset=64
          get_local $l0
          get_local $l0
          i64.load offset=64
          i64.store offset=8
          get_local $l0
          i32.const 72
          i32.add
          get_local $l0
          i32.const 8
          i32.add
          call $f29
          drop
          br $B3
        end
        get_local $l0
        i32.const 0
        i32.store offset=44
        get_local $l0
        i32.const 3
        i32.store offset=40
        get_local $l0
        get_local $l0
        i64.load offset=40
        i64.store offset=32
        get_local $l0
        i32.const 72
        i32.add
        get_local $l0
        i32.const 32
        i32.add
        call $f30
        drop
        br $B3
      end
      get_local $l0
      i32.const 0
      i32.store offset=52
      get_local $l0
      i32.const 4
      i32.store offset=48
      get_local $l0
      get_local $l0
      i64.load offset=48
      i64.store offset=24
      get_local $l0
      i32.const 72
      i32.add
      get_local $l0
      i32.const 24
      i32.add
      call $f29
      drop
    end
    get_local $l0
    i32.const 80
    i32.add
    set_global $g0)
  (func $f28 (type $t9) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i64) (local $l7 i64)
    get_global $g0
    i32.const 48
    i32.sub
    tee_local $l1
    set_local $l0
    get_local $l1
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
            call $f43
            set_local $l1
            br $B1
          end
          i32.const 0
          set_local $l1
          br $B0
        end
        get_local $l1
        get_local $p1
        i32.const 15
        i32.add
        i32.const -16
        i32.and
        i32.sub
        tee_local $l1
        set_global $g0
      end
      get_local $l1
      get_local $p1
      call $env.read_action_data
      drop
    end
    get_local $l0
    i64.const 0
    i64.store offset=8
    get_local $l0
    i64.const 0
    i64.store
    get_local $l0
    i64.const 0
    i64.store offset=16
    get_local $p1
    i32.const 7
    i32.gt_u
    i32.const 8504
    call $env.eosio_assert
    get_local $l0
    get_local $l1
    i32.const 8
    call $env.memcpy
    drop
    get_local $p1
    i32.const -8
    i32.and
    tee_local $l3
    i32.const 8
    i32.ne
    i32.const 8504
    call $env.eosio_assert
    get_local $l0
    i32.const 8
    i32.add
    tee_local $l5
    get_local $l1
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $l3
    i32.const 16
    i32.ne
    i32.const 8504
    call $env.eosio_assert
    get_local $l0
    i32.const 16
    i32.add
    tee_local $l3
    get_local $l1
    i32.const 16
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $p1
    i32.const 513
    i32.ge_u
    if $I4
      get_local $l1
      call $f46
    end
    get_local $l3
    i64.load
    set_local $l6
    get_local $l0
    i64.load
    set_local $l7
    get_local $l0
    get_local $l5
    i64.load
    i64.store offset=32
    get_local $l0
    get_local $l7
    i64.store offset=40
    get_local $l0
    get_local $l6
    i64.store offset=24
    get_local $p0
    get_local $l4
    i32.const 1
    i32.shr_s
    i32.add
    set_local $p1
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
    get_local $l0
    i32.const 40
    i32.add
    get_local $l0
    i32.const 32
    i32.add
    get_local $l0
    i32.const 24
    i32.add
    get_local $l2
    call_indirect (type $t0)
    get_local $l0
    i32.const 48
    i32.add
    set_global $g0
    i32.const 1)
  (func $f29 (type $t9) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i64)
    get_global $g0
    i32.const 32
    i32.sub
    tee_local $l1
    set_local $l0
    get_local $l1
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
            call $f43
            set_local $l1
            br $B1
          end
          i32.const 0
          set_local $l1
          br $B0
        end
        get_local $l1
        get_local $p1
        i32.const 15
        i32.add
        i32.const -16
        i32.and
        i32.sub
        tee_local $l1
        set_global $g0
      end
      get_local $l1
      get_local $p1
      call $env.read_action_data
      drop
    end
    get_local $l0
    i64.const 0
    i64.store offset=8
    get_local $l0
    i64.const 0
    i64.store
    get_local $p1
    i32.const 7
    i32.gt_u
    i32.const 8504
    call $env.eosio_assert
    get_local $l0
    get_local $l1
    i32.const 8
    call $env.memcpy
    drop
    get_local $p1
    i32.const -8
    i32.and
    i32.const 8
    i32.ne
    i32.const 8504
    call $env.eosio_assert
    get_local $l0
    i32.const 8
    i32.add
    tee_local $l4
    get_local $l1
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $p1
    i32.const 513
    i32.ge_u
    if $I4
      get_local $l1
      call $f46
    end
    get_local $l0
    i64.load
    set_local $l5
    get_local $l0
    get_local $l4
    i64.load
    i64.store offset=16
    get_local $l0
    get_local $l5
    i64.store offset=24
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
    get_local $l0
    i32.const 24
    i32.add
    get_local $l0
    i32.const 16
    i32.add
    get_local $l2
    call_indirect (type $t1)
    get_local $l0
    i32.const 32
    i32.add
    set_global $g0
    i32.const 1)
  (func $f30 (type $t9) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i64) (local $l6 i64)
    get_global $g0
    i32.const -64
    i32.add
    tee_local $l3
    set_local $l0
    get_local $l3
    set_global $g0
    get_local $p1
    i32.load offset=4
    set_local $l4
    get_local $p1
    i32.load
    set_local $l2
    i32.const 0
    set_local $p1
    call $env.action_data_size
    tee_local $l1
    if $I0
      block $B1
        get_local $l1
        i32.const 513
        i32.ge_u
        if $I2
          get_local $l1
          call $f43
          set_local $p1
          br $B1
        end
        get_local $l3
        get_local $l1
        i32.const 15
        i32.add
        i32.const -16
        i32.and
        i32.sub
        tee_local $p1
        set_global $g0
      end
      get_local $p1
      get_local $l1
      call $env.read_action_data
      drop
    end
    get_local $l0
    i64.const 0
    i64.store offset=8
    get_local $l0
    i64.const 0
    i64.store
    get_local $l0
    i64.const 0
    i64.store offset=16
    get_local $l0
    i32.const 0
    i32.store offset=24
    get_local $l0
    get_local $p1
    i32.store offset=36
    get_local $l0
    get_local $p1
    i32.store offset=32
    get_local $l0
    get_local $p1
    get_local $l1
    i32.add
    i32.store offset=40
    get_local $l0
    get_local $l0
    i32.const 32
    i32.add
    i32.store offset=48
    get_local $l0
    get_local $l0
    i32.store offset=56
    get_local $l0
    i32.const 56
    i32.add
    get_local $l0
    i32.const 48
    i32.add
    call $f31
    get_local $l1
    i32.const 513
    i32.ge_u
    if $I3
      get_local $p1
      call $f46
    end
    get_local $l0
    i32.const 26
    i32.add
    i32.load16_u
    set_local $p1
    get_local $l0
    i32.const 24
    i32.add
    i32.load16_u
    set_local $l1
    get_local $l0
    i32.const 16
    i32.add
    i64.load
    set_local $l5
    get_local $l0
    i64.load
    set_local $l6
    get_local $l0
    get_local $l0
    i32.const 8
    i32.add
    i64.load
    i64.store offset=56
    get_local $l0
    get_local $l6
    i64.store offset=32
    get_local $l0
    get_local $l5
    i64.store offset=48
    get_local $l0
    get_local $l1
    i32.store16 offset=46
    get_local $l0
    get_local $p1
    i32.store16 offset=44
    get_local $p0
    get_local $l4
    i32.const 1
    i32.shr_s
    i32.add
    set_local $p1
    get_local $l4
    i32.const 1
    i32.and
    if $I4
      get_local $p1
      i32.load
      get_local $l2
      i32.add
      i32.load
      set_local $l2
    end
    get_local $p1
    get_local $l0
    i32.const 32
    i32.add
    get_local $l0
    i32.const 56
    i32.add
    get_local $l0
    i32.const 48
    i32.add
    get_local $l0
    i32.const 46
    i32.add
    get_local $l0
    i32.const 44
    i32.add
    get_local $l2
    call_indirect (type $t2)
    get_local $l0
    i32.const -64
    i32.sub
    set_global $g0
    i32.const 1)
  (func $f31 (type $t6) (param $p0 i32) (param $p1 i32)
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
    i32.const 8504
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
    set_local $l0
    get_local $p1
    i32.load
    tee_local $p0
    i32.load offset=8
    get_local $p0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 8504
    call $env.eosio_assert
    get_local $l0
    i32.const 8
    i32.add
    get_local $p0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $p0
    get_local $p0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4
    get_local $p1
    i32.load
    tee_local $p0
    i32.load offset=8
    get_local $p0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 8504
    call $env.eosio_assert
    get_local $l0
    i32.const 16
    i32.add
    get_local $p0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $p0
    get_local $p0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4
    get_local $p1
    i32.load
    tee_local $p0
    i32.load offset=8
    get_local $p0
    i32.load offset=4
    i32.sub
    i32.const 1
    i32.gt_u
    i32.const 8504
    call $env.eosio_assert
    get_local $l0
    i32.const 24
    i32.add
    get_local $p0
    i32.load offset=4
    i32.const 2
    call $env.memcpy
    drop
    get_local $p0
    get_local $p0
    i32.load offset=4
    i32.const 2
    i32.add
    i32.store offset=4
    get_local $p1
    i32.load
    tee_local $p1
    i32.load offset=8
    get_local $p1
    i32.load offset=4
    i32.sub
    i32.const 1
    i32.gt_u
    i32.const 8504
    call $env.eosio_assert
    get_local $l0
    i32.const 26
    i32.add
    get_local $p1
    i32.load offset=4
    i32.const 2
    call $env.memcpy
    drop
    get_local $p1
    get_local $p1
    i32.load offset=4
    i32.const 2
    i32.add
    i32.store offset=4)
  (func $f32 (type $t9) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32)
    get_local $p0
    i32.load offset=8
    get_local $p0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 8504
    call $env.eosio_assert
    get_local $p1
    get_local $p0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $p0
    get_local $p0
    i32.load offset=4
    i32.const 8
    i32.add
    tee_local $l0
    i32.store offset=4
    get_local $p0
    i32.load offset=8
    get_local $l0
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 8504
    call $env.eosio_assert
    get_local $p1
    i32.const 8
    i32.add
    get_local $p0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $p0
    get_local $p0
    i32.load offset=4
    i32.const 8
    i32.add
    tee_local $l0
    i32.store offset=4
    get_local $p0
    i32.load offset=8
    get_local $l0
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 8504
    call $env.eosio_assert
    get_local $p1
    i32.const 16
    i32.add
    get_local $p0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $p0
    get_local $p0
    i32.load offset=4
    i32.const 8
    i32.add
    tee_local $l0
    i32.store offset=4
    get_local $p0
    i32.load offset=8
    get_local $l0
    i32.sub
    i32.const 7
    i32.gt_u
    i32.const 8504
    call $env.eosio_assert
    get_local $p1
    i32.const 24
    i32.add
    get_local $p0
    i32.load offset=4
    i32.const 8
    call $env.memcpy
    drop
    get_local $p0
    get_local $p0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4
    get_local $p0
    get_local $p1
    i32.const 32
    i32.add
    call $f33)
  (func $f33 (type $t9) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i64)
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
    set_local $l3
    loop $L0
      get_local $l0
      get_local $l4
      i32.load
      i32.lt_u
      i32.const 8500
      call $env.eosio_assert
      get_local $l3
      i32.load
      tee_local $l0
      i32.load8_u
      set_local $l2
      get_local $l3
      get_local $l0
      i32.const 1
      i32.add
      tee_local $l0
      i32.store
      get_local $l5
      get_local $l2
      i32.const 127
      i32.and
      get_local $l1
      i32.const 255
      i32.and
      tee_local $l1
      i32.shl
      i64.extend_u/i32
      i64.or
      set_local $l5
      get_local $l1
      i32.const 7
      i32.add
      set_local $l1
      get_local $l2
      i32.const 128
      i32.and
      br_if $L0
    end
    block $B1
      block $B2
        get_local $p1
        i32.load offset=4
        tee_local $l3
        get_local $p1
        i32.load
        tee_local $l2
        i32.sub
        tee_local $l1
        get_local $l5
        i32.wrap/i64
        tee_local $l0
        i32.lt_u
        if $I3
          get_local $p1
          get_local $l0
          get_local $l1
          i32.sub
          call $f34
          get_local $p1
          i32.load
          tee_local $l2
          get_local $p1
          i32.const 4
          i32.add
          i32.load
          tee_local $l3
          i32.ne
          br_if $B2
          br $B1
        end
        get_local $l1
        get_local $l0
        i32.gt_u
        if $I4
          get_local $p1
          i32.const 4
          i32.add
          get_local $l2
          get_local $l0
          i32.add
          tee_local $l3
          i32.store
        end
        get_local $l2
        get_local $l3
        i32.eq
        br_if $B1
      end
      get_local $p0
      i32.const 4
      i32.add
      tee_local $l0
      i32.load
      set_local $l1
      get_local $p0
      i32.const 8
      i32.add
      set_local $l4
      loop $L5
        get_local $l4
        i32.load
        get_local $l1
        i32.ne
        i32.const 8504
        call $env.eosio_assert
        get_local $l2
        get_local $l0
        i32.load
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
        get_local $l3
        get_local $l2
        i32.const 1
        i32.add
        tee_local $l2
        i32.ne
        br_if $L5
      end
    end
    get_local $p0)
  (func $f34 (type $t6) (param $p0 i32) (param $p1 i32)
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
        call $f42
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
  (func $f35 (type $t9) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32)
    get_local $p0
    i32.load offset=8
    get_local $p0
    i32.load offset=4
    i32.sub
    i32.const 7
    i32.gt_s
    i32.const 8560
    call $env.eosio_assert
    get_local $p0
    i32.load offset=4
    get_local $p1
    i32.const 8
    call $env.memcpy
    drop
    get_local $p0
    get_local $p0
    i32.load offset=4
    i32.const 8
    i32.add
    tee_local $l0
    i32.store offset=4
    get_local $p0
    i32.load offset=8
    get_local $l0
    i32.sub
    i32.const 7
    i32.gt_s
    i32.const 8560
    call $env.eosio_assert
    get_local $p0
    i32.load offset=4
    get_local $p1
    i32.const 8
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $p0
    get_local $p0
    i32.load offset=4
    i32.const 8
    i32.add
    tee_local $l0
    i32.store offset=4
    get_local $p0
    i32.load offset=8
    get_local $l0
    i32.sub
    i32.const 7
    i32.gt_s
    i32.const 8560
    call $env.eosio_assert
    get_local $p0
    i32.load offset=4
    get_local $p1
    i32.const 16
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $p0
    get_local $p0
    i32.load offset=4
    i32.const 8
    i32.add
    tee_local $l0
    i32.store offset=4
    get_local $p0
    i32.load offset=8
    get_local $l0
    i32.sub
    i32.const 7
    i32.gt_s
    i32.const 8560
    call $env.eosio_assert
    get_local $p0
    i32.load offset=4
    get_local $p1
    i32.const 24
    i32.add
    i32.const 8
    call $env.memcpy
    drop
    get_local $p0
    get_local $p0
    i32.load offset=4
    i32.const 8
    i32.add
    i32.store offset=4
    get_local $p0
    get_local $p1
    i32.const 32
    i32.add
    call $f36)
  (func $f36 (type $t9) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i64)
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
    i64.extend_u/i32
    set_local $l6
    get_local $p0
    i32.load offset=4
    set_local $l2
    get_local $p0
    i32.const 8
    i32.add
    set_local $l4
    get_local $p0
    i32.const 4
    i32.add
    set_local $l0
    loop $L0
      get_local $l6
      i32.wrap/i64
      set_local $l1
      get_local $l3
      get_local $l6
      i64.const 7
      i64.shr_u
      tee_local $l6
      i64.const 0
      i64.ne
      tee_local $l5
      i32.const 7
      i32.shl
      get_local $l1
      i32.const 127
      i32.and
      i32.or
      i32.store8 offset=15
      get_local $l4
      i32.load
      get_local $l2
      i32.sub
      i32.const 0
      i32.gt_s
      i32.const 8560
      call $env.eosio_assert
      get_local $l0
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
      tee_local $l2
      i32.store
      get_local $l5
      br_if $L0
    end
    get_local $p1
    i32.load
    tee_local $l0
    get_local $p1
    i32.const 4
    i32.add
    i32.load
    tee_local $l5
    i32.ne
    if $I1
      get_local $p0
      i32.const 8
      i32.add
      set_local $l4
      get_local $p0
      i32.const 4
      i32.add
      set_local $l1
      loop $L2
        get_local $l4
        i32.load
        get_local $l2
        i32.sub
        i32.const 0
        i32.gt_s
        i32.const 8560
        call $env.eosio_assert
        get_local $l1
        i32.load
        get_local $l0
        i32.const 1
        call $env.memcpy
        drop
        get_local $l1
        get_local $l1
        i32.load
        i32.const 1
        i32.add
        tee_local $l2
        i32.store
        get_local $l5
        get_local $l0
        i32.const 1
        i32.add
        tee_local $l0
        i32.ne
        br_if $L2
      end
    end
    get_local $l3
    i32.const 16
    i32.add
    set_global $g0
    get_local $p0)
  (func $f37 (type $t13) (param $p0 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i64)
    i32.const 9
    call $_Znwj
    tee_local $l1
    i32.const 0
    i32.store8 offset=8
    get_local $l1
    i64.const 0
    i64.store align=1
    get_local $l1
    i32.const 9
    i32.add
    set_local $l2
    block $B0
      get_local $p0
      i32.load offset=32
      tee_local $l0
      if $I1
        get_local $p0
        i32.const 36
        i32.add
        get_local $l0
        i32.store
        get_local $l0
        call $_ZdlPv
        get_local $p0
        i32.const 40
        i32.add
        tee_local $l0
        i32.const 0
        i32.store
        get_local $p0
        i64.const 0
        i64.store offset=32 align=4
        br $B0
      end
      get_local $p0
      i32.const 40
      i32.add
      set_local $l0
    end
    get_local $l0
    get_local $l2
    i32.store
    get_local $p0
    i32.const 36
    i32.add
    get_local $l2
    i32.store
    get_local $p0
    i32.const 32
    i32.add
    get_local $l1
    i32.store
    get_local $p0
    get_local $p0
    i64.load offset=8
    i64.store offset=16
    i64.const 4
    set_local $l3
    loop $L2
      get_local $l3
      i64.const 1
      i64.add
      tee_local $l3
      i64.const 13
      i64.ne
      br_if $L2
    end
    get_local $p0
    i64.const -7122829838779416576
    i64.store offset=24)
  (func $_Znwj (type $t16) (param $p0 i32) (result i32)
    (local $l0 i32) (local $l1 i32)
    block $B0
      get_local $p0
      i32.const 1
      get_local $p0
      select
      tee_local $l0
      call $f43
      tee_local $p0
      br_if $B0
      loop $L1
        i32.const 0
        set_local $p0
        i32.const 8972
        i32.load
        tee_local $l1
        i32.eqz
        br_if $B0
        get_local $l1
        call_indirect (type $t3)
        get_local $l0
        call $f43
        tee_local $p0
        i32.eqz
        br_if $L1
      end
    end
    get_local $p0)
  (func $_Znaj (type $t16) (param $p0 i32) (result i32)
    get_local $p0
    call $_Znwj)
  (func $_ZdlPv (type $t13) (param $p0 i32)
    get_local $p0
    if $I0
      get_local $p0
      call $f46
    end)
  (func $_ZdaPv (type $t13) (param $p0 i32)
    get_local $p0
    call $_ZdlPv)
  (func $f42 (type $t13) (param $p0 i32)
    call $env.abort
    unreachable)
  (func $f43 (type $t16) (param $p0 i32) (result i32)
    i32.const 8984
    get_local $p0
    call $f44)
  (func $f44 (type $t9) (param $p0 i32) (param $p1 i32) (result i32)
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
              call $f45
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
            i32.const 17380
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
  (func $f45 (type $t16) (param $p0 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32)
    get_local $p0
    i32.load offset=8388
    set_local $l4
    block $B0
      i32.const 8976
      i32.load8_u
      if $I1
        i32.const 8980
        i32.load
        set_local $l0
        br $B0
      end
      memory.size
      set_local $l0
      i32.const 8976
      i32.const 1
      i32.store8
      i32.const 8980
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
            i32.const 8980
            i32.load
            set_local $l1
          end
          i32.const 0
          set_local $l2
          i32.const 8980
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
          i32.const 8976
          i32.load8_u
          i32.eqz
          if $I8
            memory.size
            set_local $l1
            i32.const 8976
            i32.const 1
            i32.store8
            i32.const 8980
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
            i32.const 8980
            i32.load
            set_local $l5
          end
          i32.const 8980
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
  (func $f46 (type $t13) (param $p0 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32)
    block $B0
      block $B1
        get_local $p0
        i32.eqz
        br_if $B1
        i32.const 17368
        i32.load
        tee_local $l1
        i32.const 1
        i32.lt_s
        br_if $B1
        i32.const 17176
        set_local $l0
        get_local $l1
        i32.const 12
        i32.mul
        i32.const 17176
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
  (table $T0 5 5 anyfunc)
  (memory $memory 1)
  (global $g0 (mut i32) (i32.const 8192))
  (global $__heap_base i32 (i32.const 17466))
  (global $__data_end i32 (i32.const 17466))
  (export "memory" (memory 0))
  (export "__heap_base" (global 1))
  (export "__data_end" (global 2))
  (export "apply" (func $apply))
  (export "_Znwj" (func $_Znwj))
  (export "_ZdlPv" (func $_ZdlPv))
  (export "_Znaj" (func $_Znaj))
  (export "_ZdaPv" (func $_ZdaPv))
  (elem (i32.const 1) $f21 $f16 $f25 $f23)
  (data (i32.const 8192) "challenger shouldn't be the same as host")
  (data (i32.const 8233) "game already exists")
  (data (i32.const 8253) "game doesn't exists")
  (data (i32.const 8273) "this is not your game!")
  (data (i32.const 8296) "the game has ended!")
  (data (i32.const 8316) "it's not your turn yet!")
  (data (i32.const 8340) "not a valid movement!")
  (data (i32.const 8362) "onerror action's are only valid from the \22eosio\22 system account")
  (data (i32.const 8426) "object passed to iterator_to is not in multi_index")
  (data (i32.const 8477) "error reading iterator")
  (data (i32.const 8500) "get")
  (data (i32.const 8504) "read")
  (data (i32.const 8509) "cannot create objects in table of another contract")
  (data (i32.const 8560) "write")
  (data (i32.const 8566) "cannot pass end iterator to modify")
  (data (i32.const 8601) "object passed to modify is not in multi_index")
  (data (i32.const 8647) "cannot modify objects in table of another contract")
  (data (i32.const 8698) "updater cannot change primary key when modifying an object")
  (data (i32.const 8757) "cannot pass end iterator to erase")
  (data (i32.const 8791) "cannot increment end iterator")
  (data (i32.const 8821) "object passed to erase is not in multi_index")
  (data (i32.const 8866) "cannot erase objects in table of another contract")
  (data (i32.const 8916) "attempt to remove object that was not in multi_index")
  (data (i32.const 17380) "malloc_from_freed was designed to only be called after _heap was completely allocated"))
