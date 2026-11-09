(module $wasm_lib.wasm
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 i32 i32) (result i32)))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32)))
  (type (;6;) (func (param i32 i32 i32 i32 i32 i32)))
  (type (;7;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;8;) (func))
  (type (;9;) (func (param i32 i64)))
  (type (;10;) (func (param i32 i32 i32 i32 i32)))
  (type (;11;) (func (param i32 i32 i32 i32)))
  (type (;12;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;13;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;14;) (func (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (func $_RINvMNtNtCs5JHfeWeQK0x_5alloc2io5errorNtNtNtCs4NIexwV1FGX_4core2io5error5Error3newReECshuXg64jKSg3_8wasm_lib (type 2) (param i32)
    (local i32)
    call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block  ;; label = @1
      i32.const 12
      i32.const 4
      call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
      local.tee 1
      br_if 0 (;@1;)
      i32.const 4
      i32.const 12
      call $_RNvNtCs5JHfeWeQK0x_5alloc5alloc18handle_alloc_error
      unreachable
    end
    local.get 1
    i32.const 0
    i32.store offset=8
    local.get 1
    i64.const 4294967296
    i64.store align=4
    local.get 0
    i32.const 37
    local.get 1
    i32.const 1048956
    call $_RNvNtNtCs5JHfeWeQK0x_5alloc2io5error21custom_owner_from_box
    i64.extend_i32_u
    i64.const 32
    i64.shl
    i64.const 3
    i64.or
    i64.store align=4)
  (func $_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueINtNtCs5JHfeWeQK0x_5alloc5boxed3BoxNtNtCs1NJkdcuIVXo_7bincode5error9ErrorKindEECshuXg64jKSg3_8wasm_lib (type 2) (param i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 0
      i32.load
      local.tee 1
      i32.const -2147483648
      i32.xor
      i32.const 8
      local.get 1
      i32.const 0
      i32.lt_s
      select
      local.tee 2
      i32.const -1
      i32.add
      i32.const 7
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=4
        local.get 1
        i32.const 1
        call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc
        br 1 (;@1;)
      end
      local.get 0
      i32.load8_u offset=4
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.const 8
      i32.add
      call $_RNvXsd_NtNtCs4NIexwV1FGX_4core2io5errorNtB5_11CustomOwnerNtNtNtB9_3ops4drop4Drop4drop
    end
    local.get 0
    i32.const 12
    i32.const 4
    call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc)
  (func $_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBL_3BoxDNtNtB4_5error5ErrorNtNtB4_6marker4SendNtB1R_4SyncEL_EINtNtB4_7convert4FromNtNtBN_6string6StringE4from11StringErrorECshuXg64jKSg3_8wasm_lib (type 2) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc
    end)
  (func $_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshuXg64jKSg3_8wasm_lib (type 5) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      local.get 2
      local.get 1
      i32.add
      local.tee 1
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      call $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec12handle_error
      unreachable
    end
    local.get 3
    i32.const 4
    i32.add
    local.get 0
    i32.load
    local.tee 2
    local.get 0
    i32.load offset=4
    local.get 1
    local.get 2
    i32.const 1
    i32.shl
    local.tee 2
    local.get 1
    local.get 2
    i32.gt_u
    select
    local.tee 2
    i32.const 8
    local.get 2
    i32.const 8
    i32.gt_u
    select
    local.tee 2
    i32.const 1
    i32.const 1
    call $_RNvMs5_NtCs5JHfeWeQK0x_5alloc7raw_vecNtB5_11RawVecInner11finish_growCshuXg64jKSg3_8wasm_lib
    block  ;; label = @1
      local.get 3
      i32.load offset=4
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=8
      local.get 3
      i32.load offset=12
      call $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec12handle_error
      unreachable
    end
    local.get 3
    i32.load offset=8
    local.set 1
    local.get 0
    local.get 2
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_RNvMs5_NtCs5JHfeWeQK0x_5alloc7raw_vecNtB5_11RawVecInner11finish_growCshuXg64jKSg3_8wasm_lib (type 6) (param i32 i32 i32 i32 i32 i32)
    (local i32 i32 i64)
    i32.const 1
    local.set 6
    i32.const 4
    local.set 7
    block  ;; label = @1
      block  ;; label = @2
        local.get 5
        i64.extend_i32_u
        local.get 3
        i64.extend_i32_u
        i64.mul
        local.tee 8
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 8
        i32.wrap_i64
        local.tee 3
        i32.const -2147483648
        local.get 4
        i32.sub
        i32.le_u
        br_if 0 (;@2;)
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              local.get 5
              local.get 1
              i32.mul
              local.get 4
              local.get 3
              call $_RNvCs5fGHjzWXRav_7___rustc14___rust_realloc
              local.set 7
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 3
              br_if 0 (;@5;)
              local.get 4
              local.set 7
              br 2 (;@3;)
            end
            call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
            local.get 3
            local.get 4
            call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
            local.set 7
          end
          local.get 7
          br_if 0 (;@3;)
          local.get 0
          local.get 4
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 0
        local.get 7
        i32.store offset=4
        i32.const 0
        local.set 6
      end
      i32.const 8
      local.set 7
    end
    local.get 0
    local.get 7
    i32.add
    local.get 3
    i32.store
    local.get 0
    local.get 6
    i32.store)
  (func $_RINvYINtNvXs1_NtCs1NJkdcuIVXo_7bincode2deQINtBc_12DeserializerppENtNtCs8n3ERUNuD4B_10serde_core2de12Deserializer17deserialize_tuple6AccessNtNtBc_4read11SliceReaderINtNtBe_6config14WithOtherLimitINtB2E_17WithOtherTrailingINtB2E_20WithOtherIntEncodingNtB2E_14DefaultOptionsNtNtB2E_3int14FixintEncodingENtNtB2E_8trailing13AllowTrailingENtNtB2E_5limit8InfiniteEENtB13_9SeqAccess12next_elementNtNtCs5JHfeWeQK0x_5alloc6string6StringECshuXg64jKSg3_8wasm_lib (type 1) (param i32 i32)
    (local i32 i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load offset=4
              local.tee 3
              br_if 0 (;@5;)
              local.get 0
              i32.const -1
              i32.store
              br 1 (;@4;)
            end
            local.get 1
            local.get 3
            i32.const -1
            i32.add
            i32.store offset=4
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                i32.load
                local.tee 3
                i32.load offset=4
                local.tee 4
                i32.const 7
                i32.gt_u
                br_if 0 (;@6;)
                local.get 2
                i64.const 9473
                i64.store offset=16
                local.get 2
                i32.const 16
                i32.add
                call $_RNvXs_NtCs1NJkdcuIVXo_7bincode5errorINtNtCs5JHfeWeQK0x_5alloc5boxed3BoxNtB4_9ErrorKindEINtNtCs4NIexwV1FGX_4core7convert4FromNtNtNtB1s_2io5error5ErrorE4from
                local.set 1
                i32.const 1
                local.set 5
                br 1 (;@5;)
              end
              local.get 3
              local.get 4
              i32.const -8
              i32.add
              local.tee 4
              i32.store offset=4
              local.get 3
              local.get 3
              i32.load
              local.tee 1
              i32.const 8
              i32.add
              i32.store
              local.get 2
              i32.const 8
              i32.add
              local.get 1
              i64.load align=1
              call $_RNvNtNtCs1NJkdcuIVXo_7bincode6config3int17cast_u64_to_usize
              local.get 2
              i32.load offset=12
              local.set 1
              local.get 2
              i32.load offset=8
              local.set 5
            end
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 5
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 1
                  local.set 3
                  br 1 (;@6;)
                end
                block  ;; label = @7
                  local.get 4
                  local.get 1
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 16
                  i32.add
                  call $_RINvMNtNtCs5JHfeWeQK0x_5alloc2io5errorNtNtNtCs4NIexwV1FGX_4core2io5error5Error3newReECshuXg64jKSg3_8wasm_lib
                  local.get 2
                  i64.load offset=16
                  local.set 6
                  call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                  i32.const 12
                  i32.const 4
                  call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
                  local.tee 3
                  i32.eqz
                  br_if 4 (;@3;)
                  local.get 3
                  local.get 6
                  i64.store offset=4 align=4
                  local.get 3
                  i32.const -2147483648
                  i32.store
                  br 1 (;@6;)
                end
                local.get 3
                local.get 4
                local.get 1
                i32.sub
                i32.store offset=4
                local.get 3
                local.get 3
                i32.load
                local.tee 5
                local.get 1
                i32.add
                i32.store
                i32.const 1
                local.set 4
                block  ;; label = @7
                  local.get 1
                  i32.eqz
                  br_if 0 (;@7;)
                  call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                  local.get 1
                  i32.const 1
                  call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
                  local.tee 4
                  i32.eqz
                  br_if 5 (;@2;)
                  local.get 1
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 4
                  local.get 5
                  local.get 1
                  memory.copy
                end
                local.get 2
                i32.const 16
                i32.add
                local.get 4
                local.get 1
                call $_RNvNtNtCs4NIexwV1FGX_4core3str8converts9from_utf8
                local.get 2
                i32.load offset=16
                i32.const 1
                i32.ne
                br_if 1 (;@5;)
                local.get 2
                i64.load offset=20 align=4
                local.set 6
                call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                i32.const 12
                i32.const 4
                call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
                local.tee 3
                i32.eqz
                br_if 5 (;@1;)
                local.get 3
                local.get 6
                i64.store offset=4 align=4
                local.get 3
                i32.const -2147483647
                i32.store
                local.get 1
                i32.eqz
                br_if 0 (;@6;)
                local.get 4
                local.get 1
                i32.const 1
                call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc
              end
              local.get 0
              i32.const -2
              i32.store
              local.get 0
              local.get 3
              i32.store offset=4
              br 1 (;@4;)
            end
            local.get 0
            local.get 1
            i32.store offset=8
            local.get 0
            local.get 4
            i32.store offset=4
            local.get 0
            local.get 1
            i32.store
          end
          local.get 2
          i32.const 32
          i32.add
          global.set $__stack_pointer
          return
        end
        i32.const 4
        i32.const 12
        call $_RNvNtCs5JHfeWeQK0x_5alloc5alloc18handle_alloc_error
        unreachable
      end
      i32.const 1
      local.get 1
      call $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec12handle_error
      unreachable
    end
    i32.const 4
    i32.const 12
    call $_RNvNtCs5JHfeWeQK0x_5alloc5alloc18handle_alloc_error
    unreachable)
  (func $_RNvMs4_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB5_6RawVecNtNtB7_6string6StringE8grow_oneCshuXg64jKSg3_8wasm_lib (type 2) (param i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    i32.const 4
    i32.add
    local.get 0
    i32.load
    local.tee 2
    local.get 0
    i32.load offset=4
    local.get 2
    i32.const 1
    i32.shl
    local.tee 2
    i32.const 4
    local.get 2
    i32.const 4
    i32.gt_u
    select
    local.tee 2
    i32.const 4
    i32.const 12
    call $_RNvMs5_NtCs5JHfeWeQK0x_5alloc7raw_vecNtB5_11RawVecInner11finish_growCshuXg64jKSg3_8wasm_lib
    block  ;; label = @1
      local.get 1
      i32.load offset=4
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=8
      local.get 1
      i32.load offset=12
      call $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec12handle_error
      unreachable
    end
    local.get 1
    i32.load offset=8
    local.set 3
    local.get 0
    local.get 2
    i32.store
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRINtNtB8_6option6OptionhENtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib (type 0) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 0
        i32.load8_u
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        i32.const 1
        i32.add
        i32.store offset=12
        local.get 1
        i32.const 1048933
        i32.const 4
        local.get 2
        i32.const 12
        i32.add
        i32.const 1048696
        call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish
        local.set 0
        br 1 (;@1;)
      end
      local.get 1
      i32.const 1048929
      i32.const 4
      call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter9write_str
      local.set 0
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRNtNtCs5JHfeWeQK0x_5alloc6string6StringNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.tee 0
    i32.const 4
    i32.add
    i32.load
    local.get 0
    i32.const 8
    i32.add
    i32.load
    local.get 1
    call $_RNvXsh_NtCs4NIexwV1FGX_4core3fmteNtB5_5Debug3fmt)
  (func $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRNtNtNtB8_2io5error5ErrorNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_RNvXNtNtCs4NIexwV1FGX_4core2io5errorNtB2_5ErrorNtNtB6_3fmt5Debug3fmt)
  (func $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRNtNtNtB8_3str5error9Utf8ErrorNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib (type 0) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.load
    local.tee 0
    i32.const 4
    i32.add
    i32.store offset=12
    local.get 1
    i32.const 1048900
    i32.const 9
    i32.const 1048909
    i32.const 11
    local.get 0
    i32.const 1048868
    i32.const 1048920
    i32.const 9
    local.get 2
    i32.const 12
    i32.add
    i32.const 1048884
    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter26debug_struct_field2_finish
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRhNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib (type 0) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      local.get 1
      i32.load offset=8
      local.tee 2
      i32.const 33554432
      i32.and
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 2
        i32.const 67108864
        i32.and
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        call $_RNvXNtNtNtCs4NIexwV1FGX_4core3fmt3num3imphNtB6_7Display3fmt
        return
      end
      local.get 0
      local.get 1
      call $_RNvXsg_NtNtCs4NIexwV1FGX_4core3fmt3numhNtB7_8UpperHex3fmt
      return
    end
    local.get 0
    local.get 1
    call $_RNvXse_NtNtCs4NIexwV1FGX_4core3fmt3numhNtB7_8LowerHex3fmt)
  (func $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRjNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib (type 0) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      local.get 1
      i32.load offset=8
      local.tee 2
      i32.const 33554432
      i32.and
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 2
        i32.const 67108864
        i32.and
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        call $_RNvXs8_NtNtNtCs4NIexwV1FGX_4core3fmt3num3impmNtB9_7Display3fmt
        return
      end
      local.get 0
      local.get 1
      call $_RNvXs8_NtNtCs4NIexwV1FGX_4core3fmt3numjNtB7_8UpperHex3fmt
      return
    end
    local.get 0
    local.get 1
    call $_RNvXs6_NtNtCs4NIexwV1FGX_4core3fmt3numjNtB7_8LowerHex3fmt)
  (func $_RNvXs1i_NtCs4NIexwV1FGX_4core3fmtRDNtNtCs8n3ERUNuD4B_10serde_core2de8ExpectedEL_NtB6_7Display3fmtCshuXg64jKSg3_8wasm_lib (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 1
    call $_RNvXs1_NtCs8n3ERUNuD4B_10serde_core2deDNtB5_8ExpectedEL_NtNtCs4NIexwV1FGX_4core3fmt7Display3fmt)
  (func $_RNvXsZ_NtNtCs4NIexwV1FGX_4core3fmt3numjNtB7_5Debug3fmt (type 0) (param i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 1
      i32.load offset=8
      local.tee 2
      i32.const 33554432
      i32.and
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 2
        i32.const 67108864
        i32.and
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        call $_RNvXs8_NtNtNtCs4NIexwV1FGX_4core3fmt3num3impmNtB9_7Display3fmt
        return
      end
      local.get 0
      local.get 1
      call $_RNvXs8_NtNtCs4NIexwV1FGX_4core3fmt3numjNtB7_8UpperHex3fmt
      return
    end
    local.get 0
    local.get 1
    call $_RNvXs6_NtNtCs4NIexwV1FGX_4core3fmt3numjNtB7_8LowerHex3fmt)
  (func $_RNvXsn_NtCs5JHfeWeQK0x_5alloc5boxedINtB5_3BoxNtNtCs1NJkdcuIVXo_7bincode5error9ErrorKindENtNtCs4NIexwV1FGX_4core3fmt5Debug3fmtCshuXg64jKSg3_8wasm_lib (type 0) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        i32.load
                        local.tee 3
                        i32.load
                        local.tee 0
                        i32.const -2147483648
                        i32.xor
                        i32.const 8
                        local.get 0
                        i32.const 0
                        i32.lt_s
                        select
                        br_table 0 (;@10;) 1 (;@9;) 2 (;@8;) 3 (;@7;) 4 (;@6;) 5 (;@5;) 6 (;@4;) 7 (;@3;) 8 (;@2;) 0 (;@10;)
                      end
                      local.get 2
                      local.get 3
                      i32.const 4
                      i32.add
                      i32.store offset=12
                      local.get 1
                      i32.const 1048656
                      i32.const 2
                      local.get 2
                      i32.const 12
                      i32.add
                      i32.const 1048640
                      call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish
                      local.set 1
                      br 8 (;@1;)
                    end
                    local.get 2
                    local.get 3
                    i32.const 4
                    i32.add
                    i32.store offset=12
                    local.get 1
                    i32.const 1048676
                    i32.const 19
                    local.get 2
                    i32.const 12
                    i32.add
                    i32.const 1048660
                    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish
                    local.set 1
                    br 7 (;@1;)
                  end
                  local.get 2
                  local.get 3
                  i32.const 4
                  i32.add
                  i32.store offset=12
                  local.get 1
                  i32.const 1048712
                  i32.const 19
                  local.get 2
                  i32.const 12
                  i32.add
                  i32.const 1048696
                  call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish
                  local.set 1
                  br 6 (;@1;)
                end
                local.get 1
                i32.const 1048731
                i32.const 19
                call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter9write_str
                local.set 1
                br 5 (;@1;)
              end
              local.get 2
              local.get 3
              i32.const 4
              i32.add
              i32.store offset=12
              local.get 1
              i32.const 1048768
              i32.const 18
              local.get 2
              i32.const 12
              i32.add
              i32.const 1048752
              call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish
              local.set 1
              br 4 (;@1;)
            end
            local.get 1
            i32.const 1048786
            i32.const 26
            call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter9write_str
            local.set 1
            br 3 (;@1;)
          end
          local.get 1
          i32.const 1048812
          i32.const 9
          call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter9write_str
          local.set 1
          br 2 (;@1;)
        end
        local.get 1
        i32.const 1048821
        i32.const 22
        call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter9write_str
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 3
      i32.store offset=12
      local.get 1
      i32.const 1048860
      i32.const 6
      local.get 2
      i32.const 12
      i32.add
      i32.const 1048844
      call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish
      local.set 1
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_RNvYINtNtCs5JHfeWeQK0x_5alloc5boxed3BoxNtNtCs1NJkdcuIVXo_7bincode5error9ErrorKindENtNtCs8n3ERUNuD4B_10serde_core2de5Error14invalid_lengthCshuXg64jKSg3_8wasm_lib (type 4) (param i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    i32.const 1048624
    i32.store offset=12
    local.get 1
    i32.const 1048616
    i32.store offset=8
    local.get 1
    local.get 0
    i32.store offset=4
    local.get 1
    i32.const 1
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 1
    i32.const 8
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=24
    local.get 1
    i32.const 2
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 1
    i32.const 4
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=16
    local.get 1
    i32.const 36
    i32.add
    i32.const 1049214
    local.get 1
    i32.const 16
    i32.add
    call $_RNvNvNtCs5JHfeWeQK0x_5alloc3fmt6format12format_inner
    call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block  ;; label = @1
      i32.const 12
      i32.const 4
      call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
      local.tee 0
      br_if 0 (;@1;)
      i32.const 4
      i32.const 12
      call $_RNvNtCs5JHfeWeQK0x_5alloc5alloc18handle_alloc_error
      unreachable
    end
    local.get 0
    local.get 1
    i32.load offset=44
    i32.store offset=8
    local.get 0
    local.get 1
    i64.load offset=36 align=4
    i64.store align=4
    local.get 1
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_RNvYNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4from11StringErrorBX_11descriptionCshuXg64jKSg3_8wasm_lib (type 1) (param i32 i32)
    local.get 0
    i32.const 40
    i32.store offset=4
    local.get 0
    i32.const 1049927
    i32.store)
  (func $_RNvYNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4from11StringErrorBX_5causeCshuXg64jKSg3_8wasm_lib (type 1) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store)
  (func $_RNvYNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4from11StringErrorBX_7provideCshuXg64jKSg3_8wasm_lib (type 5) (param i32 i32 i32))
  (func $_RNvYNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4from11StringErrorBX_7type_idCshuXg64jKSg3_8wasm_lib (type 1) (param i32 i32)
    local.get 0
    i32.const 0
    i64.load offset=1049976 align=4
    i64.store offset=8 align=4
    local.get 0
    i32.const 0
    i64.load offset=1049968 align=4
    i64.store align=4)
  (func $entry_point (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i64 i32 i32)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 1
    i32.store offset=48
    local.get 2
    local.get 0
    i32.store offset=44
    local.get 2
    i32.const 2
    i32.store offset=56
    local.get 2
    local.get 2
    i32.const 44
    i32.add
    i32.store offset=52
    local.get 2
    i32.const 16
    i32.add
    local.get 2
    i32.const 52
    i32.add
    call $_RINvYINtNvXs1_NtCs1NJkdcuIVXo_7bincode2deQINtBc_12DeserializerppENtNtCs8n3ERUNuD4B_10serde_core2de12Deserializer17deserialize_tuple6AccessNtNtBc_4read11SliceReaderINtNtBe_6config14WithOtherLimitINtB2E_17WithOtherTrailingINtB2E_20WithOtherIntEncodingNtB2E_14DefaultOptionsNtNtB2E_3int14FixintEncodingENtNtB2E_8trailing13AllowTrailingENtNtB2E_5limit8InfiniteEENtB13_9SeqAccess12next_elementNtNtCs5JHfeWeQK0x_5alloc6string6StringECshuXg64jKSg3_8wasm_lib
    local.get 2
    i32.load offset=20
    local.set 3
    block  ;; label = @1
      local.get 2
      i32.load offset=16
      local.tee 4
      i32.const -2
      i32.eq
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 4
                            i32.const -1
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 2
                            i32.load offset=56
                            i32.eqz
                            br_if 1 (;@11;)
                            local.get 2
                            i32.load offset=24
                            local.set 5
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 2
                                i32.load offset=52
                                local.tee 1
                                i32.load offset=4
                                local.tee 0
                                i32.const 7
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 2
                                i64.const 9473
                                i64.store offset=16
                                local.get 2
                                i32.const 16
                                i32.add
                                call $_RNvXs_NtCs1NJkdcuIVXo_7bincode5errorINtNtCs5JHfeWeQK0x_5alloc5boxed3BoxNtB4_9ErrorKindEINtNtCs4NIexwV1FGX_4core7convert4FromNtNtNtB1s_2io5error5ErrorE4from
                                local.set 6
                                i32.const 1
                                local.set 0
                                br 1 (;@13;)
                              end
                              local.get 1
                              local.get 0
                              i32.const -8
                              i32.add
                              i32.store offset=4
                              local.get 1
                              local.get 1
                              i32.load
                              local.tee 0
                              i32.const 8
                              i32.add
                              i32.store
                              local.get 2
                              i32.const 8
                              i32.add
                              local.get 0
                              i64.load align=1
                              call $_RNvNtNtCs1NJkdcuIVXo_7bincode6config3int17cast_u64_to_usize
                              local.get 2
                              i32.load offset=12
                              local.set 6
                              local.get 2
                              i32.load offset=8
                              local.set 0
                            end
                            local.get 0
                            i32.const 1
                            i32.and
                            br_if 10 (;@2;)
                            local.get 2
                            local.get 6
                            i32.store offset=64
                            local.get 2
                            local.get 1
                            i32.store offset=60
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 6
                                br_if 0 (;@14;)
                                i32.const 4
                                local.set 7
                                i32.const 0
                                local.set 1
                                br 1 (;@13;)
                              end
                              call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                              local.get 6
                              i32.const 87381
                              local.get 6
                              i32.const 87381
                              i32.lt_u
                              select
                              local.tee 1
                              i32.const 12
                              i32.mul
                              local.tee 0
                              i32.const 4
                              call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
                              local.tee 7
                              i32.eqz
                              br_if 4 (;@9;)
                            end
                            local.get 2
                            i32.const 0
                            i32.store offset=76
                            local.get 2
                            local.get 7
                            i32.store offset=72
                            local.get 2
                            local.get 1
                            i32.store offset=68
                            local.get 2
                            i32.const 16
                            i32.add
                            local.get 2
                            i32.const 60
                            i32.add
                            call $_RINvYINtNvXs1_NtCs1NJkdcuIVXo_7bincode2deQINtBc_12DeserializerppENtNtCs8n3ERUNuD4B_10serde_core2de12Deserializer17deserialize_tuple6AccessNtNtBc_4read11SliceReaderINtNtBe_6config14WithOtherLimitINtB2E_17WithOtherTrailingINtB2E_20WithOtherIntEncodingNtB2E_14DefaultOptionsNtNtB2E_3int14FixintEncodingENtNtB2E_8trailing13AllowTrailingENtNtB2E_5limit8InfiniteEENtB13_9SeqAccess12next_elementNtNtCs5JHfeWeQK0x_5alloc6string6StringECshuXg64jKSg3_8wasm_lib
                            local.get 2
                            i32.load offset=16
                            local.tee 0
                            i32.const -2
                            i32.eq
                            br_if 8 (;@4;)
                            i32.const 0
                            local.set 1
                            i32.const 0
                            local.set 8
                            loop  ;; label = @13
                              local.get 0
                              i32.const -1
                              i32.eq
                              br_if 3 (;@10;)
                              local.get 2
                              i64.load offset=20 align=4
                              local.set 9
                              block  ;; label = @14
                                local.get 1
                                local.get 2
                                i32.load offset=68
                                i32.ne
                                br_if 0 (;@14;)
                                local.get 2
                                i32.const 68
                                i32.add
                                call $_RNvMs4_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB5_6RawVecNtNtB7_6string6StringE8grow_oneCshuXg64jKSg3_8wasm_lib
                                local.get 2
                                i32.load offset=72
                                local.set 7
                              end
                              local.get 7
                              local.get 8
                              i32.add
                              local.tee 10
                              local.get 0
                              i32.store
                              local.get 10
                              i32.const 4
                              i32.add
                              local.get 9
                              i64.store align=4
                              local.get 2
                              local.get 1
                              i32.const 1
                              i32.add
                              local.tee 1
                              i32.store offset=76
                              local.get 8
                              i32.const 12
                              i32.add
                              local.set 8
                              local.get 2
                              i32.const 16
                              i32.add
                              local.get 2
                              i32.const 60
                              i32.add
                              call $_RINvYINtNvXs1_NtCs1NJkdcuIVXo_7bincode2deQINtBc_12DeserializerppENtNtCs8n3ERUNuD4B_10serde_core2de12Deserializer17deserialize_tuple6AccessNtNtBc_4read11SliceReaderINtNtBe_6config14WithOtherLimitINtB2E_17WithOtherTrailingINtB2E_20WithOtherIntEncodingNtB2E_14DefaultOptionsNtNtB2E_3int14FixintEncodingENtNtB2E_8trailing13AllowTrailingENtNtB2E_5limit8InfiniteEENtB13_9SeqAccess12next_elementNtNtCs5JHfeWeQK0x_5alloc6string6StringECshuXg64jKSg3_8wasm_lib
                              local.get 2
                              i32.load offset=16
                              local.tee 0
                              i32.const -2
                              i32.ne
                              br_if 0 (;@13;)
                            end
                            local.get 2
                            i32.load offset=20
                            local.set 6
                            local.get 2
                            i32.load offset=72
                            local.set 7
                            local.get 1
                            i32.eqz
                            br_if 9 (;@3;)
                            local.get 7
                            local.set 0
                            loop  ;; label = @13
                              block  ;; label = @14
                                local.get 0
                                i32.load
                                local.tee 8
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 0
                                i32.const 4
                                i32.add
                                i32.load
                                local.get 8
                                i32.const 1
                                call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc
                              end
                              local.get 0
                              i32.const 12
                              i32.add
                              local.set 0
                              local.get 1
                              i32.const -1
                              i32.add
                              local.tee 1
                              br_if 0 (;@13;)
                              br 10 (;@3;)
                            end
                          end
                          i32.const 0
                          call $_RNvYINtNtCs5JHfeWeQK0x_5alloc5boxed3BoxNtNtCs1NJkdcuIVXo_7bincode5error9ErrorKindENtNtCs8n3ERUNuD4B_10serde_core2de5Error14invalid_lengthCshuXg64jKSg3_8wasm_lib
                          local.set 3
                          br 10 (;@1;)
                        end
                        i32.const 1
                        call $_RNvYINtNtCs5JHfeWeQK0x_5alloc5boxed3BoxNtNtCs1NJkdcuIVXo_7bincode5error9ErrorKindENtNtCs8n3ERUNuD4B_10serde_core2de5Error14invalid_lengthCshuXg64jKSg3_8wasm_lib
                        local.set 6
                        br 8 (;@2;)
                      end
                      local.get 2
                      i32.load offset=72
                      local.set 6
                      local.get 2
                      i32.load offset=68
                      local.tee 0
                      i32.const 2
                      i32.add
                      i32.const 2
                      i32.lt_u
                      br_if 7 (;@2;)
                      local.get 2
                      local.get 1
                      i32.store offset=36
                      local.get 2
                      local.get 6
                      i32.store offset=32
                      local.get 2
                      local.get 0
                      i32.store offset=28
                      local.get 2
                      local.get 5
                      i32.store offset=24
                      local.get 2
                      local.get 3
                      i32.store offset=20
                      local.get 2
                      local.get 4
                      i32.store offset=16
                      call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                      i32.const 8
                      i32.const 1
                      call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
                      local.tee 10
                      i32.eqz
                      br_if 1 (;@8;)
                      local.get 10
                      i64.const 7308604895911175534
                      i64.store align=1
                      block  ;; label = @10
                        local.get 0
                        local.get 1
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 2
                        i32.const 28
                        i32.add
                        call $_RNvMs4_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB5_6RawVecNtNtB7_6string6StringE8grow_oneCshuXg64jKSg3_8wasm_lib
                        local.get 2
                        i32.load offset=24
                        local.set 5
                        local.get 2
                        i32.load offset=32
                        local.set 6
                      end
                      local.get 6
                      local.get 8
                      i32.add
                      local.tee 3
                      i32.const 8
                      i32.store
                      local.get 3
                      i32.const 8
                      i32.add
                      i32.const 8
                      i32.store
                      local.get 3
                      i32.const 4
                      i32.add
                      local.get 10
                      i32.store
                      local.get 2
                      local.get 1
                      i32.const 1
                      i32.add
                      local.tee 11
                      i32.store offset=36
                      local.get 5
                      i32.const 16
                      i32.add
                      local.set 10
                      block  ;; label = @10
                        local.get 11
                        i32.eqz
                        br_if 0 (;@10;)
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 8
                            i32.const 12
                            i32.div_u
                            i32.const 1
                            i32.add
                            i32.const 7
                            i32.and
                            br_if 0 (;@12;)
                            local.get 6
                            local.set 0
                            br 1 (;@11;)
                          end
                          local.get 8
                          i32.const 12
                          i32.div_u
                          i32.const 1
                          i32.add
                          i32.const 7
                          i32.and
                          local.set 7
                          local.get 6
                          local.set 0
                          loop  ;; label = @12
                            local.get 0
                            local.tee 4
                            i32.const 12
                            i32.add
                            local.set 0
                            local.get 10
                            local.get 4
                            i32.const 8
                            i32.add
                            i32.load
                            i32.add
                            i32.const 8
                            i32.add
                            local.set 10
                            local.get 7
                            i32.const -1
                            i32.add
                            local.tee 7
                            br_if 0 (;@12;)
                          end
                        end
                        local.get 8
                        i32.const 84
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const 12
                        i32.add
                        local.set 7
                        loop  ;; label = @11
                          local.get 10
                          local.get 0
                          i32.const 8
                          i32.add
                          i32.load
                          i32.add
                          local.get 0
                          i32.const 20
                          i32.add
                          i32.load
                          i32.add
                          local.get 0
                          i32.const 32
                          i32.add
                          i32.load
                          i32.add
                          local.get 0
                          i32.const 44
                          i32.add
                          i32.load
                          i32.add
                          local.get 0
                          i32.const 56
                          i32.add
                          i32.load
                          i32.add
                          local.get 0
                          i32.const 68
                          i32.add
                          i32.load
                          i32.add
                          local.get 0
                          i32.const 80
                          i32.add
                          i32.load
                          i32.add
                          local.get 0
                          i32.const 92
                          i32.add
                          i32.load
                          i32.add
                          i32.const 64
                          i32.add
                          local.set 10
                          local.get 7
                          local.get 0
                          i32.const 96
                          i32.add
                          local.tee 0
                          i32.ne
                          br_if 0 (;@11;)
                        end
                      end
                      i32.const 0
                      local.set 7
                      local.get 10
                      i32.const 0
                      i32.lt_s
                      br_if 2 (;@7;)
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 10
                            br_if 0 (;@12;)
                            local.get 2
                            i32.const 0
                            i32.store offset=76
                            local.get 2
                            i64.const 4294967296
                            i64.store offset=68 align=4
                            local.get 2
                            i32.load offset=20
                            local.set 4
                            br 1 (;@11;)
                          end
                          call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                          i32.const 1
                          local.set 7
                          local.get 10
                          i32.const 1
                          call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
                          local.tee 0
                          i32.eqz
                          br_if 4 (;@7;)
                          i32.const 0
                          local.set 7
                          local.get 2
                          i32.const 0
                          i32.store offset=76
                          local.get 2
                          local.get 0
                          i32.store offset=72
                          local.get 2
                          local.get 10
                          i32.store offset=68
                          local.get 2
                          i32.load offset=20
                          local.set 4
                          local.get 10
                          i32.const 7
                          i32.gt_u
                          br_if 1 (;@10;)
                        end
                        local.get 2
                        i32.const 68
                        i32.add
                        i32.const 0
                        i32.const 8
                        call $_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshuXg64jKSg3_8wasm_lib
                        local.get 2
                        i32.load offset=72
                        local.set 0
                        local.get 2
                        i32.load offset=68
                        local.set 10
                        local.get 2
                        i32.load offset=76
                        local.set 7
                      end
                      local.get 0
                      local.get 7
                      i32.add
                      local.get 5
                      i64.extend_i32_u
                      i64.store align=1
                      local.get 2
                      local.get 7
                      i32.const 8
                      i32.add
                      local.tee 7
                      i32.store offset=76
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 5
                            local.get 10
                            local.get 7
                            i32.sub
                            i32.le_u
                            br_if 0 (;@12;)
                            local.get 2
                            i32.const 68
                            i32.add
                            local.get 7
                            local.get 5
                            call $_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshuXg64jKSg3_8wasm_lib
                            local.get 2
                            i32.load offset=68
                            local.set 10
                            local.get 2
                            i32.load offset=72
                            local.set 0
                            local.get 2
                            i32.load offset=76
                            local.set 7
                            br 1 (;@11;)
                          end
                          local.get 5
                          i32.eqz
                          br_if 1 (;@10;)
                        end
                        local.get 5
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 7
                        i32.add
                        local.get 4
                        local.get 5
                        memory.copy
                      end
                      local.get 2
                      local.get 7
                      local.get 5
                      i32.add
                      local.tee 7
                      i32.store offset=76
                      local.get 11
                      i64.extend_i32_u
                      local.set 9
                      block  ;; label = @10
                        local.get 10
                        local.get 7
                        i32.sub
                        i32.const 7
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 2
                        i32.const 68
                        i32.add
                        local.get 7
                        i32.const 8
                        call $_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshuXg64jKSg3_8wasm_lib
                        local.get 2
                        i32.load offset=72
                        local.set 0
                        local.get 2
                        i32.load offset=76
                        local.set 7
                      end
                      local.get 0
                      local.get 7
                      i32.add
                      local.get 9
                      i64.store align=1
                      local.get 2
                      local.get 7
                      i32.const 8
                      i32.add
                      local.tee 0
                      i32.store offset=76
                      block  ;; label = @10
                        local.get 1
                        i32.const -1
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 8
                        i32.const 12
                        i32.add
                        local.set 7
                        local.get 6
                        i32.const 8
                        i32.add
                        local.set 8
                        local.get 2
                        i32.load offset=68
                        local.set 10
                        loop  ;; label = @11
                          local.get 8
                          i32.const -4
                          i32.add
                          i32.load
                          local.set 6
                          local.get 8
                          i32.load
                          local.tee 1
                          i64.extend_i32_u
                          local.set 9
                          block  ;; label = @12
                            local.get 10
                            local.get 0
                            i32.sub
                            i32.const 7
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 2
                            i32.const 68
                            i32.add
                            local.get 0
                            i32.const 8
                            call $_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshuXg64jKSg3_8wasm_lib
                            local.get 2
                            i32.load offset=68
                            local.set 10
                            local.get 2
                            i32.load offset=76
                            local.set 0
                          end
                          local.get 2
                          i32.load offset=72
                          local.tee 4
                          local.get 0
                          i32.add
                          local.get 9
                          i64.store align=1
                          local.get 2
                          local.get 0
                          i32.const 8
                          i32.add
                          local.tee 0
                          i32.store offset=76
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 1
                                local.get 10
                                local.get 0
                                i32.sub
                                i32.le_u
                                br_if 0 (;@14;)
                                local.get 2
                                i32.const 68
                                i32.add
                                local.get 0
                                local.get 1
                                call $_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshuXg64jKSg3_8wasm_lib
                                local.get 2
                                i32.load offset=72
                                local.set 4
                                local.get 2
                                i32.load offset=68
                                local.set 10
                                local.get 2
                                i32.load offset=76
                                local.set 0
                                br 1 (;@13;)
                              end
                              local.get 1
                              i32.eqz
                              br_if 1 (;@12;)
                            end
                            local.get 1
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 4
                            local.get 0
                            i32.add
                            local.get 6
                            local.get 1
                            memory.copy
                          end
                          local.get 2
                          local.get 0
                          local.get 1
                          i32.add
                          local.tee 0
                          i32.store offset=76
                          local.get 8
                          i32.const 12
                          i32.add
                          local.set 8
                          local.get 7
                          i32.const -12
                          i32.add
                          local.tee 7
                          br_if 0 (;@11;)
                        end
                      end
                      local.get 2
                      i32.load offset=72
                      local.set 1
                      local.get 2
                      i32.load offset=68
                      i32.const -1
                      i32.eq
                      br_if 3 (;@6;)
                      local.get 2
                      i32.load offset=76
                      local.set 0
                      call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                      i32.const 8
                      i32.const 4
                      call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
                      local.tee 10
                      i32.eqz
                      br_if 4 (;@5;)
                      local.get 10
                      local.get 0
                      i32.store offset=4
                      local.get 10
                      local.get 1
                      i32.store
                      block  ;; label = @10
                        local.get 2
                        i32.load offset=16
                        local.tee 1
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 2
                        i32.load offset=20
                        local.get 1
                        i32.const 1
                        call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc
                      end
                      local.get 2
                      i32.load offset=32
                      local.set 7
                      block  ;; label = @10
                        local.get 2
                        i32.load offset=36
                        local.tee 0
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 7
                        local.set 1
                        loop  ;; label = @11
                          block  ;; label = @12
                            local.get 1
                            i32.load
                            local.tee 8
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 1
                            i32.const 4
                            i32.add
                            i32.load
                            local.get 8
                            i32.const 1
                            call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc
                          end
                          local.get 1
                          i32.const 12
                          i32.add
                          local.set 1
                          local.get 0
                          i32.const -1
                          i32.add
                          local.tee 0
                          br_if 0 (;@11;)
                        end
                      end
                      block  ;; label = @10
                        local.get 2
                        i32.load offset=28
                        local.tee 1
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 7
                        local.get 1
                        i32.const 12
                        i32.mul
                        i32.const 4
                        call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc
                      end
                      local.get 2
                      i32.const 80
                      i32.add
                      global.set $__stack_pointer
                      local.get 10
                      return
                    end
                    i32.const 4
                    local.get 0
                    call $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec12handle_error
                    unreachable
                  end
                  i32.const 1
                  i32.const 8
                  call $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec12handle_error
                  unreachable
                end
                local.get 7
                local.get 10
                call $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec12handle_error
                unreachable
              end
              local.get 2
              local.get 1
              i32.store offset=68
              i32.const 1050000
              i32.const 43
              local.get 2
              i32.const 68
              i32.add
              i32.const 1049984
              i32.const 1050044
              call $_RNvNtCs4NIexwV1FGX_4core6result13unwrap_failed
              unreachable
            end
            i32.const 4
            i32.const 8
            call $_RNvNtCs5JHfeWeQK0x_5alloc5alloc18handle_alloc_error
            unreachable
          end
          local.get 2
          i32.load offset=20
          local.set 6
        end
        local.get 2
        i32.load offset=68
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 7
        local.get 1
        i32.const 12
        i32.mul
        i32.const 4
        call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc
      end
      block  ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 4
        i32.const 1
        call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc
      end
      local.get 6
      local.set 3
    end
    local.get 2
    local.get 3
    i32.store offset=16
    i32.const 1050000
    i32.const 43
    local.get 2
    i32.const 16
    i32.add
    i32.const 1049984
    i32.const 1050060
    call $_RNvNtCs4NIexwV1FGX_4core6result13unwrap_failed
    unreachable)
  (func $ns_free (type 5) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc)
  (func $ns_malloc (type 0) (param i32 i32) (result i32)
    call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    local.get 0
    local.get 1
    call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc)
  (func $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $_RNvCs5fGHjzWXRav_7___rustc11___rdl_alloc
    return)
  (func $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc (type 5) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $_RNvCs5fGHjzWXRav_7___rustc13___rdl_dealloc
    return)
  (func $_RNvCs5fGHjzWXRav_7___rustc14___rust_realloc (type 7) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $_RNvCs5fGHjzWXRav_7___rustc13___rdl_realloc
    return)
  (func $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2 (type 8)
    return)
  (func $_RNvNtNtCs1NJkdcuIVXo_7bincode6config3int17cast_u64_to_usize (type 9) (param i32 i64)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 1
    i64.store offset=8
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i64.const 4294967296
          i64.lt_u
          br_if 0 (;@3;)
          local.get 2
          i32.const 2
          i64.extend_i32_u
          i64.const 32
          i64.shl
          i32.const 1050076
          i64.extend_i32_u
          i64.or
          i64.store offset=40
          local.get 2
          i32.const 20
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.get 2
          i32.const 8
          i32.add
          i64.extend_i32_u
          i64.or
          i64.store offset=32
          local.get 2
          i32.const 20
          i32.add
          i32.const 1049867
          local.get 2
          i32.const 32
          i32.add
          call $_RNvNvNtCs5JHfeWeQK0x_5alloc3fmt6format12format_inner
          call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
          i32.const 12
          i32.const 4
          call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
          local.tee 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          local.get 2
          i32.load offset=28
          i32.store offset=8
          local.get 3
          local.get 2
          i64.load offset=20 align=4
          i64.store align=4
          i32.const 1
          local.set 4
          br 1 (;@2;)
        end
        local.get 1
        i32.wrap_i64
        local.set 3
        i32.const 0
        local.set 4
      end
      local.get 0
      local.get 3
      i32.store offset=4
      local.get 0
      local.get 4
      i32.store
      local.get 2
      i32.const 48
      i32.add
      global.set $__stack_pointer
      return
    end
    i32.const 4
    i32.const 12
    call $_RNvNtCs5JHfeWeQK0x_5alloc5alloc18handle_alloc_error
    unreachable)
  (func $_RNvXs_NtCs1NJkdcuIVXo_7bincode5errorINtNtCs5JHfeWeQK0x_5alloc5boxed3BoxNtB4_9ErrorKindEINtNtCs4NIexwV1FGX_4core7convert4FromNtNtNtB1s_2io5error5ErrorE4from (type 4) (param i32) (result i32)
    (local i64)
    local.get 0
    i64.load align=4
    local.set 1
    call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block  ;; label = @1
      i32.const 12
      i32.const 4
      call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
      local.tee 0
      br_if 0 (;@1;)
      i32.const 4
      i32.const 12
      call $_RNvNtCs5JHfeWeQK0x_5alloc5alloc18handle_alloc_error
      unreachable
    end
    local.get 0
    local.get 1
    i64.store offset=4 align=4
    local.get 0
    i32.const -2147483648
    i32.store
    local.get 0)
  (func $_RNvXs0_NtCs8n3ERUNuD4B_10serde_core2deReNtB5_8Expected3fmt (type 0) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter9write_str)
  (func $_RNvXs1_NtCs8n3ERUNuD4B_10serde_core2deDNtB5_8ExpectedEL_NtNtCs4NIexwV1FGX_4core3fmt7Display3fmt (type 3) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 2
    local.get 1
    i32.load offset=12
    call_indirect (type 0))
  (func $_RNvCs5fGHjzWXRav_7___rustc18___rust_start_panic (type 0) (param i32 i32) (result i32)
    call $_RNvCs5fGHjzWXRav_7___rustc12___rust_abort
    unreachable)
  (func $_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueNtNtCs5JHfeWeQK0x_5alloc6string6StringECsiuSwmoQr08c_3std (type 2) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc
    end)
  (func $_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueNtNvNtCsiuSwmoQr08c_3std9panicking13panic_handler19FormatStringPayloadEBH_ (type 2) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.const 1
      i32.lt_s
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc
    end)
  (func $_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsiuSwmoQr08c_3std (type 10) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    block  ;; label = @1
      local.get 2
      local.get 1
      i32.add
      local.tee 1
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      call $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec12handle_error
      unreachable
    end
    local.get 5
    i32.const 4
    i32.add
    local.get 0
    i32.load
    local.tee 2
    local.get 0
    i32.load offset=4
    local.get 1
    local.get 2
    i32.const 1
    i32.shl
    local.tee 2
    local.get 1
    local.get 2
    i32.gt_u
    select
    local.tee 2
    i32.const 8
    i32.const 4
    local.get 4
    i32.const 1
    i32.eq
    select
    local.tee 1
    local.get 2
    local.get 1
    i32.gt_u
    select
    local.tee 2
    local.get 3
    local.get 4
    call $_RNvMs5_NtCs5JHfeWeQK0x_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsiuSwmoQr08c_3std
    block  ;; label = @1
      local.get 5
      i32.load offset=4
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=8
      local.get 5
      i32.load offset=12
      call $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec12handle_error
      unreachable
    end
    local.get 5
    i32.load offset=8
    local.set 4
    local.get 0
    local.get 2
    i32.store
    local.get 0
    local.get 4
    i32.store offset=4
    local.get 5
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_RINvNtNtCsiuSwmoQr08c_3std3sys9backtrace26___rust_end_short_backtraceNCNvNtB6_5alloc8rust_oom0zEB6_ (type 2) (param i32)
    local.get 0
    call $_RNCNvNtCsiuSwmoQr08c_3std5alloc8rust_oom0B5_
    unreachable)
  (func $_RNCNvNtCsiuSwmoQr08c_3std5alloc8rust_oom0B5_ (type 2) (param i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    i32.const 0
    i32.load offset=1055696
    local.tee 0
    i32.const 21
    local.get 0
    select
    call_indirect (type 1)
    unreachable)
  (func $_RINvNtNtCsiuSwmoQr08c_3std3sys9backtrace26___rust_end_short_backtraceNCNvNtB6_9panicking13panic_handler0zEB6_ (type 2) (param i32)
    local.get 0
    call $_RNCNvNtCsiuSwmoQr08c_3std9panicking13panic_handler0B5_
    unreachable)
  (func $_RNCNvNtCsiuSwmoQr08c_3std9panicking13panic_handler0B5_ (type 2) (param i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 2
      i32.load offset=4
      local.tee 3
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.load
      local.set 2
      local.get 1
      local.get 3
      i32.const 1
      i32.shr_u
      i32.store offset=4
      local.get 1
      local.get 2
      i32.store
      local.get 1
      i32.const 1050104
      local.get 0
      i32.load offset=4
      local.get 0
      i32.load offset=8
      local.tee 0
      i32.load8_u offset=8
      local.get 0
      i32.load8_u offset=9
      call $_RNvNtCsiuSwmoQr08c_3std9panicking15panic_with_hook
      unreachable
    end
    local.get 1
    i32.const -1
    i32.store
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.const 1050132
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    local.tee 0
    i32.load8_u offset=8
    local.get 0
    i32.load8_u offset=9
    call $_RNvNtCsiuSwmoQr08c_3std9panicking15panic_with_hook
    unreachable)
  (func $_RNvMs5_NtCs5JHfeWeQK0x_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsiuSwmoQr08c_3std (type 6) (param i32 i32 i32 i32 i32 i32)
    (local i32 i32 i64)
    i32.const 1
    local.set 6
    i32.const 4
    local.set 7
    block  ;; label = @1
      block  ;; label = @2
        local.get 5
        i64.extend_i32_u
        local.get 3
        i64.extend_i32_u
        i64.mul
        local.tee 8
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 8
        i32.wrap_i64
        local.tee 3
        i32.const -2147483648
        local.get 4
        i32.sub
        i32.le_u
        br_if 0 (;@2;)
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              local.get 5
              local.get 1
              i32.mul
              local.get 4
              local.get 3
              call $_RNvCs5fGHjzWXRav_7___rustc14___rust_realloc
              local.set 7
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 3
              br_if 0 (;@5;)
              local.get 4
              local.set 7
              br 2 (;@3;)
            end
            call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
            local.get 3
            local.get 4
            call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
            local.set 7
          end
          local.get 7
          br_if 0 (;@3;)
          local.get 0
          local.get 4
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 0
        local.get 7
        i32.store offset=4
        i32.const 0
        local.set 6
      end
      i32.const 8
      local.set 7
    end
    local.get 0
    local.get 7
    i32.add
    local.get 3
    i32.store
    local.get 0
    local.get 6
    i32.store)
  (func $_RNvNtCsiuSwmoQr08c_3std9panicking15panic_with_hook (type 10) (param i32 i32 i32 i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 1
                    call $_RNvNtNtCsiuSwmoQr08c_3std9panicking11panic_count8increase
                    i32.const 255
                    i32.and
                    br_table 4 (;@4;) 1 (;@7;) 0 (;@8;) 1 (;@7;)
                  end
                  i32.const 0
                  i32.load offset=1055700
                  local.tee 6
                  i32.const -1
                  i32.le_s
                  br_if 3 (;@4;)
                  local.get 6
                  i32.const 1
                  i32.add
                  local.tee 7
                  local.get 6
                  i32.lt_s
                  br_if 4 (;@3;)
                  i32.const 0
                  local.get 7
                  i32.store offset=1055700
                  i32.const 0
                  i32.load offset=1055704
                  br_if 1 (;@6;)
                  i32.const 0
                  local.get 7
                  i32.const -1
                  i32.add
                  i32.store offset=1055700
                  br 2 (;@5;)
                end
                local.get 5
                local.get 0
                local.get 1
                i32.load offset=24
                call_indirect (type 1)
                unreachable
              end
              local.get 5
              i32.const 8
              i32.add
              local.get 0
              local.get 1
              i32.load offset=20
              call_indirect (type 1)
              local.get 5
              local.get 4
              i32.store8 offset=29
              local.get 5
              local.get 3
              i32.store8 offset=28
              local.get 5
              local.get 2
              i32.store offset=24
              local.get 5
              local.get 5
              i64.load offset=8
              i64.store offset=16 align=4
              i32.const 0
              i32.load offset=1055704
              local.get 5
              i32.const 16
              i32.add
              i32.const 0
              i32.load offset=1055708
              i32.load offset=20
              call_indirect (type 1)
              i32.const 0
              i32.const 0
              i32.load offset=1055700
              local.tee 5
              i32.const -1
              i32.add
              i32.store offset=1055700
              local.get 5
              i32.const 0
              i32.le_s
              br_if 3 (;@2;)
            end
            i32.const 0
            i32.const 0
            i32.store8 offset=1055692
            local.get 3
            br_if 3 (;@1;)
          end
          unreachable
        end
        i32.const 1050336
        i32.const 28
        i32.const 1050364
        call $_RNvNtCs4NIexwV1FGX_4core6option13expect_failed
        unreachable
      end
      i32.const 1050412
      i32.const 77
      i32.const 1050452
      call $_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt
      unreachable
    end
    local.get 0
    local.get 1
    call $_RNvCs5fGHjzWXRav_7___rustc10rust_panic
    unreachable)
  (func $_RNvNtCsiuSwmoQr08c_3std5alloc24default_alloc_error_hook (type 1) (param i32 i32)
    i32.const 0
    i32.const 1
    i32.store8 offset=1055716)
  (func $_RNvCs5fGHjzWXRav_7___rustc10rust_panic (type 1) (param i32 i32)
    local.get 0
    local.get 1
    call $_RNvCs5fGHjzWXRav_7___rustc18___rust_start_panic
    drop
    unreachable)
  (func $_RNvCs5fGHjzWXRav_7___rustc11___rdl_alloc (type 0) (param i32 i32) (result i32)
    block  ;; label = @1
      local.get 1
      i32.const 9
      i32.lt_u
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE8memalignCsiuSwmoQr08c_3std
      return
    end
    local.get 0
    call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE6mallocCsiuSwmoQr08c_3std)
  (func $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE8memalignCsiuSwmoQr08c_3std (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 2
    block  ;; label = @1
      local.get 1
      i32.const -48
      i32.const 0
      i32.load offset=1055676
      i32.sub
      i32.const -9
      i32.and
      i32.const -3
      i32.add
      local.tee 3
      i32.const -64
      local.get 3
      i32.const -64
      i32.lt_u
      select
      local.get 0
      i32.const 16
      local.get 0
      i32.const 16
      i32.gt_u
      select
      local.tee 0
      i32.sub
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 16
      local.get 1
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get 1
      i32.const 11
      i32.lt_u
      select
      local.tee 3
      i32.add
      i32.const 12
      i32.add
      call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE6mallocCsiuSwmoQr08c_3std
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const -8
      i32.add
      local.set 2
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.const -1
          i32.add
          local.tee 4
          local.get 1
          i32.and
          br_if 0 (;@3;)
          local.get 2
          local.set 0
          br 1 (;@2;)
        end
        local.get 1
        i32.const -4
        i32.add
        local.tee 5
        i32.load
        local.tee 6
        i32.const -8
        i32.and
        local.get 4
        local.get 1
        i32.add
        i32.const 0
        local.get 0
        i32.sub
        i32.and
        i32.const -8
        i32.add
        local.tee 1
        i32.const 0
        local.get 0
        local.get 1
        local.get 2
        i32.sub
        i32.const 16
        i32.gt_u
        select
        i32.add
        local.tee 0
        local.get 2
        i32.sub
        local.tee 1
        i32.sub
        local.set 4
        block  ;; label = @3
          local.get 6
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 4
          local.get 0
          i32.load offset=4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store offset=4
          local.get 0
          local.get 4
          i32.add
          local.tee 4
          local.get 4
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 5
          local.get 1
          local.get 5
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 2
          local.get 1
          i32.add
          local.tee 4
          local.get 4
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          local.get 1
          call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE13dispose_chunkCsiuSwmoQr08c_3std
          br 1 (;@2;)
        end
        local.get 2
        i32.load
        local.set 2
        local.get 0
        local.get 4
        i32.store offset=4
        local.get 0
        local.get 2
        local.get 1
        i32.add
        i32.store
      end
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.const -8
        i32.and
        local.tee 2
        local.get 3
        i32.const 16
        i32.add
        i32.le_u
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        local.get 1
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 0
        local.get 3
        i32.add
        local.tee 1
        local.get 2
        local.get 3
        i32.sub
        local.tee 3
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 0
        local.get 2
        i32.add
        local.tee 2
        local.get 2
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        local.get 3
        call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE13dispose_chunkCsiuSwmoQr08c_3std
      end
      local.get 0
      i32.const 8
      i32.add
      local.set 2
    end
    local.get 2)
  (func $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE6mallocCsiuSwmoQr08c_3std (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.const 245
            i32.lt_u
            br_if 0 (;@4;)
            i32.const 0
            local.set 2
            local.get 0
            i32.const -48
            i32.const 0
            i32.load offset=1055676
            i32.sub
            i32.const -9
            i32.and
            i32.const -3
            i32.add
            local.tee 3
            i32.const -64
            local.get 3
            i32.const -64
            i32.lt_u
            select
            i32.ge_u
            br_if 3 (;@1;)
            local.get 0
            i32.const 11
            i32.add
            local.tee 2
            i32.const -8
            i32.and
            local.set 3
            i32.const 0
            i32.load offset=1055636
            local.tee 4
            i32.eqz
            br_if 2 (;@2;)
            i32.const 31
            local.set 5
            local.get 0
            i32.const 16777205
            i32.ge_u
            br_if 1 (;@3;)
            local.get 3
            i32.const 38
            local.get 2
            i32.const 8
            i32.shr_u
            i32.clz
            local.tee 0
            i32.sub
            i32.shr_u
            i32.const 1
            i32.and
            local.get 0
            i32.const 1
            i32.shl
            i32.or
            i32.const 62
            i32.xor
            local.set 5
            br 1 (;@3;)
          end
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      i32.const 0
                      i32.load offset=1055632
                      local.tee 6
                      i32.const 16
                      local.get 0
                      i32.const 11
                      i32.add
                      i32.const 504
                      i32.and
                      local.get 0
                      i32.const 11
                      i32.lt_u
                      select
                      local.tee 3
                      i32.const 3
                      i32.shr_u
                      local.tee 2
                      i32.shr_u
                      local.tee 0
                      i32.const 3
                      i32.and
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 0
                      i32.const -1
                      i32.xor
                      i32.const 1
                      i32.and
                      local.get 2
                      i32.add
                      local.tee 7
                      i32.const 3
                      i32.shl
                      local.tee 0
                      i32.const 1055368
                      i32.add
                      local.tee 2
                      local.get 0
                      i32.const 1055376
                      i32.add
                      i32.load
                      local.tee 3
                      i32.load offset=8
                      local.tee 8
                      i32.eq
                      br_if 1 (;@8;)
                      local.get 8
                      local.get 2
                      i32.store offset=12
                      local.get 2
                      local.get 8
                      i32.store offset=8
                      br 2 (;@7;)
                    end
                    local.get 3
                    i32.const 0
                    i32.load offset=1055640
                    i32.le_u
                    br_if 6 (;@2;)
                    block  ;; label = @9
                      local.get 0
                      br_if 0 (;@9;)
                      i32.const 0
                      i32.load offset=1055636
                      local.tee 0
                      i32.eqz
                      br_if 7 (;@2;)
                      local.get 0
                      i32.ctz
                      i32.const 2
                      i32.shl
                      i32.const 1055224
                      i32.add
                      i32.load
                      local.tee 8
                      i32.load offset=4
                      i32.const -8
                      i32.and
                      local.get 3
                      i32.sub
                      local.set 2
                      local.get 8
                      local.set 6
                      block  ;; label = @10
                        loop  ;; label = @11
                          block  ;; label = @12
                            local.get 8
                            i32.load offset=16
                            local.tee 0
                            br_if 0 (;@12;)
                            local.get 8
                            i32.load offset=20
                            local.tee 0
                            i32.eqz
                            br_if 2 (;@10;)
                          end
                          local.get 0
                          i32.load offset=4
                          i32.const -8
                          i32.and
                          local.get 3
                          i32.sub
                          local.tee 8
                          local.get 2
                          local.get 8
                          local.get 2
                          i32.lt_u
                          local.tee 8
                          select
                          local.set 2
                          local.get 0
                          local.get 6
                          local.get 8
                          select
                          local.set 6
                          local.get 0
                          local.set 8
                          br 0 (;@11;)
                        end
                      end
                      local.get 6
                      i32.load offset=24
                      local.set 5
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 6
                            i32.load offset=12
                            local.tee 0
                            local.get 6
                            i32.ne
                            br_if 0 (;@12;)
                            local.get 6
                            i32.const 20
                            i32.const 16
                            local.get 6
                            i32.load offset=20
                            local.tee 0
                            select
                            i32.add
                            i32.load
                            local.tee 8
                            br_if 1 (;@11;)
                            i32.const 0
                            local.set 0
                            br 2 (;@10;)
                          end
                          local.get 6
                          i32.load offset=8
                          local.tee 8
                          local.get 0
                          i32.store offset=12
                          local.get 0
                          local.get 8
                          i32.store offset=8
                          br 1 (;@10;)
                        end
                        local.get 6
                        i32.const 20
                        i32.add
                        local.get 6
                        i32.const 16
                        i32.add
                        local.get 0
                        select
                        local.set 7
                        loop  ;; label = @11
                          local.get 7
                          local.set 9
                          local.get 8
                          local.tee 0
                          i32.const 20
                          i32.add
                          local.get 0
                          i32.const 16
                          i32.add
                          local.get 0
                          i32.load offset=20
                          local.tee 8
                          select
                          local.set 7
                          local.get 0
                          i32.const 20
                          i32.const 16
                          local.get 8
                          select
                          i32.add
                          i32.load
                          local.tee 8
                          br_if 0 (;@11;)
                        end
                        local.get 9
                        i32.const 0
                        i32.store
                      end
                      local.get 5
                      i32.eqz
                      br_if 5 (;@4;)
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 6
                          local.get 6
                          i32.load offset=28
                          i32.const 2
                          i32.shl
                          i32.const 1055224
                          i32.add
                          local.tee 8
                          i32.load
                          i32.eq
                          br_if 0 (;@11;)
                          block  ;; label = @12
                            local.get 5
                            i32.load offset=16
                            local.get 6
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 5
                            local.get 0
                            i32.store offset=20
                            local.get 0
                            br_if 2 (;@10;)
                            br 8 (;@4;)
                          end
                          local.get 5
                          local.get 0
                          i32.store offset=16
                          local.get 0
                          br_if 1 (;@10;)
                          br 7 (;@4;)
                        end
                        local.get 8
                        local.get 0
                        i32.store
                        local.get 0
                        i32.eqz
                        br_if 5 (;@5;)
                      end
                      local.get 0
                      local.get 5
                      i32.store offset=24
                      block  ;; label = @10
                        local.get 6
                        i32.load offset=16
                        local.tee 8
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 8
                        i32.store offset=16
                        local.get 8
                        local.get 0
                        i32.store offset=24
                      end
                      local.get 6
                      i32.load offset=20
                      local.tee 8
                      i32.eqz
                      br_if 5 (;@4;)
                      local.get 0
                      local.get 8
                      i32.store offset=20
                      local.get 8
                      local.get 0
                      i32.store offset=24
                      br 5 (;@4;)
                    end
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        local.get 2
                        i32.shl
                        i32.const 2
                        local.get 2
                        i32.shl
                        local.tee 0
                        i32.const 0
                        local.get 0
                        i32.sub
                        i32.or
                        i32.and
                        i32.ctz
                        local.tee 9
                        i32.const 3
                        i32.shl
                        local.tee 2
                        i32.const 1055368
                        i32.add
                        local.tee 8
                        local.get 2
                        i32.const 1055376
                        i32.add
                        i32.load
                        local.tee 0
                        i32.load offset=8
                        local.tee 7
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 7
                        local.get 8
                        i32.store offset=12
                        local.get 8
                        local.get 7
                        i32.store offset=8
                        br 1 (;@9;)
                      end
                      i32.const 0
                      local.get 6
                      i32.const -2
                      local.get 9
                      i32.rotl
                      i32.and
                      i32.store offset=1055632
                    end
                    local.get 0
                    local.get 3
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    local.get 0
                    local.get 2
                    i32.add
                    local.get 2
                    local.get 3
                    i32.sub
                    local.tee 8
                    i32.store
                    local.get 0
                    local.get 3
                    i32.add
                    local.tee 6
                    local.get 8
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    i32.const 0
                    i32.load offset=1055640
                    local.tee 3
                    i32.eqz
                    br_if 2 (;@6;)
                    i32.const 0
                    i32.load offset=1055648
                    local.set 2
                    block  ;; label = @9
                      block  ;; label = @10
                        i32.const 0
                        i32.load offset=1055632
                        local.tee 7
                        i32.const 1
                        local.get 3
                        i32.const 3
                        i32.shr_u
                        i32.shl
                        local.tee 9
                        i32.and
                        br_if 0 (;@10;)
                        i32.const 0
                        local.get 7
                        local.get 9
                        i32.or
                        i32.store offset=1055632
                        local.get 3
                        i32.const -8
                        i32.and
                        i32.const 1055368
                        i32.add
                        local.tee 3
                        local.set 7
                        br 1 (;@9;)
                      end
                      local.get 3
                      i32.const -8
                      i32.and
                      local.tee 3
                      i32.const 1055368
                      i32.add
                      local.set 7
                      local.get 3
                      i32.const 1055376
                      i32.add
                      i32.load
                      local.set 3
                    end
                    local.get 7
                    local.get 2
                    i32.store offset=8
                    local.get 3
                    local.get 2
                    i32.store offset=12
                    local.get 2
                    local.get 7
                    i32.store offset=12
                    local.get 2
                    local.get 3
                    i32.store offset=8
                    br 2 (;@6;)
                  end
                  i32.const 0
                  local.get 6
                  i32.const -2
                  local.get 7
                  i32.rotl
                  i32.and
                  i32.store offset=1055632
                end
                local.get 3
                i32.const 8
                i32.add
                local.set 2
                local.get 3
                local.get 0
                i32.const 3
                i32.or
                i32.store offset=4
                local.get 3
                local.get 0
                i32.add
                local.tee 0
                local.get 0
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                br 5 (;@1;)
              end
              local.get 0
              i32.const 8
              i32.add
              local.set 2
              i32.const 0
              local.get 6
              i32.store offset=1055648
              i32.const 0
              local.get 8
              i32.store offset=1055640
              br 4 (;@1;)
            end
            i32.const 0
            i32.const 0
            i32.load offset=1055636
            i32.const -2
            local.get 6
            i32.load offset=28
            i32.rotl
            i32.and
            i32.store offset=1055636
          end
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.const 16
                i32.lt_u
                br_if 0 (;@6;)
                local.get 6
                local.get 3
                i32.const 3
                i32.or
                i32.store offset=4
                local.get 6
                local.get 3
                i32.add
                local.tee 8
                local.get 2
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 8
                local.get 2
                i32.add
                local.get 2
                i32.store
                i32.const 0
                i32.load offset=1055640
                local.tee 7
                i32.eqz
                br_if 1 (;@5;)
                i32.const 0
                i32.load offset=1055648
                local.set 0
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 0
                    i32.load offset=1055632
                    local.tee 9
                    i32.const 1
                    local.get 7
                    i32.const 3
                    i32.shr_u
                    i32.shl
                    local.tee 5
                    i32.and
                    br_if 0 (;@8;)
                    i32.const 0
                    local.get 9
                    local.get 5
                    i32.or
                    i32.store offset=1055632
                    local.get 7
                    i32.const -8
                    i32.and
                    i32.const 1055368
                    i32.add
                    local.tee 7
                    local.set 9
                    br 1 (;@7;)
                  end
                  local.get 7
                  i32.const -8
                  i32.and
                  local.tee 7
                  i32.const 1055368
                  i32.add
                  local.set 9
                  local.get 7
                  i32.const 1055376
                  i32.add
                  i32.load
                  local.set 7
                end
                local.get 9
                local.get 0
                i32.store offset=8
                local.get 7
                local.get 0
                i32.store offset=12
                local.get 0
                local.get 9
                i32.store offset=12
                local.get 0
                local.get 7
                i32.store offset=8
                br 1 (;@5;)
              end
              local.get 6
              local.get 2
              local.get 3
              i32.add
              local.tee 0
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 6
              local.get 0
              i32.add
              local.tee 0
              local.get 0
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              br 1 (;@4;)
            end
            i32.const 0
            local.get 8
            i32.store offset=1055648
            i32.const 0
            local.get 2
            i32.store offset=1055640
          end
          local.get 6
          i32.const 8
          i32.add
          local.tee 2
          i32.eqz
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        i32.const 0
        local.get 3
        i32.sub
        local.set 2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 5
                i32.const 2
                i32.shl
                i32.const 1055224
                i32.add
                i32.load
                local.tee 6
                br_if 0 (;@6;)
                i32.const 0
                local.set 8
                i32.const 0
                local.set 0
                br 1 (;@5;)
              end
              i32.const 0
              local.set 8
              local.get 3
              i32.const 0
              i32.const 25
              local.get 5
              i32.const 1
              i32.shr_u
              i32.sub
              local.get 5
              i32.const 31
              i32.eq
              select
              i32.shl
              local.set 7
              i32.const 0
              local.set 0
              loop  ;; label = @6
                block  ;; label = @7
                  local.get 6
                  local.tee 6
                  i32.load offset=4
                  i32.const -8
                  i32.and
                  local.tee 9
                  local.get 3
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 9
                  local.get 3
                  i32.sub
                  local.tee 9
                  local.get 2
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 6
                  local.set 8
                  local.get 9
                  local.set 2
                  local.get 9
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 2
                  local.get 6
                  local.set 0
                  local.get 6
                  local.set 8
                  br 3 (;@4;)
                end
                local.get 6
                i32.load offset=20
                local.tee 9
                local.get 0
                local.get 9
                local.get 6
                local.get 7
                i32.const 29
                i32.shr_u
                i32.const 4
                i32.and
                i32.add
                i32.load offset=16
                local.tee 6
                i32.ne
                select
                local.get 0
                local.get 9
                select
                local.set 0
                local.get 7
                i32.const 1
                i32.shl
                local.set 7
                local.get 6
                br_if 0 (;@6;)
              end
            end
            block  ;; label = @5
              local.get 0
              local.get 8
              i32.or
              br_if 0 (;@5;)
              i32.const 0
              local.set 8
              i32.const 2
              local.get 5
              i32.shl
              local.tee 0
              i32.const 0
              local.get 0
              i32.sub
              i32.or
              local.get 4
              i32.and
              local.tee 0
              i32.eqz
              br_if 3 (;@2;)
              local.get 0
              i32.ctz
              i32.const 2
              i32.shl
              i32.const 1055224
              i32.add
              i32.load
              local.set 0
            end
            local.get 0
            i32.eqz
            br_if 1 (;@3;)
          end
          loop  ;; label = @4
            local.get 0
            i32.load offset=4
            i32.const -8
            i32.and
            local.tee 7
            local.get 3
            i32.sub
            local.tee 6
            local.get 2
            local.get 6
            local.get 2
            i32.lt_u
            select
            local.set 9
            local.get 7
            local.get 3
            i32.lt_u
            local.set 7
            local.get 8
            local.get 0
            local.get 6
            local.get 2
            i32.ge_u
            select
            local.set 5
            block  ;; label = @5
              local.get 0
              i32.load offset=16
              local.tee 6
              br_if 0 (;@5;)
              local.get 0
              i32.load offset=20
              local.set 6
            end
            local.get 2
            local.get 9
            local.get 7
            select
            local.set 2
            local.get 8
            local.get 5
            local.get 7
            select
            local.set 8
            local.get 6
            local.set 0
            local.get 6
            br_if 0 (;@4;)
          end
        end
        local.get 8
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          i32.const 0
          i32.load offset=1055640
          local.tee 0
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 0
          local.get 3
          i32.sub
          i32.ge_u
          br_if 1 (;@2;)
        end
        local.get 8
        i32.load offset=24
        local.set 5
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 8
              i32.load offset=12
              local.tee 0
              local.get 8
              i32.ne
              br_if 0 (;@5;)
              local.get 8
              i32.const 20
              i32.const 16
              local.get 8
              i32.load offset=20
              local.tee 0
              select
              i32.add
              i32.load
              local.tee 6
              br_if 1 (;@4;)
              i32.const 0
              local.set 0
              br 2 (;@3;)
            end
            local.get 8
            i32.load offset=8
            local.tee 6
            local.get 0
            i32.store offset=12
            local.get 0
            local.get 6
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 8
          i32.const 20
          i32.add
          local.get 8
          i32.const 16
          i32.add
          local.get 0
          select
          local.set 7
          loop  ;; label = @4
            local.get 7
            local.set 9
            local.get 6
            local.tee 0
            i32.const 20
            i32.add
            local.get 0
            i32.const 16
            i32.add
            local.get 0
            i32.load offset=20
            local.tee 6
            select
            local.set 7
            local.get 0
            i32.const 20
            i32.const 16
            local.get 6
            select
            i32.add
            i32.load
            local.tee 6
            br_if 0 (;@4;)
          end
          local.get 9
          i32.const 0
          i32.store
        end
        block  ;; label = @3
          local.get 5
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 8
                local.get 8
                i32.load offset=28
                i32.const 2
                i32.shl
                i32.const 1055224
                i32.add
                local.tee 6
                i32.load
                i32.eq
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 5
                  i32.load offset=16
                  local.get 8
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 5
                  local.get 0
                  i32.store offset=20
                  local.get 0
                  br_if 2 (;@5;)
                  br 4 (;@3;)
                end
                local.get 5
                local.get 0
                i32.store offset=16
                local.get 0
                br_if 1 (;@5;)
                br 3 (;@3;)
              end
              local.get 6
              local.get 0
              i32.store
              local.get 0
              i32.eqz
              br_if 1 (;@4;)
            end
            local.get 0
            local.get 5
            i32.store offset=24
            block  ;; label = @5
              local.get 8
              i32.load offset=16
              local.tee 6
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              local.get 6
              i32.store offset=16
              local.get 6
              local.get 0
              i32.store offset=24
            end
            local.get 8
            i32.load offset=20
            local.tee 6
            i32.eqz
            br_if 1 (;@3;)
            local.get 0
            local.get 6
            i32.store offset=20
            local.get 6
            local.get 0
            i32.store offset=24
            br 1 (;@3;)
          end
          i32.const 0
          i32.const 0
          i32.load offset=1055636
          i32.const -2
          local.get 8
          i32.load offset=28
          i32.rotl
          i32.and
          i32.store offset=1055636
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.const 16
            i32.lt_u
            br_if 0 (;@4;)
            local.get 8
            local.get 3
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 8
            local.get 3
            i32.add
            local.tee 0
            local.get 2
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            local.get 2
            i32.add
            local.get 2
            i32.store
            block  ;; label = @5
              local.get 2
              i32.const 256
              i32.lt_u
              br_if 0 (;@5;)
              local.get 0
              local.get 2
              call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE18insert_large_chunkCsiuSwmoQr08c_3std
              br 2 (;@3;)
            end
            block  ;; label = @5
              block  ;; label = @6
                i32.const 0
                i32.load offset=1055632
                local.tee 6
                i32.const 1
                local.get 2
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 7
                i32.and
                br_if 0 (;@6;)
                i32.const 0
                local.get 6
                local.get 7
                i32.or
                i32.store offset=1055632
                local.get 2
                i32.const 248
                i32.and
                i32.const 1055368
                i32.add
                local.tee 2
                local.set 6
                br 1 (;@5;)
              end
              local.get 2
              i32.const 248
              i32.and
              local.tee 2
              i32.const 1055368
              i32.add
              local.set 6
              local.get 2
              i32.const 1055376
              i32.add
              i32.load
              local.set 2
            end
            local.get 6
            local.get 0
            i32.store offset=8
            local.get 2
            local.get 0
            i32.store offset=12
            local.get 0
            local.get 6
            i32.store offset=12
            local.get 0
            local.get 2
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 8
          local.get 2
          local.get 3
          i32.add
          local.tee 0
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 8
          local.get 0
          i32.add
          local.tee 0
          local.get 0
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
        end
        local.get 8
        i32.const 8
        i32.add
        local.tee 2
        br_if 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      i32.const 0
                      i32.load offset=1055640
                      local.tee 0
                      local.get 3
                      i32.ge_u
                      br_if 0 (;@9;)
                      block  ;; label = @10
                        i32.const 0
                        i32.load offset=1055644
                        local.tee 0
                        local.get 3
                        i32.gt_u
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 2
                        local.get 1
                        i32.const 4
                        i32.add
                        i32.const 1055684
                        local.get 3
                        i32.const 0
                        i32.load offset=1055676
                        local.tee 0
                        i32.add
                        i32.const 47
                        i32.add
                        i32.const 0
                        local.get 0
                        i32.sub
                        i32.and
                        call $_RNvXs_NtCsbw64Iw7W9Hn_8dlmalloc3sysNtB4_6SystemNtB6_9Allocator5alloc
                        local.get 1
                        i32.load offset=4
                        local.tee 6
                        i32.eqz
                        br_if 9 (;@1;)
                        local.get 1
                        i32.load offset=12
                        local.set 5
                        i32.const 0
                        i32.const 0
                        i32.load offset=1055656
                        local.get 1
                        i32.load offset=8
                        local.tee 9
                        i32.add
                        local.tee 0
                        i32.store offset=1055656
                        i32.const 0
                        local.get 0
                        i32.const 0
                        i32.load offset=1055660
                        local.tee 2
                        local.get 0
                        local.get 2
                        i32.gt_u
                        select
                        i32.store offset=1055660
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1055652
                              local.tee 2
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 1055352
                              local.set 0
                              loop  ;; label = @14
                                local.get 6
                                local.get 0
                                i32.load
                                local.tee 8
                                local.get 0
                                i32.load offset=4
                                local.tee 7
                                i32.add
                                i32.eq
                                br_if 2 (;@12;)
                                local.get 0
                                i32.load offset=8
                                local.tee 0
                                br_if 0 (;@14;)
                                br 3 (;@11;)
                              end
                            end
                            i32.const 0
                            i32.load offset=1055668
                            local.tee 0
                            i32.eqz
                            br_if 7 (;@5;)
                            local.get 6
                            local.get 0
                            i32.lt_u
                            br_if 7 (;@5;)
                            br 9 (;@3;)
                          end
                          local.get 2
                          local.get 6
                          i32.ge_u
                          br_if 0 (;@11;)
                          local.get 8
                          local.get 2
                          i32.gt_u
                          br_if 0 (;@11;)
                          local.get 0
                          i32.load offset=12
                          local.tee 8
                          i32.const 1
                          i32.and
                          br_if 0 (;@11;)
                          local.get 8
                          i32.const 1
                          i32.shr_u
                          local.get 5
                          i32.eq
                          br_if 3 (;@8;)
                        end
                        i32.const 0
                        i32.const 0
                        i32.load offset=1055668
                        local.tee 0
                        local.get 6
                        local.get 0
                        local.get 6
                        i32.lt_u
                        select
                        i32.store offset=1055668
                        local.get 6
                        local.get 9
                        i32.add
                        local.set 8
                        i32.const 1055352
                        local.set 0
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                loop  ;; label = @15
                                  local.get 0
                                  i32.load
                                  local.tee 7
                                  local.get 8
                                  i32.eq
                                  br_if 1 (;@14;)
                                  local.get 0
                                  i32.load offset=8
                                  local.tee 0
                                  br_if 0 (;@15;)
                                  br 2 (;@13;)
                                end
                              end
                              local.get 0
                              i32.load offset=12
                              local.tee 8
                              i32.const 1
                              i32.and
                              br_if 0 (;@13;)
                              local.get 8
                              i32.const 1
                              i32.shr_u
                              local.get 5
                              i32.eq
                              br_if 1 (;@12;)
                            end
                            i32.const 1055352
                            local.set 0
                            loop  ;; label = @13
                              block  ;; label = @14
                                local.get 0
                                i32.load
                                local.tee 8
                                local.get 2
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 2
                                local.get 8
                                local.get 0
                                i32.load offset=4
                                i32.add
                                local.tee 8
                                i32.lt_u
                                br_if 3 (;@11;)
                              end
                              local.get 0
                              i32.load offset=8
                              local.set 0
                              br 0 (;@13;)
                            end
                          end
                          local.get 0
                          local.get 6
                          i32.store
                          local.get 0
                          local.get 0
                          i32.load offset=4
                          local.get 9
                          i32.add
                          i32.store offset=4
                          local.get 6
                          i32.const 15
                          i32.add
                          i32.const -8
                          i32.and
                          i32.const -8
                          i32.add
                          local.tee 6
                          local.get 3
                          i32.const 3
                          i32.or
                          i32.store offset=4
                          local.get 7
                          i32.const 15
                          i32.add
                          i32.const -8
                          i32.and
                          i32.const -8
                          i32.add
                          local.tee 8
                          local.get 6
                          local.get 3
                          i32.add
                          local.tee 0
                          i32.sub
                          local.set 2
                          local.get 8
                          i32.const 0
                          i32.load offset=1055652
                          i32.eq
                          br_if 4 (;@7;)
                          local.get 8
                          i32.const 0
                          i32.load offset=1055648
                          i32.eq
                          br_if 5 (;@6;)
                          block  ;; label = @12
                            local.get 8
                            i32.load offset=4
                            local.tee 3
                            i32.const 3
                            i32.and
                            i32.const 1
                            i32.ne
                            br_if 0 (;@12;)
                            local.get 8
                            local.get 3
                            i32.const -8
                            i32.and
                            local.tee 3
                            call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE12unlink_chunkCsiuSwmoQr08c_3std
                            local.get 3
                            local.get 2
                            i32.add
                            local.set 2
                            local.get 8
                            local.get 3
                            i32.add
                            local.tee 8
                            i32.load offset=4
                            local.set 3
                          end
                          local.get 8
                          local.get 3
                          i32.const -2
                          i32.and
                          i32.store offset=4
                          local.get 0
                          local.get 2
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 0
                          local.get 2
                          i32.add
                          local.get 2
                          i32.store
                          block  ;; label = @12
                            local.get 2
                            i32.const 256
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 0
                            local.get 2
                            call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE18insert_large_chunkCsiuSwmoQr08c_3std
                            br 8 (;@4;)
                          end
                          block  ;; label = @12
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1055632
                              local.tee 3
                              i32.const 1
                              local.get 2
                              i32.const 3
                              i32.shr_u
                              i32.shl
                              local.tee 8
                              i32.and
                              br_if 0 (;@13;)
                              i32.const 0
                              local.get 3
                              local.get 8
                              i32.or
                              i32.store offset=1055632
                              local.get 2
                              i32.const 248
                              i32.and
                              i32.const 1055368
                              i32.add
                              local.tee 2
                              local.set 3
                              br 1 (;@12;)
                            end
                            local.get 2
                            i32.const 248
                            i32.and
                            local.tee 2
                            i32.const 1055368
                            i32.add
                            local.set 3
                            local.get 2
                            i32.const 1055376
                            i32.add
                            i32.load
                            local.set 2
                          end
                          local.get 3
                          local.get 0
                          i32.store offset=8
                          local.get 2
                          local.get 0
                          i32.store offset=12
                          local.get 0
                          local.get 3
                          i32.store offset=12
                          local.get 0
                          local.get 2
                          i32.store offset=8
                          br 7 (;@4;)
                        end
                        i32.const 0
                        local.get 6
                        i32.const 15
                        i32.add
                        i32.const -8
                        i32.and
                        local.tee 0
                        i32.const -8
                        i32.add
                        local.tee 7
                        i32.store offset=1055652
                        i32.const 0
                        local.get 6
                        local.get 0
                        i32.sub
                        local.get 9
                        i32.const -40
                        i32.add
                        local.tee 0
                        i32.add
                        i32.const 8
                        i32.add
                        local.tee 4
                        i32.store offset=1055644
                        local.get 7
                        local.get 4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 6
                        local.get 0
                        i32.add
                        i32.const 40
                        i32.store offset=4
                        i32.const 0
                        i32.const 2097152
                        i32.store offset=1055664
                        local.get 2
                        local.get 8
                        i32.const -32
                        i32.add
                        i32.const -8
                        i32.and
                        i32.const -8
                        i32.add
                        local.tee 0
                        local.get 0
                        local.get 2
                        i32.const 16
                        i32.add
                        i32.lt_u
                        select
                        local.tee 7
                        i32.const 27
                        i32.store offset=4
                        i32.const 0
                        i64.load offset=1055352 align=4
                        local.set 10
                        local.get 7
                        i32.const 16
                        i32.add
                        i32.const 0
                        i64.load offset=1055360 align=4
                        i64.store align=4
                        local.get 7
                        i32.const 8
                        i32.add
                        local.tee 0
                        local.get 10
                        i64.store align=4
                        i32.const 0
                        local.get 5
                        i32.store offset=1055364
                        i32.const 0
                        local.get 9
                        i32.store offset=1055356
                        i32.const 0
                        local.get 6
                        i32.store offset=1055352
                        i32.const 0
                        local.get 0
                        i32.store offset=1055360
                        local.get 7
                        i32.const 28
                        i32.add
                        local.set 0
                        loop  ;; label = @11
                          local.get 0
                          i32.const 7
                          i32.store
                          local.get 0
                          i32.const 4
                          i32.add
                          local.tee 0
                          local.get 8
                          i32.lt_u
                          br_if 0 (;@11;)
                        end
                        local.get 7
                        local.get 2
                        i32.eq
                        br_if 8 (;@2;)
                        local.get 7
                        local.get 7
                        i32.load offset=4
                        i32.const -2
                        i32.and
                        i32.store offset=4
                        local.get 2
                        local.get 7
                        local.get 2
                        i32.sub
                        local.tee 0
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 7
                        local.get 0
                        i32.store
                        block  ;; label = @11
                          local.get 0
                          i32.const 256
                          i32.lt_u
                          br_if 0 (;@11;)
                          local.get 2
                          local.get 0
                          call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE18insert_large_chunkCsiuSwmoQr08c_3std
                          br 9 (;@2;)
                        end
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=1055632
                            local.tee 8
                            i32.const 1
                            local.get 0
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 6
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.get 8
                            local.get 6
                            i32.or
                            i32.store offset=1055632
                            local.get 0
                            i32.const 248
                            i32.and
                            i32.const 1055368
                            i32.add
                            local.tee 0
                            local.set 8
                            br 1 (;@11;)
                          end
                          local.get 0
                          i32.const 248
                          i32.and
                          local.tee 0
                          i32.const 1055368
                          i32.add
                          local.set 8
                          local.get 0
                          i32.const 1055376
                          i32.add
                          i32.load
                          local.set 0
                        end
                        local.get 8
                        local.get 2
                        i32.store offset=8
                        local.get 0
                        local.get 2
                        i32.store offset=12
                        local.get 2
                        local.get 8
                        i32.store offset=12
                        local.get 2
                        local.get 0
                        i32.store offset=8
                        br 8 (;@2;)
                      end
                      i32.const 0
                      local.get 0
                      local.get 3
                      i32.sub
                      local.tee 2
                      i32.store offset=1055644
                      i32.const 0
                      i32.const 0
                      i32.load offset=1055652
                      local.tee 0
                      local.get 3
                      i32.add
                      local.tee 8
                      i32.store offset=1055652
                      local.get 8
                      local.get 2
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 3
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 0
                      i32.const 8
                      i32.add
                      local.set 2
                      br 8 (;@1;)
                    end
                    i32.const 0
                    i32.load offset=1055648
                    local.set 2
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        local.get 3
                        i32.sub
                        local.tee 8
                        i32.const 15
                        i32.gt_u
                        br_if 0 (;@10;)
                        i32.const 0
                        i32.const 0
                        i32.store offset=1055648
                        i32.const 0
                        i32.const 0
                        i32.store offset=1055640
                        local.get 2
                        local.get 0
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 2
                        local.get 0
                        i32.add
                        local.tee 0
                        local.get 0
                        i32.load offset=4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        br 1 (;@9;)
                      end
                      i32.const 0
                      local.get 8
                      i32.store offset=1055640
                      i32.const 0
                      local.get 2
                      local.get 3
                      i32.add
                      local.tee 6
                      i32.store offset=1055648
                      local.get 6
                      local.get 8
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 2
                      local.get 0
                      i32.add
                      local.get 8
                      i32.store
                      local.get 2
                      local.get 3
                      i32.const 3
                      i32.or
                      i32.store offset=4
                    end
                    local.get 2
                    i32.const 8
                    i32.add
                    local.set 2
                    br 7 (;@1;)
                  end
                  local.get 0
                  local.get 7
                  local.get 9
                  i32.add
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1055652
                  local.tee 0
                  i32.const 15
                  i32.add
                  i32.const -8
                  i32.and
                  local.tee 2
                  i32.const -8
                  i32.add
                  local.tee 8
                  i32.store offset=1055652
                  i32.const 0
                  local.get 0
                  local.get 2
                  i32.sub
                  i32.const 0
                  i32.load offset=1055644
                  local.get 9
                  i32.add
                  local.tee 2
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 6
                  i32.store offset=1055644
                  local.get 8
                  local.get 6
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  local.get 2
                  i32.add
                  i32.const 40
                  i32.store offset=4
                  i32.const 0
                  i32.const 2097152
                  i32.store offset=1055664
                  br 5 (;@2;)
                end
                i32.const 0
                local.get 0
                i32.store offset=1055652
                i32.const 0
                i32.const 0
                i32.load offset=1055644
                local.get 2
                i32.add
                local.tee 2
                i32.store offset=1055644
                local.get 0
                local.get 2
                i32.const 1
                i32.or
                i32.store offset=4
                br 2 (;@4;)
              end
              i32.const 0
              local.get 0
              i32.store offset=1055648
              i32.const 0
              i32.const 0
              i32.load offset=1055640
              local.get 2
              i32.add
              local.tee 2
              i32.store offset=1055640
              local.get 0
              local.get 2
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 2
              i32.add
              local.get 2
              i32.store
              br 1 (;@4;)
            end
            i32.const 0
            local.get 6
            i32.store offset=1055668
            br 1 (;@3;)
          end
          local.get 6
          i32.const 8
          i32.add
          local.set 2
          br 2 (;@1;)
        end
        i32.const 0
        local.get 5
        i32.store offset=1055364
        i32.const 0
        local.get 9
        i32.store offset=1055356
        i32.const 0
        local.get 6
        i32.store offset=1055352
        i32.const 0
        i32.const 1055368
        i32.store offset=1055380
        i32.const 0
        i32.const 1055376
        i32.store offset=1055388
        i32.const 0
        i32.const 1055368
        i32.store offset=1055376
        i32.const 0
        i32.const 1055384
        i32.store offset=1055396
        i32.const 0
        i32.const 1055376
        i32.store offset=1055384
        i32.const 0
        i32.const 1055392
        i32.store offset=1055404
        i32.const 0
        i32.const 1055384
        i32.store offset=1055392
        i32.const 0
        i32.const 1055400
        i32.store offset=1055412
        i32.const 0
        i32.const 1055392
        i32.store offset=1055400
        i32.const 0
        i32.const 1055408
        i32.store offset=1055420
        i32.const 0
        i32.const 1055400
        i32.store offset=1055408
        i32.const 0
        i32.const 1055416
        i32.store offset=1055428
        i32.const 0
        i32.const 1055408
        i32.store offset=1055416
        i32.const 0
        i32.const 1055424
        i32.store offset=1055436
        i32.const 0
        i32.const 1055416
        i32.store offset=1055424
        i32.const 0
        i32.const 0
        i32.load offset=1055680
        local.tee 0
        i32.const -1
        local.get 0
        select
        i32.store offset=1055672
        i32.const 0
        i32.const 1055432
        i32.store offset=1055444
        i32.const 0
        i32.const 1055424
        i32.store offset=1055432
        i32.const 0
        i32.const 1055432
        i32.store offset=1055440
        i32.const 0
        i32.const 1055440
        i32.store offset=1055452
        i32.const 0
        i32.const 1055440
        i32.store offset=1055448
        i32.const 0
        i32.const 1055448
        i32.store offset=1055460
        i32.const 0
        i32.const 1055448
        i32.store offset=1055456
        i32.const 0
        i32.const 1055456
        i32.store offset=1055468
        i32.const 0
        i32.const 1055456
        i32.store offset=1055464
        i32.const 0
        i32.const 1055464
        i32.store offset=1055476
        i32.const 0
        i32.const 1055464
        i32.store offset=1055472
        i32.const 0
        i32.const 1055472
        i32.store offset=1055484
        i32.const 0
        i32.const 1055472
        i32.store offset=1055480
        i32.const 0
        i32.const 1055480
        i32.store offset=1055492
        i32.const 0
        i32.const 1055488
        i32.store offset=1055500
        i32.const 0
        i32.const 1055480
        i32.store offset=1055488
        i32.const 0
        i32.const 1055488
        i32.store offset=1055496
        i32.const 0
        i32.const 1055496
        i32.store offset=1055508
        i32.const 0
        i32.const 1055504
        i32.store offset=1055516
        i32.const 0
        i32.const 1055496
        i32.store offset=1055504
        i32.const 0
        i32.const 1055504
        i32.store offset=1055512
        i32.const 0
        i32.const 1055512
        i32.store offset=1055524
        i32.const 0
        i32.const 1055512
        i32.store offset=1055520
        i32.const 0
        i32.const 1055520
        i32.store offset=1055532
        i32.const 0
        i32.const 1055520
        i32.store offset=1055528
        i32.const 0
        i32.const 1055528
        i32.store offset=1055540
        i32.const 0
        i32.const 1055528
        i32.store offset=1055536
        i32.const 0
        i32.const 1055536
        i32.store offset=1055548
        i32.const 0
        i32.const 1055536
        i32.store offset=1055544
        i32.const 0
        i32.const 1055544
        i32.store offset=1055556
        i32.const 0
        i32.const 1055544
        i32.store offset=1055552
        i32.const 0
        i32.const 1055552
        i32.store offset=1055564
        i32.const 0
        i32.const 1055552
        i32.store offset=1055560
        i32.const 0
        i32.const 1055560
        i32.store offset=1055572
        i32.const 0
        i32.const 1055560
        i32.store offset=1055568
        i32.const 0
        i32.const 1055568
        i32.store offset=1055580
        i32.const 0
        i32.const 1055576
        i32.store offset=1055588
        i32.const 0
        i32.const 1055568
        i32.store offset=1055576
        i32.const 0
        i32.const 1055584
        i32.store offset=1055596
        i32.const 0
        i32.const 1055576
        i32.store offset=1055584
        i32.const 0
        i32.const 1055592
        i32.store offset=1055604
        i32.const 0
        i32.const 1055584
        i32.store offset=1055592
        i32.const 0
        i32.const 1055600
        i32.store offset=1055612
        i32.const 0
        i32.const 1055592
        i32.store offset=1055600
        i32.const 0
        i32.const 1055608
        i32.store offset=1055620
        i32.const 0
        i32.const 1055600
        i32.store offset=1055608
        i32.const 0
        i32.const 1055616
        i32.store offset=1055628
        i32.const 0
        i32.const 1055608
        i32.store offset=1055616
        i32.const 0
        local.get 6
        i32.const 15
        i32.add
        i32.const -8
        i32.and
        local.tee 0
        i32.const -8
        i32.add
        local.tee 2
        i32.store offset=1055652
        i32.const 0
        i32.const 1055616
        i32.store offset=1055624
        i32.const 0
        local.get 6
        local.get 0
        i32.sub
        local.get 9
        i32.const -40
        i32.add
        local.tee 0
        i32.add
        i32.const 8
        i32.add
        local.tee 8
        i32.store offset=1055644
        local.get 2
        local.get 8
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 6
        local.get 0
        i32.add
        i32.const 40
        i32.store offset=4
        i32.const 0
        i32.const 2097152
        i32.store offset=1055664
      end
      i32.const 0
      local.set 2
      i32.const 0
      i32.load offset=1055644
      local.tee 0
      local.get 3
      i32.le_u
      br_if 0 (;@1;)
      i32.const 0
      local.get 0
      local.get 3
      i32.sub
      local.tee 2
      i32.store offset=1055644
      i32.const 0
      i32.const 0
      i32.load offset=1055652
      local.tee 0
      local.get 3
      i32.add
      local.tee 8
      i32.store offset=1055652
      local.get 8
      local.get 2
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 0
      local.get 3
      i32.const 3
      i32.or
      i32.store offset=4
      local.get 0
      i32.const 8
      i32.add
      local.set 2
    end
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2)
  (func $_RNvCs5fGHjzWXRav_7___rustc12___rust_abort (type 8)
    unreachable)
  (func $_RNvCs5fGHjzWXRav_7___rustc13___rdl_dealloc (type 5) (param i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const -4
        i32.add
        i32.load
        local.tee 3
        i32.const -8
        i32.and
        local.tee 4
        i32.const 4
        i32.const 8
        local.get 3
        i32.const 3
        i32.and
        local.tee 3
        select
        local.get 1
        i32.add
        i32.lt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 1
          i32.const 39
          i32.add
          i32.gt_u
          br_if 2 (;@1;)
        end
        local.get 0
        call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE4freeCsiuSwmoQr08c_3std
        return
      end
      i32.const 1050192
      i32.const 46
      i32.const 1050240
      call $_RNvNtCs4NIexwV1FGX_4core9panicking5panic
      unreachable
    end
    i32.const 1050256
    i32.const 46
    i32.const 1050304
    call $_RNvNtCs4NIexwV1FGX_4core9panicking5panic
    unreachable)
  (func $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE4freeCsiuSwmoQr08c_3std (type 2) (param i32)
    (local i32 i32 i32 i32)
    local.get 0
    i32.const -8
    i32.add
    local.tee 1
    local.get 0
    i32.const -4
    i32.add
    i32.load
    local.tee 2
    i32.const -8
    i32.and
    local.tee 0
    i32.add
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 2
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        i32.load
        local.tee 2
        local.get 0
        i32.add
        local.set 0
        block  ;; label = @3
          local.get 1
          local.get 2
          i32.sub
          local.tee 1
          i32.const 0
          i32.load offset=1055648
          i32.ne
          br_if 0 (;@3;)
          local.get 3
          i32.load offset=4
          i32.const 3
          i32.and
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 0
          i32.store offset=1055640
          local.get 3
          local.get 3
          i32.load offset=4
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 3
          local.get 0
          i32.store
          return
        end
        local.get 1
        local.get 2
        call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE12unlink_chunkCsiuSwmoQr08c_3std
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 3
                      i32.load offset=4
                      local.tee 2
                      i32.const 2
                      i32.and
                      br_if 0 (;@9;)
                      local.get 3
                      i32.const 0
                      i32.load offset=1055652
                      i32.eq
                      br_if 2 (;@7;)
                      local.get 3
                      i32.const 0
                      i32.load offset=1055648
                      i32.eq
                      br_if 5 (;@4;)
                      local.get 3
                      local.get 2
                      i32.const -8
                      i32.and
                      local.tee 2
                      call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE12unlink_chunkCsiuSwmoQr08c_3std
                      local.get 1
                      local.get 2
                      local.get 0
                      i32.add
                      local.tee 0
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 1
                      local.get 0
                      i32.add
                      local.get 0
                      i32.store
                      local.get 1
                      i32.const 0
                      i32.load offset=1055648
                      i32.ne
                      br_if 1 (;@8;)
                      i32.const 0
                      local.get 0
                      i32.store offset=1055640
                      return
                    end
                    local.get 3
                    local.get 2
                    i32.const -2
                    i32.and
                    i32.store offset=4
                    local.get 1
                    local.get 0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 1
                    local.get 0
                    i32.add
                    local.get 0
                    i32.store
                  end
                  local.get 0
                  i32.const 256
                  i32.lt_u
                  br_if 1 (;@6;)
                  local.get 1
                  local.get 0
                  call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE18insert_large_chunkCsiuSwmoQr08c_3std
                  i32.const 0
                  local.set 1
                  i32.const 0
                  i32.const 0
                  i32.load offset=1055672
                  i32.const -1
                  i32.add
                  local.tee 0
                  i32.store offset=1055672
                  local.get 0
                  i32.eqz
                  br_if 2 (;@5;)
                  br 6 (;@1;)
                end
                i32.const 0
                local.get 1
                i32.store offset=1055652
                i32.const 0
                i32.const 0
                i32.load offset=1055644
                local.get 0
                i32.add
                local.tee 0
                i32.store offset=1055644
                local.get 1
                local.get 0
                i32.const 1
                i32.or
                i32.store offset=4
                block  ;; label = @7
                  local.get 1
                  i32.const 0
                  i32.load offset=1055648
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 0
                  i32.const 0
                  i32.store offset=1055640
                  i32.const 0
                  i32.const 0
                  i32.store offset=1055648
                end
                local.get 0
                i32.const 0
                i32.load offset=1055664
                local.tee 2
                i32.le_u
                br_if 5 (;@1;)
                i32.const -48
                i32.const 0
                i32.load offset=1055676
                local.tee 1
                i32.sub
                i32.const -9
                i32.and
                i32.const 3
                i32.eq
                br_if 5 (;@1;)
                i32.const 0
                i32.load offset=1055652
                local.tee 0
                i32.eqz
                br_if 5 (;@1;)
                i32.const 0
                i32.load offset=1055644
                local.tee 4
                i32.const 41
                i32.lt_u
                br_if 4 (;@2;)
                local.get 1
                i32.eqz
                br_if 3 (;@3;)
                i32.const 1055352
                local.set 1
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 1
                    i32.load
                    local.tee 3
                    local.get 0
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 3
                    local.get 1
                    i32.load offset=4
                    i32.add
                    i32.lt_u
                    br_if 6 (;@2;)
                  end
                  local.get 1
                  i32.load offset=8
                  local.set 1
                  br 0 (;@7;)
                end
              end
              block  ;; label = @6
                block  ;; label = @7
                  i32.const 0
                  i32.load offset=1055632
                  local.tee 3
                  i32.const 1
                  local.get 0
                  i32.const 3
                  i32.shr_u
                  i32.shl
                  local.tee 2
                  i32.and
                  br_if 0 (;@7;)
                  i32.const 0
                  local.get 3
                  local.get 2
                  i32.or
                  i32.store offset=1055632
                  local.get 0
                  i32.const 248
                  i32.and
                  i32.const 1055368
                  i32.add
                  local.tee 0
                  local.set 3
                  br 1 (;@6;)
                end
                local.get 0
                i32.const 248
                i32.and
                local.tee 0
                i32.const 1055368
                i32.add
                local.set 3
                local.get 0
                i32.const 1055376
                i32.add
                i32.load
                local.set 0
              end
              local.get 3
              local.get 1
              i32.store offset=8
              local.get 0
              local.get 1
              i32.store offset=12
              local.get 1
              local.get 3
              i32.store offset=12
              local.get 1
              local.get 0
              i32.store offset=8
              return
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1055360
              local.tee 0
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              local.set 1
              loop  ;; label = @6
                local.get 1
                i32.const 1
                i32.add
                local.set 1
                local.get 0
                i32.load offset=8
                local.tee 0
                br_if 0 (;@6;)
              end
            end
            i32.const 0
            i32.const 0
            i32.load offset=1055680
            local.tee 0
            local.get 1
            local.get 0
            local.get 1
            i32.gt_u
            select
            i32.const -1
            local.get 0
            select
            i32.store offset=1055672
            return
          end
          i32.const 0
          local.get 1
          i32.store offset=1055648
          i32.const 0
          i32.const 0
          i32.load offset=1055640
          local.get 0
          i32.add
          local.tee 0
          i32.store offset=1055640
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 0
          i32.add
          local.get 0
          i32.store
          return
        end
        i32.const 1050320
        call $_RNvNtNtCs4NIexwV1FGX_4core9panicking11panic_const23panic_const_div_by_zero
        unreachable
      end
      i32.const 0
      local.set 1
      block  ;; label = @2
        i32.const 0
        i32.load offset=1055360
        local.tee 0
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 1
        loop  ;; label = @3
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 0
          i32.load offset=8
          local.tee 0
          br_if 0 (;@3;)
        end
      end
      i32.const 0
      i32.const 0
      i32.load offset=1055680
      local.tee 0
      local.get 1
      local.get 0
      local.get 1
      i32.gt_u
      select
      i32.const -1
      local.get 0
      select
      i32.store offset=1055672
      local.get 4
      local.get 2
      i32.le_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const -1
      i32.store offset=1055664
      return
    end)
  (func $_RNvCs5fGHjzWXRav_7___rustc13___rdl_realloc (type 7) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.const -4
                      i32.add
                      local.tee 4
                      i32.load
                      local.tee 5
                      i32.const -8
                      i32.and
                      local.tee 6
                      i32.const 4
                      i32.const 8
                      local.get 5
                      i32.const 3
                      i32.and
                      local.tee 7
                      select
                      local.get 1
                      i32.add
                      i32.lt_u
                      br_if 0 (;@9;)
                      local.get 1
                      i32.const 39
                      i32.add
                      local.set 8
                      block  ;; label = @10
                        local.get 7
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 6
                        local.get 8
                        i32.gt_u
                        br_if 2 (;@8;)
                      end
                      block  ;; label = @10
                        local.get 2
                        i32.const 9
                        i32.lt_u
                        br_if 0 (;@10;)
                        block  ;; label = @11
                          local.get 2
                          local.get 3
                          call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE8memalignCsiuSwmoQr08c_3std
                          local.tee 2
                          br_if 0 (;@11;)
                          i32.const 0
                          return
                        end
                        block  ;; label = @11
                          local.get 3
                          local.get 1
                          local.get 3
                          local.get 1
                          i32.lt_u
                          select
                          local.tee 3
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 2
                          local.get 0
                          local.get 3
                          memory.copy
                        end
                        local.get 4
                        i32.load
                        local.tee 3
                        i32.const -8
                        i32.and
                        local.tee 7
                        i32.const 4
                        i32.const 8
                        local.get 3
                        i32.const 3
                        i32.and
                        local.tee 3
                        select
                        local.get 1
                        i32.add
                        i32.lt_u
                        br_if 3 (;@7;)
                        local.get 3
                        i32.eqz
                        br_if 8 (;@2;)
                        local.get 7
                        local.get 8
                        i32.gt_u
                        br_if 4 (;@6;)
                        br 8 (;@2;)
                      end
                      i32.const 0
                      local.set 2
                      local.get 3
                      i32.const -48
                      i32.const 0
                      i32.load offset=1055676
                      local.tee 9
                      i32.sub
                      i32.const -9
                      i32.and
                      i32.const -3
                      i32.add
                      local.tee 1
                      i32.const -64
                      local.get 1
                      i32.const -64
                      i32.lt_u
                      select
                      i32.ge_u
                      br_if 8 (;@1;)
                      i32.const 16
                      local.get 3
                      i32.const 11
                      i32.add
                      i32.const -8
                      i32.and
                      local.get 3
                      i32.const 11
                      i32.lt_u
                      select
                      local.set 1
                      local.get 0
                      i32.const -8
                      i32.add
                      local.set 8
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 7
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 8
                                local.get 6
                                i32.add
                                local.set 7
                                local.get 6
                                local.get 1
                                i32.ge_u
                                br_if 3 (;@11;)
                                local.get 7
                                i32.const 0
                                i32.load offset=1055652
                                i32.eq
                                br_if 4 (;@10;)
                                local.get 7
                                i32.const 0
                                i32.load offset=1055648
                                i32.eq
                                br_if 2 (;@12;)
                                local.get 7
                                i32.load offset=4
                                local.tee 5
                                i32.const 2
                                i32.and
                                br_if 11 (;@3;)
                                local.get 5
                                i32.const -8
                                i32.and
                                local.tee 9
                                local.get 6
                                i32.add
                                local.tee 5
                                local.get 1
                                i32.lt_u
                                br_if 11 (;@3;)
                                local.get 7
                                local.get 9
                                call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE12unlink_chunkCsiuSwmoQr08c_3std
                                local.get 5
                                local.get 1
                                i32.sub
                                local.tee 7
                                i32.const 16
                                i32.lt_u
                                br_if 1 (;@13;)
                                local.get 4
                                local.get 1
                                local.get 4
                                i32.load
                                i32.const 1
                                i32.and
                                i32.or
                                i32.const 2
                                i32.or
                                i32.store
                                local.get 8
                                local.get 1
                                i32.add
                                local.tee 1
                                local.get 7
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                local.get 8
                                local.get 5
                                i32.add
                                local.tee 5
                                local.get 5
                                i32.load offset=4
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                local.get 1
                                local.get 7
                                call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE13dispose_chunkCsiuSwmoQr08c_3std
                                br 10 (;@4;)
                              end
                              local.get 1
                              i32.const 256
                              i32.lt_u
                              br_if 10 (;@3;)
                              local.get 6
                              local.get 1
                              i32.le_u
                              br_if 10 (;@3;)
                              local.get 8
                              i32.eqz
                              br_if 10 (;@3;)
                              local.get 6
                              local.get 1
                              i32.sub
                              local.get 9
                              i32.const 1
                              i32.shl
                              i32.gt_u
                              br_if 10 (;@3;)
                              local.get 0
                              return
                            end
                            local.get 4
                            local.get 5
                            local.get 4
                            i32.load
                            i32.const 1
                            i32.and
                            i32.or
                            i32.const 2
                            i32.or
                            i32.store
                            local.get 8
                            local.get 5
                            i32.add
                            local.tee 1
                            local.get 1
                            i32.load offset=4
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            br 8 (;@4;)
                          end
                          i32.const 0
                          i32.load offset=1055640
                          local.get 6
                          i32.add
                          local.tee 7
                          local.get 1
                          i32.lt_u
                          br_if 8 (;@3;)
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 7
                              local.get 1
                              i32.sub
                              local.tee 6
                              i32.const 15
                              i32.gt_u
                              br_if 0 (;@13;)
                              local.get 4
                              local.get 5
                              i32.const 1
                              i32.and
                              local.get 7
                              i32.or
                              i32.const 2
                              i32.or
                              i32.store
                              local.get 8
                              local.get 7
                              i32.add
                              local.tee 1
                              local.get 1
                              i32.load offset=4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              i32.const 0
                              local.set 6
                              i32.const 0
                              local.set 1
                              br 1 (;@12;)
                            end
                            local.get 4
                            local.get 1
                            local.get 5
                            i32.const 1
                            i32.and
                            i32.or
                            i32.const 2
                            i32.or
                            i32.store
                            local.get 8
                            local.get 1
                            i32.add
                            local.tee 1
                            local.get 6
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            local.get 8
                            local.get 7
                            i32.add
                            local.tee 7
                            local.get 6
                            i32.store
                            local.get 7
                            local.get 7
                            i32.load offset=4
                            i32.const -2
                            i32.and
                            i32.store offset=4
                          end
                          i32.const 0
                          local.get 1
                          i32.store offset=1055648
                          i32.const 0
                          local.get 6
                          i32.store offset=1055640
                          br 7 (;@4;)
                        end
                        local.get 6
                        local.get 1
                        i32.sub
                        local.tee 6
                        i32.const 15
                        i32.le_u
                        br_if 6 (;@4;)
                        local.get 4
                        local.get 1
                        local.get 5
                        i32.const 1
                        i32.and
                        i32.or
                        i32.const 2
                        i32.or
                        i32.store
                        local.get 8
                        local.get 1
                        i32.add
                        local.tee 1
                        local.get 6
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 7
                        local.get 7
                        i32.load offset=4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 1
                        local.get 6
                        call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE13dispose_chunkCsiuSwmoQr08c_3std
                        br 6 (;@4;)
                      end
                      i32.const 0
                      i32.load offset=1055644
                      local.get 6
                      i32.add
                      local.tee 7
                      local.get 1
                      i32.gt_u
                      br_if 4 (;@5;)
                      br 6 (;@3;)
                    end
                    i32.const 1050192
                    i32.const 46
                    i32.const 1050240
                    call $_RNvNtCs4NIexwV1FGX_4core9panicking5panic
                    unreachable
                  end
                  i32.const 1050256
                  i32.const 46
                  i32.const 1050304
                  call $_RNvNtCs4NIexwV1FGX_4core9panicking5panic
                  unreachable
                end
                i32.const 1050192
                i32.const 46
                i32.const 1050240
                call $_RNvNtCs4NIexwV1FGX_4core9panicking5panic
                unreachable
              end
              i32.const 1050256
              i32.const 46
              i32.const 1050304
              call $_RNvNtCs4NIexwV1FGX_4core9panicking5panic
              unreachable
            end
            local.get 4
            local.get 1
            local.get 5
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 8
            local.get 1
            i32.add
            local.tee 5
            local.get 7
            local.get 1
            i32.sub
            local.tee 1
            i32.const 1
            i32.or
            i32.store offset=4
            i32.const 0
            local.get 1
            i32.store offset=1055644
            i32.const 0
            local.get 5
            i32.store offset=1055652
          end
          local.get 8
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          return
        end
        local.get 3
        call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE6mallocCsiuSwmoQr08c_3std
        local.tee 1
        i32.eqz
        br_if 1 (;@1;)
        block  ;; label = @3
          local.get 3
          i32.const -4
          i32.const -8
          local.get 4
          i32.load
          local.tee 2
          i32.const 3
          i32.and
          select
          local.get 2
          i32.const -8
          i32.and
          i32.add
          local.tee 2
          local.get 3
          local.get 2
          i32.lt_u
          select
          local.tee 3
          br_if 0 (;@3;)
          local.get 1
          local.set 2
          br 1 (;@2;)
        end
        local.get 1
        local.get 0
        local.get 3
        memory.copy
        local.get 1
        local.set 2
      end
      local.get 0
      call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE4freeCsiuSwmoQr08c_3std
    end
    local.get 2)
  (func $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE12unlink_chunkCsiuSwmoQr08c_3std (type 1) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 0
    i32.load offset=12
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
            local.get 0
            i32.load offset=24
            local.set 3
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  local.get 0
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 0
                  i32.const 20
                  i32.const 16
                  local.get 0
                  i32.load offset=20
                  local.tee 2
                  select
                  i32.add
                  i32.load
                  local.tee 1
                  br_if 1 (;@6;)
                  i32.const 0
                  local.set 2
                  br 2 (;@5;)
                end
                local.get 0
                i32.load offset=8
                local.tee 1
                local.get 2
                i32.store offset=12
                local.get 2
                local.get 1
                i32.store offset=8
                br 1 (;@5;)
              end
              local.get 0
              i32.const 20
              i32.add
              local.get 0
              i32.const 16
              i32.add
              local.get 2
              select
              local.set 4
              loop  ;; label = @6
                local.get 4
                local.set 5
                local.get 1
                local.tee 2
                i32.const 20
                i32.add
                local.get 2
                i32.const 16
                i32.add
                local.get 2
                i32.load offset=20
                local.tee 1
                select
                local.set 4
                local.get 2
                i32.const 20
                i32.const 16
                local.get 1
                select
                i32.add
                i32.load
                local.tee 1
                br_if 0 (;@6;)
              end
              local.get 5
              i32.const 0
              i32.store
            end
            local.get 3
            i32.eqz
            br_if 2 (;@2;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                local.get 0
                i32.load offset=28
                i32.const 2
                i32.shl
                i32.const 1055224
                i32.add
                local.tee 1
                i32.load
                i32.eq
                br_if 0 (;@6;)
                local.get 3
                i32.load offset=16
                local.get 0
                i32.eq
                br_if 1 (;@5;)
                local.get 3
                local.get 2
                i32.store offset=20
                local.get 2
                br_if 3 (;@3;)
                br 4 (;@2;)
              end
              local.get 1
              local.get 2
              i32.store
              local.get 2
              i32.eqz
              br_if 4 (;@1;)
              br 2 (;@3;)
            end
            local.get 3
            local.get 2
            i32.store offset=16
            local.get 2
            br_if 1 (;@3;)
            br 2 (;@2;)
          end
          block  ;; label = @4
            local.get 2
            local.get 0
            i32.load offset=8
            local.tee 4
            i32.eq
            br_if 0 (;@4;)
            local.get 4
            local.get 2
            i32.store offset=12
            local.get 2
            local.get 4
            i32.store offset=8
            return
          end
          i32.const 0
          i32.const 0
          i32.load offset=1055632
          i32.const -2
          local.get 1
          i32.const 3
          i32.shr_u
          i32.rotl
          i32.and
          i32.store offset=1055632
          return
        end
        local.get 2
        local.get 3
        i32.store offset=24
        block  ;; label = @3
          local.get 0
          i32.load offset=16
          local.tee 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 1
          i32.store offset=16
          local.get 1
          local.get 2
          i32.store offset=24
        end
        local.get 0
        i32.load offset=20
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.store offset=20
        local.get 1
        local.get 2
        i32.store offset=24
        return
      end
      return
    end
    i32.const 0
    i32.const 0
    i32.load offset=1055636
    i32.const -2
    local.get 0
    i32.load offset=28
    i32.rotl
    i32.and
    i32.store offset=1055636)
  (func $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE13dispose_chunkCsiuSwmoQr08c_3std (type 1) (param i32 i32)
    (local i32 i32)
    local.get 0
    local.get 1
    i32.add
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 3
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          local.get 3
          i32.const 2
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          i32.load
          local.tee 3
          local.get 1
          i32.add
          local.set 1
          block  ;; label = @4
            local.get 0
            local.get 3
            i32.sub
            local.tee 0
            i32.const 0
            i32.load offset=1055648
            i32.ne
            br_if 0 (;@4;)
            local.get 2
            i32.load offset=4
            i32.const 3
            i32.and
            i32.const 3
            i32.ne
            br_if 1 (;@3;)
            i32.const 0
            local.get 1
            i32.store offset=1055640
            local.get 2
            local.get 2
            i32.load offset=4
            i32.const -2
            i32.and
            i32.store offset=4
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 2
            local.get 1
            i32.store
            return
          end
          local.get 0
          local.get 3
          call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE12unlink_chunkCsiuSwmoQr08c_3std
        end
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.load offset=4
              local.tee 3
              i32.const 2
              i32.and
              br_if 0 (;@5;)
              local.get 2
              i32.const 0
              i32.load offset=1055652
              i32.eq
              br_if 2 (;@3;)
              local.get 2
              i32.const 0
              i32.load offset=1055648
              i32.eq
              br_if 4 (;@1;)
              local.get 2
              local.get 3
              i32.const -8
              i32.and
              local.tee 3
              call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE12unlink_chunkCsiuSwmoQr08c_3std
              local.get 0
              local.get 3
              local.get 1
              i32.add
              local.tee 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 1
              i32.store
              local.get 0
              i32.const 0
              i32.load offset=1055648
              i32.ne
              br_if 1 (;@4;)
              i32.const 0
              local.get 1
              i32.store offset=1055640
              return
            end
            local.get 2
            local.get 3
            i32.const -2
            i32.and
            i32.store offset=4
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            local.get 1
            i32.add
            local.get 1
            i32.store
          end
          block  ;; label = @4
            local.get 1
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
            local.get 0
            local.get 1
            call $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE18insert_large_chunkCsiuSwmoQr08c_3std
            return
          end
          block  ;; label = @4
            block  ;; label = @5
              i32.const 0
              i32.load offset=1055632
              local.tee 2
              i32.const 1
              local.get 1
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 3
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.get 2
              local.get 3
              i32.or
              i32.store offset=1055632
              local.get 1
              i32.const 248
              i32.and
              i32.const 1055368
              i32.add
              local.tee 1
              local.set 2
              br 1 (;@4;)
            end
            local.get 1
            i32.const 248
            i32.and
            local.tee 1
            i32.const 1055368
            i32.add
            local.set 2
            local.get 1
            i32.const 1055376
            i32.add
            i32.load
            local.set 1
          end
          local.get 2
          local.get 0
          i32.store offset=8
          local.get 1
          local.get 0
          i32.store offset=12
          local.get 0
          local.get 2
          i32.store offset=12
          local.get 0
          local.get 1
          i32.store offset=8
          return
        end
        i32.const 0
        local.get 0
        i32.store offset=1055652
        i32.const 0
        i32.const 0
        i32.load offset=1055644
        local.get 1
        i32.add
        local.tee 1
        i32.store offset=1055644
        local.get 0
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.load offset=1055648
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        i32.const 0
        i32.store offset=1055640
        i32.const 0
        i32.const 0
        i32.store offset=1055648
      end
      return
    end
    i32.const 0
    local.get 0
    i32.store offset=1055648
    i32.const 0
    i32.const 0
    i32.load offset=1055640
    local.get 1
    i32.add
    local.tee 1
    i32.store offset=1055640
    local.get 0
    local.get 1
    i32.const 1
    i32.or
    i32.store offset=4
    local.get 0
    local.get 1
    i32.add
    local.get 1
    i32.store)
  (func $_RNvCs5fGHjzWXRav_7___rustc17rust_begin_unwind (type 2) (param i32)
    (local i32 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 0
    i64.load align=4
    local.set 2
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    local.get 2
    i64.store offset=4 align=4
    local.get 1
    i32.const 4
    i32.add
    call $_RINvNtNtCsiuSwmoQr08c_3std3sys9backtrace26___rust_end_short_backtraceNCNvNtB6_9panicking13panic_handler0zEB6_
    unreachable)
  (func $_RNvCs5fGHjzWXRav_7___rustc26___rust_alloc_error_handler (type 1) (param i32 i32)
    local.get 1
    local.get 0
    call $_RNvNtCsiuSwmoQr08c_3std5alloc8rust_oom
    unreachable)
  (func $_RNvNtCsiuSwmoQr08c_3std5alloc8rust_oom (type 1) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 1
    i32.store offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    local.get 2
    i32.const 8
    i32.add
    call $_RINvNtNtCsiuSwmoQr08c_3std3sys9backtrace26___rust_end_short_backtraceNCNvNtB6_5alloc8rust_oom0zEB6_
    unreachable)
  (func $_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE18insert_large_chunkCsiuSwmoQr08c_3std (type 1) (param i32 i32)
    (local i32 i32 i32 i32)
    i32.const 0
    local.set 2
    block  ;; label = @1
      local.get 1
      i32.const 8
      i32.shr_u
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      i32.const 31
      local.set 2
      local.get 1
      i32.const 16777216
      i32.ge_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 38
      local.get 3
      i32.clz
      local.tee 2
      i32.sub
      i32.shr_u
      i32.const 1
      i32.and
      local.get 2
      i32.const 1
      i32.shl
      i32.or
      i32.const 62
      i32.xor
      local.set 2
    end
    local.get 0
    i64.const 0
    i64.store offset=16 align=4
    local.get 0
    local.get 2
    i32.store offset=28
    local.get 2
    i32.const 2
    i32.shl
    i32.const 1055224
    i32.add
    local.set 3
    block  ;; label = @1
      i32.const 0
      i32.load offset=1055636
      i32.const 1
      local.get 2
      i32.shl
      local.tee 4
      i32.and
      br_if 0 (;@1;)
      local.get 3
      local.get 0
      i32.store
      local.get 0
      local.get 3
      i32.store offset=24
      local.get 0
      local.get 0
      i32.store offset=12
      local.get 0
      local.get 0
      i32.store offset=8
      i32.const 0
      i32.const 0
      i32.load offset=1055636
      local.get 4
      i32.or
      i32.store offset=1055636
      return
    end
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.load
          local.tee 4
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 1
          i32.ne
          br_if 0 (;@3;)
          local.get 4
          local.set 2
          br 1 (;@2;)
        end
        local.get 1
        i32.const 0
        i32.const 25
        local.get 2
        i32.const 1
        i32.shr_u
        i32.sub
        local.get 2
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set 3
        loop  ;; label = @3
          local.get 4
          local.get 3
          i32.const 29
          i32.shr_u
          i32.const 4
          i32.and
          i32.add
          local.tee 5
          i32.load offset=16
          local.tee 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.const 1
          i32.shl
          local.set 3
          local.get 2
          local.set 4
          local.get 2
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 1
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 2
      i32.load offset=8
      local.tee 3
      local.get 0
      i32.store offset=12
      local.get 2
      local.get 0
      i32.store offset=8
      local.get 0
      i32.const 0
      i32.store offset=24
      local.get 0
      local.get 2
      i32.store offset=12
      local.get 0
      local.get 3
      i32.store offset=8
      return
    end
    local.get 5
    i32.const 16
    i32.add
    local.get 0
    i32.store
    local.get 0
    local.get 4
    i32.store offset=24
    local.get 0
    local.get 0
    i32.store offset=12
    local.get 0
    local.get 0
    i32.store offset=8)
  (func $_RNvNtNtCsiuSwmoQr08c_3std9panicking11panic_count8increase (type 4) (param i32) (result i32)
    (local i32 i32)
    i32.const 0
    local.set 1
    i32.const 0
    i32.const 0
    i32.load offset=1055712
    local.tee 2
    i32.const 1
    i32.add
    i32.store offset=1055712
    block  ;; label = @1
      local.get 2
      i32.const 0
      i32.lt_s
      br_if 0 (;@1;)
      i32.const 1
      local.set 1
      i32.const 0
      i32.load8_u offset=1055692
      br_if 0 (;@1;)
      i32.const 0
      local.get 0
      i32.store8 offset=1055692
      i32.const 0
      i32.const 0
      i32.load offset=1055688
      i32.const 1
      i32.add
      i32.store offset=1055688
      i32.const 2
      local.set 1
    end
    local.get 1)
  (func $_RNvXNtCs4NIexwV1FGX_4core3anyNtNtCs5JHfeWeQK0x_5alloc6string6StringNtB2_3Any7type_idCsiuSwmoQr08c_3std (type 1) (param i32 i32)
    local.get 0
    i32.const 0
    i64.load offset=1050184 align=4
    i64.store offset=8 align=4
    local.get 0
    i32.const 0
    i64.load offset=1050176 align=4
    i64.store align=4)
  (func $_RNvXNtCs4NIexwV1FGX_4core3anyReNtB2_3Any7type_idCsiuSwmoQr08c_3std (type 1) (param i32 i32)
    local.get 0
    i32.const 0
    i64.load offset=1050168 align=4
    i64.store offset=8 align=4
    local.get 0
    i32.const 0
    i64.load offset=1050160 align=4
    i64.store align=4)
  (func $_RNvXs0_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_19FormatStringPayloadNtNtCs4NIexwV1FGX_4core3fmt7Display3fmt (type 0) (param i32 i32) (result i32)
    block  ;; label = @1
      local.get 0
      i32.load
      i32.const -1
      i32.eq
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      i32.load offset=4
      local.get 0
      i32.load offset=8
      call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter9write_str
      return
    end
    local.get 1
    i32.load
    local.get 1
    i32.load offset=4
    local.get 0
    i32.load offset=12
    i32.load
    local.tee 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_RNvNtCs4NIexwV1FGX_4core3fmt5write)
  (func $_RNvXs1_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload3get (type 1) (param i32 i32)
    local.get 0
    i32.const 1050380
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_RNvXs1_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload6as_str (type 1) (param i32 i32)
    local.get 0
    local.get 1
    i64.load align=4
    i64.store)
  (func $_RNvXs1_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload8take_box (type 1) (param i32 i32)
    (local i32 i32)
    local.get 1
    i32.load offset=4
    local.set 2
    local.get 1
    i32.load
    local.set 3
    call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block  ;; label = @1
      i32.const 8
      i32.const 4
      call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
      local.tee 1
      br_if 0 (;@1;)
      i32.const 4
      i32.const 8
      call $_RNvNtCs5JHfeWeQK0x_5alloc5alloc18handle_alloc_error
      unreachable
    end
    local.get 1
    local.get 2
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 0
    i32.const 1050380
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_RNvXs2_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCs4NIexwV1FGX_4core3fmt7Display3fmt (type 0) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter9write_str)
  (func $_RNvXsZ_NtCs5JHfeWeQK0x_5alloc6stringNtB5_6StringNtNtCs4NIexwV1FGX_4core3fmt5Write10write_char (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load offset=8
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 128
        i32.ge_u
        br_if 0 (;@2;)
        i32.const 1
        local.set 3
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 1
        i32.const 2048
        i32.ge_u
        br_if 0 (;@2;)
        i32.const 2
        local.set 3
        br 1 (;@1;)
      end
      i32.const 3
      i32.const 4
      local.get 1
      i32.const 65536
      i32.lt_u
      select
      local.set 3
    end
    block  ;; label = @1
      local.get 3
      local.get 0
      i32.load
      local.get 2
      i32.sub
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 3
      i32.const 1
      i32.const 1
      call $_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsiuSwmoQr08c_3std
    end
    local.get 0
    i32.load offset=4
    local.get 2
    i32.add
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 5
        local.get 1
        i32.const 6
        i32.shr_u
        local.set 6
        block  ;; label = @3
          local.get 1
          i32.const 2048
          i32.ge_u
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i32.store8 offset=1
          local.get 4
          local.get 6
          i32.const 192
          i32.or
          i32.store8
          br 2 (;@1;)
        end
        local.get 1
        i32.const 12
        i32.shr_u
        local.set 7
        local.get 6
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 6
        block  ;; label = @3
          local.get 1
          i32.const 65535
          i32.gt_u
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i32.store8 offset=2
          local.get 4
          local.get 6
          i32.store8 offset=1
          local.get 4
          local.get 7
          i32.const 224
          i32.or
          i32.store8
          br 2 (;@1;)
        end
        local.get 4
        local.get 5
        i32.store8 offset=3
        local.get 4
        local.get 6
        i32.store8 offset=2
        local.get 4
        local.get 7
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        i32.store8 offset=1
        local.get 4
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const -16
        i32.or
        i32.store8
        br 1 (;@1;)
      end
      local.get 4
      local.get 1
      i32.store8
    end
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store offset=8
    i32.const 0)
  (func $_RNvXsZ_NtCs5JHfeWeQK0x_5alloc6stringNtB5_6StringNtNtCs4NIexwV1FGX_4core3fmt5Write9write_str (type 3) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          local.get 0
          i32.load
          local.get 0
          i32.load offset=8
          local.tee 3
          i32.sub
          i32.le_u
          br_if 0 (;@3;)
          local.get 0
          local.get 3
          local.get 2
          i32.const 1
          i32.const 1
          call $_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsiuSwmoQr08c_3std
          local.get 0
          i32.load offset=8
          local.set 3
          br 1 (;@2;)
        end
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 3
      i32.add
      local.get 1
      local.get 2
      memory.copy
    end
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store offset=8
    i32.const 0)
  (func $_RNvXs_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB4_19FormatStringPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload3get (type 1) (param i32 i32)
    (local i32 i32 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      local.get 1
      i32.load
      i32.const -1
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      local.set 3
      local.get 2
      i32.const 0
      i32.store offset=28
      local.get 2
      i64.const 4294967296
      i64.store offset=20 align=4
      local.get 2
      i32.const 20
      i32.add
      i32.const 1050080
      local.get 3
      i32.load
      local.tee 3
      i32.load
      local.get 3
      i32.load offset=4
      call $_RNvNtCs4NIexwV1FGX_4core3fmt5write
      drop
      local.get 2
      local.get 2
      i32.load offset=28
      local.tee 3
      i32.store offset=16
      local.get 2
      local.get 2
      i64.load offset=20 align=4
      local.tee 4
      i64.store offset=8
      local.get 1
      local.get 3
      i32.store offset=8
      local.get 1
      local.get 4
      i64.store align=4
    end
    local.get 0
    i32.const 1050396
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer)
  (func $_RNvXs_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB4_19FormatStringPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload8take_box (type 1) (param i32 i32)
    (local i32 i32 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      local.get 1
      i32.load
      i32.const -1
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      local.set 3
      local.get 2
      i32.const 0
      i32.store offset=24
      local.get 2
      i64.const 4294967296
      i64.store offset=16 align=4
      local.get 2
      i32.const 16
      i32.add
      i32.const 1050080
      local.get 3
      i32.load
      local.tee 3
      i32.load
      local.get 3
      i32.load offset=4
      call $_RNvNtCs4NIexwV1FGX_4core3fmt5write
      drop
      local.get 2
      local.get 2
      i32.load offset=24
      local.tee 3
      i32.store offset=8
      local.get 2
      local.get 2
      i64.load offset=16 align=4
      local.tee 4
      i64.store
      local.get 1
      local.get 3
      i32.store offset=8
      local.get 1
      local.get 4
      i64.store align=4
    end
    local.get 1
    i32.load offset=8
    local.set 3
    local.get 1
    i32.const 0
    i32.store offset=8
    local.get 1
    i64.load align=4
    local.set 4
    local.get 1
    i64.const 4294967296
    i64.store align=4
    local.get 2
    local.get 3
    i32.store offset=24
    local.get 2
    local.get 4
    i64.store offset=16
    call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block  ;; label = @1
      i32.const 12
      i32.const 4
      call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
      local.tee 1
      br_if 0 (;@1;)
      i32.const 4
      i32.const 12
      call $_RNvNtCs5JHfeWeQK0x_5alloc5alloc18handle_alloc_error
      unreachable
    end
    local.get 1
    local.get 2
    i32.load offset=24
    i32.store offset=8
    local.get 1
    local.get 2
    i64.load offset=16
    i64.store align=4
    local.get 0
    i32.const 1050396
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer)
  (func $_RNvYINtNvNtCsiuSwmoQr08c_3std9panicking11begin_panic7PayloadReENtNtCs4NIexwV1FGX_4core5panic12PanicPayload6as_strB9_ (type 1) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store)
  (func $_RNvYNtNtCs5JHfeWeQK0x_5alloc6string6StringNtNtCs4NIexwV1FGX_4core3fmt5Write9write_fmtCsiuSwmoQr08c_3std (type 3) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 1050080
    local.get 1
    local.get 2
    call $_RNvNtCs4NIexwV1FGX_4core3fmt5write)
  (func $_RNvXs_NtCsbw64Iw7W9Hn_8dlmalloc3sysNtB4_6SystemNtB6_9Allocator5alloc (type 5) (param i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        i32.load8_u offset=1055717
        local.set 3
        i32.const 0
        i32.const 1
        i32.store8 offset=1055717
        i32.const 1055728
        local.set 4
        i32.const 1114112
        i32.const 1055728
        i32.le_u
        br_if 0 (;@2;)
        local.get 2
        i32.const 1114112
        i32.const 1055728
        i32.sub
        i32.gt_u
        br_if 0 (;@2;)
        local.get 3
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 1114112
        i32.const 1055728
        i32.sub
        local.set 2
        br 1 (;@1;)
      end
      i32.const 0
      local.set 4
      block  ;; label = @2
        local.get 2
        i32.const 16
        i32.shr_u
        local.get 2
        i32.const 65535
        i32.and
        i32.const 0
        i32.ne
        i32.add
        local.tee 2
        memory.grow
        local.tee 3
        i32.const -1
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 2
      i32.const 16
      i32.shl
      local.tee 2
      i32.const -16
      i32.add
      local.get 2
      local.get 3
      i32.const 16
      i32.shl
      local.tee 4
      i32.const 0
      local.get 2
      i32.sub
      i32.eq
      select
      local.set 2
    end
    local.get 0
    i32.const 0
    i32.store offset=8
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 4
    i32.store)
  (func $_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueNtNtCs5JHfeWeQK0x_5alloc6string6StringEBF_ (type 2) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc
    end)
  (func $_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalEBa_ (type 5) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      local.get 2
      local.get 1
      i32.add
      local.tee 1
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      call $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec12handle_error
      unreachable
    end
    local.get 3
    i32.const 4
    i32.add
    local.get 0
    i32.load
    local.tee 2
    local.get 0
    i32.load offset=4
    local.get 1
    local.get 2
    i32.const 1
    i32.shl
    local.tee 2
    local.get 1
    local.get 2
    i32.gt_u
    select
    local.tee 2
    i32.const 8
    local.get 2
    i32.const 8
    i32.gt_u
    select
    local.tee 2
    call $_RNvMs5_NtCs5JHfeWeQK0x_5alloc7raw_vecNtB5_11RawVecInner11finish_growB7_
    block  ;; label = @1
      local.get 3
      i32.load offset=4
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=8
      local.get 3
      i32.load offset=12
      call $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec12handle_error
      unreachable
    end
    local.get 3
    i32.load offset=8
    local.set 1
    local.get 0
    local.get 2
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec12handle_error (type 1) (param i32 i32)
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      call $_RNvNtCs5JHfeWeQK0x_5alloc5alloc18handle_alloc_error
      unreachable
    end
    call $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec17capacity_overflow
    unreachable)
  (func $_RNvMs5_NtCs5JHfeWeQK0x_5alloc7raw_vecNtB5_11RawVecInner11finish_growB7_ (type 11) (param i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.const 0
        i32.ge_s
        br_if 0 (;@2;)
        i32.const 1
        local.set 1
        i32.const 4
        local.set 2
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              local.get 1
              i32.const 1
              local.get 3
              call $_RNvCs5fGHjzWXRav_7___rustc14___rust_realloc
              local.set 1
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 3
              br_if 0 (;@5;)
              i32.const 1
              local.set 1
              br 2 (;@3;)
            end
            call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
            local.get 3
            i32.const 1
            call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
            local.set 1
          end
          local.get 1
          br_if 0 (;@3;)
          i32.const 1
          local.set 1
          local.get 0
          i32.const 1
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 0
        local.get 1
        i32.store offset=4
        i32.const 0
        local.set 1
      end
      i32.const 8
      local.set 2
    end
    local.get 0
    local.get 2
    i32.add
    local.get 3
    i32.store
    local.get 0
    local.get 1
    i32.store)
  (func $_RINvNvNtNtCs5JHfeWeQK0x_5alloc2io5error21custom_owner_from_box12drop_box_rawDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB1h_6marker4SendNtB1O_4SyncEL_EB8_ (type 1) (param i32 i32)
    (local i32)
    block  ;; label = @1
      local.get 1
      i32.load
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      call_indirect (type 2)
    end
    block  ;; label = @1
      local.get 1
      i32.load offset=4
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 1
      i32.load offset=8
      call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc
    end)
  (func $_RINvNvNtNtCs5JHfeWeQK0x_5alloc2io5error21custom_owner_from_box12drop_box_rawNtNtNtCs4NIexwV1FGX_4core2io5error6CustomEB8_ (type 2) (param i32)
    local.get 0
    call $_RNvXs9_NtNtCs4NIexwV1FGX_4core2io5errorNtB5_6CustomNtNtNtB9_3ops4drop4Drop4drop
    local.get 0
    i32.const 20
    i32.const 4
    call $_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc)
  (func $_RNvNtCs5JHfeWeQK0x_5alloc5alloc18handle_alloc_error (type 1) (param i32 i32)
    local.get 1
    local.get 0
    call $_RNvCs5fGHjzWXRav_7___rustc26___rust_alloc_error_handler
    unreachable)
  (func $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec17capacity_overflow (type 8)
    i32.const 1050468
    i32.const 35
    i32.const 1050488
    call $_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt
    unreachable)
  (func $_RNvNtNtCs5JHfeWeQK0x_5alloc2io5error21custom_owner_from_box (type 3) (param i32 i32 i32) (result i32)
    (local i32)
    call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block  ;; label = @1
      i32.const 20
      i32.const 4
      call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
      local.tee 3
      br_if 0 (;@1;)
      i32.const 4
      i32.const 20
      call $_RNvNtCs5JHfeWeQK0x_5alloc5alloc18handle_alloc_error
      unreachable
    end
    local.get 3
    local.get 0
    i32.store8 offset=16
    local.get 3
    i32.const 37
    i32.store offset=12
    local.get 3
    i32.const 38
    i32.store offset=8
    local.get 3
    local.get 2
    i32.store offset=4
    local.get 3
    local.get 1
    i32.store
    local.get 3)
  (func $_RNvNvNtCs5JHfeWeQK0x_5alloc3fmt6format12format_inner (type 5) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 1
                  i32.shr_u
                  local.set 4
                  br 1 (;@6;)
                end
                local.get 1
                i32.load8_u
                local.tee 4
                i32.eqz
                br_if 1 (;@5;)
                i32.const 0
                local.set 5
                local.get 1
                local.set 6
                i32.const 0
                local.set 7
                loop  ;; label = @7
                  local.get 6
                  i32.const 1
                  i32.add
                  local.set 6
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 4
                      i32.extend8_s
                      i32.const -1
                      i32.gt_s
                      br_if 0 (;@9;)
                      block  ;; label = @10
                        local.get 4
                        i32.const 255
                        i32.and
                        i32.const 128
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 5
                        local.get 6
                        i32.load16_u align=1
                        local.tee 4
                        i32.add
                        local.set 5
                        local.get 6
                        local.get 4
                        i32.add
                        i32.const 2
                        i32.add
                        local.set 6
                        br 2 (;@8;)
                      end
                      local.get 6
                      local.get 4
                      i32.const 3
                      i32.and
                      i32.const 8
                      i32.rotr
                      local.tee 8
                      i32.const 5
                      i32.shl
                      i32.const 1073741824
                      i32.and
                      local.get 8
                      i32.const 7
                      i32.shl
                      i32.or
                      i32.const 29
                      i32.shr_u
                      i32.add
                      local.get 4
                      i32.const 1
                      i32.shr_u
                      i32.const 2
                      i32.and
                      i32.add
                      local.get 4
                      i32.const 2
                      i32.shr_u
                      i32.const 2
                      i32.and
                      i32.add
                      local.set 6
                      local.get 5
                      i32.eqz
                      local.get 7
                      i32.or
                      local.set 7
                      br 1 (;@8;)
                    end
                    local.get 6
                    local.get 4
                    i32.const 255
                    i32.and
                    local.tee 4
                    i32.add
                    local.set 6
                    local.get 5
                    local.get 4
                    i32.add
                    local.set 5
                  end
                  local.get 6
                  i32.load8_u
                  local.tee 4
                  br_if 0 (;@7;)
                end
                i32.const 0
                local.set 4
                local.get 7
                local.get 5
                i32.const 16
                i32.lt_u
                i32.and
                br_if 0 (;@6;)
                i32.const 0
                local.set 7
                local.get 5
                i32.const 1
                i32.shl
                local.tee 4
                i32.const 0
                i32.lt_s
                br_if 4 (;@2;)
              end
              local.get 4
              br_if 1 (;@4;)
            end
            i32.const 1
            local.set 6
            i32.const 0
            local.set 4
            br 1 (;@3;)
          end
          call $_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2
          i32.const 1
          local.set 7
          local.get 4
          i32.const 1
          call $_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc
          local.tee 6
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 3
        i32.const 0
        i32.store offset=8
        local.get 3
        local.get 6
        i32.store offset=4
        local.get 3
        local.get 4
        i32.store
        local.get 3
        i32.const 1050504
        local.get 1
        local.get 2
        call $_RNvNtCs4NIexwV1FGX_4core3fmt5write
        i32.eqz
        br_if 1 (;@1;)
        i32.const 1050544
        i32.const 86
        local.get 3
        i32.const 15
        i32.add
        i32.const 1050528
        i32.const 1050632
        call $_RNvNtCs4NIexwV1FGX_4core6result13unwrap_failed
        unreachable
      end
      local.get 7
      local.get 4
      call $_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec12handle_error
      unreachable
    end
    local.get 0
    local.get 3
    i32.load offset=8
    i32.store offset=8
    local.get 0
    local.get 3
    i64.load align=4
    i64.store align=4
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_RNvXs0_NvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBd_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB12_6marker4SendNtB1z_4SyncEL_EINtNtB12_7convert4FromNtNtBf_6string6StringE4fromNtB5_11StringErrorNtNtB12_3fmt5Debug3fmt (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    local.get 1
    call $_RNvXsh_NtCs4NIexwV1FGX_4core3fmteNtB5_5Debug3fmt)
  (func $_RNvXsK_NtCs4NIexwV1FGX_4core3fmtNtB5_5ErrorNtB5_5Debug3fmt (type 0) (param i32 i32) (result i32)
    local.get 1
    i32.const 1050648
    i32.const 5
    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter9write_str)
  (func $_RNvXsZ_NtCs5JHfeWeQK0x_5alloc6stringNtB5_6StringNtNtCs4NIexwV1FGX_4core3fmt5Write10write_char.1 (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load offset=8
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 128
        i32.ge_u
        br_if 0 (;@2;)
        i32.const 1
        local.set 3
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 1
        i32.const 2048
        i32.ge_u
        br_if 0 (;@2;)
        i32.const 2
        local.set 3
        br 1 (;@1;)
      end
      i32.const 3
      i32.const 4
      local.get 1
      i32.const 65536
      i32.lt_u
      select
      local.set 3
    end
    block  ;; label = @1
      local.get 3
      local.get 0
      i32.load
      local.get 2
      i32.sub
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 3
      call $_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalEBa_
    end
    local.get 0
    i32.load offset=4
    local.get 2
    i32.add
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 5
        local.get 1
        i32.const 6
        i32.shr_u
        local.set 6
        block  ;; label = @3
          local.get 1
          i32.const 2048
          i32.ge_u
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i32.store8 offset=1
          local.get 4
          local.get 6
          i32.const 192
          i32.or
          i32.store8
          br 2 (;@1;)
        end
        local.get 1
        i32.const 12
        i32.shr_u
        local.set 7
        local.get 6
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 6
        block  ;; label = @3
          local.get 1
          i32.const 65535
          i32.gt_u
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i32.store8 offset=2
          local.get 4
          local.get 6
          i32.store8 offset=1
          local.get 4
          local.get 7
          i32.const 224
          i32.or
          i32.store8
          br 2 (;@1;)
        end
        local.get 4
        local.get 5
        i32.store8 offset=3
        local.get 4
        local.get 6
        i32.store8 offset=2
        local.get 4
        local.get 7
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        i32.store8 offset=1
        local.get 4
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const -16
        i32.or
        i32.store8
        br 1 (;@1;)
      end
      local.get 4
      local.get 1
      i32.store8
    end
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store offset=8
    i32.const 0)
  (func $_RNvXsZ_NtCs5JHfeWeQK0x_5alloc6stringNtB5_6StringNtNtCs4NIexwV1FGX_4core3fmt5Write9write_str.1 (type 3) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          local.get 0
          i32.load
          local.get 0
          i32.load offset=8
          local.tee 3
          i32.sub
          i32.le_u
          br_if 0 (;@3;)
          local.get 0
          local.get 3
          local.get 2
          call $_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalEBa_
          local.get 0
          i32.load offset=8
          local.set 3
          br 1 (;@2;)
        end
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 3
      i32.add
      local.get 1
      local.get 2
      memory.copy
    end
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store offset=8
    i32.const 0)
  (func $_RNvXs_NvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4fromNtB4_11StringErrorNtNtB11_3fmt7Display3fmt (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    local.get 1
    call $_RNvXsi_NtCs4NIexwV1FGX_4core3fmteNtB5_7Display3fmt)
  (func $_RNvYNtNtCs5JHfeWeQK0x_5alloc6string6StringNtNtCs4NIexwV1FGX_4core3fmt5Write9write_fmtB6_ (type 3) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 1050504
    local.get 1
    local.get 2
    call $_RNvNtCs4NIexwV1FGX_4core3fmt5write)
  (func $_RNvNtCs4NIexwV1FGX_4core9panicking5panic (type 5) (param i32 i32 i32)
    local.get 0
    local.get 1
    i32.const 1
    i32.shl
    i32.const 1
    i32.or
    local.get 2
    call $_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt
    unreachable)
  (func $_RNvXsd_NtNtNtCs4NIexwV1FGX_4core3fmt3num3impyNtB9_7Display3fmt (type 0) (param i32 i32) (result i32)
    (local i32 i32 i64 i64 i64 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 20
    local.set 3
    local.get 0
    i64.load
    local.tee 4
    local.set 5
    block  ;; label = @1
      local.get 4
      i64.const 1000
      i64.lt_u
      br_if 0 (;@1;)
      i32.const 20
      local.set 3
      local.get 4
      local.set 5
      loop  ;; label = @2
        local.get 2
        i32.const 12
        i32.add
        local.get 3
        i32.add
        local.tee 0
        i32.const -4
        i32.add
        local.get 5
        local.tee 6
        local.get 6
        i64.const 10000
        i64.div_u
        local.tee 5
        i64.const 10000
        i64.mul
        i64.sub
        i32.wrap_i64
        local.tee 7
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 8
        i32.const 1
        i32.shl
        i32.load16_u offset=1053335 align=1
        i32.store16 align=1
        local.get 0
        i32.const -2
        i32.add
        local.get 7
        local.get 8
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.load16_u offset=1053335 align=1
        i32.store16 align=1
        local.get 3
        i32.const -4
        i32.add
        local.set 3
        local.get 6
        i64.const 9999999
        i64.gt_u
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      local.get 5
      i64.const 9
      i64.le_u
      br_if 0 (;@1;)
      local.get 2
      i32.const 12
      i32.add
      local.get 3
      i32.const -2
      i32.add
      local.tee 3
      i32.add
      local.get 5
      i32.wrap_i64
      local.tee 0
      local.get 0
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 0
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.load16_u offset=1053335 align=1
      i32.store16 align=1
      local.get 0
      i64.extend_i32_u
      local.set 5
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i64.eqz
        br_if 0 (;@2;)
        local.get 5
        i64.eqz
        br_if 1 (;@1;)
      end
      local.get 2
      i32.const 12
      i32.add
      local.get 3
      i32.const -1
      i32.add
      local.tee 3
      i32.add
      local.get 5
      i32.wrap_i64
      i32.const 1
      i32.shl
      i32.load8_u offset=1053336
      i32.store8
    end
    local.get 1
    i32.const 1
    i32.const 1
    i32.const 0
    local.get 2
    i32.const 12
    i32.add
    local.get 3
    i32.add
    i32.const 20
    local.get 3
    i32.sub
    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral
    local.set 3
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 3)
  (func $_RNvXs1i_NtCs4NIexwV1FGX_4core3fmtReNtB6_7Display3fmtB8_ (type 0) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter3pad)
  (func $_RNvNtCs4NIexwV1FGX_4core3fmt5write (type 7) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          local.get 2
          i32.load8_u
          local.tee 5
          br_if 1 (;@2;)
          i32.const 0
          local.set 5
          br 2 (;@1;)
        end
        local.get 0
        local.get 2
        local.get 3
        i32.const 1
        i32.shr_u
        local.get 1
        i32.load offset=12
        call_indirect (type 3)
        local.set 5
        br 1 (;@1;)
      end
      local.get 1
      i32.load offset=12
      local.set 6
      i32.const 0
      local.set 7
      loop  ;; label = @2
        local.get 2
        i32.const 1
        i32.add
        local.set 8
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 5
                  i32.extend8_s
                  i32.const -1
                  i32.gt_s
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 255
                  i32.and
                  local.tee 9
                  i32.const 128
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 9
                  i32.const 192
                  i32.ne
                  br_if 3 (;@4;)
                  local.get 4
                  local.get 1
                  i32.store offset=4
                  local.get 4
                  local.get 0
                  i32.store
                  local.get 4
                  i64.const 1610612768
                  i64.store offset=8 align=4
                  local.get 3
                  local.get 7
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee 5
                  i32.load
                  local.get 4
                  local.get 5
                  i32.load offset=4
                  call_indirect (type 0)
                  i32.eqz
                  br_if 2 (;@5;)
                  i32.const 1
                  local.set 5
                  br 6 (;@1;)
                end
                block  ;; label = @7
                  local.get 0
                  local.get 8
                  local.get 5
                  i32.const 255
                  i32.and
                  local.tee 5
                  local.get 6
                  call_indirect (type 3)
                  br_if 0 (;@7;)
                  local.get 8
                  local.get 5
                  i32.add
                  local.set 2
                  br 4 (;@3;)
                end
                i32.const 1
                local.set 5
                br 5 (;@1;)
              end
              block  ;; label = @6
                local.get 0
                local.get 2
                i32.const 3
                i32.add
                local.tee 5
                local.get 2
                i32.load16_u offset=1 align=1
                local.tee 2
                local.get 6
                call_indirect (type 3)
                br_if 0 (;@6;)
                local.get 5
                local.get 2
                i32.add
                local.set 2
                br 3 (;@3;)
              end
              i32.const 1
              local.set 5
              br 4 (;@1;)
            end
            local.get 7
            i32.const 1
            i32.add
            local.set 7
            local.get 8
            local.set 2
            br 1 (;@3;)
          end
          i32.const 1610612768
          local.set 10
          block  ;; label = @4
            local.get 5
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 5
            i32.add
            local.set 8
            local.get 2
            i32.load offset=1 align=1
            local.set 10
          end
          i32.const 0
          local.set 9
          block  ;; label = @4
            block  ;; label = @5
              local.get 5
              i32.const 2
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.set 11
              local.get 8
              local.set 2
              br 1 (;@4;)
            end
            local.get 8
            i32.const 2
            i32.add
            local.set 2
            local.get 8
            i32.load16_u align=1
            local.set 11
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 5
              i32.const 4
              i32.and
              br_if 0 (;@5;)
              local.get 2
              local.set 8
              br 1 (;@4;)
            end
            local.get 2
            i32.const 2
            i32.add
            local.set 8
            local.get 2
            i32.load16_u align=1
            local.set 9
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 5
              i32.const 8
              i32.and
              br_if 0 (;@5;)
              local.get 8
              local.set 2
              br 1 (;@4;)
            end
            local.get 8
            i32.const 2
            i32.add
            local.set 2
            local.get 8
            i32.load16_u align=1
            local.set 7
          end
          block  ;; label = @4
            local.get 5
            i32.const 16
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            local.get 11
            i32.const 65535
            i32.and
            i32.const 3
            i32.shl
            i32.add
            i32.load16_u offset=4
            local.set 11
          end
          block  ;; label = @4
            local.get 5
            i32.const 32
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            local.get 9
            i32.const 65535
            i32.and
            i32.const 3
            i32.shl
            i32.add
            i32.load16_u offset=4
            local.set 9
          end
          local.get 4
          local.get 9
          i32.store16 offset=14
          local.get 4
          local.get 11
          i32.store16 offset=12
          local.get 4
          local.get 10
          i32.store offset=8
          local.get 4
          local.get 1
          i32.store offset=4
          local.get 4
          local.get 0
          i32.store
          block  ;; label = @4
            local.get 3
            local.get 7
            i32.const 3
            i32.shl
            i32.add
            local.tee 5
            i32.load
            local.get 4
            local.get 5
            i32.load offset=4
            call_indirect (type 0)
            i32.eqz
            br_if 0 (;@4;)
            i32.const 1
            local.set 5
            br 3 (;@1;)
          end
          local.get 7
          i32.const 1
          i32.add
          local.set 7
        end
        local.get 2
        i32.load8_u
        local.tee 5
        br_if 0 (;@2;)
      end
      i32.const 0
      local.set 5
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 5)
  (func $_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt (type 5) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    i32.store offset=16
    local.get 3
    local.get 0
    i32.store offset=12
    local.get 3
    i32.const 1
    i32.store16 offset=28
    local.get 3
    local.get 2
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 12
    i32.add
    i32.store offset=20
    local.get 3
    i32.const 20
    i32.add
    call $_RNvCs5fGHjzWXRav_7___rustc17rust_begin_unwind
    unreachable)
  (func $_RNvMNtNtCs4NIexwV1FGX_4core4char7methodsc16escape_debug_ext (type 5) (param i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      local.get 1
                                      br_table 6 (;@11;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 4 (;@13;) 3 (;@14;) 1 (;@16;) 1 (;@16;) 5 (;@12;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 8 (;@9;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 7 (;@10;) 0 (;@17;)
                                    end
                                    local.get 1
                                    i32.const 92
                                    i32.eq
                                    br_if 1 (;@15;)
                                  end
                                  local.get 1
                                  i32.const -32
                                  i32.add
                                  i32.const 95
                                  i32.lt_u
                                  br_if 12 (;@3;)
                                  local.get 1
                                  i32.const 2097150
                                  i32.and
                                  i32.const 65438
                                  i32.eq
                                  br_if 12 (;@3;)
                                  local.get 1
                                  i32.const 32
                                  i32.lt_u
                                  br_if 13 (;@2;)
                                  local.get 1
                                  i32.const -127
                                  i32.add
                                  i32.const 33
                                  i32.lt_u
                                  br_if 13 (;@2;)
                                  local.get 1
                                  i32.const -57344
                                  i32.add
                                  i32.const 6400
                                  i32.lt_u
                                  br_if 13 (;@2;)
                                  local.get 1
                                  i32.const -983040
                                  i32.add
                                  i32.const 65534
                                  i32.lt_u
                                  br_if 13 (;@2;)
                                  local.get 1
                                  i32.const 32
                                  i32.eq
                                  br_if 13 (;@2;)
                                  local.get 1
                                  i32.const -1048576
                                  i32.add
                                  i32.const 65534
                                  i32.lt_u
                                  br_if 13 (;@2;)
                                  local.get 1
                                  i32.const 133
                                  i32.lt_u
                                  br_if 11 (;@4;)
                                  local.get 1
                                  call $_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data11white_space6lookup
                                  br_if 13 (;@2;)
                                  block  ;; label = @16
                                    local.get 1
                                    i32.const 768
                                    i32.lt_u
                                    br_if 0 (;@16;)
                                    local.get 2
                                    i32.const 1
                                    i32.and
                                    br_if 8 (;@8;)
                                  end
                                  local.get 1
                                  i32.const 172
                                  i32.gt_u
                                  br_if 8 (;@7;)
                                  br 11 (;@4;)
                                end
                                local.get 0
                                i64.const 0
                                i64.store offset=2 align=2
                                local.get 0
                                i32.const 23644
                                i32.store16
                                br 9 (;@5;)
                              end
                              local.get 0
                              i64.const 0
                              i64.store offset=2 align=2
                              local.get 0
                              i32.const 28252
                              i32.store16
                              br 8 (;@5;)
                            end
                            local.get 0
                            i64.const 0
                            i64.store offset=2 align=2
                            local.get 0
                            i32.const 29788
                            i32.store16
                            br 7 (;@5;)
                          end
                          local.get 0
                          i64.const 0
                          i64.store offset=2 align=2
                          local.get 0
                          i32.const 29276
                          i32.store16
                          br 6 (;@5;)
                        end
                        local.get 0
                        i64.const 0
                        i64.store offset=2 align=2
                        local.get 0
                        i32.const 12380
                        i32.store16
                        br 5 (;@5;)
                      end
                      local.get 2
                      i32.const 256
                      i32.and
                      i32.eqz
                      br_if 6 (;@3;)
                      local.get 0
                      i64.const 0
                      i64.store offset=2 align=2
                      local.get 0
                      i32.const 10076
                      i32.store16
                      br 4 (;@5;)
                    end
                    local.get 2
                    i32.const 16777215
                    i32.and
                    i32.const 65536
                    i32.ge_u
                    br_if 2 (;@6;)
                    br 5 (;@3;)
                  end
                  local.get 1
                  call $_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data15grapheme_extend11lookup_slow
                  br_if 5 (;@2;)
                end
                local.get 1
                call $_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data28default_ignorable_code_point11lookup_slow
                br_if 4 (;@2;)
                local.get 1
                call $_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data2cf11lookup_slow
                i32.eqz
                br_if 2 (;@4;)
                br 4 (;@2;)
              end
              local.get 0
              i64.const 0
              i64.store offset=2 align=2
              local.get 0
              i32.const 8796
              i32.store16
            end
            i32.const 2
            local.set 1
            i32.const 0
            local.set 2
            br 3 (;@1;)
          end
          local.get 1
          call $_RNvMNtNtCs4NIexwV1FGX_4core4char7methodsc11is_assigned
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 0
        local.get 1
        i32.store
        i32.const 129
        local.set 1
        i32.const 128
        local.set 2
        br 1 (;@1;)
      end
      local.get 3
      i32.const 0
      i32.store8 offset=8
      local.get 3
      i32.const 0
      i32.store16 offset=6
      local.get 3
      local.get 1
      i32.const 20
      i32.shr_u
      i32.load8_u offset=1053215
      i32.store8 offset=9
      local.get 3
      local.get 1
      i32.const 4
      i32.shr_u
      i32.const 15
      i32.and
      i32.load8_u offset=1053215
      i32.store8 offset=13
      local.get 3
      local.get 1
      i32.const 8
      i32.shr_u
      i32.const 15
      i32.and
      i32.load8_u offset=1053215
      i32.store8 offset=12
      local.get 3
      local.get 1
      i32.const 12
      i32.shr_u
      i32.const 15
      i32.and
      i32.load8_u offset=1053215
      i32.store8 offset=11
      local.get 3
      local.get 1
      i32.const 16
      i32.shr_u
      i32.const 15
      i32.and
      i32.load8_u offset=1053215
      i32.store8 offset=10
      local.get 3
      i32.const 6
      i32.add
      local.get 1
      i32.const 1
      i32.or
      i32.clz
      i32.const 2
      i32.shr_u
      local.tee 2
      i32.add
      local.tee 4
      i32.const 123
      i32.store8
      local.get 4
      i32.const -1
      i32.add
      i32.const 117
      i32.store8
      local.get 3
      i32.const 6
      i32.add
      local.get 2
      i32.const -2
      i32.add
      local.tee 2
      i32.add
      i32.const 92
      i32.store8
      local.get 0
      local.get 3
      i64.load offset=6 align=2
      i64.store align=4
      local.get 3
      i32.const 125
      i32.store8 offset=15
      local.get 3
      local.get 1
      i32.const 15
      i32.and
      i32.load8_u offset=1053215
      i32.store8 offset=14
      local.get 0
      local.get 3
      i32.load16_u offset=14
      i32.store16 offset=8
      i32.const 10
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store8 offset=13
    local.get 0
    local.get 2
    i32.store8 offset=12
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_RNvNtNtCs4NIexwV1FGX_4core3str8converts9from_utf8 (type 5) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i64 i32)
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      local.get 2
      i32.const -7
      i32.add
      local.tee 3
      local.get 3
      local.get 2
      i32.gt_u
      select
      local.set 4
      local.get 1
      i32.const 3
      i32.add
      i32.const -4
      i32.and
      local.get 1
      i32.sub
      local.set 5
      i32.const 0
      local.set 3
      loop  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                local.get 3
                i32.add
                i32.load8_u
                local.tee 6
                i32.extend8_s
                local.tee 7
                i32.const 0
                i32.lt_s
                br_if 0 (;@6;)
                local.get 5
                local.get 3
                i32.sub
                i32.const 3
                i32.and
                br_if 1 (;@5;)
                local.get 3
                local.get 4
                i32.ge_u
                br_if 2 (;@4;)
                loop  ;; label = @7
                  local.get 1
                  local.get 3
                  i32.add
                  local.tee 6
                  i32.const 4
                  i32.add
                  i32.load
                  local.get 6
                  i32.load
                  i32.or
                  i32.const -2139062144
                  i32.and
                  br_if 3 (;@4;)
                  local.get 3
                  i32.const 8
                  i32.add
                  local.tee 3
                  local.get 4
                  i32.lt_u
                  br_if 0 (;@7;)
                  br 3 (;@4;)
                end
              end
              i64.const 1103806595072
              local.set 8
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 6
                                i32.load8_u offset=1053535
                                i32.const -2
                                i32.add
                                br_table 0 (;@14;) 1 (;@13;) 2 (;@12;) 7 (;@7;)
                              end
                              local.get 3
                              i32.const 1
                              i32.add
                              local.tee 6
                              local.get 2
                              i32.lt_u
                              br_if 2 (;@11;)
                              i64.const 0
                              local.set 8
                              br 6 (;@7;)
                            end
                            local.get 3
                            i32.const 1
                            i32.add
                            local.tee 9
                            local.get 2
                            i32.lt_u
                            br_if 2 (;@10;)
                            i64.const 0
                            local.set 8
                            br 5 (;@7;)
                          end
                          local.get 3
                          i32.const 1
                          i32.add
                          local.tee 9
                          local.get 2
                          i32.lt_u
                          br_if 2 (;@9;)
                          i64.const 0
                          local.set 8
                          br 4 (;@7;)
                        end
                        local.get 1
                        local.get 6
                        i32.add
                        i32.load8_s
                        i32.const -65
                        i32.gt_s
                        br_if 3 (;@7;)
                        br 4 (;@6;)
                      end
                      local.get 1
                      local.get 9
                      i32.add
                      i32.load8_s
                      local.set 9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 6
                            i32.const -224
                            i32.add
                            br_table 0 (;@12;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 1 (;@11;) 2 (;@10;)
                          end
                          local.get 9
                          i32.const -32
                          i32.and
                          i32.const -96
                          i32.eq
                          br_if 3 (;@8;)
                          br 4 (;@7;)
                        end
                        local.get 9
                        i32.const -97
                        i32.gt_s
                        br_if 3 (;@7;)
                        br 2 (;@8;)
                      end
                      block  ;; label = @10
                        local.get 7
                        i32.const 31
                        i32.add
                        i32.const 255
                        i32.and
                        i32.const 12
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 7
                        i32.const -2
                        i32.and
                        i32.const -18
                        i32.ne
                        br_if 3 (;@7;)
                        local.get 9
                        i32.const -64
                        i32.lt_s
                        br_if 2 (;@8;)
                        br 3 (;@7;)
                      end
                      local.get 9
                      i32.const -64
                      i32.lt_s
                      br_if 1 (;@8;)
                      br 2 (;@7;)
                    end
                    local.get 1
                    local.get 9
                    i32.add
                    i32.load8_s
                    local.set 9
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 6
                            i32.const -240
                            i32.add
                            br_table 1 (;@11;) 0 (;@12;) 0 (;@12;) 0 (;@12;) 2 (;@10;) 0 (;@12;)
                          end
                          local.get 7
                          i32.const 15
                          i32.add
                          i32.const 255
                          i32.and
                          i32.const 2
                          i32.gt_u
                          br_if 4 (;@7;)
                          local.get 9
                          i32.const -64
                          i32.lt_s
                          br_if 2 (;@9;)
                          br 4 (;@7;)
                        end
                        local.get 9
                        i32.const 112
                        i32.add
                        i32.const 255
                        i32.and
                        i32.const 48
                        i32.lt_u
                        br_if 1 (;@9;)
                        br 3 (;@7;)
                      end
                      local.get 9
                      i32.const -113
                      i32.gt_s
                      br_if 2 (;@7;)
                    end
                    block  ;; label = @9
                      local.get 3
                      i32.const 2
                      i32.add
                      local.tee 6
                      local.get 2
                      i32.lt_u
                      br_if 0 (;@9;)
                      i64.const 0
                      local.set 8
                      br 2 (;@7;)
                    end
                    block  ;; label = @9
                      local.get 1
                      local.get 6
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 0 (;@9;)
                      i64.const 2203318222848
                      local.set 8
                      br 2 (;@7;)
                    end
                    i64.const 0
                    local.set 8
                    local.get 3
                    i32.const 3
                    i32.add
                    local.tee 6
                    local.get 2
                    i32.ge_u
                    br_if 1 (;@7;)
                    local.get 1
                    local.get 6
                    i32.add
                    i32.load8_s
                    i32.const -64
                    i32.lt_s
                    br_if 2 (;@6;)
                    i64.const 3302829850624
                    local.set 8
                    br 1 (;@7;)
                  end
                  i64.const 0
                  local.set 8
                  local.get 3
                  i32.const 2
                  i32.add
                  local.tee 6
                  local.get 2
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 6
                  i32.add
                  i32.load8_s
                  i32.const -65
                  i32.le_s
                  br_if 1 (;@6;)
                  i64.const 2203318222848
                  local.set 8
                end
                local.get 0
                local.get 8
                local.get 3
                i64.extend_i32_u
                i64.or
                i64.store offset=4 align=4
                local.get 0
                i32.const 1
                i32.store
                return
              end
              local.get 6
              i32.const 1
              i32.add
              local.set 3
              br 2 (;@3;)
            end
            local.get 3
            i32.const 1
            i32.add
            local.set 3
            br 1 (;@3;)
          end
          local.get 3
          local.get 2
          i32.ge_u
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 1
            local.get 3
            i32.add
            i32.load8_s
            i32.const 0
            i32.lt_s
            br_if 1 (;@3;)
            local.get 2
            local.get 3
            i32.const 1
            i32.add
            local.tee 3
            i32.ne
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        local.get 3
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0
    local.get 2
    i32.store offset=8
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    i32.const 0
    i32.store)
  (func $_RNvMNtNtCs4NIexwV1FGX_4core4char7methodsc11is_assigned (type 4) (param i32) (result i32)
    (local i32)
    i32.const 1
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 888
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 262142
        i32.lt_u
        br_if 1 (;@1;)
        local.get 0
        i32.const 917505
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        i32.const -917536
        i32.add
        i32.const 96
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -917760
        i32.add
        i32.const 240
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -983040
        i32.add
        i32.const 65534
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -1048576
        i32.add
        i32.const 65534
        i32.lt_u
        local.set 1
      end
      local.get 1
      return
    end
    local.get 0
    call $_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data13cn_planes_0_311lookup_slow
    i32.const 1
    i32.xor)
  (func $_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data13cn_planes_0_311lookup_slow (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 1
    i32.const 0
    i32.const 27
    local.get 0
    i32.const 111356
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 13
    i32.add
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1053828
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 7
    i32.add
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1053828
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 3
    i32.add
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1053828
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 2
    i32.add
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1053828
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 1
    i32.add
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1053828
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    i32.const 2
    i32.shl
    i32.load offset=1053828
    i32.const 11
    i32.shl
    local.tee 3
    local.get 0
    i32.const 11
    i32.shl
    local.tee 4
    i32.eq
    local.get 3
    local.get 4
    i32.lt_u
    i32.add
    local.get 2
    i32.add
    local.tee 3
    i32.const 2
    i32.shl
    local.tee 2
    i32.const 1053828
    i32.add
    local.set 5
    local.get 2
    i32.load offset=1053828
    i32.const 21
    i32.shr_u
    local.set 2
    i32.const 1461
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.const 52
        i32.gt_u
        br_if 0 (;@2;)
        local.get 5
        i32.load offset=4
        i32.const 21
        i32.shr_u
        local.set 4
        local.get 3
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 5
      i32.const -4
      i32.add
      i32.load
      i32.const 2097151
      i32.and
      local.set 1
    end
    block  ;; label = @1
      local.get 4
      local.get 2
      i32.const -1
      i32.xor
      i32.add
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.sub
      local.set 3
      local.get 4
      i32.const -1
      i32.add
      local.set 4
      i32.const 0
      local.set 0
      loop  ;; label = @2
        local.get 0
        local.get 2
        i32.const 1050909
        i32.add
        i32.load8_u
        i32.add
        local.tee 0
        local.get 3
        i32.gt_u
        br_if 1 (;@1;)
        local.get 4
        local.get 2
        i32.const 1
        i32.add
        local.tee 2
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 2
    i32.const 1
    i32.and)
  (func $_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data11white_space6lookup (type 4) (param i32) (result i32)
    (local i32 i32)
    i32.const 0
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.const 8
              i32.shr_u
              local.tee 2
              i32.const -22
              i32.add
              br_table 0 (;@5;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 3 (;@2;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 4 (;@1;) 1 (;@4;) 2 (;@3;)
            end
            local.get 0
            i32.const 5760
            i32.eq
            local.set 1
            br 3 (;@1;)
          end
          local.get 0
          i32.const 12288
          i32.eq
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        br_if 1 (;@1;)
        local.get 0
        i32.const 255
        i32.and
        i32.load8_u offset=1050653
        local.set 1
        br 1 (;@1;)
      end
      local.get 0
      i32.const 255
      i32.and
      i32.load8_u offset=1050653
      i32.const 2
      i32.and
      i32.const 1
      i32.shr_u
      local.set 1
    end
    local.get 1
    i32.const 1
    i32.and)
  (func $_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data15grapheme_extend11lookup_slow (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 1
    i32.const 0
    i32.const 16
    local.get 0
    i32.const 69291
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 8
    i32.or
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1054044
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 4
    i32.or
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1054044
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 2
    i32.or
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1054044
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 1
    i32.add
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1054044
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 1
    i32.add
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1054044
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    i32.const 2
    i32.shl
    i32.load offset=1054044
    i32.const 11
    i32.shl
    local.tee 3
    local.get 0
    i32.const 11
    i32.shl
    local.tee 4
    i32.eq
    local.get 3
    local.get 4
    i32.lt_u
    i32.add
    local.get 2
    i32.add
    local.tee 3
    i32.const 2
    i32.shl
    local.tee 2
    i32.const 1054044
    i32.add
    local.set 5
    local.get 2
    i32.load offset=1054044
    i32.const 21
    i32.shr_u
    local.set 2
    i32.const 767
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.const 31
        i32.gt_u
        br_if 0 (;@2;)
        local.get 5
        i32.load offset=4
        i32.const 21
        i32.shr_u
        local.set 4
        local.get 3
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 5
      i32.const -4
      i32.add
      i32.load
      i32.const 2097151
      i32.and
      local.set 1
    end
    block  ;; label = @1
      local.get 4
      local.get 2
      i32.const -1
      i32.xor
      i32.add
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.sub
      local.set 3
      local.get 4
      i32.const -1
      i32.add
      local.set 4
      i32.const 0
      local.set 0
      loop  ;; label = @2
        local.get 0
        local.get 2
        i32.const 1052370
        i32.add
        i32.load8_u
        i32.add
        local.tee 0
        local.get 3
        i32.gt_u
        br_if 1 (;@1;)
        local.get 4
        local.get 2
        i32.const 1
        i32.add
        local.tee 2
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 2
    i32.const 1
    i32.and)
  (func $_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data28default_ignorable_code_point11lookup_slow (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 1
    i32.const 0
    i32.const 6
    local.get 0
    i32.const 65024
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 3
    i32.add
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1054176
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 1
    i32.add
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1054176
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 1
    i32.add
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1054176
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    i32.const 2
    i32.shl
    i32.load offset=1054176
    i32.const 11
    i32.shl
    local.tee 3
    local.get 0
    i32.const 11
    i32.shl
    local.tee 4
    i32.eq
    local.get 3
    local.get 4
    i32.lt_u
    i32.add
    local.get 2
    i32.add
    local.tee 3
    i32.const 2
    i32.shl
    local.tee 2
    i32.const 1054176
    i32.add
    local.set 5
    local.get 2
    i32.load offset=1054176
    i32.const 21
    i32.shr_u
    local.set 2
    i32.const 35
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.const 10
        i32.gt_u
        br_if 0 (;@2;)
        local.get 5
        i32.load offset=4
        i32.const 21
        i32.shr_u
        local.set 4
        local.get 3
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 5
      i32.const -4
      i32.add
      i32.load
      i32.const 2097151
      i32.and
      local.set 1
    end
    block  ;; label = @1
      local.get 4
      local.get 2
      i32.const -1
      i32.xor
      i32.add
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.sub
      local.set 3
      local.get 4
      i32.const -1
      i32.add
      local.set 4
      i32.const 0
      local.set 0
      loop  ;; label = @2
        local.get 0
        local.get 2
        i32.const 1053137
        i32.add
        i32.load8_u
        i32.add
        local.tee 0
        local.get 3
        i32.gt_u
        br_if 1 (;@1;)
        local.get 4
        local.get 2
        i32.const 1
        i32.add
        local.tee 2
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 2
    i32.const 1
    i32.and)
  (func $_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data2cf11lookup_slow (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 1
    i32.const 0
    i32.const 5
    local.get 0
    i32.const 69821
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 3
    i32.add
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1054224
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 1
    i32.add
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1054224
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    local.get 2
    i32.const 1
    i32.add
    local.tee 2
    local.get 0
    local.get 2
    i32.const 2
    i32.shl
    i32.load offset=1054224
    i32.const 2097151
    i32.and
    i32.lt_u
    select
    local.tee 2
    i32.const 2
    i32.shl
    i32.load offset=1054224
    i32.const 11
    i32.shl
    local.tee 3
    local.get 0
    i32.const 11
    i32.shl
    local.tee 4
    i32.eq
    local.get 3
    local.get 4
    i32.lt_u
    i32.add
    local.get 2
    i32.add
    local.tee 3
    i32.const 2
    i32.shl
    local.tee 2
    i32.const 1054224
    i32.add
    local.set 5
    local.get 2
    i32.load offset=1054224
    i32.const 21
    i32.shr_u
    local.set 2
    i32.const 43
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.const 9
        i32.gt_u
        br_if 0 (;@2;)
        local.get 5
        i32.load offset=4
        i32.const 21
        i32.shr_u
        local.set 4
        local.get 3
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 5
      i32.const -4
      i32.add
      i32.load
      i32.const 2097151
      i32.and
      local.set 1
    end
    block  ;; label = @1
      local.get 4
      local.get 2
      i32.const -1
      i32.xor
      i32.add
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.sub
      local.set 3
      local.get 4
      i32.const -1
      i32.add
      local.set 4
      i32.const 0
      local.set 0
      loop  ;; label = @2
        local.get 0
        local.get 2
        i32.const 1053172
        i32.add
        i32.load8_u
        i32.add
        local.tee 0
        local.get 3
        i32.gt_u
        br_if 1 (;@1;)
        local.get 4
        local.get 2
        i32.const 1
        i32.add
        local.tee 2
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 2
    i32.const 1
    i32.and)
  (func $_RNvNtCs4NIexwV1FGX_4core9panicking18panic_bounds_check (type 5) (param i32 i32 i32)
    (local i32 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    i32.store offset=12
    local.get 3
    local.get 0
    i32.store offset=8
    local.get 3
    i32.const 2
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.tee 4
    local.get 3
    i32.const 8
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=24
    local.get 3
    local.get 4
    local.get 3
    i32.const 12
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=16
    i32.const 1049039
    local.get 3
    i32.const 16
    i32.add
    local.get 2
    call $_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt
    unreachable)
  (func $_RNvXs0_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write9write_str (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load offset=4
    local.set 3
    local.get 0
    i32.load
    local.set 4
    local.get 0
    i32.load offset=8
    local.set 5
    i32.const 0
    local.set 6
    i32.const 0
    local.set 7
    i32.const 0
    local.set 8
    i32.const 0
    local.set 9
    block  ;; label = @1
      loop  ;; label = @2
        local.get 9
        i32.const 1
        i32.and
        br_if 1 (;@1;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            local.get 8
            i32.lt_u
            br_if 0 (;@4;)
            loop  ;; label = @5
              local.get 1
              local.get 8
              i32.add
              local.set 9
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 2
                          local.get 8
                          i32.sub
                          local.tee 10
                          i32.const 7
                          i32.gt_u
                          br_if 0 (;@11;)
                          local.get 2
                          local.get 8
                          i32.ne
                          br_if 1 (;@10;)
                          local.get 2
                          local.set 8
                          br 7 (;@4;)
                        end
                        local.get 9
                        i32.const 3
                        i32.add
                        i32.const -4
                        i32.and
                        local.tee 0
                        local.get 9
                        i32.eq
                        br_if 1 (;@9;)
                        local.get 0
                        local.get 9
                        i32.sub
                        local.set 0
                        i32.const 0
                        local.set 11
                        loop  ;; label = @11
                          local.get 9
                          local.get 11
                          i32.add
                          i32.load8_u
                          i32.const 10
                          i32.eq
                          br_if 5 (;@6;)
                          local.get 0
                          local.get 11
                          i32.const 1
                          i32.add
                          local.tee 11
                          i32.ne
                          br_if 0 (;@11;)
                        end
                        local.get 0
                        local.get 10
                        i32.const -8
                        i32.add
                        local.tee 12
                        i32.gt_u
                        br_if 3 (;@7;)
                        br 2 (;@8;)
                      end
                      i32.const 0
                      local.set 11
                      loop  ;; label = @10
                        local.get 9
                        local.get 11
                        i32.add
                        i32.load8_u
                        i32.const 10
                        i32.eq
                        br_if 4 (;@6;)
                        local.get 10
                        local.get 11
                        i32.const 1
                        i32.add
                        local.tee 11
                        i32.ne
                        br_if 0 (;@10;)
                      end
                      local.get 2
                      local.set 8
                      br 5 (;@4;)
                    end
                    local.get 10
                    i32.const -8
                    i32.add
                    local.set 12
                    i32.const 0
                    local.set 0
                  end
                  loop  ;; label = @8
                    i32.const 16843008
                    local.get 9
                    local.get 0
                    i32.add
                    local.tee 11
                    i32.load
                    local.tee 13
                    i32.const 168430090
                    i32.xor
                    i32.sub
                    local.get 13
                    i32.or
                    i32.const 16843008
                    local.get 11
                    i32.const 4
                    i32.add
                    i32.load
                    local.tee 11
                    i32.const 168430090
                    i32.xor
                    i32.sub
                    local.get 11
                    i32.or
                    i32.and
                    i32.const -2139062144
                    i32.and
                    i32.const -2139062144
                    i32.ne
                    br_if 1 (;@7;)
                    local.get 0
                    i32.const 8
                    i32.add
                    local.tee 0
                    local.get 12
                    i32.le_u
                    br_if 0 (;@8;)
                  end
                end
                block  ;; label = @7
                  local.get 10
                  local.get 0
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 2
                  local.set 8
                  br 3 (;@4;)
                end
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 9
                    local.get 0
                    i32.add
                    i32.load8_u
                    i32.const 10
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 0
                    local.set 11
                    br 2 (;@6;)
                  end
                  local.get 10
                  local.get 0
                  i32.const 1
                  i32.add
                  local.tee 0
                  i32.ne
                  br_if 0 (;@7;)
                end
                local.get 2
                local.set 8
                br 2 (;@4;)
              end
              local.get 11
              local.get 8
              i32.add
              local.tee 0
              i32.const 1
              i32.add
              local.set 8
              block  ;; label = @6
                local.get 0
                local.get 2
                i32.ge_u
                br_if 0 (;@6;)
                local.get 9
                local.get 11
                i32.add
                i32.load8_u
                i32.const 10
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                local.set 9
                local.get 8
                local.set 10
                local.get 8
                local.set 0
                br 3 (;@3;)
              end
              local.get 8
              local.get 2
              i32.le_u
              br_if 0 (;@5;)
            end
          end
          local.get 2
          local.get 7
          i32.eq
          br_if 2 (;@1;)
          i32.const 1
          local.set 9
          local.get 7
          local.set 10
          local.get 2
          local.set 0
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 5
            i32.load8_u
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            i32.const 1054270
            i32.const 4
            local.get 3
            i32.load offset=12
            call_indirect (type 3)
            br_if 1 (;@3;)
          end
          local.get 0
          local.get 7
          i32.sub
          local.set 13
          i32.const 0
          local.set 11
          block  ;; label = @4
            local.get 0
            local.get 7
            i32.eq
            br_if 0 (;@4;)
            local.get 1
            local.get 0
            i32.add
            i32.const -1
            i32.add
            i32.load8_u
            i32.const 10
            i32.eq
            local.set 11
          end
          local.get 1
          local.get 7
          i32.add
          local.set 0
          local.get 5
          local.get 11
          i32.store8
          local.get 10
          local.set 7
          local.get 4
          local.get 0
          local.get 13
          local.get 3
          i32.load offset=12
          call_indirect (type 3)
          i32.eqz
          br_if 1 (;@2;)
        end
      end
      i32.const 1
      local.set 6
    end
    local.get 6)
  (func $_RNvMs2_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_11DebugStruct5field (type 12) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    i32.const 1
    local.set 6
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      local.get 0
      i32.load8_u offset=5
      local.set 7
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 8
        i32.load8_u offset=10
        i32.const 128
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 6
        local.get 8
        i32.load
        i32.const 1053235
        i32.const 1053232
        local.get 7
        i32.const 1
        i32.and
        local.tee 7
        select
        i32.const 2
        i32.const 3
        local.get 7
        select
        local.get 8
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 3)
        br_if 1 (;@1;)
        local.get 8
        i32.load
        local.get 1
        local.get 2
        local.get 8
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 3)
        br_if 1 (;@1;)
        local.get 8
        i32.load
        i32.const 1053237
        i32.const 2
        local.get 8
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 3)
        br_if 1 (;@1;)
        local.get 3
        local.get 8
        local.get 4
        i32.load offset=12
        call_indirect (type 0)
        local.set 6
        br 1 (;@1;)
      end
      i32.const 1
      local.set 6
      block  ;; label = @2
        local.get 7
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 8
        i32.load
        i32.const 1053239
        i32.const 3
        local.get 8
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 3)
        br_if 1 (;@1;)
      end
      i32.const 1
      local.set 6
      local.get 5
      i32.const 1
      i32.store8 offset=15
      local.get 5
      i32.const 1053252
      i32.store offset=20
      local.get 5
      local.get 8
      i64.load align=4
      i64.store align=4
      local.get 5
      local.get 8
      i64.load offset=8 align=4
      i64.store offset=24 align=4
      local.get 5
      local.get 5
      i32.const 15
      i32.add
      i32.store offset=8
      local.get 5
      local.get 5
      i32.store offset=16
      local.get 5
      local.get 1
      local.get 2
      call $_RNvXs0_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write9write_str
      br_if 0 (;@1;)
      local.get 5
      i32.const 1053237
      i32.const 2
      call $_RNvXs0_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write9write_str
      br_if 0 (;@1;)
      local.get 3
      local.get 5
      i32.const 16
      i32.add
      local.get 4
      i32.load offset=12
      call_indirect (type 0)
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=16
      i32.const 1053242
      i32.const 2
      local.get 5
      i32.load offset=20
      i32.load offset=12
      call_indirect (type 3)
      local.set 6
    end
    local.get 0
    i32.const 1
    i32.store8 offset=5
    local.get 0
    local.get 6
    i32.store8 offset=4
    local.get 5
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral (type 13) (param i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i64)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        local.get 5
        i32.const 1
        i32.add
        local.set 6
        local.get 0
        i32.load offset=8
        local.set 7
        i32.const 45
        local.set 8
        br 1 (;@1;)
      end
      i32.const 43
      i32.const -1
      local.get 0
      i32.load offset=8
      local.tee 7
      i32.const 2097152
      i32.and
      local.tee 1
      select
      local.set 8
      local.get 1
      i32.const 21
      i32.shr_u
      local.get 5
      i32.add
      local.set 6
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 7
        i32.const 8388608
        i32.and
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.const 16
          i32.lt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          call $_RNvNtNtCs4NIexwV1FGX_4core3str5count14do_count_chars
          local.set 1
          br 1 (;@2;)
        end
        block  ;; label = @3
          local.get 3
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          br 1 (;@2;)
        end
        local.get 3
        i32.const 3
        i32.and
        local.set 9
        i32.const 0
        local.set 10
        i32.const 0
        local.set 1
        block  ;; label = @3
          local.get 3
          i32.const 4
          i32.lt_u
          br_if 0 (;@3;)
          local.get 3
          i32.const 12
          i32.and
          local.set 11
          i32.const 0
          local.set 10
          i32.const 0
          local.set 1
          loop  ;; label = @4
            local.get 1
            local.get 2
            local.get 10
            i32.add
            local.tee 12
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 12
            i32.const 1
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 12
            i32.const 2
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 12
            i32.const 3
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 1
            local.get 11
            local.get 10
            i32.const 4
            i32.add
            local.tee 10
            i32.ne
            br_if 0 (;@4;)
          end
          local.get 9
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 2
        local.get 10
        i32.add
        local.set 12
        loop  ;; label = @3
          local.get 1
          local.get 12
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 1
          local.get 12
          i32.const 1
          i32.add
          local.set 12
          local.get 9
          i32.const -1
          i32.add
          local.tee 9
          br_if 0 (;@3;)
        end
      end
      local.get 1
      local.get 6
      i32.add
      local.set 6
    end
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 6
                        local.get 0
                        i32.load16_u offset=12
                        local.tee 12
                        i32.ge_u
                        br_if 0 (;@10;)
                        local.get 7
                        i32.const 16777216
                        i32.and
                        br_if 4 (;@6;)
                        local.get 12
                        local.get 6
                        i32.sub
                        local.set 13
                        i32.const 0
                        local.set 6
                        local.get 7
                        i32.const 29
                        i32.shr_u
                        i32.const 3
                        i32.and
                        br_table 3 (;@7;) 1 (;@9;) 2 (;@8;) 1 (;@9;) 3 (;@7;)
                      end
                      local.get 0
                      i32.load
                      local.tee 1
                      local.get 0
                      i32.load offset=4
                      local.tee 12
                      local.get 8
                      local.get 2
                      local.get 3
                      call $_RNvNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB7_9Formatter12pad_integral12write_prefix
                      i32.eqz
                      br_if 7 (;@2;)
                      i32.const 1
                      return
                    end
                    local.get 13
                    local.set 6
                    br 1 (;@7;)
                  end
                  local.get 13
                  i32.const 65534
                  i32.and
                  i32.const 1
                  i32.shr_u
                  local.set 6
                end
                local.get 7
                i32.const 2097151
                i32.and
                local.set 9
                local.get 0
                i32.load offset=4
                local.set 11
                local.get 0
                i32.load
                local.set 12
                local.get 6
                i32.const 65535
                i32.and
                local.tee 10
                i32.eqz
                br_if 1 (;@5;)
                local.get 11
                i32.load offset=16
                local.set 0
                i32.const 0
                local.set 1
                i32.const 1
                local.set 7
                loop  ;; label = @7
                  local.get 12
                  local.get 9
                  local.get 0
                  call_indirect (type 0)
                  br_if 6 (;@1;)
                  local.get 1
                  i32.const 1
                  i32.add
                  local.tee 1
                  i32.const 65535
                  i32.and
                  local.get 10
                  i32.ge_u
                  br_if 2 (;@5;)
                  br 0 (;@7;)
                end
              end
              local.get 0
              local.get 0
              i64.load offset=8 align=4
              local.tee 14
              i32.wrap_i64
              i32.const -1612709888
              i32.and
              i32.const 536870960
              i32.or
              i32.store offset=8
              i32.const 1
              local.set 7
              local.get 0
              i32.load
              local.tee 9
              local.get 0
              i32.load offset=4
              local.tee 11
              local.get 8
              local.get 2
              local.get 3
              call $_RNvNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB7_9Formatter12pad_integral12write_prefix
              br_if 4 (;@1;)
              local.get 12
              local.get 6
              i32.const 65535
              i32.and
              i32.eq
              br_if 2 (;@3;)
              local.get 11
              i32.load offset=16
              local.set 10
              i32.const 0
              local.set 1
              local.get 12
              local.get 6
              i32.sub
              i32.const 65535
              i32.and
              local.set 12
              br 1 (;@4;)
            end
            i32.const 1
            local.set 7
            local.get 12
            local.get 11
            local.get 8
            local.get 2
            local.get 3
            call $_RNvNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB7_9Formatter12pad_integral12write_prefix
            br_if 3 (;@1;)
            local.get 12
            local.get 4
            local.get 5
            local.get 11
            i32.load offset=12
            call_indirect (type 3)
            br_if 3 (;@1;)
            block  ;; label = @5
              local.get 13
              i32.const 65535
              i32.and
              local.get 6
              i32.const 65535
              i32.and
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              return
            end
            local.get 11
            i32.load offset=16
            local.set 10
            i32.const 0
            local.set 1
            local.get 13
            local.get 6
            i32.sub
            i32.const 65535
            i32.and
            local.set 0
            loop  ;; label = @5
              local.get 12
              local.get 9
              local.get 10
              call_indirect (type 0)
              local.tee 7
              br_if 4 (;@1;)
              local.get 1
              i32.const 1
              i32.add
              local.tee 1
              i32.const 65535
              i32.and
              local.get 0
              i32.lt_u
              br_if 0 (;@5;)
              br 4 (;@1;)
            end
          end
          block  ;; label = @4
            loop  ;; label = @5
              local.get 9
              i32.const 48
              local.get 10
              call_indirect (type 0)
              br_if 1 (;@4;)
              local.get 1
              i32.const 1
              i32.add
              local.tee 1
              i32.const 65535
              i32.and
              local.get 12
              i32.ge_u
              br_if 2 (;@3;)
              br 0 (;@5;)
            end
          end
          i32.const 1
          return
        end
        local.get 9
        local.get 4
        local.get 5
        local.get 11
        i32.load offset=12
        call_indirect (type 3)
        br_if 1 (;@1;)
        local.get 0
        local.get 14
        i64.store offset=8 align=4
        i32.const 0
        return
      end
      local.get 1
      local.get 4
      local.get 5
      local.get 12
      i32.load offset=12
      call_indirect (type 3)
      local.set 7
    end
    local.get 7)
  (func $_RNvNtNtCs4NIexwV1FGX_4core3str5count14do_count_chars (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 0
        i32.const 3
        i32.add
        i32.const -4
        i32.and
        local.tee 2
        local.get 0
        i32.sub
        local.tee 3
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        local.get 3
        i32.sub
        local.tee 4
        i32.const 2
        i32.shr_u
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.const 3
        i32.and
        local.set 6
        i32.const 0
        local.set 7
        i32.const 0
        local.set 1
        block  ;; label = @3
          local.get 2
          local.get 0
          i32.eq
          br_if 0 (;@3;)
          i32.const 0
          local.set 8
          i32.const 0
          local.set 1
          block  ;; label = @4
            local.get 0
            local.get 2
            i32.sub
            local.tee 9
            i32.const -4
            i32.gt_u
            br_if 0 (;@4;)
            i32.const 0
            local.set 8
            i32.const 0
            local.set 1
            loop  ;; label = @5
              local.get 1
              local.get 0
              local.get 8
              i32.add
              local.tee 2
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 2
              i32.const 1
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 2
              i32.const 2
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 2
              i32.const 3
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set 1
              local.get 8
              i32.const 4
              i32.add
              local.tee 8
              br_if 0 (;@5;)
            end
          end
          local.get 0
          local.get 8
          i32.add
          local.set 2
          loop  ;; label = @4
            local.get 1
            local.get 2
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 1
            local.get 2
            i32.const 1
            i32.add
            local.set 2
            local.get 9
            i32.const 1
            i32.add
            local.tee 9
            br_if 0 (;@4;)
          end
        end
        local.get 0
        local.get 3
        i32.add
        local.set 9
        block  ;; label = @3
          local.get 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 9
          local.get 4
          i32.const 2147483644
          i32.and
          i32.add
          local.tee 2
          i32.load8_s
          i32.const -65
          i32.gt_s
          local.set 7
          local.get 6
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 7
          local.get 2
          i32.load8_s offset=1
          i32.const -65
          i32.gt_s
          i32.add
          local.set 7
          local.get 6
          i32.const 2
          i32.eq
          br_if 0 (;@3;)
          local.get 7
          local.get 2
          i32.load8_s offset=2
          i32.const -65
          i32.gt_s
          i32.add
          local.set 7
        end
        local.get 7
        local.get 1
        i32.add
        local.set 8
        loop  ;; label = @3
          local.get 9
          local.set 3
          local.get 5
          i32.eqz
          br_if 2 (;@1;)
          local.get 5
          i32.const 192
          local.get 5
          i32.const 192
          i32.lt_u
          select
          local.tee 7
          i32.const 3
          i32.and
          local.set 6
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              i32.const 2
              i32.shl
              local.tee 4
              i32.const 1008
              i32.and
              local.tee 1
              br_if 0 (;@5;)
              i32.const 0
              local.set 2
              br 1 (;@4;)
            end
            local.get 3
            local.get 1
            i32.add
            local.set 0
            i32.const 0
            local.set 2
            local.get 3
            local.set 1
            loop  ;; label = @5
              local.get 1
              i32.const 12
              i32.add
              i32.load
              local.tee 9
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 9
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 1
              i32.const 8
              i32.add
              i32.load
              local.tee 9
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 9
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 1
              i32.const 4
              i32.add
              i32.load
              local.tee 9
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 9
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 1
              i32.load
              local.tee 9
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 9
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 2
              i32.add
              i32.add
              i32.add
              i32.add
              local.set 2
              local.get 1
              i32.const 16
              i32.add
              local.tee 1
              local.get 0
              i32.ne
              br_if 0 (;@5;)
            end
          end
          local.get 5
          local.get 7
          i32.sub
          local.set 5
          local.get 3
          local.get 4
          i32.add
          local.set 9
          local.get 2
          i32.const 8
          i32.shr_u
          i32.const 16711935
          i32.and
          local.get 2
          i32.const 16711935
          i32.and
          i32.add
          i32.const 65537
          i32.mul
          i32.const 16
          i32.shr_u
          local.get 8
          i32.add
          local.set 8
          local.get 6
          i32.eqz
          br_if 0 (;@3;)
        end
        local.get 3
        local.get 7
        i32.const 252
        i32.and
        i32.const 2
        i32.shl
        i32.add
        local.tee 2
        i32.load
        local.tee 1
        i32.const -1
        i32.xor
        i32.const 7
        i32.shr_u
        local.get 1
        i32.const 6
        i32.shr_u
        i32.or
        i32.const 16843009
        i32.and
        local.set 1
        block  ;; label = @3
          local.get 6
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=4
          local.tee 9
          i32.const -1
          i32.xor
          i32.const 7
          i32.shr_u
          local.get 9
          i32.const 6
          i32.shr_u
          i32.or
          i32.const 16843009
          i32.and
          local.get 1
          i32.add
          local.set 1
          local.get 6
          i32.const 2
          i32.eq
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=8
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 7
          i32.shr_u
          local.get 2
          i32.const 6
          i32.shr_u
          i32.or
          i32.const 16843009
          i32.and
          local.get 1
          i32.add
          local.set 1
        end
        local.get 1
        i32.const 8
        i32.shr_u
        i32.const 459007
        i32.and
        local.get 1
        i32.const 16711935
        i32.and
        i32.add
        i32.const 65537
        i32.mul
        i32.const 16
        i32.shr_u
        local.get 8
        i32.add
        local.set 8
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      local.get 1
      i32.const 3
      i32.and
      local.set 2
      i32.const 0
      local.set 9
      i32.const 0
      local.set 8
      block  ;; label = @2
        local.get 1
        i32.const 4
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const -4
        i32.and
        local.set 5
        i32.const 0
        local.set 8
        i32.const 0
        local.set 9
        loop  ;; label = @3
          local.get 8
          local.get 0
          local.get 9
          i32.add
          local.tee 1
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 1
          i32.const 1
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 1
          i32.const 2
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 1
          i32.const 3
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 8
          local.get 5
          local.get 9
          i32.const 4
          i32.add
          local.tee 9
          i32.ne
          br_if 0 (;@3;)
        end
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 0
      local.get 9
      i32.add
      local.set 1
      loop  ;; label = @2
        local.get 8
        local.get 1
        i32.load8_s
        i32.const -65
        i32.gt_s
        i32.add
        local.set 8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    local.get 8)
  (func $_RNvNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB7_9Formatter12pad_integral12write_prefix (type 12) (param i32 i32 i32 i32 i32) (result i32)
    block  ;; label = @1
      local.get 2
      i32.const -1
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 1
      i32.load offset=16
      call_indirect (type 0)
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1
      return
    end
    block  ;; label = @1
      local.get 3
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    local.get 0
    local.get 3
    local.get 4
    local.get 1
    i32.load offset=12
    call_indirect (type 3))
  (func $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish (type 12) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    i32.const 1
    local.set 6
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 7
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=4
      local.tee 8
      i32.load offset=12
      local.tee 9
      call_indirect (type 3)
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load8_u offset=10
          i32.const 128
          i32.and
          br_if 0 (;@3;)
          i32.const 1
          local.set 6
          local.get 7
          i32.const 1053245
          i32.const 1
          local.get 9
          call_indirect (type 3)
          br_if 2 (;@1;)
          local.get 3
          local.get 0
          local.get 4
          i32.load offset=12
          call_indirect (type 0)
          i32.eqz
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        local.get 7
        i32.const 1053246
        i32.const 2
        local.get 9
        call_indirect (type 3)
        br_if 1 (;@1;)
        i32.const 1
        local.set 6
        local.get 5
        i32.const 1
        i32.store8 offset=15
        local.get 5
        local.get 8
        i32.store offset=4
        local.get 5
        local.get 7
        i32.store
        local.get 5
        i32.const 1053252
        i32.store offset=20
        local.get 5
        local.get 0
        i64.load offset=8 align=4
        i64.store offset=24 align=4
        local.get 5
        local.get 5
        i32.const 15
        i32.add
        i32.store offset=8
        local.get 5
        local.get 5
        i32.store offset=16
        local.get 3
        local.get 5
        i32.const 16
        i32.add
        local.get 4
        i32.load offset=12
        call_indirect (type 0)
        br_if 1 (;@1;)
        local.get 5
        i32.load offset=16
        i32.const 1053242
        i32.const 2
        local.get 5
        i32.load offset=20
        i32.load offset=12
        call_indirect (type 3)
        br_if 1 (;@1;)
      end
      block  ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        local.get 0
        i32.load8_u offset=10
        i32.const 128
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 6
        local.get 0
        i32.load
        i32.const 1053248
        i32.const 1
        local.get 0
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 3)
        br_if 1 (;@1;)
      end
      local.get 0
      i32.load
      i32.const 1053244
      i32.const 1
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 3)
      local.set 6
    end
    local.get 5
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 6)
  (func $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter26debug_struct_field2_finish (type 14) (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 11
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.get 1
    local.get 2
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 3)
    local.set 2
    local.get 11
    i32.const 0
    i32.store8 offset=13
    local.get 11
    local.get 2
    i32.store8 offset=12
    local.get 11
    local.get 0
    i32.store offset=8
    local.get 11
    i32.const 8
    i32.add
    local.get 3
    local.get 4
    local.get 5
    local.get 6
    call $_RNvMs2_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_11DebugStruct5field
    local.get 7
    local.get 8
    local.get 9
    local.get 10
    call $_RNvMs2_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_11DebugStruct5field
    local.set 10
    local.get 11
    i32.load8_u offset=13
    local.tee 2
    local.get 11
    i32.load8_u offset=12
    local.tee 1
    i32.or
    local.set 0
    block  ;; label = @1
      local.get 2
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 10
        i32.load
        local.tee 0
        i32.load8_u offset=10
        i32.const 128
        i32.and
        br_if 0 (;@2;)
        local.get 0
        i32.load
        i32.const 1053249
        i32.const 2
        local.get 0
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 3)
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      i32.load
      i32.const 1053231
      i32.const 1
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 3)
      local.set 0
    end
    local.get 11
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
    i32.const 1
    i32.and)
  (func $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter3pad (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.const 402653184
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 3
                  i32.const 268435456
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load16_u offset=14
                  local.tee 4
                  br_if 1 (;@6;)
                  i32.const 0
                  local.set 2
                  br 2 (;@5;)
                end
                block  ;; label = @7
                  local.get 2
                  i32.const 16
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 2
                  call $_RNvNtNtCs4NIexwV1FGX_4core3str5count14do_count_chars
                  local.set 5
                  br 4 (;@3;)
                end
                block  ;; label = @7
                  local.get 2
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 5
                  br 4 (;@3;)
                end
                local.get 2
                i32.const 3
                i32.and
                local.set 6
                i32.const 0
                local.set 7
                i32.const 0
                local.set 5
                block  ;; label = @7
                  local.get 2
                  i32.const 4
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 12
                  i32.and
                  local.set 4
                  i32.const 0
                  local.set 5
                  i32.const 0
                  local.set 7
                  loop  ;; label = @8
                    local.get 5
                    local.get 1
                    local.get 7
                    i32.add
                    local.tee 8
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 8
                    i32.const 1
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 8
                    i32.const 2
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 8
                    i32.const 3
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.set 5
                    local.get 4
                    local.get 7
                    i32.const 4
                    i32.add
                    local.tee 7
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  local.get 6
                  i32.eqz
                  br_if 4 (;@3;)
                end
                local.get 1
                local.get 7
                i32.add
                local.set 8
                loop  ;; label = @7
                  local.get 5
                  local.get 8
                  i32.load8_s
                  i32.const -65
                  i32.gt_s
                  i32.add
                  local.set 5
                  local.get 8
                  i32.const 1
                  i32.add
                  local.set 8
                  local.get 6
                  i32.const -1
                  i32.add
                  local.tee 6
                  br_if 0 (;@7;)
                  br 4 (;@3;)
                end
              end
              local.get 1
              local.get 2
              i32.add
              local.set 7
              i32.const 0
              local.set 2
              local.get 1
              local.set 8
              local.get 4
              local.set 6
              loop  ;; label = @6
                local.get 8
                local.tee 5
                local.get 7
                i32.eq
                br_if 2 (;@4;)
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 5
                    i32.load8_s
                    local.tee 8
                    i32.const -1
                    i32.le_s
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 1
                    i32.add
                    local.set 8
                    br 1 (;@7;)
                  end
                  block  ;; label = @8
                    local.get 8
                    i32.const -32
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 2
                    i32.add
                    local.set 8
                    br 1 (;@7;)
                  end
                  local.get 5
                  i32.const 4
                  i32.const 3
                  local.get 8
                  i32.const -17
                  i32.gt_u
                  select
                  i32.add
                  local.set 8
                end
                local.get 8
                local.get 5
                i32.sub
                local.get 2
                i32.add
                local.set 2
                local.get 6
                i32.const -1
                i32.add
                local.tee 6
                br_if 0 (;@6;)
              end
            end
            i32.const 0
            local.set 6
          end
          local.get 4
          local.get 6
          i32.sub
          local.set 5
        end
        local.get 5
        local.get 0
        i32.load16_u offset=12
        local.tee 8
        i32.ge_u
        br_if 0 (;@2;)
        local.get 8
        local.get 5
        i32.sub
        local.set 9
        i32.const 0
        local.set 4
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 3
              i32.const 29
              i32.shr_u
              i32.const 3
              i32.and
              br_table 2 (;@3;) 0 (;@5;) 1 (;@4;) 2 (;@3;) 2 (;@3;)
            end
            local.get 9
            local.set 4
            br 1 (;@3;)
          end
          local.get 9
          i32.const 65534
          i32.and
          i32.const 1
          i32.shr_u
          local.set 4
        end
        local.get 3
        i32.const 2097151
        i32.and
        local.set 6
        local.get 0
        i32.load offset=4
        local.set 10
        local.get 0
        i32.load
        local.set 8
        block  ;; label = @3
          local.get 4
          i32.const 65535
          i32.and
          local.tee 7
          i32.eqz
          br_if 0 (;@3;)
          local.get 10
          i32.load offset=16
          local.set 3
          i32.const 0
          local.set 5
          i32.const 1
          local.set 0
          loop  ;; label = @4
            local.get 8
            local.get 6
            local.get 3
            call_indirect (type 0)
            br_if 3 (;@1;)
            local.get 5
            i32.const 1
            i32.add
            local.tee 5
            i32.const 65535
            i32.and
            local.get 7
            i32.lt_u
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 8
          local.get 1
          local.get 2
          local.get 10
          i32.load offset=12
          call_indirect (type 3)
          i32.eqz
          br_if 0 (;@3;)
          i32.const 1
          return
        end
        block  ;; label = @3
          local.get 9
          i32.const 65535
          i32.and
          local.get 4
          i32.const 65535
          i32.and
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          return
        end
        local.get 10
        i32.load offset=16
        local.set 2
        i32.const 0
        local.set 5
        local.get 9
        local.get 4
        i32.sub
        i32.const 65535
        i32.and
        local.set 7
        loop  ;; label = @3
          local.get 8
          local.get 6
          local.get 2
          call_indirect (type 0)
          local.tee 0
          br_if 2 (;@1;)
          local.get 5
          i32.const 1
          i32.add
          local.tee 5
          i32.const 65535
          i32.and
          local.get 7
          i32.ge_u
          br_if 2 (;@1;)
          br 0 (;@3;)
        end
      end
      local.get 0
      i32.load
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 3)
      local.set 0
    end
    local.get 0)
  (func $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter9write_str (type 3) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 2
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 3))
  (func $_RNvXs8_NtNtNtCs4NIexwV1FGX_4core3fmt3num3impmNtB9_7Display3fmt (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 10
    local.set 3
    local.get 0
    i32.load
    local.tee 4
    local.set 5
    block  ;; label = @1
      local.get 4
      i32.const 1000
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 10
      local.set 3
      local.get 4
      local.set 5
      loop  ;; label = @2
        local.get 2
        i32.const 6
        i32.add
        local.get 3
        i32.add
        local.tee 6
        i32.const -4
        i32.add
        local.get 5
        local.tee 0
        local.get 0
        i32.const 10000
        i32.div_u
        local.tee 5
        i32.const 10000
        i32.mul
        i32.sub
        local.tee 7
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 8
        i32.const 1
        i32.shl
        i32.load16_u offset=1053335 align=1
        i32.store16 align=1
        local.get 6
        i32.const -2
        i32.add
        local.get 7
        local.get 8
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.load16_u offset=1053335 align=1
        i32.store16 align=1
        local.get 3
        i32.const -4
        i32.add
        local.set 3
        local.get 0
        i32.const 9999999
        i32.gt_u
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 5
        i32.const 9
        i32.gt_u
        br_if 0 (;@2;)
        local.get 5
        local.set 0
        br 1 (;@1;)
      end
      local.get 2
      i32.const 6
      i32.add
      local.get 3
      i32.const -2
      i32.add
      local.tee 3
      i32.add
      local.get 5
      local.get 5
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 0
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.load16_u offset=1053335 align=1
      i32.store16 align=1
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 2
      i32.const 6
      i32.add
      local.get 3
      i32.const -1
      i32.add
      local.tee 3
      i32.add
      local.get 0
      i32.const 1
      i32.shl
      i32.load8_u offset=1053336
      i32.store8
    end
    local.get 1
    i32.const 1
    i32.const 1
    i32.const 0
    local.get 2
    i32.const 6
    i32.add
    local.get 3
    i32.add
    i32.const 10
    local.get 3
    i32.sub
    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral
    local.set 3
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3)
  (func $_RNvNtCs4NIexwV1FGX_4core3str16slice_error_fail (type 10) (param i32 i32 i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    local.get 4
    call $_RNvNtCs4NIexwV1FGX_4core3str19slice_error_fail_rt
    unreachable)
  (func $_RNvNtCs4NIexwV1FGX_4core3str19slice_error_fail_rt (type 10) (param i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    local.get 3
    i32.store offset=4
    local.get 5
    local.get 2
    i32.store
    local.get 5
    local.get 1
    i32.store offset=8
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    local.get 1
                    i32.gt_u
                    local.tee 6
                    br_if 0 (;@8;)
                    local.get 3
                    local.get 1
                    i32.gt_u
                    local.tee 7
                    br_if 1 (;@7;)
                    local.get 2
                    local.get 3
                    i32.gt_u
                    br_if 2 (;@6;)
                    local.get 2
                    i32.eqz
                    br_if 4 (;@4;)
                    local.get 2
                    local.get 1
                    i32.ge_u
                    br_if 4 (;@4;)
                    local.get 0
                    local.get 2
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    br_if 4 (;@4;)
                    block  ;; label = @9
                      local.get 0
                      local.get 2
                      i32.const -1
                      i32.add
                      local.tee 8
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.gt_s
                      br_if 0 (;@9;)
                      local.get 2
                      i32.const -2
                      i32.add
                      local.tee 3
                      local.get 2
                      i32.const -3
                      i32.add
                      local.get 0
                      local.get 3
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.gt_s
                      select
                      local.set 8
                    end
                    local.get 1
                    local.get 2
                    i32.const 1
                    i32.add
                    local.tee 3
                    local.get 1
                    local.get 3
                    i32.gt_u
                    select
                    local.set 7
                    local.get 2
                    local.get 1
                    local.get 6
                    select
                    local.set 3
                    loop  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 3
                          local.get 2
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 0
                          local.get 2
                          i32.add
                          i32.load8_s
                          i32.const -65
                          i32.le_s
                          br_if 1 (;@10;)
                          br 6 (;@5;)
                        end
                        local.get 3
                        local.get 1
                        i32.const 1053276
                        call $_RNvNtCs4NIexwV1FGX_4core9panicking18panic_bounds_check
                        unreachable
                      end
                      local.get 2
                      i32.const 1
                      i32.add
                      local.tee 2
                      local.get 1
                      i32.lt_u
                      br_if 0 (;@9;)
                    end
                    local.get 7
                    local.set 2
                    br 3 (;@5;)
                  end
                  local.get 5
                  i32.const 2
                  i64.extend_i32_u
                  i64.const 32
                  i64.shl
                  local.tee 9
                  local.get 5
                  i32.const 8
                  i32.add
                  i64.extend_i32_u
                  i64.or
                  i64.store offset=32
                  local.get 5
                  local.get 9
                  local.get 5
                  i64.extend_i32_u
                  i64.or
                  i64.store offset=24
                  i32.const 1049094
                  local.get 5
                  i32.const 24
                  i32.add
                  local.get 4
                  call $_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt
                  unreachable
                end
                local.get 5
                i32.const 2
                i64.extend_i32_u
                i64.const 32
                i64.shl
                local.tee 9
                local.get 5
                i32.const 8
                i32.add
                i64.extend_i32_u
                i64.or
                i64.store offset=32
                local.get 5
                local.get 9
                local.get 5
                i32.const 4
                i32.add
                i64.extend_i32_u
                i64.or
                i64.store offset=24
                i32.const 1049155
                local.get 5
                i32.const 24
                i32.add
                local.get 4
                call $_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt
                unreachable
              end
              local.get 5
              i32.const 2
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.tee 9
              local.get 5
              i32.const 4
              i32.add
              i64.extend_i32_u
              i64.or
              i64.store offset=32
              local.get 5
              local.get 9
              local.get 5
              i64.extend_i32_u
              i64.or
              i64.store offset=24
              i32.const 1049000
              local.get 5
              i32.const 24
              i32.add
              local.get 4
              call $_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt
              unreachable
            end
            local.get 5
            local.get 8
            i32.store offset=12
            local.get 5
            local.get 2
            i32.store offset=16
            block  ;; label = @5
              local.get 8
              local.get 2
              i32.gt_u
              br_if 0 (;@5;)
              local.get 2
              local.get 1
              i32.gt_u
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 8
                local.get 1
                i32.eq
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 8
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 8
                  i32.add
                  i32.load8_s
                  i32.const -65
                  i32.le_s
                  br_if 2 (;@5;)
                end
                local.get 2
                local.get 1
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 2
                i32.add
                i32.load8_s
                i32.const -65
                i32.le_s
                br_if 1 (;@5;)
              end
              local.get 8
              local.get 2
              i32.eq
              br_if 2 (;@3;)
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  local.get 8
                  i32.add
                  local.tee 2
                  i32.load8_s
                  local.tee 3
                  i32.const -1
                  i32.le_s
                  br_if 0 (;@7;)
                  local.get 3
                  i32.const 255
                  i32.and
                  local.set 3
                  br 1 (;@6;)
                end
                local.get 2
                i32.load8_u offset=1
                i32.const 63
                i32.and
                local.set 1
                local.get 3
                i32.const 31
                i32.and
                local.set 0
                block  ;; label = @7
                  local.get 3
                  i32.const -33
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 0
                  i32.const 6
                  i32.shl
                  local.get 1
                  i32.or
                  local.set 3
                  br 1 (;@6;)
                end
                local.get 1
                i32.const 6
                i32.shl
                local.get 2
                i32.load8_u offset=2
                i32.const 63
                i32.and
                i32.or
                local.set 1
                block  ;; label = @7
                  local.get 3
                  i32.const -16
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 0
                  i32.const 12
                  i32.shl
                  i32.or
                  local.set 3
                  br 1 (;@6;)
                end
                local.get 1
                i32.const 6
                i32.shl
                local.get 2
                i32.load8_u offset=3
                i32.const 63
                i32.and
                i32.or
                local.get 0
                i32.const 18
                i32.shl
                i32.const 1835008
                i32.and
                i32.or
                local.set 3
              end
              local.get 5
              local.get 3
              i32.store offset=20
              local.get 5
              i32.const 44
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.get 5
              i32.const 12
              i32.add
              i64.extend_i32_u
              i64.or
              i64.store offset=40
              local.get 5
              i32.const 45
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.get 5
              i32.const 20
              i32.add
              i64.extend_i32_u
              i64.or
              i64.store offset=32
              local.get 5
              i32.const 2
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.get 5
              i64.extend_i32_u
              i64.or
              i64.store offset=24
              i32.const 1049705
              local.get 5
              i32.const 24
              i32.add
              local.get 4
              call $_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt
              unreachable
            end
            local.get 0
            local.get 1
            local.get 8
            local.get 2
            local.get 4
            call $_RNvNtCs4NIexwV1FGX_4core3str16slice_error_fail
            unreachable
          end
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          local.get 1
          i32.ge_u
          br_if 2 (;@1;)
          local.get 0
          local.get 3
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          br_if 2 (;@1;)
          block  ;; label = @4
            local.get 0
            local.get 3
            i32.const -1
            i32.add
            local.tee 8
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            br_if 0 (;@4;)
            local.get 3
            i32.const -2
            i32.add
            local.tee 2
            local.get 3
            i32.const -3
            i32.add
            local.get 0
            local.get 2
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            select
            local.set 8
          end
          local.get 1
          local.get 3
          i32.const 1
          i32.add
          local.tee 2
          local.get 1
          local.get 2
          i32.gt_u
          select
          local.set 6
          local.get 3
          local.get 1
          local.get 7
          select
          local.set 2
          block  ;; label = @4
            loop  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  local.get 3
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 3
                  i32.add
                  i32.load8_s
                  i32.const -65
                  i32.le_s
                  br_if 1 (;@6;)
                  br 3 (;@4;)
                end
                local.get 2
                local.get 1
                i32.const 1053276
                call $_RNvNtCs4NIexwV1FGX_4core9panicking18panic_bounds_check
                unreachable
              end
              local.get 3
              i32.const 1
              i32.add
              local.tee 3
              local.get 1
              i32.lt_u
              br_if 0 (;@5;)
            end
            local.get 6
            local.set 3
          end
          local.get 5
          local.get 8
          i32.store offset=12
          local.get 5
          local.get 3
          i32.store offset=16
          local.get 8
          local.get 3
          i32.gt_u
          br_if 1 (;@2;)
          local.get 3
          local.get 1
          i32.gt_u
          br_if 1 (;@2;)
          block  ;; label = @4
            local.get 8
            local.get 1
            i32.eq
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 8
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              local.get 8
              i32.add
              i32.load8_s
              i32.const -65
              i32.le_s
              br_if 3 (;@2;)
            end
            local.get 3
            local.get 1
            i32.eq
            br_if 0 (;@4;)
            local.get 0
            local.get 3
            i32.add
            i32.load8_s
            i32.const -65
            i32.le_s
            br_if 2 (;@2;)
          end
          local.get 8
          local.get 3
          i32.eq
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              local.get 8
              i32.add
              local.tee 2
              i32.load8_s
              local.tee 3
              i32.const -1
              i32.le_s
              br_if 0 (;@5;)
              local.get 3
              i32.const 255
              i32.and
              local.set 3
              br 1 (;@4;)
            end
            local.get 2
            i32.load8_u offset=1
            i32.const 63
            i32.and
            local.set 1
            local.get 3
            i32.const 31
            i32.and
            local.set 0
            block  ;; label = @5
              local.get 3
              i32.const -33
              i32.gt_u
              br_if 0 (;@5;)
              local.get 0
              i32.const 6
              i32.shl
              local.get 1
              i32.or
              local.set 3
              br 1 (;@4;)
            end
            local.get 1
            i32.const 6
            i32.shl
            local.get 2
            i32.load8_u offset=2
            i32.const 63
            i32.and
            i32.or
            local.set 1
            block  ;; label = @5
              local.get 3
              i32.const -16
              i32.ge_u
              br_if 0 (;@5;)
              local.get 1
              local.get 0
              i32.const 12
              i32.shl
              i32.or
              local.set 3
              br 1 (;@4;)
            end
            local.get 1
            i32.const 6
            i32.shl
            local.get 2
            i32.load8_u offset=3
            i32.const 63
            i32.and
            i32.or
            local.get 0
            i32.const 18
            i32.shl
            i32.const 1835008
            i32.and
            i32.or
            local.set 3
          end
          local.get 5
          local.get 3
          i32.store offset=20
          local.get 5
          i32.const 44
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.get 5
          i32.const 12
          i32.add
          i64.extend_i32_u
          i64.or
          i64.store offset=40
          local.get 5
          i32.const 45
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.get 5
          i32.const 20
          i32.add
          i64.extend_i32_u
          i64.or
          i64.store offset=32
          local.get 5
          i32.const 2
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.get 5
          i32.const 4
          i32.add
          i64.extend_i32_u
          i64.or
          i64.store offset=24
          i32.const 1049787
          local.get 5
          i32.const 24
          i32.add
          local.get 4
          call $_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt
          unreachable
        end
        local.get 4
        call $_RNvNtCs4NIexwV1FGX_4core6option13unwrap_failed
        unreachable
      end
      local.get 0
      local.get 1
      local.get 8
      local.get 3
      local.get 4
      call $_RNvNtCs4NIexwV1FGX_4core3str16slice_error_fail
      unreachable
    end
    local.get 5
    i32.const 2
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.tee 9
    local.get 5
    i32.const 8
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=32
    local.get 5
    local.get 9
    local.get 5
    i32.const 4
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=24
    i32.const 1049155
    local.get 5
    i32.const 24
    i32.add
    local.get 4
    call $_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt
    unreachable)
  (func $_RNvXs_NtNtCs4NIexwV1FGX_4core3ops5rangeINtB4_5RangejENtNtB8_3fmt5Debug3fmtB8_ (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.load offset=8
            local.tee 3
            i32.const 33554432
            i32.and
            br_if 0 (;@4;)
            local.get 3
            i32.const 67108864
            i32.and
            br_if 1 (;@3;)
            local.get 0
            local.get 1
            call $_RNvXs8_NtNtNtCs4NIexwV1FGX_4core3fmt3num3impmNtB9_7Display3fmt
            i32.eqz
            br_if 2 (;@2;)
            i32.const 1
            local.set 4
            br 3 (;@1;)
          end
          local.get 0
          i32.load
          local.set 4
          i32.const 0
          local.set 3
          loop  ;; label = @4
            local.get 2
            i32.const 8
            i32.add
            local.get 3
            i32.add
            i32.const 7
            i32.add
            local.get 4
            i32.const 15
            i32.and
            i32.load8_u offset=1053215
            i32.store8
            local.get 3
            i32.const -1
            i32.add
            local.set 3
            local.get 4
            i32.const 4
            i32.shr_u
            local.tee 4
            br_if 0 (;@4;)
          end
          i32.const 1
          local.set 4
          local.get 1
          i32.const 1
          i32.const 1054364
          i32.const 2
          local.get 2
          i32.const 8
          i32.add
          local.get 3
          i32.add
          i32.const 8
          i32.add
          i32.const 0
          local.get 3
          i32.sub
          call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral
          i32.eqz
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        local.get 0
        i32.load
        local.set 4
        i32.const 0
        local.set 3
        loop  ;; label = @3
          local.get 2
          i32.const 8
          i32.add
          local.get 3
          i32.add
          i32.const 7
          i32.add
          local.get 4
          i32.const 15
          i32.and
          i32.load8_u offset=1054377
          i32.store8
          local.get 3
          i32.const -1
          i32.add
          local.set 3
          local.get 4
          i32.const 4
          i32.shr_u
          local.tee 4
          br_if 0 (;@3;)
        end
        i32.const 1
        local.set 4
        local.get 1
        i32.const 1
        i32.const 1054364
        i32.const 2
        local.get 2
        i32.const 8
        i32.add
        local.get 3
        i32.add
        i32.const 8
        i32.add
        i32.const 0
        local.get 3
        i32.sub
        call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral
        br_if 1 (;@1;)
      end
      block  ;; label = @2
        local.get 1
        i32.load
        i32.const 1054268
        i32.const 2
        local.get 1
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 3)
        i32.eqz
        br_if 0 (;@2;)
        i32.const 1
        local.set 4
        br 1 (;@1;)
      end
      local.get 0
      i32.const 4
      i32.add
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load offset=8
          local.tee 4
          i32.const 33554432
          i32.and
          br_if 0 (;@3;)
          local.get 4
          i32.const 67108864
          i32.and
          br_if 1 (;@2;)
          local.get 3
          local.get 1
          call $_RNvXs8_NtNtNtCs4NIexwV1FGX_4core3fmt3num3impmNtB9_7Display3fmt
          local.set 4
          br 2 (;@1;)
        end
        local.get 3
        i32.load
        local.set 4
        i32.const 0
        local.set 3
        loop  ;; label = @3
          local.get 2
          i32.const 8
          i32.add
          local.get 3
          i32.add
          i32.const 7
          i32.add
          local.get 4
          i32.const 15
          i32.and
          i32.load8_u offset=1053215
          i32.store8
          local.get 3
          i32.const -1
          i32.add
          local.set 3
          local.get 4
          i32.const 4
          i32.shr_u
          local.tee 4
          br_if 0 (;@3;)
        end
        local.get 1
        i32.const 1
        i32.const 1054364
        i32.const 2
        local.get 2
        i32.const 8
        i32.add
        local.get 3
        i32.add
        i32.const 8
        i32.add
        i32.const 0
        local.get 3
        i32.sub
        call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral
        local.set 4
        br 1 (;@1;)
      end
      local.get 3
      i32.load
      local.set 4
      i32.const 0
      local.set 3
      loop  ;; label = @2
        local.get 2
        i32.const 8
        i32.add
        local.get 3
        i32.add
        i32.const 7
        i32.add
        local.get 4
        i32.const 15
        i32.and
        i32.load8_u offset=1054377
        i32.store8
        local.get 3
        i32.const -1
        i32.add
        local.set 3
        local.get 4
        i32.const 4
        i32.shr_u
        local.tee 4
        br_if 0 (;@2;)
      end
      local.get 1
      i32.const 1
      i32.const 1054364
      i32.const 2
      local.get 2
      i32.const 8
      i32.add
      local.get 3
      i32.add
      i32.const 8
      i32.add
      i32.const 0
      local.get 3
      i32.sub
      call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral
      local.set 4
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4)
  (func $_RNvXsj_NtCs4NIexwV1FGX_4core3fmtcNtB5_5Debug3fmt (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 1
    local.set 3
    block  ;; label = @1
      local.get 1
      i32.load
      local.tee 4
      i32.const 39
      local.get 1
      i32.load offset=4
      local.tee 5
      i32.load offset=16
      local.tee 1
      call_indirect (type 0)
      br_if 0 (;@1;)
      local.get 2
      local.get 0
      i32.load
      i32.const 257
      call $_RNvMNtNtCs4NIexwV1FGX_4core4char7methodsc16escape_debug_ext
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.load8_u offset=13
          local.tee 3
          i32.const 129
          i32.lt_u
          br_if 0 (;@3;)
          local.get 4
          local.get 2
          i32.load
          local.get 1
          call_indirect (type 0)
          i32.eqz
          br_if 1 (;@2;)
          i32.const 1
          local.set 3
          br 2 (;@1;)
        end
        local.get 4
        local.get 2
        local.get 2
        i32.load8_u offset=12
        local.tee 0
        i32.add
        local.get 3
        local.get 0
        i32.sub
        local.get 5
        i32.load offset=12
        call_indirect (type 3)
        i32.eqz
        br_if 0 (;@2;)
        i32.const 1
        local.set 3
        br 1 (;@1;)
      end
      local.get 4
      i32.const 39
      local.get 1
      call_indirect (type 0)
      local.set 3
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3)
  (func $_RNvNtCs4NIexwV1FGX_4core6option13unwrap_failed (type 2) (param i32)
    i32.const 1053292
    i32.const 43
    local.get 0
    call $_RNvNtCs4NIexwV1FGX_4core9panicking5panic
    unreachable)
  (func $_RNvNtCs4NIexwV1FGX_4core6option13expect_failed (type 5) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 46
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 3
    i64.extend_i32_u
    i64.or
    i64.store offset=8
    i32.const 1049249
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt
    unreachable)
  (func $_RNvNtCs4NIexwV1FGX_4core6result13unwrap_failed (type 10) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    local.get 1
    i32.store offset=4
    local.get 5
    local.get 0
    i32.store
    local.get 5
    local.get 3
    i32.store offset=12
    local.get 5
    local.get 2
    i32.store offset=8
    local.get 5
    i32.const 47
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 5
    i32.const 8
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=24
    local.get 5
    i32.const 46
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 5
    i64.extend_i32_u
    i64.or
    i64.store offset=16
    i32.const 1049245
    local.get 5
    i32.const 16
    i32.add
    local.get 4
    call $_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt
    unreachable)
  (func $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRDNtB6_5DebugEL_Bx_3fmtB8_ (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 0))
  (func $_RNvXs6_NtNtCs4NIexwV1FGX_4core3fmt3numjNtB7_8LowerHex3fmt (type 0) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 2
      i32.const 8
      i32.add
      local.get 0
      i32.add
      i32.const 7
      i32.add
      local.get 3
      i32.const 15
      i32.and
      i32.load8_u offset=1053215
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      local.tee 3
      br_if 0 (;@1;)
    end
    local.get 1
    i32.const 1
    i32.const 1054364
    i32.const 2
    local.get 2
    i32.const 8
    i32.add
    local.get 0
    i32.add
    i32.const 8
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_RNvNtNtCs4NIexwV1FGX_4core9panicking11panic_const23panic_const_div_by_zero (type 2) (param i32)
    i32.const 1053791
    i32.const 51
    local.get 0
    call $_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt
    unreachable)
  (func $_RNvXNtNtCs4NIexwV1FGX_4core2io5errorNtB2_5ErrorNtNtB6_3fmt5Debug3fmt (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $_RNvXs2_NtNtCs4NIexwV1FGX_4core2io5errorNtNtB5_4repr4ReprNtNtB9_3fmt5Debug3fmt)
  (func $_RNvXs2_NtNtCs4NIexwV1FGX_4core2io5errorNtNtB5_4repr4ReprNtNtB9_3fmt5Debug3fmt (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load8_u
              br_table 0 (;@5;) 1 (;@4;) 2 (;@3;) 3 (;@2;) 0 (;@5;)
            end
            local.get 2
            local.get 0
            i32.load offset=4
            i32.store offset=8
            local.get 2
            local.get 1
            i32.load
            i32.const 1054274
            i32.const 2
            local.get 1
            i32.load offset=4
            i32.load offset=12
            call_indirect (type 3)
            i32.store8 offset=24
            local.get 2
            local.get 1
            i32.store offset=20
            local.get 2
            i32.const 0
            i32.store8 offset=25
            local.get 2
            i32.const 20
            i32.add
            i32.const 1054292
            i32.const 4
            local.get 2
            i32.const 8
            i32.add
            i32.const 1054276
            call $_RNvMs2_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_11DebugStruct5field
            local.set 1
            local.get 2
            local.get 2
            i32.load offset=8
            i32.const 0
            i32.load offset=1055684
            i32.load offset=4
            call_indirect (type 4)
            i32.store8 offset=15
            local.get 1
            i32.const 1054312
            i32.const 4
            local.get 2
            i32.const 15
            i32.add
            i32.const 1054296
            call $_RNvMs2_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_11DebugStruct5field
            local.set 1
            local.get 2
            local.get 2
            i32.const 8
            i32.add
            i32.store offset=16
            local.get 1
            i32.const 1054332
            i32.const 7
            local.get 2
            i32.const 16
            i32.add
            i32.const 1054316
            call $_RNvMs2_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_11DebugStruct5field
            local.set 3
            local.get 2
            i32.load8_u offset=25
            local.tee 1
            local.get 2
            i32.load8_u offset=24
            local.tee 4
            i32.or
            local.set 0
            local.get 1
            i32.const 1
            i32.ne
            br_if 3 (;@1;)
            local.get 4
            i32.const 1
            i32.and
            br_if 3 (;@1;)
            block  ;; label = @5
              local.get 3
              i32.load
              local.tee 1
              i32.load8_u offset=10
              i32.const 128
              i32.and
              br_if 0 (;@5;)
              local.get 1
              i32.load
              i32.const 1053249
              i32.const 2
              local.get 1
              i32.load offset=4
              i32.load offset=12
              call_indirect (type 3)
              local.set 0
              br 4 (;@1;)
            end
            local.get 1
            i32.load
            i32.const 1053231
            i32.const 1
            local.get 1
            i32.load offset=4
            i32.load offset=12
            call_indirect (type 3)
            local.set 0
            br 3 (;@1;)
          end
          local.get 0
          i32.load8_u offset=1
          local.set 4
          i32.const 1
          local.set 0
          local.get 1
          i32.load
          i32.const 1054339
          i32.const 4
          local.get 1
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 3)
          br_if 2 (;@1;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load8_u offset=10
              i32.const 128
              i32.and
              br_if 0 (;@5;)
              i32.const 1
              local.set 0
              local.get 1
              i32.load
              i32.const 1053245
              i32.const 1
              local.get 1
              i32.load offset=4
              i32.load offset=12
              call_indirect (type 3)
              br_if 4 (;@1;)
              local.get 1
              i32.load
              local.get 4
              i32.const 2
              i32.shl
              i32.load offset=1055048
              local.get 4
              i32.load8_u offset=1055004
              local.get 1
              i32.load offset=4
              i32.load offset=12
              call_indirect (type 3)
              i32.eqz
              br_if 1 (;@4;)
              br 4 (;@1;)
            end
            local.get 1
            i32.load
            i32.const 1053246
            i32.const 2
            local.get 1
            i32.load offset=4
            i32.load offset=12
            call_indirect (type 3)
            br_if 3 (;@1;)
            i32.const 1
            local.set 0
            local.get 2
            i32.const 1
            i32.store8 offset=16
            local.get 2
            local.get 1
            i64.load align=4
            i64.store offset=20 align=4
            local.get 4
            i32.const 2
            i32.shl
            i32.load offset=1055048
            local.set 3
            local.get 2
            local.get 2
            i32.const 16
            i32.add
            i32.store offset=28
            local.get 2
            i32.const 20
            i32.add
            local.get 3
            local.get 4
            i32.load8_u offset=1055004
            call $_RNvXs0_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write9write_str
            br_if 3 (;@1;)
            local.get 2
            i32.const 20
            i32.add
            i32.const 1053242
            i32.const 2
            call $_RNvXs0_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write9write_str
            br_if 3 (;@1;)
          end
          local.get 1
          i32.load
          i32.const 1053244
          i32.const 1
          local.get 1
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 3)
          local.set 0
          br 2 (;@1;)
        end
        local.get 0
        i32.load offset=4
        local.set 0
        local.get 1
        i32.load
        i32.const 1054343
        i32.const 5
        local.get 1
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 3)
        local.set 4
        local.get 2
        i32.const 0
        i32.store8 offset=25
        local.get 2
        local.get 4
        i32.store8 offset=24
        local.get 2
        local.get 1
        i32.store offset=20
        local.get 2
        i32.const 20
        i32.add
        i32.const 1054312
        i32.const 4
        local.get 0
        i32.const 8
        i32.add
        i32.const 1054296
        call $_RNvMs2_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_11DebugStruct5field
        i32.const 1054332
        i32.const 7
        local.get 0
        i32.const 1054348
        call $_RNvMs2_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_11DebugStruct5field
        local.set 3
        local.get 2
        i32.load8_u offset=25
        local.tee 1
        local.get 2
        i32.load8_u offset=24
        local.tee 4
        i32.or
        local.set 0
        local.get 1
        i32.const 1
        i32.ne
        br_if 1 (;@1;)
        local.get 4
        i32.const 1
        i32.and
        br_if 1 (;@1;)
        block  ;; label = @3
          local.get 3
          i32.load
          local.tee 1
          i32.load8_u offset=10
          i32.const 128
          i32.and
          br_if 0 (;@3;)
          local.get 1
          i32.load
          i32.const 1053249
          i32.const 2
          local.get 1
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 3)
          local.set 0
          br 2 (;@1;)
        end
        local.get 1
        i32.load
        i32.const 1053231
        i32.const 1
        local.get 1
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 3)
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      i32.load offset=4
      local.get 1
      call $_RNvXs8_NtNtCs4NIexwV1FGX_4core2io5errorNtB5_6CustomNtNtB9_3fmt5Debug3fmt
      local.set 0
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 0
    i32.const 1
    i32.and)
  (func $_RNvXNtNtNtCs4NIexwV1FGX_4core3fmt3num3imphNtB6_7Display3fmt (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 3
    local.set 3
    local.get 0
    i32.load8_u
    local.tee 0
    local.set 4
    block  ;; label = @1
      local.get 0
      i32.const 10
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 1
      local.set 3
      local.get 2
      local.get 0
      local.get 0
      i32.const 100
      i32.div_u
      local.tee 4
      i32.const 100
      i32.mul
      i32.sub
      i32.const 255
      i32.and
      i32.const 1
      i32.shl
      i32.load16_u offset=1053335 align=1
      i32.store16 offset=14 align=1
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 2
      i32.const 13
      i32.add
      local.get 3
      i32.const -1
      i32.add
      local.tee 3
      i32.add
      local.get 4
      i32.const 1
      i32.shl
      i32.load8_u offset=1053336
      i32.store8
    end
    local.get 1
    i32.const 1
    i32.const 1
    i32.const 0
    local.get 2
    i32.const 13
    i32.add
    local.get 3
    i32.add
    local.get 3
    i32.const 3
    i32.xor
    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral
    local.set 3
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3)
  (func $_RNvXs0_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write10write_char (type 0) (param i32 i32) (result i32)
    (local i32 i32)
    local.get 0
    i32.load offset=4
    local.set 2
    local.get 0
    i32.load
    local.set 3
    block  ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 0
      i32.load8_u
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const 1054270
      i32.const 4
      local.get 2
      i32.load offset=12
      call_indirect (type 3)
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1
      return
    end
    local.get 0
    local.get 1
    i32.const 10
    i32.eq
    i32.store8
    local.get 3
    local.get 1
    local.get 2
    i32.load offset=16
    call_indirect (type 0))
  (func $_RNvXsg_NtNtCs4NIexwV1FGX_4core3fmt3numhNtB7_8UpperHex3fmt (type 0) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load8_u
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 2
      i32.const 14
      i32.add
      local.get 0
      i32.add
      i32.const 1
      i32.add
      local.get 3
      i32.const 15
      i32.and
      i32.const 1054377
      i32.add
      i32.load8_u
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      i32.const 15
      i32.and
      local.tee 3
      br_if 0 (;@1;)
    end
    local.get 1
    i32.const 1
    i32.const 1054364
    i32.const 2
    local.get 2
    i32.const 14
    i32.add
    local.get 0
    i32.add
    i32.const 2
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_RNvXs9_NtNtNtCs4NIexwV1FGX_4core3fmt3num3implNtB9_7Display3fmt (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 10
    local.set 3
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 4
      local.get 4
      i32.const 31
      i32.shr_s
      local.tee 0
      i32.xor
      local.get 0
      i32.sub
      local.tee 5
      i32.const 1000
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 10
      local.set 3
      loop  ;; label = @2
        local.get 2
        i32.const 6
        i32.add
        local.get 3
        i32.add
        local.tee 6
        i32.const -4
        i32.add
        local.get 5
        local.tee 0
        local.get 0
        i32.const 10000
        i32.div_u
        local.tee 5
        i32.const 10000
        i32.mul
        i32.sub
        local.tee 7
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 8
        i32.const 1
        i32.shl
        i32.load16_u offset=1053335 align=1
        i32.store16 align=1
        local.get 6
        i32.const -2
        i32.add
        local.get 7
        local.get 8
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.load16_u offset=1053335 align=1
        i32.store16 align=1
        local.get 3
        i32.const -4
        i32.add
        local.set 3
        local.get 0
        i32.const 9999999
        i32.gt_u
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 5
        i32.const 9
        i32.gt_u
        br_if 0 (;@2;)
        local.get 5
        local.set 0
        br 1 (;@1;)
      end
      local.get 2
      i32.const 6
      i32.add
      local.get 3
      i32.const -2
      i32.add
      local.tee 3
      i32.add
      local.get 5
      local.get 5
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 0
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.load16_u offset=1053335 align=1
      i32.store16 align=1
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 2
      i32.const 6
      i32.add
      local.get 3
      i32.const -1
      i32.add
      local.tee 3
      i32.add
      local.get 0
      i32.const 1
      i32.shl
      i32.load8_u offset=1053336
      i32.store8
    end
    local.get 1
    local.get 4
    i32.const -1
    i32.xor
    i32.const 31
    i32.shr_u
    i32.const 1
    i32.const 0
    local.get 2
    i32.const 6
    i32.add
    local.get 3
    i32.add
    i32.const 10
    local.get 3
    i32.sub
    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral
    local.set 3
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3)
  (func $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtReNtB6_5Debug3fmtB8_ (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 1
    call $_RNvXsh_NtCs4NIexwV1FGX_4core3fmteNtB5_5Debug3fmt)
  (func $_RNvXsh_NtCs4NIexwV1FGX_4core3fmteNtB5_5Debug3fmt (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const 1
    local.set 4
    block  ;; label = @1
      local.get 2
      i32.load
      local.tee 5
      i32.const 34
      local.get 2
      i32.load offset=4
      local.tee 6
      i32.load offset=16
      local.tee 7
      call_indirect (type 0)
      br_if 0 (;@1;)
      i32.const 0
      local.set 8
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              br_if 0 (;@5;)
              i32.const 0
              local.set 2
              br 1 (;@4;)
            end
            i32.const 0
            local.get 1
            i32.sub
            local.set 9
            i32.const 0
            local.set 10
            local.get 1
            local.set 11
            local.get 0
            local.set 12
            block  ;; label = @5
              loop  ;; label = @6
                local.get 12
                local.get 11
                i32.add
                local.set 13
                i32.const 0
                local.set 2
                block  ;; label = @7
                  loop  ;; label = @8
                    local.get 12
                    local.get 2
                    i32.add
                    local.tee 14
                    i32.load8_u
                    local.tee 15
                    i32.const -127
                    i32.add
                    i32.const 255
                    i32.and
                    i32.const 161
                    i32.lt_u
                    br_if 1 (;@7;)
                    local.get 15
                    i32.const 34
                    i32.eq
                    br_if 1 (;@7;)
                    local.get 15
                    i32.const 92
                    i32.eq
                    br_if 1 (;@7;)
                    local.get 11
                    local.get 2
                    i32.const 1
                    i32.add
                    local.tee 2
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  local.get 10
                  local.get 11
                  i32.add
                  local.set 2
                  br 2 (;@5;)
                end
                local.get 14
                i32.const 1
                i32.add
                local.set 12
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 14
                    i32.load8_s
                    local.tee 15
                    i32.const -1
                    i32.le_s
                    br_if 0 (;@8;)
                    local.get 15
                    i32.const 255
                    i32.and
                    local.set 15
                    br 1 (;@7;)
                  end
                  local.get 12
                  i32.load8_u
                  i32.const 63
                  i32.and
                  local.set 11
                  local.get 15
                  i32.const 31
                  i32.and
                  local.set 16
                  local.get 14
                  i32.const 2
                  i32.add
                  local.set 12
                  block  ;; label = @8
                    local.get 15
                    i32.const -33
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 16
                    i32.const 6
                    i32.shl
                    local.get 11
                    i32.or
                    local.set 15
                    br 1 (;@7;)
                  end
                  local.get 11
                  i32.const 6
                  i32.shl
                  local.get 12
                  i32.load8_u
                  i32.const 63
                  i32.and
                  i32.or
                  local.set 11
                  local.get 14
                  i32.const 3
                  i32.add
                  local.set 12
                  block  ;; label = @8
                    local.get 15
                    i32.const -16
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 11
                    local.get 16
                    i32.const 12
                    i32.shl
                    i32.or
                    local.set 15
                    br 1 (;@7;)
                  end
                  local.get 11
                  i32.const 6
                  i32.shl
                  local.get 12
                  i32.load8_u
                  i32.const 63
                  i32.and
                  i32.or
                  local.get 16
                  i32.const 18
                  i32.shl
                  i32.const 1835008
                  i32.and
                  i32.or
                  local.set 15
                  local.get 14
                  i32.const 4
                  i32.add
                  local.set 12
                end
                local.get 3
                local.get 15
                i32.const 65537
                call $_RNvMNtNtCs4NIexwV1FGX_4core4char7methodsc16escape_debug_ext
                block  ;; label = @7
                  local.get 3
                  i32.load8_u offset=13
                  local.tee 14
                  local.get 3
                  i32.load8_u offset=12
                  local.tee 16
                  i32.sub
                  local.tee 17
                  i32.const 255
                  i32.and
                  i32.const 1
                  i32.eq
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 8
                      local.get 10
                      local.get 2
                      i32.add
                      local.tee 11
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 11
                      local.get 1
                      i32.gt_u
                      br_if 0 (;@9;)
                      block  ;; label = @10
                        local.get 8
                        local.get 1
                        i32.eq
                        br_if 0 (;@10;)
                        block  ;; label = @11
                          local.get 8
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 0
                          local.get 8
                          i32.add
                          i32.load8_s
                          i32.const -65
                          i32.le_s
                          br_if 2 (;@9;)
                        end
                        local.get 11
                        local.get 9
                        i32.add
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 10
                        i32.add
                        local.get 2
                        i32.add
                        i32.load8_s
                        i32.const -65
                        i32.le_s
                        br_if 1 (;@9;)
                      end
                      local.get 5
                      local.get 0
                      local.get 8
                      i32.add
                      local.get 10
                      local.get 8
                      i32.sub
                      local.get 2
                      i32.add
                      local.get 6
                      i32.load offset=12
                      local.tee 11
                      call_indirect (type 3)
                      i32.eqz
                      br_if 1 (;@8;)
                      br 7 (;@2;)
                    end
                    local.get 0
                    local.get 1
                    local.get 8
                    local.get 11
                    i32.const 1054396
                    call $_RNvNtCs4NIexwV1FGX_4core3str16slice_error_fail
                    unreachable
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 14
                      i32.const 129
                      i32.lt_u
                      br_if 0 (;@9;)
                      local.get 5
                      local.get 3
                      i32.load
                      local.get 7
                      call_indirect (type 0)
                      br_if 7 (;@2;)
                      br 1 (;@8;)
                    end
                    local.get 5
                    local.get 3
                    local.get 16
                    i32.add
                    local.get 17
                    local.get 11
                    call_indirect (type 3)
                    br_if 6 (;@2;)
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 15
                      i32.const 128
                      i32.ge_u
                      br_if 0 (;@9;)
                      i32.const 1
                      local.set 14
                      br 1 (;@8;)
                    end
                    block  ;; label = @9
                      local.get 15
                      i32.const 2048
                      i32.ge_u
                      br_if 0 (;@9;)
                      i32.const 2
                      local.set 14
                      br 1 (;@8;)
                    end
                    i32.const 3
                    i32.const 4
                    local.get 15
                    i32.const 65536
                    i32.lt_u
                    select
                    local.set 14
                  end
                  local.get 14
                  local.get 10
                  i32.add
                  local.get 2
                  i32.add
                  local.set 8
                end
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 15
                    i32.const 128
                    i32.ge_u
                    br_if 0 (;@8;)
                    i32.const 1
                    local.set 15
                    br 1 (;@7;)
                  end
                  block  ;; label = @8
                    local.get 15
                    i32.const 2048
                    i32.ge_u
                    br_if 0 (;@8;)
                    i32.const 2
                    local.set 15
                    br 1 (;@7;)
                  end
                  i32.const 3
                  i32.const 4
                  local.get 15
                  i32.const 65536
                  i32.lt_u
                  select
                  local.set 15
                end
                local.get 15
                local.get 10
                i32.add
                local.tee 15
                local.get 2
                i32.add
                local.set 10
                local.get 13
                local.get 12
                i32.sub
                local.tee 11
                br_if 0 (;@6;)
              end
              local.get 15
              local.get 2
              i32.add
              local.set 2
            end
            local.get 8
            local.get 2
            i32.gt_u
            br_if 1 (;@3;)
            local.get 2
            local.get 1
            i32.gt_u
            br_if 1 (;@3;)
            local.get 8
            local.get 1
            i32.eq
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 8
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              local.get 8
              i32.add
              i32.load8_s
              i32.const -65
              i32.le_s
              br_if 2 (;@3;)
            end
            local.get 2
            local.get 1
            i32.eq
            br_if 0 (;@4;)
            local.get 0
            local.get 2
            i32.add
            i32.load8_s
            i32.const -64
            i32.lt_s
            br_if 1 (;@3;)
          end
          local.get 5
          local.get 0
          local.get 8
          i32.add
          local.get 2
          local.get 8
          i32.sub
          local.get 6
          i32.load offset=12
          call_indirect (type 3)
          br_if 2 (;@1;)
          local.get 5
          i32.const 34
          local.get 7
          call_indirect (type 0)
          local.set 4
          br 2 (;@1;)
        end
        local.get 0
        local.get 1
        local.get 8
        local.get 2
        i32.const 1054412
        call $_RNvNtCs4NIexwV1FGX_4core3str16slice_error_fail
        unreachable
      end
      i32.const 1
      local.set 4
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4)
  (func $_RNvXs8_NtNtCs4NIexwV1FGX_4core2io5errorNtB5_6CustomNtNtB9_3fmt5Debug3fmt (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 1
    i32.load
    i32.const 1054366
    i32.const 6
    local.get 1
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 3)
    local.set 3
    local.get 2
    i32.const 0
    i32.store8 offset=13
    local.get 2
    local.get 3
    i32.store8 offset=12
    local.get 2
    local.get 1
    i32.store offset=8
    local.get 2
    i32.const 8
    i32.add
    i32.const 1054312
    i32.const 4
    local.get 0
    i32.const 16
    i32.add
    i32.const 1054296
    call $_RNvMs2_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_11DebugStruct5field
    i32.const 1054372
    i32.const 5
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_RNvMs2_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_11DebugStruct5field
    local.set 4
    local.get 2
    i32.load8_u offset=13
    local.tee 0
    local.get 2
    i32.load8_u offset=12
    local.tee 3
    i32.or
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 3
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 4
        i32.load
        local.tee 1
        i32.load8_u offset=10
        i32.const 128
        i32.and
        br_if 0 (;@2;)
        local.get 1
        i32.load
        i32.const 1053249
        i32.const 2
        local.get 1
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 3)
        local.set 1
        br 1 (;@1;)
      end
      local.get 1
      i32.load
      i32.const 1053231
      i32.const 1
      local.get 1
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 3)
      local.set 1
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
    i32.const 1
    i32.and)
  (func $_RNvXs9_NtNtCs4NIexwV1FGX_4core3fmt8buildersINtB5_6FromFnNCNCNvXs2_NtNtB9_2io5errorNtNtB12_4repr4ReprNtB7_5Debug3fmt00ENtB7_7Display3fmtB9_ (type 0) (param i32 i32) (result i32)
    local.get 0
    i32.load
    i32.load
    local.get 1
    i32.const 0
    i32.load offset=1055684
    i32.load
    call_indirect (type 0))
  (func $_RNvXse_NtNtCs4NIexwV1FGX_4core3fmt3numhNtB7_8LowerHex3fmt (type 0) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load8_u
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 2
      i32.const 14
      i32.add
      local.get 0
      i32.add
      i32.const 1
      i32.add
      local.get 3
      i32.const 15
      i32.and
      i32.const 1053215
      i32.add
      i32.load8_u
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      i32.const 15
      i32.and
      local.tee 3
      br_if 0 (;@1;)
    end
    local.get 1
    i32.const 1
    i32.const 1054364
    i32.const 2
    local.get 2
    i32.const 14
    i32.add
    local.get 0
    i32.add
    i32.const 2
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_RNvXs8_NtNtCs4NIexwV1FGX_4core3fmt3numjNtB7_8UpperHex3fmt (type 0) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 2
      i32.const 8
      i32.add
      local.get 0
      i32.add
      i32.const 7
      i32.add
      local.get 3
      i32.const 15
      i32.and
      i32.load8_u offset=1054377
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      local.tee 3
      br_if 0 (;@1;)
    end
    local.get 1
    i32.const 1
    i32.const 1054364
    i32.const 2
    local.get 2
    i32.const 8
    i32.add
    local.get 0
    i32.add
    i32.const 8
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_RNvXs8_NtNtCs4NIexwV1FGX_4core3fmt8buildersINtB5_6FromFnNCNvXs2_NtNtB9_2io5errorNtNtB10_4repr4ReprNtB7_5Debug3fmt0EB1y_3fmtB9_ (type 0) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 1
    i32.load offset=4
    local.set 3
    local.get 1
    i32.load
    local.set 1
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 48
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 2
    i32.const 4
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=8
    local.get 1
    local.get 3
    i32.const 1049921
    local.get 2
    i32.const 8
    i32.add
    call $_RNvNtCs4NIexwV1FGX_4core3fmt5write
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_RNvXs9_NtNtCs4NIexwV1FGX_4core2io5errorNtB5_6CustomNtNtNtB9_3ops4drop4Drop4drop (type 2) (param i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    call_indirect (type 1))
  (func $_RNvXsQ_NtNtCs4NIexwV1FGX_4core3fmt3numlNtB7_5Debug3fmt (type 0) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load offset=8
          local.tee 3
          i32.const 33554432
          i32.and
          br_if 0 (;@3;)
          local.get 3
          i32.const 67108864
          i32.and
          br_if 1 (;@2;)
          local.get 0
          local.get 1
          call $_RNvXs9_NtNtNtCs4NIexwV1FGX_4core3fmt3num3implNtB9_7Display3fmt
          local.set 0
          br 2 (;@1;)
        end
        local.get 0
        i32.load
        local.set 3
        i32.const 0
        local.set 0
        loop  ;; label = @3
          local.get 2
          i32.const 8
          i32.add
          local.get 0
          i32.add
          i32.const 7
          i32.add
          local.get 3
          i32.const 15
          i32.and
          i32.load8_u offset=1053215
          i32.store8
          local.get 0
          i32.const -1
          i32.add
          local.set 0
          local.get 3
          i32.const 4
          i32.shr_u
          local.tee 3
          br_if 0 (;@3;)
        end
        local.get 1
        i32.const 1
        i32.const 1054364
        i32.const 2
        local.get 2
        i32.const 8
        i32.add
        local.get 0
        i32.add
        i32.const 8
        i32.add
        i32.const 0
        local.get 0
        i32.sub
        call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      i32.load
      local.set 3
      i32.const 0
      local.set 0
      loop  ;; label = @2
        local.get 2
        i32.const 8
        i32.add
        local.get 0
        i32.add
        i32.const 7
        i32.add
        local.get 3
        i32.const 15
        i32.and
        i32.load8_u offset=1054377
        i32.store8
        local.get 0
        i32.const -1
        i32.add
        local.set 0
        local.get 3
        i32.const 4
        i32.shr_u
        local.tee 3
        br_if 0 (;@2;)
      end
      local.get 1
      i32.const 1
      i32.const 1054364
      i32.const 2
      local.get 2
      i32.const 8
      i32.add
      local.get 0
      i32.add
      i32.const 8
      i32.add
      i32.const 0
      local.get 0
      i32.sub
      call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integral
      local.set 0
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_RNvXsd_NtNtCs4NIexwV1FGX_4core2io5errorNtB5_11CustomOwnerNtNtNtB9_3ops4drop4Drop4drop (type 2) (param i32)
    local.get 0
    i32.load
    local.tee 0
    local.get 0
    i32.load offset=12
    call_indirect (type 2))
  (func $_RNvXsi_NtCs4NIexwV1FGX_4core3fmteNtB5_7Display3fmt (type 3) (param i32 i32 i32) (result i32)
    local.get 2
    local.get 0
    local.get 1
    call $_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter3pad)
  (func $_RNvXsn_NtNtCs4NIexwV1FGX_4core2io5errorNtB5_9ErrorKindNtNtB9_3fmt5Debug3fmt (type 0) (param i32 i32) (result i32)
    local.get 1
    i32.load
    local.get 0
    i32.load8_u
    local.tee 0
    i32.const 2
    i32.shl
    i32.load offset=1055048
    local.get 0
    i32.load8_u offset=1055004
    local.get 1
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 3))
  (func $_RNvYNCNvMs5_NtNtCs4NIexwV1FGX_4core2io5errorNtBa_11OsFunctions7DEFAULT0INtNtNtBe_3ops8function6FnOnceTlQNtNtBe_3fmt9FormatterEE9call_onceBe_ (type 0) (param i32 i32) (result i32)
    i32.const 0)
  (func $_RNvYNCNvMs5_NtNtCs4NIexwV1FGX_4core2io5errorNtBa_11OsFunctions7DEFAULTs0_0INtNtNtBe_3ops8function6FnOnceTlEE9call_onceBe_ (type 4) (param i32) (result i32)
    i32.const 0)
  (func $_RNvYNCNvMs5_NtNtCs4NIexwV1FGX_4core2io5errorNtBa_11OsFunctions7DEFAULTs_0INtNtNtBe_3ops8function6FnOnceTlEE9call_onceBe_ (type 4) (param i32) (result i32)
    i32.const 43)
  (func $_RNvYNtNtNtCs4NIexwV1FGX_4core3fmt8builders10PadAdapterNtB6_5Write9write_fmtB8_ (type 3) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 1053252
    local.get 1
    local.get 2
    call $_RNvNtCs4NIexwV1FGX_4core3fmt5write)
  (table (;0;) 59 59 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (export "memory" (memory 0))
  (export "entry_point" (func $entry_point))
  (export "ns_free" (func $ns_free))
  (export "ns_malloc" (func $ns_malloc))
  (elem (;0;) (i32.const 1) func $_RNvXs1i_NtCs4NIexwV1FGX_4core3fmtRDNtNtCs8n3ERUNuD4B_10serde_core2de8ExpectedEL_NtB6_7Display3fmtCshuXg64jKSg3_8wasm_lib $_RNvXs8_NtNtNtCs4NIexwV1FGX_4core3fmt3num3impmNtB9_7Display3fmt $_RNvXs0_NtCs8n3ERUNuD4B_10serde_core2deReNtB5_8Expected3fmt $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRNtNtNtB8_2io5error5ErrorNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRNtNtNtB8_3str5error9Utf8ErrorNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRhNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRjNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRNtNtCs5JHfeWeQK0x_5alloc6string6StringNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib $_RNvXsZ_NtNtCs4NIexwV1FGX_4core3fmt3numjNtB7_5Debug3fmt $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRINtNtB8_6option6OptionhENtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib $_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBL_3BoxDNtNtB4_5error5ErrorNtNtB4_6marker4SendNtB1R_4SyncEL_EINtNtB4_7convert4FromNtNtBN_6string6StringE4from11StringErrorECshuXg64jKSg3_8wasm_lib $_RNvXs_NvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4fromNtB4_11StringErrorNtNtB11_3fmt7Display3fmt $_RNvXs0_NvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBd_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB12_6marker4SendNtB1z_4SyncEL_EINtNtB12_7convert4FromNtNtBf_6string6StringE4fromNtB5_11StringErrorNtNtB12_3fmt5Debug3fmt $_RNvYNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4from11StringErrorBX_5causeCshuXg64jKSg3_8wasm_lib $_RNvYNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4from11StringErrorBX_7type_idCshuXg64jKSg3_8wasm_lib $_RNvYNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4from11StringErrorBX_11descriptionCshuXg64jKSg3_8wasm_lib $_RNvYNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4from11StringErrorBX_7provideCshuXg64jKSg3_8wasm_lib $_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueINtNtCs5JHfeWeQK0x_5alloc5boxed3BoxNtNtCs1NJkdcuIVXo_7bincode5error9ErrorKindEECshuXg64jKSg3_8wasm_lib $_RNvXsn_NtCs5JHfeWeQK0x_5alloc5boxedINtB5_3BoxNtNtCs1NJkdcuIVXo_7bincode5error9ErrorKindENtNtCs4NIexwV1FGX_4core3fmt5Debug3fmtCshuXg64jKSg3_8wasm_lib $_RNvXsd_NtNtNtCs4NIexwV1FGX_4core3fmt3num3impyNtB9_7Display3fmt $_RNvNtCsiuSwmoQr08c_3std5alloc24default_alloc_error_hook $_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueNtNtCs5JHfeWeQK0x_5alloc6string6StringECsiuSwmoQr08c_3std $_RNvXsZ_NtCs5JHfeWeQK0x_5alloc6stringNtB5_6StringNtNtCs4NIexwV1FGX_4core3fmt5Write9write_str $_RNvXsZ_NtCs5JHfeWeQK0x_5alloc6stringNtB5_6StringNtNtCs4NIexwV1FGX_4core3fmt5Write10write_char $_RNvYNtNtCs5JHfeWeQK0x_5alloc6string6StringNtNtCs4NIexwV1FGX_4core3fmt5Write9write_fmtCsiuSwmoQr08c_3std $_RNvXs2_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCs4NIexwV1FGX_4core3fmt7Display3fmt $_RNvXs1_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload8take_box $_RNvXs1_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload3get $_RNvXs1_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload6as_str $_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueNtNvNtCsiuSwmoQr08c_3std9panicking13panic_handler19FormatStringPayloadEBH_ $_RNvXs0_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_19FormatStringPayloadNtNtCs4NIexwV1FGX_4core3fmt7Display3fmt $_RNvXs_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB4_19FormatStringPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload8take_box $_RNvXs_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB4_19FormatStringPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload3get $_RNvYINtNvNtCsiuSwmoQr08c_3std9panicking11begin_panic7PayloadReENtNtCs4NIexwV1FGX_4core5panic12PanicPayload6as_strB9_ $_RNvXNtCs4NIexwV1FGX_4core3anyReNtB2_3Any7type_idCsiuSwmoQr08c_3std $_RNvXNtCs4NIexwV1FGX_4core3anyNtNtCs5JHfeWeQK0x_5alloc6string6StringNtB2_3Any7type_idCsiuSwmoQr08c_3std $_RINvNvNtNtCs5JHfeWeQK0x_5alloc2io5error21custom_owner_from_box12drop_box_rawNtNtNtCs4NIexwV1FGX_4core2io5error6CustomEB8_ $_RINvNvNtNtCs5JHfeWeQK0x_5alloc2io5error21custom_owner_from_box12drop_box_rawDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB1h_6marker4SendNtB1O_4SyncEL_EB8_ $_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueNtNtCs5JHfeWeQK0x_5alloc6string6StringEBF_ $_RNvXsZ_NtCs5JHfeWeQK0x_5alloc6stringNtB5_6StringNtNtCs4NIexwV1FGX_4core3fmt5Write9write_str.1 $_RNvXsZ_NtCs5JHfeWeQK0x_5alloc6stringNtB5_6StringNtNtCs4NIexwV1FGX_4core3fmt5Write10write_char.1 $_RNvYNtNtCs5JHfeWeQK0x_5alloc6string6StringNtNtCs4NIexwV1FGX_4core3fmt5Write9write_fmtB6_ $_RNvXsK_NtCs4NIexwV1FGX_4core3fmtNtB5_5ErrorNtB5_5Debug3fmt $_RNvXs_NtNtCs4NIexwV1FGX_4core3ops5rangeINtB4_5RangejENtNtB8_3fmt5Debug3fmtB8_ $_RNvXsj_NtCs4NIexwV1FGX_4core3fmtcNtB5_5Debug3fmt $_RNvXs1i_NtCs4NIexwV1FGX_4core3fmtReNtB6_7Display3fmtB8_ $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRDNtB6_5DebugEL_Bx_3fmtB8_ $_RNvXs9_NtNtCs4NIexwV1FGX_4core3fmt8buildersINtB5_6FromFnNCNCNvXs2_NtNtB9_2io5errorNtNtB12_4repr4ReprNtB7_5Debug3fmt00ENtB7_7Display3fmtB9_ $_RNvXs0_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write9write_str $_RNvXs0_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write10write_char $_RNvYNtNtNtCs4NIexwV1FGX_4core3fmt8builders10PadAdapterNtB6_5Write9write_fmtB8_ $_RNvYNCNvMs5_NtNtCs4NIexwV1FGX_4core2io5errorNtBa_11OsFunctions7DEFAULT0INtNtNtBe_3ops8function6FnOnceTlQNtNtBe_3fmt9FormatterEE9call_onceBe_ $_RNvYNCNvMs5_NtNtCs4NIexwV1FGX_4core2io5errorNtBa_11OsFunctions7DEFAULTs_0INtNtNtBe_3ops8function6FnOnceTlEE9call_onceBe_ $_RNvYNCNvMs5_NtNtCs4NIexwV1FGX_4core2io5errorNtBa_11OsFunctions7DEFAULTs0_0INtNtNtBe_3ops8function6FnOnceTlEE9call_onceBe_ $_RNvXsQ_NtNtCs4NIexwV1FGX_4core3fmt3numlNtB7_5Debug3fmt $_RNvXsn_NtNtCs4NIexwV1FGX_4core2io5errorNtB5_9ErrorKindNtNtB9_3fmt5Debug3fmt $_RNvXs8_NtNtCs4NIexwV1FGX_4core3fmt8buildersINtB5_6FromFnNCNvXs2_NtNtB9_2io5errorNtNtB10_4repr4ReprNtB7_5Debug3fmt0EB1y_3fmtB9_ $_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtReNtB6_5Debug3fmtB8_)
  (data $.rodata (i32.const 1048576) "struct SomeDataStruct with 2 elements\00\00\00\00\00\10\00%\00\00\00\00\00\00\00\08\00\00\00\04\00\00\00\03\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00\04\00\00\00Io\00\00\00\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00InvalidUtf8Encoding\00\00\00\00\00\04\00\00\00\04\00\00\00\06\00\00\00InvalidBoolEncodingInvalidCharEncoding\00\00\00\00\00\00\04\00\00\00\04\00\00\00\07\00\00\00InvalidTagEncodingDeserializeAnyNotSupportedSizeLimitSequenceMustHaveLength\00\00\00\00\00\04\00\00\00\04\00\00\00\08\00\00\00Custom\00\00\00\00\00\00\04\00\00\00\04\00\00\00\09\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00\0a\00\00\00Utf8Errorvalid_up_toerror_lenNoneSome\00\00\00\0b\00\00\00\0c\00\00\00\04\00\00\00\0c\00\00\00\0b\00\00\00\0c\00\00\00\04\00\00\00\0d\00\00\00\0c\00\00\00l\01\10\00\0e\00\00\00\0f\00\00\00\10\00\00\00\0e\00\00\00\11\00\00\00\15byte range starts at \c0\0d but ends at \c0\00 index out of bounds: the len is \c0\12 but the index is \c0\00\11start byte index \c0' is out of bounds for string of length \c0\00\0fend byte index \c0' is out of bounds for string of length \c0\00\0finvalid length \c0\0b, expected \c0\00\c0\02: \c0\00/rustc/58cf8f9c35b355199727627d07ae34a4e2846d67/library/alloc/src/fmt.rs\00/rustc/58cf8f9c35b355199727627d07ae34a4e2846d67/library/std/src/sys/sync/rwlock/no_threads.rs\00/rustc/58cf8f9c35b355199727627d07ae34a4e2846d67/library/core/src/fmt/mod.rs\00/rustc/58cf8f9c35b355199727627d07ae34a4e2846d67/library/core/src/str/mod.rs\00/rustc/58cf8f9c35b355199727627d07ae34a4e2846d67/library/alloc/src/raw_vec/mod.rs\00/rust/deps/dlmalloc-0.2.14/src/dlmalloc.rs\00src/lib.rs\00\11start byte index \c0& is not a char boundary; it is inside \c0\08 (bytes \c0\0b of string)\00\0fend byte index \c0& is not a char boundary; it is inside \c0\08 (bytes \c0\0b of string)\00\0dInvalid size \c0\22: sizes must fit in a usize (0 to \c0\01)\00\01\22\c0\01\22\00description() is deprecated; use Display\00\ac,\b3+.t\16\0b\d0\12\bf\b9b\a7,\0f\12\00\00\00\04\00\00\00\04\00\00\00\13\00\00\00called `Result::unwrap()` on an `Err` value\00^\04\10\00\0a\00\00\00\17\00\00\00<\00\00\00^\04\10\00\0a\00\00\00\15\00\00\00G\00\00\00\ff\ff\ff\ff\16\00\00\00\0c\00\00\00\04\00\00\00\17\00\00\00\18\00\00\00\19\00\00\00\00\00\00\00\08\00\00\00\04\00\00\00\1a\00\00\00\1b\00\00\00\1c\00\00\00\1d\00\00\00\1e\00\00\00\10\00\00\00\04\00\00\00\1f\00\00\00 \00\00\00!\00\00\00\22\00\00\00\5c\f6\e9_\dc\02\f6\b9\f1\c1pl\f2a\c1$;7MJ\82\10E\09\c3\a5a\d1? :\8fassertion failed: psize >= size + min_overhead\00\003\04\10\00*\00\00\00\e9\04\00\00\09\00\00\00assertion failed: psize <= size + max_overhead\00\003\04\10\00*\00\00\00\ef\04\00\00\0d\00\00\003\04\10\00*\00\00\00M\05\00\00\1e\00\00\00rwlock overflowed read locks\ec\02\10\00]\00\00\00\18\00\00\00,\00\00\00\00\00\00\00\08\00\00\00\04\00\00\00#\00\00\00\16\00\00\00\0c\00\00\00\04\00\00\00$\00\00\00rwlock has not been locked for reading\00\00\ec\02\10\00]\00\00\00A\00\00\00\09\00\00\00capacity overflow\00\00\00\e2\03\10\00P\00\00\00\1c\00\00\00\05\00\00\00'\00\00\00\0c\00\00\00\04\00\00\00(\00\00\00)\00\00\00*\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00+\00\00\00a formatting trait implementation returned an error when the underlying stream did not\00\00\a3\02\10\00H\00\00\00\91\02\00\00\0e\00\00\00Error\02\02\02\02\02\02\02\02\02\02\02\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\02\00\00\00\00\00\02\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\06\04\07\01\01\01\14\01\00\01&\022\02\03\017\08\1b\04\06\0b\00\01<\02e\0e;\021\02\0f\01\1c\02\01\01\0b\05\22\05\ed\01\08\02\02\02\16\01\07\01\01\03\04\02\09\02\02\02\04\08\01\04\02\01\05\02\19\02\03\01\06\04\02\02\16\01\07\01\02\01\02\01\02\02\01\01\05\04\02\02\03\03\01\07\04\01\01\07\11\0a\03\01\09\01\03\01\16\01\07\01\02\01\05\02\0a\01\03\01\03\02\01\0f\04\02\0c\07\07\01\03\01\08\02\02\02\16\01\07\01\02\01\05\02\09\02\02\02\03\07\03\04\02\01\05\02\12\0a\02\01\06\03\03\01\04\03\02\01\01\01\02\03\02\03\03\03\0c\04\05\03\03\01\04\02\01\06\01\0e\15\05\0d\01\03\01\17\01\10\02\09\01\03\01\04\07\02\01\03\01\02\02\04\02\0a\07\16\01\03\01\17\01\0a\01\05\02\09\01\03\01\04\07\02\05\03\01\04\02\0a\01\03\0c\0d\01\03\013\01\03\01\06\04\10\02\1a\01\03\01\12\03\18\01\09\01\01\02\07\03\01\04\06\01\01\01\08\06\0a\02\03\0c:\04\1d%\02\01\01\01\05\01\18\01\01\01\17\02\05\01\01\01\07\01\0a\02\04 H\01$\04'\01$\01\0f\01\0d%\c6\01\01\05\01\02\00\01\04\02\07\01\01\01\04\02)\01\04\02!\01\04\02\07\01\01\01\04\02\0f\019\01\04\02C\02 \03\1a\06V\02\06\02\00\03Y\07\16\09\18\09\14\0c\0d\01\03\01\02\0c^\02\0a\06\0a\06\1a\06Y\07+\05F\0a\1f\01\0c\04\0c\04\01\03*\02\05\0b,\04\1a\06\0b\03>\02A\01\1d\02\0b\06\0a\06\0e\02.\02\0c\14M\01\a6\08<\03\0f\03>\05+\02\0b\08+\05\00\02\06\02&\02\06\02\08\01\01\01\01\01\01\01\1f\025\01\0f\01\0e\02\06\01\13\02\03\01\09\01e\01\0c\02\1b\01\0d\03\22\0e!\0f\8c\04\00\16\0b\15\00\02\00\05-\01\01\05\01\028\07\02\0e\18\09\07\01\07\01\07\01\07\01\07\01\07\01\07\01\07\01~\22\1a\01Y\0c\d6\1aP\01V\02g\05+\01^\01V\090\01\00\037\09\00\14\b8\08\dd\14<\03\0a\068\08F\08\0c\06t\0b\1e\03N\01\0b\04!\017\09\0e\02\0a\02g\18\1c\0a\06\02\06\02\06\09\07\01\07\01<\04~\02\0a\06\00\0c\17\041\04\00\02j&\07\0c\05\05\1a\01\05\01\01\01\02\01\02\01\00 *\063\01\13\01\04\04\05\01\87\02\01\01\be\03\06\02\06\02\06\02\03\03\07\01\07\0a\05\02\0c\01\1a\01\13\01\02\01\0f\02\0e\22{\05\03\04-\03X\01\0d\03\01/.\82\1d\031\0f\1c\04$\09\1e\05+\05\1e\01%\04\0e*\9e\02\0a\06$\04$\04(\084\0b\0c\01\0f\01\07\01\02\01\0b\01\0f\01\07\01\02\034\0c\00\09\16\0a\08\18\06\01*\01\09E\06\02\01\01,\01\02\03\01\02\17\01H\08\090\13\01\02\05!\03\1b\05\1b&8\04\14\022\01\02\05\08\01\03\01\1d\02\03\04\0a\07\09\07@ '\04\0c\096\03\1d\02\1b\05\1a\07\04\0c\07PI73\0d3\07.\08\0a\06&\03\1d\08\02\d0\1f\01*\01\03\02\02\10\06\08\09!.\08*\16\1a&\1c\14\17\09N\04$\09D\0a\01\02\19\07\0a\065\01\12\08'\09`\01\14\0b\12\01/>\07\01\01\01\04\01\0f\01\0b\06;\05\0a\06\04\01\08\02\02\02\16\01\07\01\02\01\05\01\0a\02\02\02\03\02\01\06\01\05\07\02\07\03\05\0b\0a\01\01\02\01\01&\01\0a\01\01\02\01\01\04\01\0a\01\02\08\02\1d\5c\01\05\1eH\08\0a\a66\02&\22E\0b\0a\06\0d\13:\06\0a\06\14\1c\1b\02\0f\04\17\b9<dS\0c\08\02\01\02\08\01\02\01\1e\01\02\02\0c\09\0aF\08\02.\02\0b\1bH\08S\0dI\07\0aV\08X\22\0e\0a\06\09\01-\01\0e\0a\1d\03 \02\16\01\0eI\07\01\02\01,\03\01\01\02\01\09\08\0a\06\06\01\02\01%\01\02\01\06\07\0a\06,\04\0a\f6\19\07\11\01)\03\1dU\01\0f2\0d\00fo\01\05\0b\c4\00c\0d\00\0a\00\05\00\00:\00\00\07\1f\01\0a\04Q\01\0a\06\1e\02\06\0aF\0a\0a\01\07\01\15\05\13\00:\c6[\05\19\02\19,K\049\07\11@\05\0b\07\09\00) as\00\04\01\07\01\02\01\00\0f\01\1d\03\02\01\0e\04\08\00\00k\05\0d\03\09\07\0a\02\08\00\fd\03\00\06\17\0f\11\0f.\02\17\09t<\f6\0a'\02\c2\15Fz\14\0c\14\0cW\09\19\87U\01G\01\02\02\01\02\02\02\04\01\0c\01\01\01\07\01A\01\04\02\08\01\07\01\1c\01\04\01\05\01\01\03\07\01\00\02\00\02\00\0f\05\01\0f\00\1f\06\06\d5\07\01\11\02\07\01\02\01\05\05>!\01p-\03\0e\02\0a\04\02\00\1f\11:\05\01\00*\d6+\04\01\c0\1f\01\16\08\02\e0\07\01\04\01\02\01\0f\01\c5\02\10)L\04\0a\04\02\00DL=\c2\04\01\1b\01\02\01\01\02\01\01\0a\01\04\01\01\01\01\06\01\04\01\01\01\01\01\01\03\01\02\01\01\02\01\01\01\01\01\01\01\01\01\01\02\01\01\02\04\01\07\01\04\01\04\01\01\01\0a\01\11\05\03\01\05\01\114\02\00,\04d\0c\0f\02\0f\01\0f\01%\0a\ae8\1d\0d,\04\09\07\02\0e\06\9a\00\03\11\03\0d\03\da\06\0c\04\01\0f\0c\048\08\0a\06(\08\1e\02\0c\04\02\0e\09'\00\08\0e\02\0d\03\0b\039\01\01\04\10\02\0c\04\0a\07\93\01g\00\00 \00\02\00\02\00\0f\00\00\00\00\00\05\00\00\00\00p\00\07\00-\01\01\01\02\01\02\01\01H\0b0\15\10\01e\07\02\06\02\02\01\04#\01\1e\1b[\0b:\09\09\01\18\04\01\09\01\03\01\05+\03;\09*\18\01 7\01\01\01\04\08\04\01\03\07\0a\02\1d\01:\01\01\01\02\04\08\01\09\01\0a\02\1a\01\02\029\01\04\02\04\02\02\03\03\01\1e\02\03\01\0b\029\01\04\05\01\02\04\01\14\02\16\06\01\01:\01\01\02\01\04\08\01\07\03\0a\02\1e\01;\01\01\01\0c\01\09\01(\01\03\017\01\01\03\05\03\01\04\07\02\0b\02\1d\01:\01\02\02\01\01\03\03\01\04\07\02\0b\02\1c\029\02\01\01\02\04\08\01\09\01\0a\02\1d\01H\01\04\01\02\03\01\01\08\01Q\01\02\07\0c\08b\01\02\09\0b\07I\02\1b\01\01\01\01\017\0e\01\05\01\02\05\0b\01$\09\01f\04\01\06\01\02\02\02\19\02\04\03\10\04\0d\01\02\02\06\01\0f\01\00\03\00\04\1c\03\1d\02\1e\02@\02\01\07\08\01\02\0b\09\01-\03\01\01u\02\22\01v\03\04\02\09\01\06\03\db\02\02\01:\01\01\07\01\01\01\01\02\08\06\0a\02\010.\02\0c\14\040\0a\04\03&\09\0c\02 \04\02\068\01\01\02\03\01\01\058\08\02\02\98\03\01\0d\01\07\04\01\06\01\03\02\c6@\00\01\c3!\00\03\8d\01` \00\06i\02\00\04\01\0a \02P\02\00\01\03\01\04\01\19\02\05\01\97\02\1a\12\0d\01&\08\19\0b\01\01,\030\01\02\04\02\02\02\01$\01C\06\02\02\02\02\0c\01\08\01/\013\01\01\03\02\02\05\02\01\01*\02\08\01\ee\01\02\01\04\01\00\01\00\10\10\10\00\02\00\01\e2\01\95\05\00\03\01\02\05\04(\03\04\01\a5\02\00\04A\05\00\02M\06F\0b1\04{\016\0f)\01\02\02\0a\031\04\02\02\07\01=\03$\05\01\08>\01\0c\024\09\01\01\08\04\02\01_\03\02\04\06\01\02\01\9d\01\03\08\15\029\02\01\01\01\01\0c\01\09\01\0e\07\03\05C\01\02\06\01\01\02\01\01\03\04\03\01\01\0e\02U\08\02\03\01\01\17\01Q\01\02\06\01\01\02\01\01\02\01\02\eb\01\02\04\06\02\01\02\1b\02U\08\02\01\01\02j\01\01\01\02\08e\01\01\01\02\04\01\05\00\09\01\02\f5\01\0a\04\04\01\90\04\02\02\04\01 \0a(\06\02\04\08\01\09\06\02\03.\0d\01\02\c6\01\01\03\01\01\c9\07\01\06\01\01R\16\02\07\01\02\01\02z\06\03\01\01\02\01\07\01\01H\02\03\01\01\01\00\02\0b\024\05\05\03\17\01\00\01\06\0f\00\0c\03\03\00\05;\07\00\01?\04Q\01\0b\02\00\02\00.\02\17\00\05\03\06\08\08\02\07\1e\04\94\03\007\042\08\01\0e\01\16\05\01\0f\00\07\01\11\02\07\01\02\01\05d\01\a0\07\00\01=\04\00\04\fe\02\f3\01\02\01\07\02\05\01\00\07m\07\00`\80\f0\00\ad\01\00\01\00\01\00\02\00\02U\05\00\05\1a\051\10\00\01\00\10\ef\01\a0\01O\09\00\04\00\08\00\00\00\ad\01\00\06\16\01\c0\011\01\00\02P\01\00\01\00\05\1a\051\05\01\0a\00\01\f9\03\00\01\0f\01\00\10\00\04\00\08\00\01\1e`\000123456789abcdef} { , :  {\0a,\0a)((\0a, }\00\00\00\00\00\0c\00\00\00\04\00\00\001\00\00\002\00\00\003\00\00\00\96\03\10\00K\00\00\00\e6\01\00\00\14\00\00\00called `Option::unwrap()` on a `None` value00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\04\04\04\04\04\00\00\00\00\00\00\00\00\00\00\00attempt to divide by zero4\00\00\005\00\00\006\00\00\00x\03\00\000\05 \00\0e\07`\01I\12 \03\9d\16\a0(\16\1f\a0-*$\a07t+`=\f4,\e0=\8d\a4 >,\a6\a0D\a4\d7 En\fa\a0K\d0\fd`L7\07\a1N\9a#\e1[\90/a\85V4A\86\fbC\a1\86GF\e1\86\00a!\87\00hA\879j\81\87@m\a1\87\d6\8c\81\8a\f0\af\e1\8c#\b1\81\8d\fc\b2a\8e\00\bc\a1\8f\00\cc\c1\8f\b4\ce\01\91\a6\d6a\91\cc\d7a\99\8c\da\a1\99\00\df\e1\99\90\e2\81\9a\d0\e4\c1\9dq\ec\81\9e\00\f0A\a2\d9\f6A\abX\faa\ae\00\00\e2\b1\e0\a6\82\b4\1e\b8\a2\b4\ae\ce\e2\b4\e1\eb\22\b5^\eeb\b5\00\f8\a2\b5\1e\fa\c2\b5\00\00\e3\b5K\13\03\b6z4#\b6\fe\ffc\b6\fe\ff\94\b6\00\03\00\00\83\04 \00\91\05`\00]\13\a0\00\12\17 \1f\0c `\1f\ef,`+*0\e0+o\a6\a0,\02\a8 -\1e\fb .\00\fe`6\9e\ff\a06\fd\01!7\01\0aa7$\0d!8\ab\0e\a19/\18!:\f3\1e!K@4\a1S\1ea\e1T\f0jaUOo\e1U\9d\bcaV\00\cfaWe\d1\a1W\00\da!X\00\e0\a1Y\ae\e2![\ec\e4\e1\5c\d0\e8a] \00\ee^\f0\01\7f_O\03\00\00\1c\06`\00_\11\a0\00\b4\17\e0\00\0b  \01d1\a0\01\00\fe`\02\a0\bc\a1\02s\d1\a1\03\00\00\ee\03\00\10.\04\00\10_\04\00\06\00\00\90\08`\00\0e\18`\01\0b \e0\01\ff\fe \02\bd\10!\0304\a1\03\a0\bc!\04s\d1a\04\01\00\ae\04\80\00\ff\04..    Os\00\00\00\00\04\00\00\00\04\00\00\007\00\00\00code\00\00\00\00\01\00\00\00\01\00\00\008\00\00\00kind\00\00\00\00\04\00\00\00\04\00\00\009\00\00\00messageKindError\00\00\00\00\08\00\00\00\04\00\00\00:\00\00\000xCustomerror0123456789ABCDEF\00\00\00J\03\10\00K\00\00\00\85\0b\00\00&\00\00\00J\03\10\00K\00\00\00\8e\0b\00\00\1a\00\00\00NotFoundPermissionDeniedConnectionRefusedConnectionResetHostUnreachableNetworkUnreachableConnectionAbortedNotConnectedAddrInUseAddrNotAvailableNetworkDownBrokenPipeAlreadyExistsWouldBlockNotADirectoryIsADirectoryDirectoryNotEmptyReadOnlyFilesystemFilesystemLoopStaleNetworkFileHandleInvalidInputInvalidDataTimedOutWriteZeroStorageFullNotSeekableQuotaExceededFileTooLargeResourceBusyExecutableFileBusyDeadlockCrossesDevicesTooManyLinksInvalidFilenameArgumentListTooLongInterruptedUnsupportedUnexpectedEofOutOfMemoryInProgressTooManyOpenFilesInputOutputErrorOtherUncategorized\00\00\08\10\11\0f\0f\12\11\0c\09\10\0b\0a\0d\0a\0d\0c\11\12\0e\16\0c\0b\08\09\0b\0b\0d\0c\0c\12\08\0e\0c\0f\13\0b\0b\0d\0b\0a\10\10\05\0d\dc\16\10\00\e4\16\10\00\f4\16\10\00\05\17\10\00\14\17\10\00#\17\10\005\17\10\00F\17\10\00R\17\10\00[\17\10\00k\17\10\00v\17\10\00\80\17\10\00\8d\17\10\00\97\17\10\00\a4\17\10\00\b0\17\10\00\c1\17\10\00\d3\17\10\00\e1\17\10\00\f7\17\10\00\03\18\10\00\0e\18\10\00\16\18\10\00\1f\18\10\00*\18\10\005\18\10\00B\18\10\00N\18\10\00Z\18\10\00l\18\10\00t\18\10\00\82\18\10\00\8e\18\10\00\9d\18\10\00\b0\18\10\00\bb\18\10\00\c6\18\10\00\d3\18\10\00\de\18\10\00\e8\18\10\00\f8\18\10\00\08\19\10\00\0d\19\10\00")
  (data $.data (i32.const 1055224) "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\ff\0f\00\00x\14\10\00")
  (@custom "name" "\00\0e\0dwasm_lib.wasm\01\fee\90\01\00m_RINvMNtNtCs5JHfeWeQK0x_5alloc2io5errorNtNtNtCs4NIexwV1FGX_4core2io5error5Error3newReECshuXg64jKSg3_8wasm_lib\01\8e\01_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueINtNtCs5JHfeWeQK0x_5alloc5boxed3BoxNtNtCs1NJkdcuIVXo_7bincode5error9ErrorKindEECshuXg64jKSg3_8wasm_lib\02\eb\01_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBL_3BoxDNtNtB4_5error5ErrorNtNtB4_6marker4SendNtB1R_4SyncEL_EINtNtB4_7convert4FromNtNtBN_6string6StringE4from11StringErrorECshuXg64jKSg3_8wasm_lib\03\89\01_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshuXg64jKSg3_8wasm_lib\04\5c_RNvMs5_NtCs5JHfeWeQK0x_5alloc7raw_vecNtB5_11RawVecInner11finish_growCshuXg64jKSg3_8wasm_lib\05\c3\03_RINvYINtNvXs1_NtCs1NJkdcuIVXo_7bincode2deQINtBc_12DeserializerppENtNtCs8n3ERUNuD4B_10serde_core2de12Deserializer17deserialize_tuple6AccessNtNtBc_4read11SliceReaderINtNtBe_6config14WithOtherLimitINtB2E_17WithOtherTrailingINtB2E_20WithOtherIntEncodingNtB2E_14DefaultOptionsNtNtB2E_3int14FixintEncodingENtNtB2E_8trailing13AllowTrailingENtNtB2E_5limit8InfiniteEENtB13_9SeqAccess12next_elementNtNtCs5JHfeWeQK0x_5alloc6string6StringECshuXg64jKSg3_8wasm_lib\06i_RNvMs4_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB5_6RawVecNtNtB7_6string6StringE8grow_oneCshuXg64jKSg3_8wasm_lib\07a_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRINtNtB8_6option6OptionhENtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib\08o_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRNtNtCs5JHfeWeQK0x_5alloc6string6StringNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib\09a_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRNtNtNtB8_2io5error5ErrorNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib\0af_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRNtNtNtB8_3str5error9Utf8ErrorNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib\0bJ_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRhNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib\0cJ_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRjNtB6_5Debug3fmtCshuXg64jKSg3_8wasm_lib\0dy_RNvXs1i_NtCs4NIexwV1FGX_4core3fmtRDNtNtCs8n3ERUNuD4B_10serde_core2de8ExpectedEL_NtB6_7Display3fmtCshuXg64jKSg3_8wasm_lib\0e7_RNvXsZ_NtNtCs4NIexwV1FGX_4core3fmt3numjNtB7_5Debug3fmt\0f\95\01_RNvXsn_NtCs5JHfeWeQK0x_5alloc5boxedINtB5_3BoxNtNtCs1NJkdcuIVXo_7bincode5error9ErrorKindENtNtCs4NIexwV1FGX_4core3fmt5Debug3fmtCshuXg64jKSg3_8wasm_lib\10\a1\01_RNvYINtNtCs5JHfeWeQK0x_5alloc5boxed3BoxNtNtCs1NJkdcuIVXo_7bincode5error9ErrorKindENtNtCs8n3ERUNuD4B_10serde_core2de5Error14invalid_lengthCshuXg64jKSg3_8wasm_lib\11\e9\01_RNvYNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4from11StringErrorBX_11descriptionCshuXg64jKSg3_8wasm_lib\12\e2\01_RNvYNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4from11StringErrorBX_5causeCshuXg64jKSg3_8wasm_lib\13\e4\01_RNvYNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4from11StringErrorBX_7provideCshuXg64jKSg3_8wasm_lib\14\e4\01_RNvYNtNvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4from11StringErrorBX_7type_idCshuXg64jKSg3_8wasm_lib\15\0bentry_point\16\07ns_free\17\09ns_malloc\18*_RNvCs5fGHjzWXRav_7___rustc12___rust_alloc\19,_RNvCs5fGHjzWXRav_7___rustc14___rust_dealloc\1a,_RNvCs5fGHjzWXRav_7___rustc14___rust_realloc\1bA_RNvCs5fGHjzWXRav_7___rustc35___rust_no_alloc_shim_is_unstable_v2\1c<_RNvNtNtCs1NJkdcuIVXo_7bincode6config3int17cast_u64_to_usize\1d\9c\01_RNvXs_NtCs1NJkdcuIVXo_7bincode5errorINtNtCs5JHfeWeQK0x_5alloc5boxed3BoxNtB4_9ErrorKindEINtNtCs4NIexwV1FGX_4core7convert4FromNtNtNtB1s_2io5error5ErrorE4from\1e;_RNvXs0_NtCs8n3ERUNuD4B_10serde_core2deReNtB5_8Expected3fmt\1f`_RNvXs1_NtCs8n3ERUNuD4B_10serde_core2deDNtB5_8ExpectedEL_NtNtCs4NIexwV1FGX_4core3fmt7Display3fmt 0_RNvCs5fGHjzWXRav_7___rustc18___rust_start_panic!a_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueNtNtCs5JHfeWeQK0x_5alloc6string6StringECsiuSwmoQr08c_3std\22r_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueNtNvNtCsiuSwmoQr08c_3std9panicking13panic_handler19FormatStringPayloadEBH_#\84\01_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsiuSwmoQr08c_3std$d_RINvNtNtCsiuSwmoQr08c_3std3sys9backtrace26___rust_end_short_backtraceNCNvNtB6_5alloc8rust_oom0zEB6_%-_RNCNvNtCsiuSwmoQr08c_3std5alloc8rust_oom0B5_&n_RINvNtNtCsiuSwmoQr08c_3std3sys9backtrace26___rust_end_short_backtraceNCNvNtB6_9panicking13panic_handler0zEB6_'7_RNCNvNtCsiuSwmoQr08c_3std9panicking13panic_handler0B5_(W_RNvMs5_NtCs5JHfeWeQK0x_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsiuSwmoQr08c_3std)3_RNvNtCsiuSwmoQr08c_3std9panicking15panic_with_hook*8_RNvNtCsiuSwmoQr08c_3std5alloc24default_alloc_error_hook+'_RNvCs5fGHjzWXRav_7___rustc10rust_panic,)_RNvCs5fGHjzWXRav_7___rustc11___rdl_alloc-g_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE8memalignCsiuSwmoQr08c_3std.e_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE6mallocCsiuSwmoQr08c_3std/*_RNvCs5fGHjzWXRav_7___rustc12___rust_abort0+_RNvCs5fGHjzWXRav_7___rustc13___rdl_dealloc1c_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE4freeCsiuSwmoQr08c_3std2+_RNvCs5fGHjzWXRav_7___rustc13___rdl_realloc3l_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE12unlink_chunkCsiuSwmoQr08c_3std4m_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE13dispose_chunkCsiuSwmoQr08c_3std5._RNvCs5fGHjzWXRav_7___rustc17rust_begin_unwind68_RNvCs5fGHjzWXRav_7___rustc26___rust_alloc_error_handler7'_RNvNtCsiuSwmoQr08c_3std5alloc8rust_oom8r_RNvMs0_NtCsbw64Iw7W9Hn_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE18insert_large_chunkCsiuSwmoQr08c_3std9:_RNvNtNtCsiuSwmoQr08c_3std9panicking11panic_count8increase:g_RNvXNtCs4NIexwV1FGX_4core3anyNtNtCs5JHfeWeQK0x_5alloc6string6StringNtB2_3Any7type_idCsiuSwmoQr08c_3std;C_RNvXNtCs4NIexwV1FGX_4core3anyReNtB2_3Any7type_idCsiuSwmoQr08c_3std<x_RNvXs0_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_19FormatStringPayloadNtNtCs4NIexwV1FGX_4core3fmt7Display3fmt=}_RNvXs1_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload3get>\80\01_RNvXs1_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload6as_str?\82\01_RNvXs1_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload8take_box@u_RNvXs2_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCs4NIexwV1FGX_4core3fmt7Display3fmtA^_RNvXsZ_NtCs5JHfeWeQK0x_5alloc6stringNtB5_6StringNtNtCs4NIexwV1FGX_4core3fmt5Write10write_charB\5c_RNvXsZ_NtCs5JHfeWeQK0x_5alloc6stringNtB5_6StringNtNtCs4NIexwV1FGX_4core3fmt5Write9write_strC\7f_RNvXs_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB4_19FormatStringPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload3getD\84\01_RNvXs_NvNtCsiuSwmoQr08c_3std9panicking13panic_handlerNtB4_19FormatStringPayloadNtNtCs4NIexwV1FGX_4core5panic12PanicPayload8take_boxEu_RNvYINtNvNtCsiuSwmoQr08c_3std9panicking11begin_panic7PayloadReENtNtCs4NIexwV1FGX_4core5panic12PanicPayload6as_strB9_Fh_RNvYNtNtCs5JHfeWeQK0x_5alloc6string6StringNtNtCs4NIexwV1FGX_4core3fmt5Write9write_fmtCsiuSwmoQr08c_3stdGE_RNvXs_NtCsbw64Iw7W9Hn_8dlmalloc3sysNtB4_6SystemNtB6_9Allocator5allocHR_RINvNtCs4NIexwV1FGX_4core3ptr9drop_glueNtNtCs5JHfeWeQK0x_5alloc6string6StringEBF_Iu_RINvNvMs2_NtCs5JHfeWeQK0x_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalEBa_J0_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec12handle_errorKH_RNvMs5_NtCs5JHfeWeQK0x_5alloc7raw_vecNtB5_11RawVecInner11finish_growB7_L\97\01_RINvNvNtNtCs5JHfeWeQK0x_5alloc2io5error21custom_owner_from_box12drop_box_rawDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB1h_6marker4SendNtB1O_4SyncEL_EB8_Mz_RINvNvNtNtCs5JHfeWeQK0x_5alloc2io5error21custom_owner_from_box12drop_box_rawNtNtNtCs4NIexwV1FGX_4core2io5error6CustomEB8_N4_RNvNtCs5JHfeWeQK0x_5alloc5alloc18handle_alloc_errorO5_RNvNtCs5JHfeWeQK0x_5alloc7raw_vec17capacity_overflowP<_RNvNtNtCs5JHfeWeQK0x_5alloc2io5error21custom_owner_from_boxQ5_RNvNvNtCs5JHfeWeQK0x_5alloc3fmt6format12format_innerR\de\01_RNvXs0_NvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBd_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB12_6marker4SendNtB1z_4SyncEL_EINtNtB12_7convert4FromNtNtBf_6string6StringE4fromNtB5_11StringErrorNtNtB12_3fmt5Debug3fmtS;_RNvXsK_NtCs4NIexwV1FGX_4core3fmtNtB5_5ErrorNtB5_5Debug3fmtT^_RNvXsZ_NtCs5JHfeWeQK0x_5alloc6stringNtB5_6StringNtNtCs4NIexwV1FGX_4core3fmt5Write10write_charU\5c_RNvXsZ_NtCs5JHfeWeQK0x_5alloc6stringNtB5_6StringNtNtCs4NIexwV1FGX_4core3fmt5Write9write_strV\df\01_RNvXs_NvXsf_NtNtCs5JHfeWeQK0x_5alloc5boxed7convertINtBc_3BoxDNtNtCs4NIexwV1FGX_4core5error5ErrorNtNtB11_6marker4SendNtB1y_4SyncEL_EINtNtB11_7convert4FromNtNtBe_6string6StringE4fromNtB4_11StringErrorNtNtB11_3fmt7Display3fmtWY_RNvYNtNtCs5JHfeWeQK0x_5alloc6string6StringNtNtCs4NIexwV1FGX_4core3fmt5Write9write_fmtB6_X)_RNvNtCs4NIexwV1FGX_4core9panicking5panicY?_RNvXsd_NtNtNtCs4NIexwV1FGX_4core3fmt3num3impyNtB9_7Display3fmtZ8_RNvXs1i_NtCs4NIexwV1FGX_4core3fmtReNtB6_7Display3fmtB8_[#_RNvNtCs4NIexwV1FGX_4core3fmt5write\5c-_RNvNtCs4NIexwV1FGX_4core9panicking9panic_fmt]<_RNvMNtNtCs4NIexwV1FGX_4core4char7methodsc16escape_debug_ext^2_RNvNtNtCs4NIexwV1FGX_4core3str8converts9from_utf8_7_RNvMNtNtCs4NIexwV1FGX_4core4char7methodsc11is_assigned`O_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data13cn_planes_0_311lookup_slowaG_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data11white_space6lookupbQ_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data15grapheme_extend11lookup_slowc^_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data28default_ignorable_code_point11lookup_slowdC_RNvNtNtNtCs4NIexwV1FGX_4core7unicode12unicode_data2cf11lookup_slowe7_RNvNtCs4NIexwV1FGX_4core9panicking18panic_bounds_checkfR_RNvXs0_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write9write_strgD_RNvMs2_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_11DebugStruct5fieldh>_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter12pad_integrali5_RNvNtNtCs4NIexwV1FGX_4core3str5count14do_count_charsjN_RNvNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB7_9Formatter12pad_integral12write_prefixkK_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter25debug_tuple_field1_finishlL_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter26debug_struct_field2_finishm4_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter3padn:_RNvMsa_NtCs4NIexwV1FGX_4core3fmtNtB5_9Formatter9write_stro?_RNvXs8_NtNtNtCs4NIexwV1FGX_4core3fmt3num3impmNtB9_7Display3fmtp/_RNvNtCs4NIexwV1FGX_4core3str16slice_error_failq2_RNvNtCs4NIexwV1FGX_4core3str19slice_error_fail_rtrN_RNvXs_NtNtCs4NIexwV1FGX_4core3ops5rangeINtB4_5RangejENtNtB8_3fmt5Debug3fmtB8_s1_RNvXsj_NtCs4NIexwV1FGX_4core3fmtcNtB5_5Debug3fmtt/_RNvNtCs4NIexwV1FGX_4core6option13unwrap_failedu/_RNvNtCs4NIexwV1FGX_4core6option13expect_failedv/_RNvNtCs4NIexwV1FGX_4core6result13unwrap_failedw<_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtRDNtB6_5DebugEL_Bx_3fmtB8_x:_RNvXs6_NtNtCs4NIexwV1FGX_4core3fmt3numjNtB7_8LowerHex3fmtyK_RNvNtNtCs4NIexwV1FGX_4core9panicking11panic_const23panic_const_div_by_zerozE_RNvXNtNtCs4NIexwV1FGX_4core2io5errorNtB2_5ErrorNtNtB6_3fmt5Debug3fmt{N_RNvXs2_NtNtCs4NIexwV1FGX_4core2io5errorNtNtB5_4repr4ReprNtNtB9_3fmt5Debug3fmt|<_RNvXNtNtNtCs4NIexwV1FGX_4core3fmt3num3imphNtB6_7Display3fmt}T_RNvXs0_NtNtCs4NIexwV1FGX_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write10write_char~:_RNvXsg_NtNtCs4NIexwV1FGX_4core3fmt3numhNtB7_8UpperHex3fmt\7f?_RNvXs9_NtNtNtCs4NIexwV1FGX_4core3fmt3num3implNtB9_7Display3fmt\80\016_RNvXs1g_NtCs4NIexwV1FGX_4core3fmtReNtB6_5Debug3fmtB8_\81\011_RNvXsh_NtCs4NIexwV1FGX_4core3fmteNtB5_5Debug3fmt\82\01I_RNvXs8_NtNtCs4NIexwV1FGX_4core2io5errorNtB5_6CustomNtNtB9_3fmt5Debug3fmt\83\01\8b\01_RNvXs9_NtNtCs4NIexwV1FGX_4core3fmt8buildersINtB5_6FromFnNCNCNvXs2_NtNtB9_2io5errorNtNtB12_4repr4ReprNtB7_5Debug3fmt00ENtB7_7Display3fmtB9_\84\01:_RNvXse_NtNtCs4NIexwV1FGX_4core3fmt3numhNtB7_8LowerHex3fmt\85\01:_RNvXs8_NtNtCs4NIexwV1FGX_4core3fmt3numjNtB7_8UpperHex3fmt\86\01\7f_RNvXs8_NtNtCs4NIexwV1FGX_4core3fmt8buildersINtB5_6FromFnNCNvXs2_NtNtB9_2io5errorNtNtB10_4repr4ReprNtB7_5Debug3fmt0EB1y_3fmtB9_\87\01P_RNvXs9_NtNtCs4NIexwV1FGX_4core2io5errorNtB5_6CustomNtNtNtB9_3ops4drop4Drop4drop\88\017_RNvXsQ_NtNtCs4NIexwV1FGX_4core3fmt3numlNtB7_5Debug3fmt\89\01V_RNvXsd_NtNtCs4NIexwV1FGX_4core2io5errorNtB5_11CustomOwnerNtNtNtB9_3ops4drop4Drop4drop\8a\013_RNvXsi_NtCs4NIexwV1FGX_4core3fmteNtB5_7Display3fmt\8b\01L_RNvXsn_NtNtCs4NIexwV1FGX_4core2io5errorNtB5_9ErrorKindNtNtB9_3fmt5Debug3fmt\8c\01\8d\01_RNvYNCNvMs5_NtNtCs4NIexwV1FGX_4core2io5errorNtBa_11OsFunctions7DEFAULT0INtNtNtBe_3ops8function6FnOnceTlQNtNtBe_3fmt9FormatterEE9call_onceBe_\8d\01z_RNvYNCNvMs5_NtNtCs4NIexwV1FGX_4core2io5errorNtBa_11OsFunctions7DEFAULTs0_0INtNtNtBe_3ops8function6FnOnceTlEE9call_onceBe_\8e\01y_RNvYNCNvMs5_NtNtCs4NIexwV1FGX_4core2io5errorNtBa_11OsFunctions7DEFAULTs_0INtNtNtBe_3ops8function6FnOnceTlEE9call_onceBe_\8f\01O_RNvYNtNtNtCs4NIexwV1FGX_4core3fmt8builders10PadAdapterNtB6_5Write9write_fmtB8_\07\12\01\00\0f__stack_pointer\09\11\02\00\07.rodata\01\05.data")
  (@custom "producers" "\02\08language\01\04Rust\00\0cprocessed-by\01\05rustc\1d1.99.1 (58cf8f9c3 2026-10-13)")
  (@custom "target_features" "\08+\0bbulk-memory+\0fbulk-memory-opt+\16call-indirect-overlong+\0amultivalue+\0fmutable-globals+\13nontrapping-fptoint+\0freference-types+\08sign-ext"))
