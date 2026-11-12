(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (param i32 i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32)))
  (type (;4;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;5;) (func))
  (type (;6;) (func (param i32)))
  (type (;7;) (func (param i32) (result i32)))
  (type (;8;) (func (param i32 i32 i32 i32 i32)))
  (type (;9;) (func (param i32 i32) (result i32 i32)))
  (type (;10;) (func (result i32 i32)))
  (import "./rust_interface_bg.js" "__wbindgen_init_externref_table" (func (;0;) (type 5)))
  (func (;1;) (type 7) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 8
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.const 245
          i32.ge_u
          if  ;; label = @4
            local.get 0
            i32.const -65588
            i32.gt_u
            if  ;; label = @5
              i32.const 0
              local.set 0
              br 4 (;@1;)
            end
            local.get 0
            i32.const 11
            i32.add
            local.tee 2
            i32.const -8
            i32.and
            local.set 5
            i32.const 1051748
            i32.load
            local.tee 9
            i32.eqz
            br_if 2 (;@2;)
            i32.const 31
            local.set 6
            local.get 0
            i32.const 16777205
            i32.ge_u
            br_if 1 (;@3;)
            local.get 5
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
            i32.sub
            i32.const 62
            i32.add
            local.set 6
            br 1 (;@3;)
          end
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 1051744
                    i32.load
                    local.tee 4
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
                    local.tee 5
                    i32.const 3
                    i32.shr_u
                    local.tee 0
                    i32.shr_u
                    local.tee 1
                    i32.const 3
                    i32.and
                    if  ;; label = @9
                      local.get 1
                      i32.const -1
                      i32.xor
                      i32.const 1
                      i32.and
                      local.get 0
                      i32.add
                      local.tee 7
                      i32.const 3
                      i32.shl
                      local.tee 1
                      i32.const 1051480
                      i32.add
                      local.tee 0
                      local.get 1
                      i32.const 1051488
                      i32.add
                      i32.load
                      local.tee 2
                      i32.load offset=8
                      local.tee 3
                      i32.eq
                      br_if 1 (;@8;)
                      local.get 3
                      local.get 0
                      i32.store offset=12
                      local.get 0
                      local.get 3
                      i32.store offset=8
                      br 2 (;@7;)
                    end
                    local.get 5
                    i32.const 1051752
                    i32.load
                    i32.le_u
                    br_if 6 (;@2;)
                    local.get 1
                    br_if 2 (;@6;)
                    i32.const 1051748
                    i32.load
                    local.tee 0
                    i32.eqz
                    br_if 6 (;@2;)
                    local.get 0
                    i32.ctz
                    i32.const 2
                    i32.shl
                    i32.const 1051336
                    i32.add
                    i32.load
                    local.tee 1
                    i32.load offset=4
                    i32.const -8
                    i32.and
                    local.get 5
                    i32.sub
                    local.set 3
                    local.get 1
                    local.set 2
                    loop  ;; label = @9
                      block  ;; label = @10
                        local.get 1
                        i32.load offset=16
                        local.tee 0
                        br_if 0 (;@10;)
                        local.get 1
                        i32.load offset=20
                        local.tee 0
                        br_if 0 (;@10;)
                        local.get 2
                        i32.load offset=24
                        local.set 6
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 2
                            local.get 2
                            i32.load offset=12
                            local.tee 0
                            i32.eq
                            if  ;; label = @13
                              local.get 2
                              i32.const 20
                              i32.const 16
                              local.get 2
                              i32.load offset=20
                              local.tee 0
                              select
                              i32.add
                              i32.load
                              local.tee 1
                              br_if 1 (;@12;)
                              i32.const 0
                              local.set 0
                              br 2 (;@11;)
                            end
                            local.get 2
                            i32.load offset=8
                            local.tee 1
                            local.get 0
                            i32.store offset=12
                            local.get 0
                            local.get 1
                            i32.store offset=8
                            br 1 (;@11;)
                          end
                          local.get 2
                          i32.const 20
                          i32.add
                          local.get 2
                          i32.const 16
                          i32.add
                          local.get 0
                          select
                          local.set 4
                          loop  ;; label = @12
                            local.get 4
                            local.set 7
                            local.get 1
                            local.tee 0
                            i32.const 20
                            i32.add
                            local.get 0
                            i32.const 16
                            i32.add
                            local.get 0
                            i32.load offset=20
                            local.tee 1
                            select
                            local.set 4
                            local.get 0
                            i32.const 20
                            i32.const 16
                            local.get 1
                            select
                            i32.add
                            i32.load
                            local.tee 1
                            br_if 0 (;@12;)
                          end
                          local.get 7
                          i32.const 0
                          i32.store
                        end
                        local.get 6
                        i32.eqz
                        br_if 6 (;@4;)
                        block  ;; label = @11
                          local.get 2
                          i32.load offset=28
                          i32.const 2
                          i32.shl
                          i32.const 1051336
                          i32.add
                          local.tee 1
                          i32.load
                          local.get 2
                          i32.ne
                          if  ;; label = @12
                            local.get 2
                            local.get 6
                            i32.load offset=16
                            i32.ne
                            if  ;; label = @13
                              local.get 6
                              local.get 0
                              i32.store offset=20
                              local.get 0
                              br_if 2 (;@11;)
                              br 9 (;@4;)
                            end
                            local.get 6
                            local.get 0
                            i32.store offset=16
                            local.get 0
                            br_if 1 (;@11;)
                            br 8 (;@4;)
                          end
                          local.get 1
                          local.get 0
                          i32.store
                          local.get 0
                          i32.eqz
                          br_if 6 (;@5;)
                        end
                        local.get 0
                        local.get 6
                        i32.store offset=24
                        local.get 2
                        i32.load offset=16
                        local.tee 1
                        if  ;; label = @11
                          local.get 0
                          local.get 1
                          i32.store offset=16
                          local.get 1
                          local.get 0
                          i32.store offset=24
                        end
                        local.get 2
                        i32.load offset=20
                        local.tee 1
                        i32.eqz
                        br_if 6 (;@4;)
                        local.get 0
                        local.get 1
                        i32.store offset=20
                        local.get 1
                        local.get 0
                        i32.store offset=24
                        br 6 (;@4;)
                      end
                      local.get 0
                      i32.load offset=4
                      i32.const -8
                      i32.and
                      local.get 5
                      i32.sub
                      local.tee 1
                      local.get 3
                      local.get 1
                      local.get 3
                      i32.lt_u
                      local.tee 1
                      select
                      local.set 3
                      local.get 0
                      local.get 2
                      local.get 1
                      select
                      local.set 2
                      local.get 0
                      local.set 1
                      br 0 (;@9;)
                    end
                    unreachable
                  end
                  i32.const 1051744
                  local.get 4
                  i32.const -2
                  local.get 7
                  i32.rotl
                  i32.and
                  i32.store
                end
                local.get 2
                i32.const 8
                i32.add
                local.set 0
                local.get 2
                local.get 1
                i32.const 3
                i32.or
                i32.store offset=4
                local.get 1
                local.get 2
                i32.add
                local.tee 1
                local.get 1
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                br 5 (;@1;)
              end
              block  ;; label = @6
                i32.const 2
                local.get 0
                i32.shl
                local.tee 2
                i32.const 0
                local.get 2
                i32.sub
                i32.or
                local.get 1
                local.get 0
                i32.shl
                i32.and
                i32.ctz
                local.tee 7
                i32.const 3
                i32.shl
                local.tee 1
                i32.const 1051480
                i32.add
                local.tee 2
                local.get 1
                i32.const 1051488
                i32.add
                i32.load
                local.tee 0
                i32.load offset=8
                local.tee 3
                i32.ne
                if  ;; label = @7
                  local.get 3
                  local.get 2
                  i32.store offset=12
                  local.get 2
                  local.get 3
                  i32.store offset=8
                  br 1 (;@6;)
                end
                i32.const 1051744
                local.get 4
                i32.const -2
                local.get 7
                i32.rotl
                i32.and
                i32.store
              end
              local.get 0
              local.get 5
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 0
              local.get 5
              i32.add
              local.tee 6
              local.get 1
              local.get 5
              i32.sub
              local.tee 7
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 7
              i32.store
              i32.const 1051752
              i32.load
              local.tee 2
              if  ;; label = @6
                i32.const 1051760
                i32.load
                local.set 1
                block  ;; label = @7
                  i32.const 1051744
                  i32.load
                  local.tee 4
                  i32.const 1
                  local.get 2
                  i32.const 3
                  i32.shr_u
                  i32.shl
                  local.tee 3
                  i32.and
                  i32.eqz
                  if  ;; label = @8
                    i32.const 1051744
                    local.get 3
                    local.get 4
                    i32.or
                    i32.store
                    local.get 2
                    i32.const -8
                    i32.and
                    i32.const 1051480
                    i32.add
                    local.tee 3
                    local.set 4
                    br 1 (;@7;)
                  end
                  local.get 2
                  i32.const -8
                  i32.and
                  local.tee 2
                  i32.const 1051480
                  i32.add
                  local.set 4
                  local.get 2
                  i32.const 1051488
                  i32.add
                  i32.load
                  local.set 3
                end
                local.get 4
                local.get 1
                i32.store offset=8
                local.get 3
                local.get 1
                i32.store offset=12
                local.get 1
                local.get 4
                i32.store offset=12
                local.get 1
                local.get 3
                i32.store offset=8
              end
              local.get 0
              i32.const 8
              i32.add
              local.set 0
              i32.const 1051760
              local.get 6
              i32.store
              i32.const 1051752
              local.get 7
              i32.store
              br 4 (;@1;)
            end
            i32.const 1051748
            i32.const 1051748
            i32.load
            i32.const -2
            local.get 2
            i32.load offset=28
            i32.rotl
            i32.and
            i32.store
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 3
              i32.const 16
              i32.ge_u
              if  ;; label = @6
                local.get 2
                local.get 5
                i32.const 3
                i32.or
                i32.store offset=4
                local.get 2
                local.get 5
                i32.add
                local.tee 7
                local.get 3
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 3
                local.get 7
                i32.add
                local.get 3
                i32.store
                i32.const 1051752
                i32.load
                local.tee 1
                i32.eqz
                br_if 1 (;@5;)
                i32.const 1051760
                i32.load
                local.set 0
                block  ;; label = @7
                  i32.const 1051744
                  i32.load
                  local.tee 4
                  i32.const 1
                  local.get 1
                  i32.const 3
                  i32.shr_u
                  i32.shl
                  local.tee 6
                  i32.and
                  i32.eqz
                  if  ;; label = @8
                    i32.const 1051744
                    local.get 4
                    local.get 6
                    i32.or
                    i32.store
                    local.get 1
                    i32.const -8
                    i32.and
                    i32.const 1051480
                    i32.add
                    local.tee 4
                    local.set 1
                    br 1 (;@7;)
                  end
                  local.get 1
                  i32.const -8
                  i32.and
                  local.tee 4
                  i32.const 1051480
                  i32.add
                  local.set 1
                  local.get 4
                  i32.const 1051488
                  i32.add
                  i32.load
                  local.set 4
                end
                local.get 1
                local.get 0
                i32.store offset=8
                local.get 4
                local.get 0
                i32.store offset=12
                local.get 0
                local.get 1
                i32.store offset=12
                local.get 0
                local.get 4
                i32.store offset=8
                br 1 (;@5;)
              end
              local.get 2
              local.get 3
              local.get 5
              i32.add
              local.tee 0
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 0
              local.get 2
              i32.add
              local.tee 0
              local.get 0
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              br 1 (;@4;)
            end
            i32.const 1051760
            local.get 7
            i32.store
            i32.const 1051752
            local.get 3
            i32.store
          end
          local.get 2
          i32.const 8
          i32.add
          local.tee 0
          i32.eqz
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        i32.const 0
        local.get 5
        i32.sub
        local.set 3
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 6
              i32.const 2
              i32.shl
              i32.const 1051336
              i32.add
              i32.load
              local.tee 2
              i32.eqz
              if  ;; label = @6
                i32.const 0
                local.set 0
                br 1 (;@5;)
              end
              local.get 5
              i32.const 25
              local.get 6
              i32.const 1
              i32.shr_u
              i32.sub
              i32.const 0
              local.get 6
              i32.const 31
              i32.ne
              select
              i32.shl
              local.set 4
              i32.const 0
              local.set 0
              loop  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  i32.load offset=4
                  i32.const -8
                  i32.and
                  local.tee 7
                  local.get 5
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 7
                  local.get 5
                  i32.sub
                  local.tee 7
                  local.get 3
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 2
                  local.set 1
                  local.get 7
                  local.tee 3
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 3
                  local.get 1
                  local.set 0
                  br 3 (;@4;)
                end
                local.get 2
                i32.load offset=20
                local.tee 7
                local.get 0
                local.get 7
                local.get 2
                local.get 4
                i32.const 29
                i32.shr_u
                i32.const 4
                i32.and
                i32.add
                i32.load offset=16
                local.tee 2
                i32.ne
                select
                local.get 0
                local.get 7
                select
                local.set 0
                local.get 4
                i32.const 1
                i32.shl
                local.set 4
                local.get 2
                br_if 0 (;@6;)
              end
            end
            local.get 0
            local.get 1
            i32.or
            i32.eqz
            if  ;; label = @5
              i32.const 0
              local.set 1
              i32.const 2
              local.get 6
              i32.shl
              local.tee 0
              i32.const 0
              local.get 0
              i32.sub
              i32.or
              local.get 9
              i32.and
              local.tee 0
              i32.eqz
              br_if 3 (;@2;)
              local.get 0
              i32.ctz
              i32.const 2
              i32.shl
              i32.const 1051336
              i32.add
              i32.load
              local.set 0
            end
            local.get 0
            i32.eqz
            br_if 1 (;@3;)
          end
          loop  ;; label = @4
            local.get 3
            local.get 0
            i32.load offset=4
            i32.const -8
            i32.and
            local.tee 2
            local.get 5
            i32.sub
            local.tee 4
            local.get 3
            local.get 3
            local.get 4
            i32.gt_u
            local.tee 4
            select
            local.get 2
            local.get 5
            i32.lt_u
            local.tee 2
            select
            local.set 3
            local.get 1
            local.get 0
            local.get 1
            local.get 4
            select
            local.get 2
            select
            local.set 1
            local.get 0
            i32.load offset=16
            local.tee 2
            if (result i32)  ;; label = @5
              local.get 2
            else
              local.get 0
              i32.load offset=20
            end
            local.tee 0
            br_if 0 (;@4;)
          end
        end
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 5
        i32.const 1051752
        i32.load
        local.tee 0
        i32.le_u
        local.get 3
        local.get 0
        local.get 5
        i32.sub
        i32.ge_u
        i32.and
        br_if 0 (;@2;)
        local.get 1
        i32.load offset=24
        local.set 6
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            local.get 1
            i32.load offset=12
            local.tee 0
            i32.eq
            if  ;; label = @5
              local.get 1
              i32.const 20
              i32.const 16
              local.get 1
              i32.load offset=20
              local.tee 0
              select
              i32.add
              i32.load
              local.tee 2
              br_if 1 (;@4;)
              i32.const 0
              local.set 0
              br 2 (;@3;)
            end
            local.get 1
            i32.load offset=8
            local.tee 2
            local.get 0
            i32.store offset=12
            local.get 0
            local.get 2
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 1
          i32.const 20
          i32.add
          local.get 1
          i32.const 16
          i32.add
          local.get 0
          select
          local.set 4
          loop  ;; label = @4
            local.get 4
            local.set 7
            local.get 2
            local.tee 0
            i32.const 20
            i32.add
            local.get 0
            i32.const 16
            i32.add
            local.get 0
            i32.load offset=20
            local.tee 2
            select
            local.set 4
            local.get 0
            i32.const 20
            i32.const 16
            local.get 2
            select
            i32.add
            i32.load
            local.tee 2
            br_if 0 (;@4;)
          end
          local.get 7
          i32.const 0
          i32.store
        end
        block  ;; label = @3
          local.get 6
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load offset=28
              i32.const 2
              i32.shl
              i32.const 1051336
              i32.add
              local.tee 2
              i32.load
              local.get 1
              i32.ne
              if  ;; label = @6
                local.get 1
                local.get 6
                i32.load offset=16
                i32.ne
                if  ;; label = @7
                  local.get 6
                  local.get 0
                  i32.store offset=20
                  local.get 0
                  br_if 2 (;@5;)
                  br 4 (;@3;)
                end
                local.get 6
                local.get 0
                i32.store offset=16
                local.get 0
                br_if 1 (;@5;)
                br 3 (;@3;)
              end
              local.get 2
              local.get 0
              i32.store
              local.get 0
              i32.eqz
              br_if 1 (;@4;)
            end
            local.get 0
            local.get 6
            i32.store offset=24
            local.get 1
            i32.load offset=16
            local.tee 2
            if  ;; label = @5
              local.get 0
              local.get 2
              i32.store offset=16
              local.get 2
              local.get 0
              i32.store offset=24
            end
            local.get 1
            i32.load offset=20
            local.tee 2
            i32.eqz
            br_if 1 (;@3;)
            local.get 0
            local.get 2
            i32.store offset=20
            local.get 2
            local.get 0
            i32.store offset=24
            br 1 (;@3;)
          end
          i32.const 1051748
          i32.const 1051748
          i32.load
          i32.const -2
          local.get 1
          i32.load offset=28
          i32.rotl
          i32.and
          i32.store
        end
        block  ;; label = @3
          local.get 3
          i32.const 16
          i32.ge_u
          if  ;; label = @4
            local.get 1
            local.get 5
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 1
            local.get 5
            i32.add
            local.tee 0
            local.get 3
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            local.get 3
            i32.add
            local.get 3
            i32.store
            local.get 3
            i32.const 256
            i32.ge_u
            if  ;; label = @5
              local.get 0
              local.get 3
              call 8
              br 2 (;@3;)
            end
            block  ;; label = @5
              i32.const 1051744
              i32.load
              local.tee 2
              i32.const 1
              local.get 3
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 4
              i32.and
              i32.eqz
              if  ;; label = @6
                i32.const 1051744
                local.get 2
                local.get 4
                i32.or
                i32.store
                local.get 3
                i32.const 248
                i32.and
                i32.const 1051480
                i32.add
                local.tee 3
                local.set 2
                br 1 (;@5;)
              end
              local.get 3
              i32.const 248
              i32.and
              local.tee 4
              i32.const 1051480
              i32.add
              local.set 2
              local.get 4
              i32.const 1051488
              i32.add
              i32.load
              local.set 3
            end
            local.get 2
            local.get 0
            i32.store offset=8
            local.get 3
            local.get 0
            i32.store offset=12
            local.get 0
            local.get 2
            i32.store offset=12
            local.get 0
            local.get 3
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 1
          local.get 3
          local.get 5
          i32.add
          local.tee 0
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          local.get 1
          i32.add
          local.tee 0
          local.get 0
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
        end
        local.get 1
        i32.const 8
        i32.add
        local.tee 0
        br_if 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 5
                i32.const 1051752
                i32.load
                local.tee 1
                i32.gt_u
                if  ;; label = @7
                  local.get 5
                  i32.const 1051756
                  i32.load
                  local.tee 0
                  i32.ge_u
                  if  ;; label = @8
                    local.get 8
                    i32.const 4
                    i32.add
                    local.set 0
                    block (result i32)  ;; label = @9
                      block  ;; label = @10
                        local.get 5
                        i32.const 65583
                        i32.add
                        i32.const -65536
                        i32.and
                        local.tee 1
                        i32.eqz
                        br_if 0 (;@10;)
                        i32.const 1051789
                        i32.load8_u
                        i32.const 1051789
                        i32.const 1
                        i32.store8
                        i32.const 1051792
                        local.set 2
                        local.get 1
                        i32.const 62320
                        i32.gt_u
                        i32.or
                        br_if 0 (;@10;)
                        i32.const 62320
                        br 1 (;@9;)
                      end
                      i32.const 0
                      local.set 2
                      i32.const 0
                      local.get 1
                      i32.const 16
                      i32.shr_u
                      local.get 1
                      i32.const 65535
                      i32.and
                      i32.const 0
                      i32.ne
                      i32.add
                      local.tee 1
                      memory.grow
                      local.tee 4
                      i32.const -1
                      i32.eq
                      br_if 0 (;@9;)
                      drop
                      local.get 1
                      i32.const 16
                      i32.shl
                      local.tee 1
                      i32.const 16
                      i32.sub
                      local.get 1
                      local.get 4
                      i32.const 16
                      i32.shl
                      local.tee 2
                      i32.const 0
                      local.get 1
                      i32.sub
                      i32.eq
                      select
                    end
                    local.set 1
                    local.get 0
                    i32.const 0
                    i32.store offset=8
                    local.get 0
                    local.get 1
                    i32.store offset=4
                    local.get 0
                    local.get 2
                    i32.store
                    local.get 8
                    i32.load offset=4
                    local.tee 1
                    i32.eqz
                    if  ;; label = @9
                      i32.const 0
                      local.set 0
                      br 8 (;@1;)
                    end
                    local.get 8
                    i32.load offset=12
                    local.set 7
                    i32.const 1051768
                    local.get 8
                    i32.load offset=8
                    local.tee 4
                    i32.const 1051768
                    i32.load
                    i32.add
                    local.tee 0
                    i32.store
                    i32.const 1051772
                    local.get 0
                    i32.const 1051772
                    i32.load
                    local.tee 2
                    local.get 0
                    local.get 2
                    i32.gt_u
                    select
                    i32.store
                    block  ;; label = @9
                      block  ;; label = @10
                        i32.const 1051764
                        i32.load
                        local.tee 2
                        if  ;; label = @11
                          i32.const 1051464
                          local.set 0
                          loop  ;; label = @12
                            local.get 1
                            local.get 0
                            i32.load
                            local.tee 3
                            local.get 0
                            i32.load offset=4
                            local.tee 6
                            i32.add
                            i32.eq
                            br_if 2 (;@10;)
                            local.get 0
                            i32.load offset=8
                            local.tee 0
                            br_if 0 (;@12;)
                          end
                          br 2 (;@9;)
                        end
                        i32.const 1051780
                        i32.load
                        local.tee 0
                        i32.const 0
                        local.get 0
                        local.get 1
                        i32.le_u
                        select
                        i32.eqz
                        if  ;; label = @11
                          i32.const 1051780
                          local.get 1
                          i32.store
                        end
                        i32.const 1051784
                        i32.const 4095
                        i32.store
                        i32.const 1051476
                        local.get 7
                        i32.store
                        i32.const 1051468
                        local.get 4
                        i32.store
                        i32.const 1051464
                        local.get 1
                        i32.store
                        i32.const 1051492
                        i32.const 1051480
                        i32.store
                        i32.const 1051500
                        i32.const 1051488
                        i32.store
                        i32.const 1051488
                        i32.const 1051480
                        i32.store
                        i32.const 1051508
                        i32.const 1051496
                        i32.store
                        i32.const 1051496
                        i32.const 1051488
                        i32.store
                        i32.const 1051516
                        i32.const 1051504
                        i32.store
                        i32.const 1051504
                        i32.const 1051496
                        i32.store
                        i32.const 1051524
                        i32.const 1051512
                        i32.store
                        i32.const 1051512
                        i32.const 1051504
                        i32.store
                        i32.const 1051532
                        i32.const 1051520
                        i32.store
                        i32.const 1051520
                        i32.const 1051512
                        i32.store
                        i32.const 1051540
                        i32.const 1051528
                        i32.store
                        i32.const 1051528
                        i32.const 1051520
                        i32.store
                        i32.const 1051548
                        i32.const 1051536
                        i32.store
                        i32.const 1051536
                        i32.const 1051528
                        i32.store
                        i32.const 1051556
                        i32.const 1051544
                        i32.store
                        i32.const 1051544
                        i32.const 1051536
                        i32.store
                        i32.const 1051552
                        i32.const 1051544
                        i32.store
                        i32.const 1051564
                        i32.const 1051552
                        i32.store
                        i32.const 1051560
                        i32.const 1051552
                        i32.store
                        i32.const 1051572
                        i32.const 1051560
                        i32.store
                        i32.const 1051568
                        i32.const 1051560
                        i32.store
                        i32.const 1051580
                        i32.const 1051568
                        i32.store
                        i32.const 1051576
                        i32.const 1051568
                        i32.store
                        i32.const 1051588
                        i32.const 1051576
                        i32.store
                        i32.const 1051584
                        i32.const 1051576
                        i32.store
                        i32.const 1051596
                        i32.const 1051584
                        i32.store
                        i32.const 1051592
                        i32.const 1051584
                        i32.store
                        i32.const 1051604
                        i32.const 1051592
                        i32.store
                        i32.const 1051600
                        i32.const 1051592
                        i32.store
                        i32.const 1051612
                        i32.const 1051600
                        i32.store
                        i32.const 1051608
                        i32.const 1051600
                        i32.store
                        i32.const 1051620
                        i32.const 1051608
                        i32.store
                        i32.const 1051628
                        i32.const 1051616
                        i32.store
                        i32.const 1051616
                        i32.const 1051608
                        i32.store
                        i32.const 1051636
                        i32.const 1051624
                        i32.store
                        i32.const 1051624
                        i32.const 1051616
                        i32.store
                        i32.const 1051644
                        i32.const 1051632
                        i32.store
                        i32.const 1051632
                        i32.const 1051624
                        i32.store
                        i32.const 1051652
                        i32.const 1051640
                        i32.store
                        i32.const 1051640
                        i32.const 1051632
                        i32.store
                        i32.const 1051660
                        i32.const 1051648
                        i32.store
                        i32.const 1051648
                        i32.const 1051640
                        i32.store
                        i32.const 1051668
                        i32.const 1051656
                        i32.store
                        i32.const 1051656
                        i32.const 1051648
                        i32.store
                        i32.const 1051676
                        i32.const 1051664
                        i32.store
                        i32.const 1051664
                        i32.const 1051656
                        i32.store
                        i32.const 1051684
                        i32.const 1051672
                        i32.store
                        i32.const 1051672
                        i32.const 1051664
                        i32.store
                        i32.const 1051692
                        i32.const 1051680
                        i32.store
                        i32.const 1051680
                        i32.const 1051672
                        i32.store
                        i32.const 1051700
                        i32.const 1051688
                        i32.store
                        i32.const 1051688
                        i32.const 1051680
                        i32.store
                        i32.const 1051708
                        i32.const 1051696
                        i32.store
                        i32.const 1051696
                        i32.const 1051688
                        i32.store
                        i32.const 1051716
                        i32.const 1051704
                        i32.store
                        i32.const 1051704
                        i32.const 1051696
                        i32.store
                        i32.const 1051724
                        i32.const 1051712
                        i32.store
                        i32.const 1051712
                        i32.const 1051704
                        i32.store
                        i32.const 1051732
                        i32.const 1051720
                        i32.store
                        i32.const 1051720
                        i32.const 1051712
                        i32.store
                        i32.const 1051740
                        i32.const 1051728
                        i32.store
                        i32.const 1051728
                        i32.const 1051720
                        i32.store
                        i32.const 1051764
                        local.get 1
                        i32.const 15
                        i32.add
                        i32.const -8
                        i32.and
                        local.tee 0
                        i32.const 8
                        i32.sub
                        local.tee 2
                        i32.store
                        i32.const 1051736
                        i32.const 1051728
                        i32.store
                        i32.const 1051756
                        local.get 4
                        i32.const 40
                        i32.sub
                        local.tee 4
                        local.get 1
                        local.get 0
                        i32.sub
                        i32.add
                        i32.const 8
                        i32.add
                        local.tee 0
                        i32.store
                        local.get 2
                        local.get 0
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 1
                        local.get 4
                        i32.add
                        i32.const 40
                        i32.store offset=4
                        i32.const 1051776
                        i32.const 2097152
                        i32.store
                        br 8 (;@2;)
                      end
                      local.get 2
                      local.get 3
                      i32.lt_u
                      local.get 1
                      local.get 2
                      i32.le_u
                      i32.or
                      br_if 0 (;@9;)
                      local.get 0
                      i32.load offset=12
                      local.tee 3
                      i32.const 1
                      i32.and
                      br_if 0 (;@9;)
                      local.get 3
                      i32.const 1
                      i32.shr_u
                      local.get 7
                      i32.eq
                      br_if 3 (;@6;)
                    end
                    i32.const 1051780
                    i32.const 1051780
                    i32.load
                    local.tee 0
                    local.get 1
                    local.get 0
                    local.get 1
                    i32.lt_u
                    select
                    i32.store
                    local.get 1
                    local.get 4
                    i32.add
                    local.set 3
                    i32.const 1051464
                    local.set 0
                    block  ;; label = @9
                      block  ;; label = @10
                        loop  ;; label = @11
                          local.get 3
                          local.get 0
                          i32.load
                          local.tee 6
                          i32.ne
                          if  ;; label = @12
                            local.get 0
                            i32.load offset=8
                            local.tee 0
                            br_if 1 (;@11;)
                            br 2 (;@10;)
                          end
                        end
                        local.get 0
                        i32.load offset=12
                        local.tee 3
                        i32.const 1
                        i32.and
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const 1
                        i32.shr_u
                        local.get 7
                        i32.eq
                        br_if 1 (;@9;)
                      end
                      i32.const 1051464
                      local.set 0
                      loop  ;; label = @10
                        block  ;; label = @11
                          local.get 2
                          local.get 0
                          i32.load
                          local.tee 3
                          i32.ge_u
                          if  ;; label = @12
                            local.get 2
                            local.get 3
                            local.get 0
                            i32.load offset=4
                            i32.add
                            local.tee 6
                            i32.lt_u
                            br_if 1 (;@11;)
                          end
                          local.get 0
                          i32.load offset=8
                          local.set 0
                          br 1 (;@10;)
                        end
                      end
                      i32.const 1051764
                      local.get 1
                      i32.const 15
                      i32.add
                      i32.const -8
                      i32.and
                      local.tee 0
                      i32.const 8
                      i32.sub
                      local.tee 3
                      i32.store
                      i32.const 1051756
                      local.get 4
                      i32.const 40
                      i32.sub
                      local.tee 9
                      local.get 1
                      local.get 0
                      i32.sub
                      i32.add
                      i32.const 8
                      i32.add
                      local.tee 0
                      i32.store
                      local.get 3
                      local.get 0
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 1
                      local.get 9
                      i32.add
                      i32.const 40
                      i32.store offset=4
                      i32.const 1051776
                      i32.const 2097152
                      i32.store
                      local.get 2
                      local.get 6
                      i32.const 32
                      i32.sub
                      i32.const -8
                      i32.and
                      i32.const 8
                      i32.sub
                      local.tee 0
                      local.get 0
                      local.get 2
                      i32.const 16
                      i32.add
                      i32.lt_u
                      select
                      local.tee 3
                      i32.const 27
                      i32.store offset=4
                      i32.const 1051464
                      i64.load align=4
                      local.set 10
                      local.get 3
                      i32.const 16
                      i32.add
                      i32.const 1051472
                      i64.load align=4
                      i64.store align=4
                      local.get 3
                      i32.const 8
                      i32.add
                      local.tee 0
                      local.get 10
                      i64.store align=4
                      i32.const 1051476
                      local.get 7
                      i32.store
                      i32.const 1051468
                      local.get 4
                      i32.store
                      i32.const 1051464
                      local.get 1
                      i32.store
                      i32.const 1051472
                      local.get 0
                      i32.store
                      local.get 3
                      i32.const 28
                      i32.add
                      local.set 0
                      loop  ;; label = @10
                        local.get 0
                        i32.const 7
                        i32.store
                        local.get 0
                        i32.const 4
                        i32.add
                        local.tee 0
                        local.get 6
                        i32.lt_u
                        br_if 0 (;@10;)
                      end
                      local.get 2
                      local.get 3
                      i32.eq
                      br_if 7 (;@2;)
                      local.get 3
                      local.get 3
                      i32.load offset=4
                      i32.const -2
                      i32.and
                      i32.store offset=4
                      local.get 2
                      local.get 3
                      local.get 2
                      i32.sub
                      local.tee 0
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 3
                      local.get 0
                      i32.store
                      local.get 0
                      i32.const 256
                      i32.ge_u
                      if  ;; label = @10
                        local.get 2
                        local.get 0
                        call 8
                        br 8 (;@2;)
                      end
                      block  ;; label = @10
                        i32.const 1051744
                        i32.load
                        local.tee 1
                        i32.const 1
                        local.get 0
                        i32.const 3
                        i32.shr_u
                        i32.shl
                        local.tee 4
                        i32.and
                        i32.eqz
                        if  ;; label = @11
                          i32.const 1051744
                          local.get 1
                          local.get 4
                          i32.or
                          i32.store
                          local.get 0
                          i32.const 248
                          i32.and
                          i32.const 1051480
                          i32.add
                          local.tee 0
                          local.set 1
                          br 1 (;@10;)
                        end
                        local.get 0
                        i32.const 248
                        i32.and
                        local.tee 0
                        i32.const 1051480
                        i32.add
                        local.set 1
                        local.get 0
                        i32.const 1051488
                        i32.add
                        i32.load
                        local.set 0
                      end
                      local.get 1
                      local.get 2
                      i32.store offset=8
                      local.get 0
                      local.get 2
                      i32.store offset=12
                      local.get 2
                      local.get 1
                      i32.store offset=12
                      local.get 2
                      local.get 0
                      i32.store offset=8
                      br 7 (;@2;)
                    end
                    local.get 0
                    local.get 1
                    i32.store
                    local.get 0
                    local.get 0
                    i32.load offset=4
                    local.get 4
                    i32.add
                    i32.store offset=4
                    local.get 1
                    i32.const 15
                    i32.add
                    i32.const -8
                    i32.and
                    i32.const 8
                    i32.sub
                    local.tee 4
                    local.get 5
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    local.get 6
                    i32.const 15
                    i32.add
                    i32.const -8
                    i32.and
                    i32.const 8
                    i32.sub
                    local.tee 3
                    local.get 4
                    local.get 5
                    i32.add
                    local.tee 0
                    i32.sub
                    local.set 5
                    local.get 3
                    i32.const 1051764
                    i32.load
                    i32.eq
                    br_if 3 (;@5;)
                    local.get 3
                    i32.const 1051760
                    i32.load
                    i32.eq
                    br_if 4 (;@4;)
                    local.get 3
                    i32.load offset=4
                    local.tee 2
                    i32.const 3
                    i32.and
                    i32.const 1
                    i32.eq
                    if  ;; label = @9
                      local.get 3
                      local.get 2
                      i32.const -8
                      i32.and
                      local.tee 1
                      call 7
                      local.get 1
                      local.get 5
                      i32.add
                      local.set 5
                      local.get 1
                      local.get 3
                      i32.add
                      local.tee 3
                      i32.load offset=4
                      local.set 2
                    end
                    local.get 3
                    local.get 2
                    i32.const -2
                    i32.and
                    i32.store offset=4
                    local.get 0
                    local.get 5
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 0
                    local.get 5
                    i32.add
                    local.get 5
                    i32.store
                    local.get 5
                    i32.const 256
                    i32.ge_u
                    if  ;; label = @9
                      local.get 0
                      local.get 5
                      call 8
                      br 6 (;@3;)
                    end
                    block  ;; label = @9
                      i32.const 1051744
                      i32.load
                      local.tee 1
                      i32.const 1
                      local.get 5
                      i32.const 3
                      i32.shr_u
                      i32.shl
                      local.tee 2
                      i32.and
                      i32.eqz
                      if  ;; label = @10
                        i32.const 1051744
                        local.get 1
                        local.get 2
                        i32.or
                        i32.store
                        local.get 5
                        i32.const 248
                        i32.and
                        i32.const 1051480
                        i32.add
                        local.tee 5
                        local.set 3
                        br 1 (;@9;)
                      end
                      local.get 5
                      i32.const 248
                      i32.and
                      local.tee 1
                      i32.const 1051480
                      i32.add
                      local.set 3
                      local.get 1
                      i32.const 1051488
                      i32.add
                      i32.load
                      local.set 5
                    end
                    local.get 3
                    local.get 0
                    i32.store offset=8
                    local.get 5
                    local.get 0
                    i32.store offset=12
                    local.get 0
                    local.get 3
                    i32.store offset=12
                    local.get 0
                    local.get 5
                    i32.store offset=8
                    br 5 (;@3;)
                  end
                  i32.const 1051756
                  local.get 0
                  local.get 5
                  i32.sub
                  local.tee 1
                  i32.store
                  i32.const 1051764
                  i32.const 1051764
                  i32.load
                  local.tee 0
                  local.get 5
                  i32.add
                  local.tee 2
                  i32.store
                  local.get 2
                  local.get 1
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  local.get 5
                  i32.const 3
                  i32.or
                  i32.store offset=4
                  local.get 0
                  i32.const 8
                  i32.add
                  local.set 0
                  br 6 (;@1;)
                end
                i32.const 1051760
                i32.load
                local.set 0
                block  ;; label = @7
                  local.get 1
                  local.get 5
                  i32.sub
                  local.tee 2
                  i32.const 15
                  i32.le_u
                  if  ;; label = @8
                    i32.const 1051760
                    i32.const 0
                    i32.store
                    i32.const 1051752
                    i32.const 0
                    i32.store
                    local.get 0
                    local.get 1
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    local.get 0
                    local.get 1
                    i32.add
                    local.tee 1
                    local.get 1
                    i32.load offset=4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    br 1 (;@7;)
                  end
                  i32.const 1051752
                  local.get 2
                  i32.store
                  i32.const 1051760
                  local.get 0
                  local.get 5
                  i32.add
                  local.tee 4
                  i32.store
                  local.get 4
                  local.get 2
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  local.get 1
                  i32.add
                  local.get 2
                  i32.store
                  local.get 0
                  local.get 5
                  i32.const 3
                  i32.or
                  i32.store offset=4
                end
                local.get 0
                i32.const 8
                i32.add
                local.set 0
                br 5 (;@1;)
              end
              local.get 0
              local.get 4
              local.get 6
              i32.add
              i32.store offset=4
              i32.const 1051764
              i32.const 1051764
              i32.load
              local.tee 0
              i32.const 15
              i32.add
              i32.const -8
              i32.and
              local.tee 1
              i32.const 8
              i32.sub
              local.tee 2
              i32.store
              i32.const 1051756
              i32.const 1051756
              i32.load
              local.get 4
              i32.add
              local.tee 4
              local.get 0
              local.get 1
              i32.sub
              i32.add
              i32.const 8
              i32.add
              local.tee 1
              i32.store
              local.get 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 4
              i32.add
              i32.const 40
              i32.store offset=4
              i32.const 1051776
              i32.const 2097152
              i32.store
              br 3 (;@2;)
            end
            i32.const 1051764
            local.get 0
            i32.store
            i32.const 1051756
            i32.const 1051756
            i32.load
            local.get 5
            i32.add
            local.tee 1
            i32.store
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            br 1 (;@3;)
          end
          i32.const 1051760
          local.get 0
          i32.store
          i32.const 1051752
          i32.const 1051752
          i32.load
          local.get 5
          i32.add
          local.tee 1
          i32.store
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
        local.get 4
        i32.const 8
        i32.add
        local.set 0
        br 1 (;@1;)
      end
      i32.const 0
      local.set 0
      i32.const 1051756
      i32.load
      local.tee 1
      local.get 5
      i32.le_u
      br_if 0 (;@1;)
      i32.const 1051756
      local.get 1
      local.get 5
      i32.sub
      local.tee 1
      i32.store
      i32.const 1051764
      i32.const 1051764
      i32.load
      local.tee 0
      local.get 5
      i32.add
      local.tee 2
      i32.store
      local.get 2
      local.get 1
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 0
      local.get 5
      i32.const 3
      i32.or
      i32.store offset=4
      local.get 0
      i32.const 8
      i32.add
      local.set 0
    end
    local.get 8
    i32.const 16
    i32.add
    global.set 0
    local.get 0)
  (func (;2;) (type 4) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.const 4
                  i32.sub
                  local.tee 7
                  i32.load
                  local.tee 8
                  i32.const -8
                  i32.and
                  local.tee 4
                  i32.const 4
                  i32.const 8
                  local.get 8
                  i32.const 3
                  i32.and
                  local.tee 5
                  select
                  local.get 1
                  i32.add
                  i32.ge_u
                  if  ;; label = @8
                    local.get 5
                    i32.const 0
                    local.get 1
                    i32.const 39
                    i32.add
                    local.tee 6
                    local.get 4
                    i32.lt_u
                    select
                    br_if 1 (;@7;)
                    block  ;; label = @9
                      local.get 2
                      i32.const 9
                      i32.ge_u
                      if  ;; label = @10
                        local.get 2
                        local.get 3
                        call 6
                        local.tee 2
                        br_if 1 (;@9;)
                        i32.const 0
                        return
                      end
                      i32.const 0
                      local.set 2
                      local.get 3
                      i32.const -65588
                      i32.gt_u
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
                      i32.const 8
                      i32.sub
                      local.set 6
                      local.get 5
                      i32.eqz
                      if  ;; label = @10
                        local.get 6
                        i32.eqz
                        local.get 1
                        i32.const 256
                        i32.lt_u
                        i32.or
                        local.get 4
                        local.get 1
                        i32.sub
                        i32.const 131072
                        i32.gt_u
                        local.get 1
                        local.get 4
                        i32.ge_u
                        i32.or
                        i32.or
                        br_if 7 (;@3;)
                        local.get 0
                        return
                      end
                      local.get 4
                      local.get 6
                      i32.add
                      local.set 5
                      block  ;; label = @10
                        local.get 1
                        local.get 4
                        i32.gt_u
                        if  ;; label = @11
                          local.get 5
                          i32.const 1051764
                          i32.load
                          i32.eq
                          br_if 1 (;@10;)
                          i32.const 1051760
                          i32.load
                          local.get 5
                          i32.ne
                          if  ;; label = @12
                            local.get 5
                            i32.load offset=4
                            local.tee 8
                            i32.const 2
                            i32.and
                            br_if 9 (;@3;)
                            local.get 8
                            i32.const -8
                            i32.and
                            local.tee 8
                            local.get 4
                            i32.add
                            local.tee 4
                            local.get 1
                            i32.lt_u
                            br_if 9 (;@3;)
                            local.get 5
                            local.get 8
                            call 7
                            local.get 4
                            local.get 1
                            i32.sub
                            local.tee 5
                            i32.const 16
                            i32.ge_u
                            if  ;; label = @13
                              local.get 7
                              local.get 1
                              local.get 7
                              i32.load
                              i32.const 1
                              i32.and
                              i32.or
                              i32.const 2
                              i32.or
                              i32.store
                              local.get 1
                              local.get 6
                              i32.add
                              local.tee 1
                              local.get 5
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 4
                              local.get 6
                              i32.add
                              local.tee 4
                              local.get 4
                              i32.load offset=4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              local.get 1
                              local.get 5
                              call 5
                              br 9 (;@4;)
                            end
                            local.get 7
                            local.get 4
                            local.get 7
                            i32.load
                            i32.const 1
                            i32.and
                            i32.or
                            i32.const 2
                            i32.or
                            i32.store
                            local.get 4
                            local.get 6
                            i32.add
                            local.tee 1
                            local.get 1
                            i32.load offset=4
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            br 8 (;@4;)
                          end
                          i32.const 1051752
                          i32.load
                          local.get 4
                          i32.add
                          local.tee 4
                          local.get 1
                          i32.lt_u
                          br_if 8 (;@3;)
                          block  ;; label = @12
                            local.get 4
                            local.get 1
                            i32.sub
                            local.tee 5
                            i32.const 15
                            i32.le_u
                            if  ;; label = @13
                              local.get 7
                              local.get 8
                              i32.const 1
                              i32.and
                              local.get 4
                              i32.or
                              i32.const 2
                              i32.or
                              i32.store
                              local.get 4
                              local.get 6
                              i32.add
                              local.tee 1
                              local.get 1
                              i32.load offset=4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              i32.const 0
                              local.set 5
                              i32.const 0
                              local.set 1
                              br 1 (;@12;)
                            end
                            local.get 7
                            local.get 1
                            local.get 8
                            i32.const 1
                            i32.and
                            i32.or
                            i32.const 2
                            i32.or
                            i32.store
                            local.get 1
                            local.get 6
                            i32.add
                            local.tee 1
                            local.get 5
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            local.get 4
                            local.get 6
                            i32.add
                            local.tee 4
                            local.get 5
                            i32.store
                            local.get 4
                            local.get 4
                            i32.load offset=4
                            i32.const -2
                            i32.and
                            i32.store offset=4
                          end
                          i32.const 1051760
                          local.get 1
                          i32.store
                          i32.const 1051752
                          local.get 5
                          i32.store
                          br 7 (;@4;)
                        end
                        local.get 4
                        local.get 1
                        i32.sub
                        local.tee 4
                        i32.const 15
                        i32.le_u
                        br_if 6 (;@4;)
                        local.get 7
                        local.get 1
                        local.get 8
                        i32.const 1
                        i32.and
                        i32.or
                        i32.const 2
                        i32.or
                        i32.store
                        local.get 1
                        local.get 6
                        i32.add
                        local.tee 1
                        local.get 4
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 5
                        local.get 5
                        i32.load offset=4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 1
                        local.get 4
                        call 5
                        br 6 (;@4;)
                      end
                      i32.const 1051756
                      i32.load
                      local.get 4
                      i32.add
                      local.tee 4
                      local.get 1
                      i32.gt_u
                      br_if 4 (;@5;)
                      br 6 (;@3;)
                    end
                    local.get 3
                    local.get 1
                    local.get 1
                    local.get 3
                    i32.gt_u
                    select
                    local.tee 3
                    if  ;; label = @9
                      local.get 2
                      local.get 0
                      local.get 3
                      memory.copy
                    end
                    local.get 7
                    i32.load
                    local.tee 3
                    i32.const -8
                    i32.and
                    local.tee 7
                    local.get 1
                    i32.const 4
                    i32.const 8
                    local.get 3
                    i32.const 3
                    i32.and
                    local.tee 1
                    select
                    i32.add
                    i32.lt_u
                    br_if 2 (;@6;)
                    local.get 1
                    i32.eqz
                    local.get 6
                    local.get 7
                    i32.ge_u
                    i32.or
                    br_if 6 (;@2;)
                    i32.const 1049000
                    i32.const 1049048
                    call 26
                    unreachable
                  end
                  i32.const 1048936
                  i32.const 1048984
                  call 26
                  unreachable
                end
                i32.const 1049000
                i32.const 1049048
                call 26
                unreachable
              end
              i32.const 1048936
              i32.const 1048984
              call 26
              unreachable
            end
            local.get 7
            local.get 1
            local.get 8
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 1
            local.get 6
            i32.add
            local.tee 5
            local.get 4
            local.get 1
            i32.sub
            local.tee 1
            i32.const 1
            i32.or
            i32.store offset=4
            i32.const 1051756
            local.get 1
            i32.store
            i32.const 1051764
            local.get 5
            i32.store
          end
          local.get 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          return
        end
        local.get 3
        call 1
        local.tee 1
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        i32.const -4
        i32.const -8
        local.get 7
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
        local.get 2
        local.get 3
        i32.gt_u
        select
        local.tee 2
        if  ;; label = @3
          local.get 1
          local.get 0
          local.get 2
          memory.copy
        end
        local.get 1
        local.set 2
      end
      local.get 0
      call 3
    end
    local.get 2)
  (func (;3;) (type 6) (param i32)
    (local i32 i32 i32 i32)
    local.get 0
    i32.const 8
    i32.sub
    local.tee 1
    local.get 0
    i32.const 4
    i32.sub
    i32.load
    local.tee 3
    i32.const -8
    i32.and
    local.tee 0
    i32.add
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 3
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        i32.load
        local.tee 3
        local.get 0
        i32.add
        local.set 0
        local.get 1
        local.get 3
        i32.sub
        local.tee 1
        i32.const 1051760
        i32.load
        i32.eq
        if  ;; label = @3
          local.get 2
          i32.load offset=4
          i32.const 3
          i32.and
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          i32.const 1051752
          local.get 0
          i32.store
          local.get 2
          local.get 2
          i32.load offset=4
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          local.get 0
          i32.store
          return
        end
        local.get 1
        local.get 3
        call 7
      end
      block  ;; label = @2
        block  ;; label = @3
          i32.const 1051784
          block (result i32)  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.load offset=4
                    local.tee 3
                    i32.const 2
                    i32.and
                    i32.eqz
                    if  ;; label = @9
                      local.get 2
                      i32.const 1051764
                      i32.load
                      i32.eq
                      br_if 2 (;@7;)
                      local.get 2
                      i32.const 1051760
                      i32.load
                      i32.eq
                      br_if 3 (;@6;)
                      local.get 2
                      local.get 3
                      i32.const -8
                      i32.and
                      local.tee 2
                      call 7
                      local.get 1
                      local.get 0
                      local.get 2
                      i32.add
                      local.tee 0
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 1
                      i32.add
                      local.get 0
                      i32.store
                      local.get 1
                      i32.const 1051760
                      i32.load
                      i32.ne
                      br_if 1 (;@8;)
                      i32.const 1051752
                      local.get 0
                      i32.store
                      return
                    end
                    local.get 2
                    local.get 3
                    i32.const -2
                    i32.and
                    i32.store offset=4
                    local.get 1
                    local.get 0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 0
                    local.get 1
                    i32.add
                    local.get 0
                    i32.store
                  end
                  local.get 0
                  i32.const 256
                  i32.lt_u
                  br_if 4 (;@3;)
                  local.get 1
                  local.get 0
                  call 8
                  i32.const 1051784
                  i32.const 1051784
                  i32.load
                  i32.const 1
                  i32.sub
                  local.tee 0
                  i32.store
                  local.get 0
                  br_if 6 (;@1;)
                  i32.const 1051472
                  i32.load
                  local.tee 0
                  br_if 2 (;@5;)
                  i32.const 4095
                  br 3 (;@4;)
                end
                i32.const 1051764
                local.get 1
                i32.store
                i32.const 1051756
                i32.const 1051756
                i32.load
                local.get 0
                i32.add
                local.tee 0
                i32.store
                local.get 1
                local.get 0
                i32.const 1
                i32.or
                i32.store offset=4
                i32.const 1051760
                i32.load
                local.get 1
                i32.eq
                if  ;; label = @7
                  i32.const 1051752
                  i32.const 0
                  i32.store
                  i32.const 1051760
                  i32.const 0
                  i32.store
                end
                local.get 0
                i32.const 1051776
                i32.load
                local.tee 2
                i32.le_u
                br_if 5 (;@1;)
                i32.const 1051764
                i32.load
                local.tee 0
                i32.eqz
                br_if 5 (;@1;)
                i32.const 1051756
                i32.load
                local.tee 3
                i32.const 41
                i32.lt_u
                br_if 4 (;@2;)
                i32.const 1051464
                local.set 1
                loop  ;; label = @7
                  local.get 0
                  local.get 1
                  i32.load
                  local.tee 4
                  i32.ge_u
                  if  ;; label = @8
                    local.get 0
                    local.get 4
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
                unreachable
              end
              i32.const 1051760
              local.get 1
              i32.store
              i32.const 1051752
              i32.const 1051752
              i32.load
              local.get 0
              i32.add
              local.tee 0
              i32.store
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 0
              i32.store
              return
            end
            i32.const 0
            local.set 1
            loop  ;; label = @5
              local.get 1
              i32.const 1
              i32.add
              local.set 1
              local.get 0
              i32.load offset=8
              local.tee 0
              br_if 0 (;@5;)
            end
            i32.const 4095
            local.get 1
            local.get 1
            i32.const 4095
            i32.le_u
            select
          end
          i32.store
          return
        end
        block  ;; label = @3
          i32.const 1051744
          i32.load
          local.tee 2
          i32.const 1
          local.get 0
          i32.const 3
          i32.shr_u
          i32.shl
          local.tee 3
          i32.and
          i32.eqz
          if  ;; label = @4
            i32.const 1051744
            local.get 2
            local.get 3
            i32.or
            i32.store
            local.get 0
            i32.const 248
            i32.and
            i32.const 1051480
            i32.add
            local.tee 0
            local.set 2
            br 1 (;@3;)
          end
          local.get 0
          i32.const 248
          i32.and
          local.tee 0
          i32.const 1051480
          i32.add
          local.set 2
          local.get 0
          i32.const 1051488
          i32.add
          i32.load
          local.set 0
        end
        local.get 2
        local.get 1
        i32.store offset=8
        local.get 0
        local.get 1
        i32.store offset=12
        local.get 1
        local.get 2
        i32.store offset=12
        local.get 1
        local.get 0
        i32.store offset=8
        return
      end
      i32.const 1051784
      i32.const 1051472
      i32.load
      local.tee 0
      if (result i32)  ;; label = @2
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
        i32.const 4095
        local.get 1
        local.get 1
        i32.const 4095
        i32.le_u
        select
      else
        i32.const 4095
      end
      i32.store
      local.get 2
      local.get 3
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 1051776
      i32.const -1
      i32.store
    end)
  (func (;4;) (type 4) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 6
    global.set 0
    block (result i32)  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.const 1
        i32.and
        i32.eqz
        if  ;; label = @3
          local.get 2
          i32.load8_u
          local.tee 5
          br_if 1 (;@2;)
          i32.const 0
          br 2 (;@1;)
        end
        local.get 0
        local.get 2
        local.get 3
        i32.const 1
        i32.shr_u
        local.get 1
        i32.load offset=12
        call_indirect (type 2)
        br 1 (;@1;)
      end
      local.get 1
      i32.load offset=12
      local.set 10
      loop  ;; label = @2
        local.get 2
        i32.const 1
        i32.add
        local.set 4
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 5
                i32.extend8_s
                i32.const 0
                i32.lt_s
                if  ;; label = @7
                  local.get 5
                  i32.const 255
                  i32.and
                  local.tee 8
                  i32.const 128
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 8
                  i32.const 192
                  i32.ne
                  br_if 3 (;@4;)
                  local.get 6
                  local.get 1
                  i32.store offset=4
                  local.get 6
                  local.get 0
                  i32.store
                  local.get 6
                  i64.const 1610612768
                  i64.store offset=8 align=4
                  local.get 3
                  local.get 7
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee 2
                  i32.load
                  local.get 6
                  local.get 2
                  i32.load offset=4
                  call_indirect (type 1)
                  i32.eqz
                  br_if 2 (;@5;)
                  i32.const 1
                  br 6 (;@1;)
                end
                local.get 0
                local.get 4
                local.get 5
                i32.const 255
                i32.and
                local.tee 2
                local.get 10
                call_indirect (type 2)
                i32.eqz
                if  ;; label = @7
                  local.get 2
                  local.get 4
                  i32.add
                  local.set 2
                  br 4 (;@3;)
                end
                i32.const 1
                br 5 (;@1;)
              end
              local.get 0
              local.get 2
              i32.const 3
              i32.add
              local.tee 4
              local.get 2
              i32.load16_u offset=1 align=1
              local.tee 2
              local.get 10
              call_indirect (type 2)
              i32.eqz
              if  ;; label = @6
                local.get 2
                local.get 4
                i32.add
                local.set 2
                br 3 (;@3;)
              end
              i32.const 1
              br 4 (;@1;)
            end
            local.get 7
            i32.const 1
            i32.add
            local.set 7
            local.get 4
            local.set 2
            br 1 (;@3;)
          end
          i32.const 1610612768
          local.set 11
          local.get 5
          i32.const 1
          i32.and
          if  ;; label = @4
            local.get 2
            i32.load offset=1 align=1
            local.set 11
            local.get 2
            i32.const 5
            i32.add
            local.set 4
          end
          i32.const 0
          local.set 8
          block (result i32)  ;; label = @4
            local.get 5
            i32.const 2
            i32.and
            i32.eqz
            if  ;; label = @5
              i32.const 0
              local.set 9
              local.get 4
              br 1 (;@4;)
            end
            local.get 4
            i32.load16_u align=1
            local.set 9
            local.get 4
            i32.const 2
            i32.add
          end
          local.set 2
          local.get 5
          i32.const 4
          i32.and
          if (result i32)  ;; label = @4
            local.get 2
            i32.load16_u align=1
            local.set 8
            local.get 2
            i32.const 2
            i32.add
          else
            local.get 2
          end
          local.set 4
          local.get 5
          i32.const 8
          i32.and
          if (result i32)  ;; label = @4
            local.get 4
            i32.load16_u align=1
            local.set 7
            local.get 4
            i32.const 2
            i32.add
          else
            local.get 4
          end
          local.set 2
          local.get 5
          i32.const 16
          i32.and
          if  ;; label = @4
            local.get 3
            local.get 9
            i32.const 3
            i32.shl
            i32.add
            i32.load16_u offset=4
            local.set 9
          end
          local.get 6
          local.get 5
          i32.const 32
          i32.and
          if (result i32)  ;; label = @4
            local.get 3
            local.get 8
            i32.const 3
            i32.shl
            i32.add
            i32.load16_u offset=4
          else
            local.get 8
          end
          i32.store16 offset=14
          local.get 6
          local.get 9
          i32.store16 offset=12
          local.get 6
          local.get 11
          i32.store offset=8
          local.get 6
          local.get 1
          i32.store offset=4
          local.get 6
          local.get 0
          i32.store
          i32.const 1
          local.get 3
          local.get 7
          i32.const 3
          i32.shl
          i32.add
          local.tee 4
          i32.load
          local.get 6
          local.get 4
          i32.load offset=4
          call_indirect (type 1)
          br_if 2 (;@1;)
          drop
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
    end
    local.get 6
    i32.const 16
    i32.add
    global.set 0)
  (func (;5;) (type 0) (param i32 i32)
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
          local.get 0
          local.get 3
          i32.sub
          local.tee 0
          i32.const 1051760
          i32.load
          i32.eq
          if  ;; label = @4
            local.get 2
            i32.load offset=4
            i32.const 3
            i32.and
            i32.const 3
            i32.ne
            br_if 1 (;@3;)
            i32.const 1051752
            local.get 1
            i32.store
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
          call 7
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load offset=4
            local.tee 3
            i32.const 2
            i32.and
            i32.eqz
            if  ;; label = @5
              local.get 2
              i32.const 1051764
              i32.load
              i32.eq
              br_if 2 (;@3;)
              local.get 2
              i32.const 1051760
              i32.load
              i32.eq
              br_if 4 (;@1;)
              local.get 2
              local.get 3
              i32.const -8
              i32.and
              local.tee 2
              call 7
              local.get 0
              local.get 1
              local.get 2
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
              i32.const 1051760
              i32.load
              i32.ne
              br_if 1 (;@4;)
              i32.const 1051752
              local.get 1
              i32.store
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
          local.get 1
          i32.const 256
          i32.ge_u
          if  ;; label = @4
            local.get 0
            local.get 1
            call 8
            return
          end
          block  ;; label = @4
            i32.const 1051744
            i32.load
            local.tee 2
            i32.const 1
            local.get 1
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 3
            i32.and
            i32.eqz
            if  ;; label = @5
              i32.const 1051744
              local.get 2
              local.get 3
              i32.or
              i32.store
              local.get 1
              i32.const 248
              i32.and
              i32.const 1051480
              i32.add
              local.tee 1
              local.set 2
              br 1 (;@4;)
            end
            local.get 1
            i32.const 248
            i32.and
            local.tee 1
            i32.const 1051480
            i32.add
            local.set 2
            local.get 1
            i32.const 1051488
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
        i32.const 1051764
        local.get 0
        i32.store
        i32.const 1051756
        i32.const 1051756
        i32.load
        local.get 1
        i32.add
        local.tee 1
        i32.store
        local.get 0
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        i32.const 1051760
        i32.load
        i32.ne
        br_if 0 (;@2;)
        i32.const 1051752
        i32.const 0
        i32.store
        i32.const 1051760
        i32.const 0
        i32.store
      end
      return
    end
    i32.const 1051760
    local.get 0
    i32.store
    i32.const 1051752
    i32.const 1051752
    i32.load
    local.get 1
    i32.add
    local.tee 1
    i32.store
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
  (func (;6;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 1
      i32.const -65587
      i32.const 16
      local.get 0
      local.get 0
      i32.const 16
      i32.le_u
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
      local.tee 4
      i32.add
      i32.const 12
      i32.add
      call 1
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 8
      i32.sub
      local.set 1
      block  ;; label = @2
        local.get 0
        i32.const 1
        i32.sub
        local.tee 3
        local.get 2
        i32.and
        i32.eqz
        if  ;; label = @3
          local.get 1
          local.set 0
          br 1 (;@2;)
        end
        local.get 2
        i32.const 4
        i32.sub
        local.tee 5
        i32.load
        local.tee 6
        i32.const -8
        i32.and
        local.get 2
        local.get 3
        i32.add
        i32.const 0
        local.get 0
        i32.sub
        i32.and
        i32.const 8
        i32.sub
        local.tee 2
        local.get 0
        i32.const 0
        local.get 2
        local.get 1
        i32.sub
        i32.const 16
        i32.le_u
        select
        i32.add
        local.tee 0
        local.get 1
        i32.sub
        local.tee 2
        i32.sub
        local.set 3
        local.get 6
        i32.const 3
        i32.and
        if  ;; label = @3
          local.get 0
          local.get 3
          local.get 0
          i32.load offset=4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store offset=4
          local.get 0
          local.get 3
          i32.add
          local.tee 3
          local.get 3
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 5
          local.get 2
          local.get 5
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 1
          local.get 2
          i32.add
          local.tee 3
          local.get 3
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 2
          call 5
          br 1 (;@2;)
        end
        local.get 1
        i32.load
        local.set 1
        local.get 0
        local.get 3
        i32.store offset=4
        local.get 0
        local.get 1
        local.get 2
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
        local.get 4
        i32.const 16
        i32.add
        i32.le_u
        br_if 0 (;@2;)
        local.get 0
        local.get 4
        local.get 1
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 0
        local.get 4
        i32.add
        local.tee 1
        local.get 2
        local.get 4
        i32.sub
        local.tee 4
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
        local.get 4
        call 5
      end
      local.get 0
      i32.const 8
      i32.add
      local.set 3
    end
    local.get 3)
  (func (;7;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 0
    i32.load offset=12
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 256
          i32.ge_u
          if  ;; label = @4
            local.get 0
            i32.load offset=24
            local.set 3
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                local.get 2
                i32.eq
                if  ;; label = @7
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
              local.get 0
              i32.load offset=28
              i32.const 2
              i32.shl
              i32.const 1051336
              i32.add
              local.tee 1
              i32.load
              local.get 0
              i32.ne
              if  ;; label = @6
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
          local.get 0
          i32.load offset=8
          local.tee 0
          local.get 2
          i32.ne
          if  ;; label = @4
            local.get 0
            local.get 2
            i32.store offset=12
            local.get 2
            local.get 0
            i32.store offset=8
            return
          end
          i32.const 1051744
          i32.const 1051744
          i32.load
          i32.const -2
          local.get 1
          i32.const 3
          i32.shr_u
          i32.rotl
          i32.and
          i32.store
          return
        end
        local.get 2
        local.get 3
        i32.store offset=24
        local.get 0
        i32.load offset=16
        local.tee 1
        if  ;; label = @3
          local.get 2
          local.get 1
          i32.store offset=16
          local.get 1
          local.get 2
          i32.store offset=24
        end
        local.get 0
        i32.load offset=20
        local.tee 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        i32.store offset=20
        local.get 0
        local.get 2
        i32.store offset=24
        return
      end
      return
    end
    i32.const 1051748
    i32.const 1051748
    i32.load
    i32.const -2
    local.get 0
    i32.load offset=28
    i32.rotl
    i32.and
    i32.store)
  (func (;8;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 0
    i64.const 0
    i64.store offset=16 align=4
    local.get 0
    block (result i32)  ;; label = @1
      i32.const 0
      local.get 1
      i32.const 8
      i32.shr_u
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      drop
      i32.const 31
      local.get 1
      i32.const 16777216
      i32.ge_u
      br_if 0 (;@1;)
      drop
      local.get 1
      i32.const 38
      local.get 3
      i32.clz
      local.tee 3
      i32.sub
      i32.shr_u
      i32.const 1
      i32.and
      local.get 3
      i32.const 1
      i32.shl
      i32.or
      i32.const 62
      i32.xor
    end
    local.tee 2
    i32.store offset=28
    local.get 2
    i32.const 2
    i32.shl
    i32.const 1051336
    i32.add
    local.set 4
    i32.const 1
    local.get 2
    i32.shl
    local.tee 3
    i32.const 1051748
    i32.load
    i32.and
    i32.eqz
    if  ;; label = @1
      local.get 4
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
      i32.store offset=8
      i32.const 1051748
      i32.const 1051748
      i32.load
      local.get 3
      i32.or
      i32.store
      return
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 4
        i32.load
        local.tee 3
        i32.load offset=4
        i32.const -8
        i32.and
        i32.eq
        if  ;; label = @3
          local.get 3
          local.set 2
          br 1 (;@2;)
        end
        local.get 1
        i32.const 25
        local.get 2
        i32.const 1
        i32.shr_u
        i32.sub
        i32.const 0
        local.get 2
        i32.const 31
        i32.ne
        select
        i32.shl
        local.set 5
        loop  ;; label = @3
          local.get 3
          local.get 5
          i32.const 29
          i32.shr_u
          i32.const 4
          i32.and
          i32.add
          local.tee 4
          i32.load offset=16
          local.tee 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 5
          i32.const 1
          i32.shl
          local.set 5
          local.get 2
          local.set 3
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
      local.tee 1
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
      local.get 1
      i32.store offset=8
      return
    end
    local.get 4
    i32.const 16
    i32.add
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
    i32.store offset=8)
  (func (;9;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load offset=8
    local.set 4
    block (result i32)  ;; label = @1
      i32.const 1
      local.get 1
      i32.const 128
      i32.lt_u
      br_if 0 (;@1;)
      drop
      i32.const 2
      local.get 1
      i32.const 2048
      i32.lt_u
      br_if 0 (;@1;)
      drop
      i32.const 3
      i32.const 4
      local.get 1
      i32.const 65536
      i32.lt_u
      select
    end
    local.tee 6
    local.get 0
    i32.load
    local.get 4
    i32.sub
    i32.gt_u
    if  ;; label = @1
      local.get 0
      local.get 4
      local.get 6
      call 12
    end
    local.get 0
    i32.load offset=4
    local.get 4
    i32.add
    local.set 2
    block  ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if  ;; label = @2
        local.get 1
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 5
        local.get 1
        i32.const 6
        i32.shr_u
        local.set 3
        local.get 1
        i32.const 2048
        i32.lt_u
        if  ;; label = @3
          local.get 2
          local.get 5
          i32.store8 offset=1
          local.get 2
          local.get 3
          i32.const 192
          i32.or
          i32.store8
          br 2 (;@1;)
        end
        local.get 1
        i32.const 12
        i32.shr_u
        local.set 7
        local.get 3
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 3
        local.get 1
        i32.const 65535
        i32.le_u
        if  ;; label = @3
          local.get 2
          local.get 5
          i32.store8 offset=2
          local.get 2
          local.get 3
          i32.store8 offset=1
          local.get 2
          local.get 7
          i32.const 224
          i32.or
          i32.store8
          br 2 (;@1;)
        end
        local.get 2
        local.get 5
        i32.store8 offset=3
        local.get 2
        local.get 3
        i32.store8 offset=2
        local.get 2
        local.get 7
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        i32.store8 offset=1
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const -16
        i32.or
        i32.store8
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.store8
    end
    local.get 0
    local.get 4
    local.get 6
    i32.add
    i32.store offset=8
    i32.const 0)
  (func (;10;) (type 8) (param i32 i32 i32 i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 5
    global.set 0
    i32.const 1051332
    i32.const 1051332
    i32.load
    local.tee 6
    i32.const 1
    i32.add
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block (result i32)  ;; label = @8
                    i32.const 0
                    local.get 6
                    i32.const 0
                    i32.lt_s
                    br_if 0 (;@8;)
                    drop
                    i32.const 1
                    i32.const 1051312
                    i32.load8_u
                    br_if 0 (;@8;)
                    drop
                    i32.const 1051312
                    i32.const 1
                    i32.store8
                    i32.const 1051308
                    i32.const 1051308
                    i32.load
                    i32.const 1
                    i32.add
                    i32.store
                    i32.const 2
                  end
                  i32.const 255
                  i32.and
                  br_table 3 (;@4;) 1 (;@6;) 0 (;@7;) 1 (;@6;)
                end
                i32.const 1051320
                i32.load
                local.tee 6
                i32.const 0
                i32.lt_s
                br_if 2 (;@4;)
                local.get 6
                local.get 6
                i32.const 1
                i32.add
                local.tee 7
                i32.gt_s
                br_if 3 (;@3;)
                i32.const 1051320
                local.get 7
                i32.store
                i32.const 1051324
                i32.load
                i32.eqz
                br_if 1 (;@5;)
                local.get 5
                i32.const 8
                i32.add
                local.get 0
                local.get 1
                i32.load offset=20
                call_indirect (type 0)
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
                i32.const 1051324
                i32.load
                local.get 5
                i32.const 16
                i32.add
                i32.const 1051328
                i32.load
                i32.load offset=20
                call_indirect (type 0)
                br 1 (;@5;)
              end
              local.get 5
              local.get 0
              local.get 1
              i32.load offset=24
              call_indirect (type 0)
              unreachable
            end
            i32.const 1051320
            i32.const 1051320
            i32.load
            local.tee 0
            i32.const 1
            i32.sub
            i32.store
            local.get 0
            i32.const 0
            i32.le_s
            br_if 2 (;@2;)
            i32.const 1051312
            i32.const 0
            i32.store8
            local.get 3
            br_if 3 (;@1;)
          end
          unreachable
        end
        global.get 0
        i32.const 16
        i32.sub
        local.tee 0
        global.set 0
        local.get 0
        i32.const 28
        i32.store offset=4
        local.get 0
        i32.const 1049064
        i32.store
        local.get 0
        local.get 0
        i64.extend_i32_u
        i64.const 77309411328
        i64.or
        i64.store offset=8
        i32.const 1048716
        local.get 0
        i32.const 8
        i32.add
        i32.const 1049092
        call 18
        unreachable
      end
      i32.const 1049140
      i32.const 77
      i32.const 1049180
      call 18
      unreachable
    end
    unreachable)
  (func (;11;) (type 0) (param i32 i32)
    (local i32 i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.load
    i32.const -1
    i32.eq
    if  ;; label = @1
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
      i32.const 1048604
      local.get 3
      i32.load
      local.tee 3
      i32.load
      local.get 3
      i32.load offset=4
      call 4
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
    i32.const 12
    i32.const 4
    call 24
    local.tee 1
    i32.eqz
    if  ;; label = @1
      i32.const 4
      i32.const 12
      call 36
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
    i32.const 1049124
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set 0)
  (func (;12;) (type 3) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 2
    local.get 1
    local.get 2
    i32.add
    local.tee 4
    i32.gt_u
    if  ;; label = @1
      i32.const 0
      i32.const 0
      call 25
      unreachable
    end
    local.get 3
    i32.const 4
    i32.add
    local.set 5
    local.get 0
    i32.load
    local.tee 2
    local.set 6
    local.get 0
    i32.load offset=4
    local.set 8
    i32.const 1
    local.set 7
    i32.const 4
    local.set 1
    block  ;; label = @1
      i32.const 8
      local.get 4
      local.get 2
      i32.const 1
      i32.shl
      local.tee 2
      local.get 2
      local.get 4
      i32.lt_u
      select
      local.tee 2
      local.get 2
      i32.const 8
      i32.le_u
      select
      local.tee 4
      local.tee 2
      i32.const 2147483647
      i32.gt_u
      if  ;; label = @2
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          block (result i32)  ;; label = @4
            local.get 6
            if  ;; label = @5
              local.get 8
              local.get 6
              i32.const 1
              local.get 2
              call 2
              br 1 (;@4;)
            end
            local.get 2
            i32.eqz
            if  ;; label = @5
              i32.const 1
              local.set 1
              br 2 (;@3;)
            end
            local.get 2
            i32.const 1
            call 24
          end
          local.tee 1
          br_if 0 (;@3;)
          local.get 5
          i32.const 1
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 5
        local.get 1
        i32.store offset=4
        i32.const 0
        local.set 7
      end
      i32.const 8
      local.set 1
    end
    local.get 1
    local.get 5
    i32.add
    local.get 2
    i32.store
    local.get 5
    local.get 7
    i32.store
    local.get 3
    i32.load offset=4
    i32.const 1
    i32.eq
    if  ;; label = @1
      local.get 3
      i32.load offset=8
      local.get 3
      i32.load offset=12
      call 25
      unreachable
    end
    local.get 3
    i32.load offset=8
    local.set 1
    local.get 0
    local.get 4
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func (;13;) (type 3) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 2
    local.get 1
    local.get 2
    i32.add
    local.tee 1
    i32.gt_u
    if  ;; label = @1
      i32.const 0
      i32.const 0
      call 25
      unreachable
    end
    local.get 3
    i32.const 4
    i32.add
    local.set 4
    local.get 0
    i32.load offset=4
    local.set 6
    block (result i32)  ;; label = @1
      i32.const 8
      local.get 1
      local.get 0
      i32.load
      local.tee 2
      i32.const 1
      i32.shl
      local.tee 5
      local.get 1
      local.get 5
      i32.gt_u
      select
      local.tee 1
      local.get 1
      i32.const 8
      i32.le_u
      select
      local.tee 5
      local.tee 1
      i32.const 0
      i32.lt_s
      if  ;; label = @2
        i32.const 1
        local.set 2
        i32.const 0
        local.set 1
        i32.const 4
        br 1 (;@1;)
      end
      block (result i32)  ;; label = @2
        block  ;; label = @3
          block (result i32)  ;; label = @4
            local.get 2
            if  ;; label = @5
              local.get 6
              local.get 2
              i32.const 1
              local.get 1
              call 2
              br 1 (;@4;)
            end
            local.get 1
            i32.eqz
            if  ;; label = @5
              i32.const 1
              local.set 2
              br 2 (;@3;)
            end
            local.get 1
            i32.const 1
            call 24
          end
          local.tee 2
          br_if 0 (;@3;)
          local.get 4
          i32.const 1
          i32.store offset=4
          i32.const 1
          br 1 (;@2;)
        end
        local.get 4
        local.get 2
        i32.store offset=4
        i32.const 0
      end
      local.set 2
      i32.const 8
    end
    local.get 4
    i32.add
    local.get 1
    i32.store
    local.get 4
    local.get 2
    i32.store
    local.get 3
    i32.load offset=4
    i32.const 1
    i32.eq
    if  ;; label = @1
      local.get 3
      i32.load offset=8
      local.get 3
      i32.load offset=12
      call 25
      unreachable
    end
    local.get 3
    i32.load offset=8
    local.set 1
    local.get 0
    local.get 5
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func (;14;) (type 0) (param i32 i32)
    (local i32 i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.load
    i32.const -1
    i32.eq
    if  ;; label = @1
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
      i32.const 1048604
      local.get 3
      i32.load
      local.tee 3
      i32.load
      local.get 3
      i32.load offset=4
      call 4
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
    i32.const 1049124
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set 0)
  (func (;15;) (type 0) (param i32 i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.const 4
      i32.sub
      i32.load
      local.tee 2
      i32.const -8
      i32.and
      local.tee 3
      i32.const 4
      i32.const 8
      local.get 2
      i32.const 3
      i32.and
      local.tee 2
      select
      local.get 1
      i32.add
      i32.ge_u
      if  ;; label = @2
        local.get 2
        i32.const 0
        local.get 3
        local.get 1
        i32.const 39
        i32.add
        i32.gt_u
        select
        br_if 1 (;@1;)
        local.get 0
        call 3
        return
      end
      i32.const 1048936
      i32.const 1048984
      call 26
      unreachable
    end
    i32.const 1049000
    i32.const 1049048
    call 26
    unreachable)
  (func (;16;) (type 2) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.sub
        local.get 2
        i32.lt_u
        if  ;; label = @3
          local.get 0
          local.get 3
          local.get 2
          call 12
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
    local.get 2
    local.get 3
    i32.add
    i32.store offset=8
    i32.const 0)
  (func (;17;) (type 0) (param i32 i32)
    (local i32 i32)
    local.get 1
    i32.load offset=4
    local.set 2
    local.get 1
    i32.load
    local.set 3
    i32.const 8
    i32.const 4
    call 24
    local.tee 1
    i32.eqz
    if  ;; label = @1
      i32.const 4
      i32.const 8
      call 36
      unreachable
    end
    local.get 1
    local.get 2
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 0
    i32.const 1049108
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func (;18;) (type 3) (param i32 i32 i32)
    (local i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
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
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 3
    i32.const 20
    i32.add
    local.tee 0
    i64.load align=4
    local.set 4
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    local.get 4
    i64.store offset=4 align=4
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    local.get 1
    i32.const 4
    i32.add
    local.tee 1
    i32.load
    local.tee 2
    i32.load offset=4
    local.tee 3
    i32.const 1
    i32.and
    if  ;; label = @1
      local.get 2
      i32.load
      local.set 2
      local.get 0
      local.get 3
      i32.const 1
      i32.shr_u
      i32.store offset=4
      local.get 0
      local.get 2
      i32.store
      local.get 0
      i32.const 1048628
      local.get 1
      i32.load offset=4
      local.get 1
      i32.load offset=8
      local.tee 0
      i32.load8_u offset=8
      local.get 0
      i32.load8_u offset=9
      call 10
      unreachable
    end
    local.get 0
    i32.const -1
    i32.store
    local.get 0
    local.get 1
    i32.store offset=12
    local.get 0
    i32.const 1048656
    local.get 1
    i32.load offset=4
    local.get 1
    i32.load offset=8
    local.tee 0
    i32.load8_u offset=8
    local.get 0
    i32.load8_u offset=9
    call 10
    unreachable)
  (func (;19;) (type 1) (param i32 i32) (result i32)
    block  ;; label = @1
      local.get 1
      i32.popcnt
      i32.const 1
      i32.ne
      local.get 0
      i32.const -2147483648
      local.get 1
      i32.sub
      i32.gt_u
      i32.or
      br_if 0 (;@1;)
      local.get 0
      if  ;; label = @2
        local.get 0
        local.get 1
        call 24
        local.tee 1
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 1
      return
    end
    unreachable)
  (func (;20;) (type 1) (param i32 i32) (result i32)
    local.get 0
    i32.load
    i32.const -1
    i32.ne
    if  ;; label = @1
      local.get 1
      local.get 0
      i32.load offset=4
      local.get 0
      i32.load offset=8
      call 30
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
    call 4)
  (func (;21;) (type 4) (param i32 i32 i32 i32) (result i32)
    block  ;; label = @1
      local.get 3
      i32.popcnt
      i32.const 1
      i32.ne
      local.get 1
      i32.const -2147483648
      local.get 3
      i32.sub
      i32.gt_u
      i32.or
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 3
      local.get 2
      call 2
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      return
    end
    unreachable)
  (func (;22;) (type 9) (param i32 i32) (result i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 15
    global.set 0
    global.get 0
    i32.const 16
    i32.sub
    local.tee 16
    global.set 0
    local.get 16
    i32.const 4
    i32.add
    local.set 19
    global.get 0
    i32.const 32
    i32.sub
    local.tee 9
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.eqz
              if  ;; label = @6
                i32.const 1
                local.set 10
                br 1 (;@5;)
              end
              i32.const 1
              local.set 8
              local.get 1
              i32.const 1
              call 24
              local.tee 10
              i32.eqz
              br_if 3 (;@2;)
              block  ;; label = @6
                local.get 1
                i32.const 16
                i32.lt_u
                if  ;; label = @7
                  local.get 10
                  local.set 2
                  local.get 1
                  local.set 3
                  local.get 0
                  local.set 8
                  br 1 (;@6;)
                end
                local.get 1
                i32.const 2147483632
                i32.and
                local.set 4
                local.get 1
                local.set 3
                loop  ;; label = @7
                  local.get 5
                  local.get 10
                  i32.add
                  local.set 2
                  local.get 0
                  local.get 5
                  i32.add
                  local.tee 8
                  i32.const 1
                  i32.add
                  i32.load8_s
                  local.tee 6
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  local.get 8
                  i32.load8_s
                  local.tee 7
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  local.get 8
                  i32.const 2
                  i32.add
                  i32.load8_s
                  local.tee 11
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  local.get 8
                  i32.const 3
                  i32.add
                  i32.load8_s
                  local.tee 12
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  local.get 8
                  i32.const 4
                  i32.add
                  i32.load8_s
                  local.tee 13
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  local.get 8
                  i32.const 5
                  i32.add
                  i32.load8_s
                  local.tee 17
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  local.get 8
                  i32.const 6
                  i32.add
                  i32.load8_s
                  local.tee 14
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  local.get 8
                  i32.const 7
                  i32.add
                  i32.load8_s
                  local.tee 18
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  local.get 8
                  i32.const 8
                  i32.add
                  i32.load8_s
                  local.tee 20
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  local.get 8
                  i32.const 9
                  i32.add
                  i32.load8_s
                  local.tee 21
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  local.get 8
                  i32.const 10
                  i32.add
                  i32.load8_s
                  local.tee 22
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  local.get 8
                  i32.const 11
                  i32.add
                  i32.load8_s
                  local.tee 23
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  local.get 8
                  i32.const 12
                  i32.add
                  i32.load8_s
                  local.tee 24
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  local.get 8
                  i32.const 13
                  i32.add
                  i32.load8_s
                  local.tee 25
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  local.get 8
                  i32.const 14
                  i32.add
                  i32.load8_s
                  local.tee 26
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  local.get 8
                  i32.const 15
                  i32.add
                  i32.load8_s
                  local.tee 27
                  i32.const -1
                  i32.xor
                  i32.const 128
                  i32.and
                  i32.const 7
                  i32.shr_u
                  i32.add
                  i32.const 255
                  i32.and
                  i32.const 16
                  i32.ne
                  if  ;; label = @8
                    local.get 5
                    local.set 4
                    br 2 (;@6;)
                  end
                  local.get 2
                  i32.const 15
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 27
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 27
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 14
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 26
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 26
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 13
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 25
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 25
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 12
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 24
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 24
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 11
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 23
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 23
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 10
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 22
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 22
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 9
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 21
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 21
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 20
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 20
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 7
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 18
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 18
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 6
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 14
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 14
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 5
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 17
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 17
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 4
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 13
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 13
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 3
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 12
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 12
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 2
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 11
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 11
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 1
                  i32.add
                  i32.const 32
                  i32.const 0
                  local.get 6
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 6
                  i32.xor
                  i32.store8
                  local.get 2
                  i32.const 32
                  i32.const 0
                  local.get 7
                  i32.const 97
                  i32.sub
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  select
                  local.get 7
                  i32.xor
                  i32.store8
                  local.get 5
                  i32.const 16
                  i32.add
                  local.set 5
                  local.get 3
                  i32.const 16
                  i32.sub
                  local.tee 3
                  i32.const 15
                  i32.gt_u
                  br_if 0 (;@7;)
                end
                local.get 1
                local.get 5
                i32.eq
                br_if 1 (;@5;)
                local.get 0
                local.get 5
                i32.add
                local.set 8
                local.get 5
                local.get 10
                i32.add
                local.set 2
              end
              local.get 3
              local.get 4
              i32.add
              loop  ;; label = @6
                local.get 8
                i32.load8_s
                local.tee 6
                i32.const 0
                i32.lt_s
                br_if 2 (;@4;)
                local.get 2
                i32.const 32
                i32.const 0
                local.get 6
                i32.const 97
                i32.sub
                i32.const 255
                i32.and
                i32.const 26
                i32.lt_u
                select
                local.get 6
                i32.xor
                i32.store8
                local.get 2
                i32.const 1
                i32.add
                local.set 2
                local.get 8
                i32.const 1
                i32.add
                local.set 8
                local.get 4
                i32.const 1
                i32.add
                local.set 4
                local.get 3
                i32.const 1
                i32.sub
                local.tee 3
                br_if 0 (;@6;)
              end
              local.set 4
            end
            local.get 9
            local.get 4
            i32.store offset=16
            local.get 9
            local.get 10
            i32.store offset=12
            local.get 9
            local.get 1
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 9
          local.get 4
          i32.store offset=16
          local.get 9
          local.get 10
          i32.store offset=12
          local.get 9
          local.get 1
          i32.store offset=8
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 8
          i32.add
          local.set 17
          loop  ;; label = @4
            block (result i32)  ;; label = @5
              local.get 8
              i32.load8_s
              local.tee 3
              i32.const 0
              i32.ge_s
              if  ;; label = @6
                local.get 3
                i32.const 255
                i32.and
                local.set 2
                local.get 8
                i32.const 1
                i32.add
                br 1 (;@5;)
              end
              local.get 8
              i32.load8_u offset=1
              i32.const 63
              i32.and
              local.set 5
              local.get 3
              i32.const 31
              i32.and
              local.set 2
              local.get 3
              i32.const -33
              i32.le_u
              if  ;; label = @6
                local.get 2
                i32.const 6
                i32.shl
                local.get 5
                i32.or
                local.set 2
                local.get 8
                i32.const 2
                i32.add
                br 1 (;@5;)
              end
              local.get 8
              i32.load8_u offset=2
              i32.const 63
              i32.and
              local.get 5
              i32.const 6
              i32.shl
              i32.or
              local.set 5
              local.get 3
              i32.const -16
              i32.lt_u
              if  ;; label = @6
                local.get 5
                local.get 2
                i32.const 12
                i32.shl
                i32.or
                local.set 2
                local.get 8
                i32.const 3
                i32.add
                br 1 (;@5;)
              end
              local.get 2
              i32.const 18
              i32.shl
              i32.const 1835008
              i32.and
              local.get 8
              i32.load8_u offset=3
              i32.const 63
              i32.and
              local.get 5
              i32.const 6
              i32.shl
              i32.or
              i32.or
              local.set 2
              local.get 8
              i32.const 4
              i32.add
            end
            local.set 8
            local.get 9
            i32.const 20
            i32.add
            local.set 7
            global.get 0
            i32.const 16
            i32.sub
            local.tee 11
            global.set 0
            block  ;; label = @5
              local.get 2
              i32.const 181
              i32.ge_u
              if  ;; label = @6
                local.get 11
                i32.const 4
                i32.add
                local.set 5
                i32.const 0
                local.set 3
                block  ;; label = @7
                  local.get 2
                  i32.const 131072
                  i32.ge_u
                  if  ;; label = @8
                    local.get 5
                    i32.const -1
                    i32.store
                    br 1 (;@7;)
                  end
                  local.get 2
                  i32.const 12
                  i32.shr_u
                  i32.const 1048560
                  i32.and
                  i32.const 1051232
                  i32.add
                  local.tee 12
                  i32.load
                  local.set 13
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 12
                        i32.load offset=4
                        local.tee 6
                        br_table 2 (;@8;) 1 (;@9;) 0 (;@10;)
                      end
                      local.get 2
                      i32.const 65535
                      i32.and
                      local.set 14
                      loop  ;; label = @10
                        local.get 3
                        local.get 6
                        i32.const 1
                        i32.shr_u
                        local.tee 18
                        local.get 3
                        i32.add
                        local.tee 3
                        local.get 13
                        local.get 3
                        i32.const 6
                        i32.mul
                        i32.add
                        i32.load16_u
                        local.get 14
                        i32.gt_u
                        select
                        local.set 3
                        local.get 6
                        local.get 18
                        i32.sub
                        local.tee 6
                        i32.const 1
                        i32.gt_u
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 13
                    local.get 3
                    i32.const 6
                    i32.mul
                    i32.add
                    local.tee 3
                    i32.load16_u
                    local.tee 6
                    local.get 2
                    i32.const 65535
                    i32.and
                    local.tee 13
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 6
                    local.get 3
                    i32.const 2
                    i32.add
                    i32.load8_u
                    i32.add
                    i32.const 65535
                    i32.and
                    local.get 13
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 3
                    i32.load8_u offset=3
                    local.get 2
                    local.get 6
                    i32.xor
                    i32.and
                    i32.const 1
                    i32.and
                    br_if 0 (;@8;)
                    local.get 5
                    i64.const 0
                    i64.store offset=4 align=4
                    local.get 5
                    local.get 2
                    i32.const 65536
                    i32.and
                    local.get 3
                    i32.load16_u offset=4
                    local.get 2
                    i32.add
                    i32.const 65535
                    i32.and
                    i32.or
                    i32.store
                    br 1 (;@7;)
                  end
                  local.get 12
                  i32.load offset=8
                  local.set 13
                  i32.const 0
                  local.set 3
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 12
                          i32.load offset=12
                          local.tee 6
                          br_table 2 (;@9;) 1 (;@10;) 0 (;@11;)
                        end
                        local.get 2
                        i32.const 65535
                        i32.and
                        local.set 12
                        loop  ;; label = @11
                          local.get 3
                          local.get 6
                          i32.const 1
                          i32.shr_u
                          local.tee 14
                          local.get 3
                          i32.add
                          local.tee 3
                          local.get 13
                          local.get 3
                          i32.const 3
                          i32.shl
                          i32.add
                          i32.load16_u
                          local.get 12
                          i32.gt_u
                          select
                          local.set 3
                          local.get 6
                          local.get 14
                          i32.sub
                          local.tee 6
                          i32.const 1
                          i32.gt_u
                          br_if 0 (;@11;)
                        end
                      end
                      local.get 13
                      local.get 3
                      i32.const 3
                      i32.shl
                      i32.add
                      local.tee 3
                      i32.load16_u
                      local.get 2
                      i32.const 65535
                      i32.and
                      i32.eq
                      br_if 1 (;@8;)
                    end
                    local.get 5
                    i32.const -1
                    i32.store
                    br 1 (;@7;)
                  end
                  local.get 5
                  local.get 2
                  i32.const 65536
                  i32.and
                  local.tee 6
                  local.get 3
                  i32.load16_u offset=6
                  i32.or
                  i32.store offset=8
                  local.get 5
                  local.get 6
                  local.get 3
                  i32.load16_u offset=4
                  i32.or
                  i32.store offset=4
                  local.get 5
                  local.get 6
                  local.get 3
                  i32.load16_u offset=2
                  i32.or
                  i32.store
                end
                local.get 7
                i64.const 0
                i64.store offset=4 align=4
                local.get 7
                local.get 2
                i32.store
                local.get 11
                i32.load offset=4
                i32.const -1
                i32.eq
                br_if 1 (;@5;)
                local.get 7
                local.get 11
                i32.load offset=12
                i32.store offset=8
                local.get 7
                local.get 11
                i64.load offset=4 align=4
                i64.store align=4
                br 1 (;@5;)
              end
              local.get 7
              i64.const 0
              i64.store offset=4 align=4
              local.get 7
              local.get 2
              i32.const 95
              i32.and
              local.get 2
              local.get 2
              i32.const 97
              i32.sub
              i32.const 26
              i32.lt_u
              select
              i32.store
            end
            local.get 11
            i32.const 16
            i32.add
            global.set 0
            local.get 9
            block (result i32)  ;; label = @5
              local.get 9
              i32.load offset=24
              local.tee 3
              i32.eqz
              if  ;; label = @6
                block (result i32)  ;; label = @7
                  i32.const 1
                  local.get 9
                  i32.load offset=20
                  local.tee 2
                  i32.const 128
                  i32.lt_u
                  local.tee 5
                  br_if 0 (;@7;)
                  drop
                  i32.const 2
                  local.get 2
                  i32.const 2048
                  i32.lt_u
                  br_if 0 (;@7;)
                  drop
                  i32.const 3
                  i32.const 4
                  local.get 2
                  i32.const 65536
                  i32.lt_u
                  select
                end
                local.tee 6
                local.get 9
                i32.load offset=8
                local.get 4
                i32.sub
                i32.gt_u
                if  ;; label = @7
                  local.get 9
                  i32.const 8
                  i32.add
                  local.get 4
                  local.get 6
                  call 13
                  local.get 9
                  i32.load offset=12
                  local.set 10
                end
                local.get 4
                local.get 10
                i32.add
                local.set 3
                block  ;; label = @7
                  local.get 5
                  i32.eqz
                  if  ;; label = @8
                    local.get 2
                    i32.const 63
                    i32.and
                    i32.const -128
                    i32.or
                    local.set 5
                    local.get 2
                    i32.const 6
                    i32.shr_u
                    local.set 7
                    local.get 2
                    i32.const 2048
                    i32.lt_u
                    if  ;; label = @9
                      local.get 3
                      local.get 5
                      i32.store8 offset=1
                      local.get 3
                      local.get 7
                      i32.const 192
                      i32.or
                      i32.store8
                      br 2 (;@7;)
                    end
                    local.get 2
                    i32.const 12
                    i32.shr_u
                    local.set 11
                    local.get 7
                    i32.const 63
                    i32.and
                    i32.const -128
                    i32.or
                    local.set 7
                    local.get 2
                    i32.const 65535
                    i32.le_u
                    if  ;; label = @9
                      local.get 3
                      local.get 5
                      i32.store8 offset=2
                      local.get 3
                      local.get 7
                      i32.store8 offset=1
                      local.get 3
                      local.get 11
                      i32.const 224
                      i32.or
                      i32.store8
                      br 2 (;@7;)
                    end
                    local.get 3
                    local.get 5
                    i32.store8 offset=3
                    local.get 3
                    local.get 7
                    i32.store8 offset=2
                    local.get 3
                    local.get 11
                    i32.const 63
                    i32.and
                    i32.const -128
                    i32.or
                    i32.store8 offset=1
                    local.get 3
                    local.get 2
                    i32.const 18
                    i32.shr_u
                    i32.const -16
                    i32.or
                    i32.store8
                    br 1 (;@7;)
                  end
                  local.get 3
                  local.get 2
                  i32.store8
                end
                local.get 4
                local.get 6
                i32.add
                br 1 (;@5;)
              end
              local.get 9
              i32.load offset=20
              local.set 2
              local.get 9
              i32.load offset=28
              local.tee 5
              if  ;; label = @6
                block (result i32)  ;; label = @7
                  i32.const 1
                  local.get 2
                  i32.const 128
                  i32.lt_u
                  local.tee 6
                  br_if 0 (;@7;)
                  drop
                  i32.const 2
                  local.get 2
                  i32.const 2048
                  i32.lt_u
                  br_if 0 (;@7;)
                  drop
                  i32.const 3
                  i32.const 4
                  local.get 2
                  i32.const 65536
                  i32.lt_u
                  select
                end
                local.tee 7
                local.get 9
                i32.load offset=8
                local.get 4
                i32.sub
                i32.gt_u
                if (result i32)  ;; label = @7
                  local.get 9
                  i32.const 8
                  i32.add
                  local.get 4
                  local.get 7
                  call 13
                  local.get 9
                  i32.load offset=12
                else
                  local.get 10
                end
                local.get 4
                i32.add
                local.set 10
                block  ;; label = @7
                  local.get 6
                  i32.eqz
                  if  ;; label = @8
                    local.get 2
                    i32.const 63
                    i32.and
                    i32.const -128
                    i32.or
                    local.set 6
                    local.get 2
                    i32.const 6
                    i32.shr_u
                    local.set 11
                    local.get 2
                    i32.const 2048
                    i32.lt_u
                    if  ;; label = @9
                      local.get 10
                      local.get 6
                      i32.store8 offset=1
                      local.get 10
                      local.get 11
                      i32.const 192
                      i32.or
                      i32.store8
                      br 2 (;@7;)
                    end
                    local.get 2
                    i32.const 12
                    i32.shr_u
                    local.set 12
                    local.get 11
                    i32.const 63
                    i32.and
                    i32.const -128
                    i32.or
                    local.set 11
                    local.get 2
                    i32.const 65535
                    i32.le_u
                    if  ;; label = @9
                      local.get 10
                      local.get 6
                      i32.store8 offset=2
                      local.get 10
                      local.get 11
                      i32.store8 offset=1
                      local.get 10
                      local.get 12
                      i32.const 224
                      i32.or
                      i32.store8
                      br 2 (;@7;)
                    end
                    local.get 10
                    local.get 6
                    i32.store8 offset=3
                    local.get 10
                    local.get 11
                    i32.store8 offset=2
                    local.get 10
                    local.get 12
                    i32.const 63
                    i32.and
                    i32.const -128
                    i32.or
                    i32.store8 offset=1
                    local.get 10
                    local.get 2
                    i32.const 18
                    i32.shr_u
                    i32.const -16
                    i32.or
                    i32.store8
                    br 1 (;@7;)
                  end
                  local.get 10
                  local.get 2
                  i32.store8
                end
                local.get 9
                local.get 4
                local.get 7
                i32.add
                local.tee 2
                i32.store offset=16
                block (result i32)  ;; label = @7
                  i32.const 1
                  local.get 3
                  i32.const 128
                  i32.lt_u
                  local.tee 6
                  br_if 0 (;@7;)
                  drop
                  i32.const 2
                  local.get 3
                  i32.const 2048
                  i32.lt_u
                  br_if 0 (;@7;)
                  drop
                  i32.const 3
                  i32.const 4
                  local.get 3
                  i32.const 65536
                  i32.lt_u
                  select
                end
                local.tee 7
                local.get 9
                i32.load offset=8
                local.get 2
                i32.sub
                i32.gt_u
                if  ;; label = @7
                  local.get 9
                  i32.const 8
                  i32.add
                  local.get 2
                  local.get 7
                  call 13
                end
                local.get 9
                i32.load offset=12
                local.tee 10
                local.get 2
                i32.add
                local.set 4
                block  ;; label = @7
                  local.get 6
                  i32.eqz
                  if  ;; label = @8
                    local.get 3
                    i32.const 63
                    i32.and
                    i32.const -128
                    i32.or
                    local.set 6
                    local.get 3
                    i32.const 6
                    i32.shr_u
                    local.set 11
                    local.get 3
                    i32.const 2048
                    i32.lt_u
                    if  ;; label = @9
                      local.get 4
                      local.get 6
                      i32.store8 offset=1
                      local.get 4
                      local.get 11
                      i32.const 192
                      i32.or
                      i32.store8
                      br 2 (;@7;)
                    end
                    local.get 3
                    i32.const 12
                    i32.shr_u
                    local.set 12
                    local.get 11
                    i32.const 63
                    i32.and
                    i32.const -128
                    i32.or
                    local.set 11
                    local.get 3
                    i32.const 65535
                    i32.le_u
                    if  ;; label = @9
                      local.get 4
                      local.get 6
                      i32.store8 offset=2
                      local.get 4
                      local.get 11
                      i32.store8 offset=1
                      local.get 4
                      local.get 12
                      i32.const 224
                      i32.or
                      i32.store8
                      br 2 (;@7;)
                    end
                    local.get 4
                    local.get 6
                    i32.store8 offset=3
                    local.get 4
                    local.get 11
                    i32.store8 offset=2
                    local.get 4
                    local.get 12
                    i32.const 63
                    i32.and
                    i32.const -128
                    i32.or
                    i32.store8 offset=1
                    local.get 4
                    local.get 3
                    i32.const 18
                    i32.shr_u
                    i32.const -16
                    i32.or
                    i32.store8
                    br 1 (;@7;)
                  end
                  local.get 4
                  local.get 3
                  i32.store8
                end
                local.get 9
                local.get 2
                local.get 7
                i32.add
                local.tee 3
                i32.store offset=16
                block (result i32)  ;; label = @7
                  i32.const 1
                  local.get 5
                  i32.const 128
                  i32.lt_u
                  local.tee 6
                  br_if 0 (;@7;)
                  drop
                  i32.const 2
                  local.get 5
                  i32.const 2048
                  i32.lt_u
                  br_if 0 (;@7;)
                  drop
                  i32.const 3
                  i32.const 4
                  local.get 5
                  i32.const 65536
                  i32.lt_u
                  select
                end
                local.tee 2
                local.get 9
                i32.load offset=8
                local.get 3
                i32.sub
                i32.gt_u
                if  ;; label = @7
                  local.get 9
                  i32.const 8
                  i32.add
                  local.get 3
                  local.get 2
                  call 13
                  local.get 9
                  i32.load offset=12
                  local.set 10
                end
                local.get 3
                local.get 10
                i32.add
                local.set 4
                local.get 6
                i32.eqz
                if  ;; label = @7
                  local.get 5
                  i32.const 63
                  i32.and
                  i32.const -128
                  i32.or
                  local.set 6
                  local.get 5
                  i32.const 6
                  i32.shr_u
                  local.set 7
                  local.get 5
                  i32.const 2048
                  i32.lt_u
                  if  ;; label = @8
                    local.get 4
                    local.get 6
                    i32.store8 offset=1
                    local.get 4
                    local.get 7
                    i32.const 192
                    i32.or
                    i32.store8
                    local.get 2
                    local.get 3
                    i32.add
                    br 3 (;@5;)
                  end
                  local.get 5
                  i32.const 12
                  i32.shr_u
                  local.set 11
                  local.get 7
                  i32.const 63
                  i32.and
                  i32.const -128
                  i32.or
                  local.set 7
                  local.get 5
                  i32.const 65535
                  i32.le_u
                  if  ;; label = @8
                    local.get 4
                    local.get 6
                    i32.store8 offset=2
                    local.get 4
                    local.get 7
                    i32.store8 offset=1
                    local.get 4
                    local.get 11
                    i32.const 224
                    i32.or
                    i32.store8
                    local.get 2
                    local.get 3
                    i32.add
                    br 3 (;@5;)
                  end
                  local.get 4
                  local.get 6
                  i32.store8 offset=3
                  local.get 4
                  local.get 7
                  i32.store8 offset=2
                  local.get 4
                  local.get 11
                  i32.const 63
                  i32.and
                  i32.const -128
                  i32.or
                  i32.store8 offset=1
                  local.get 4
                  local.get 5
                  i32.const 18
                  i32.shr_u
                  i32.const -16
                  i32.or
                  i32.store8
                  local.get 2
                  local.get 3
                  i32.add
                  br 2 (;@5;)
                end
                local.get 4
                local.get 5
                i32.store8
                local.get 2
                local.get 3
                i32.add
                br 1 (;@5;)
              end
              block (result i32)  ;; label = @6
                i32.const 1
                local.get 2
                i32.const 128
                i32.lt_u
                local.tee 5
                br_if 0 (;@6;)
                drop
                i32.const 2
                local.get 2
                i32.const 2048
                i32.lt_u
                br_if 0 (;@6;)
                drop
                i32.const 3
                i32.const 4
                local.get 2
                i32.const 65536
                i32.lt_u
                select
              end
              local.tee 6
              local.get 9
              i32.load offset=8
              local.get 4
              i32.sub
              i32.gt_u
              if (result i32)  ;; label = @6
                local.get 9
                i32.const 8
                i32.add
                local.get 4
                local.get 6
                call 13
                local.get 9
                i32.load offset=12
              else
                local.get 10
              end
              local.get 4
              i32.add
              local.set 10
              block  ;; label = @6
                local.get 5
                i32.eqz
                if  ;; label = @7
                  local.get 2
                  i32.const 63
                  i32.and
                  i32.const -128
                  i32.or
                  local.set 5
                  local.get 2
                  i32.const 6
                  i32.shr_u
                  local.set 7
                  local.get 2
                  i32.const 2048
                  i32.lt_u
                  if  ;; label = @8
                    local.get 10
                    local.get 5
                    i32.store8 offset=1
                    local.get 10
                    local.get 7
                    i32.const 192
                    i32.or
                    i32.store8
                    br 2 (;@6;)
                  end
                  local.get 2
                  i32.const 12
                  i32.shr_u
                  local.set 11
                  local.get 7
                  i32.const 63
                  i32.and
                  i32.const -128
                  i32.or
                  local.set 7
                  local.get 2
                  i32.const 65535
                  i32.le_u
                  if  ;; label = @8
                    local.get 10
                    local.get 5
                    i32.store8 offset=2
                    local.get 10
                    local.get 7
                    i32.store8 offset=1
                    local.get 10
                    local.get 11
                    i32.const 224
                    i32.or
                    i32.store8
                    br 2 (;@6;)
                  end
                  local.get 10
                  local.get 5
                  i32.store8 offset=3
                  local.get 10
                  local.get 7
                  i32.store8 offset=2
                  local.get 10
                  local.get 11
                  i32.const 63
                  i32.and
                  i32.const -128
                  i32.or
                  i32.store8 offset=1
                  local.get 10
                  local.get 2
                  i32.const 18
                  i32.shr_u
                  i32.const -16
                  i32.or
                  i32.store8
                  br 1 (;@6;)
                end
                local.get 10
                local.get 2
                i32.store8
              end
              local.get 9
              local.get 4
              local.get 6
              i32.add
              local.tee 2
              i32.store offset=16
              block (result i32)  ;; label = @6
                i32.const 1
                local.get 3
                i32.const 128
                i32.lt_u
                local.tee 6
                br_if 0 (;@6;)
                drop
                i32.const 2
                local.get 3
                i32.const 2048
                i32.lt_u
                br_if 0 (;@6;)
                drop
                i32.const 3
                i32.const 4
                local.get 3
                i32.const 65536
                i32.lt_u
                select
              end
              local.tee 5
              local.get 9
              i32.load offset=8
              local.get 2
              i32.sub
              i32.gt_u
              if  ;; label = @6
                local.get 9
                i32.const 8
                i32.add
                local.get 2
                local.get 5
                call 13
              end
              local.get 9
              i32.load offset=12
              local.tee 10
              local.get 2
              i32.add
              local.set 4
              local.get 6
              i32.eqz
              if  ;; label = @6
                local.get 3
                i32.const 63
                i32.and
                i32.const -128
                i32.or
                local.set 6
                local.get 3
                i32.const 6
                i32.shr_u
                local.set 7
                local.get 3
                i32.const 2048
                i32.lt_u
                if  ;; label = @7
                  local.get 4
                  local.get 6
                  i32.store8 offset=1
                  local.get 4
                  local.get 7
                  i32.const 192
                  i32.or
                  i32.store8
                  local.get 2
                  local.get 5
                  i32.add
                  br 2 (;@5;)
                end
                local.get 3
                i32.const 12
                i32.shr_u
                local.set 11
                local.get 7
                i32.const 63
                i32.and
                i32.const -128
                i32.or
                local.set 7
                local.get 3
                i32.const 65535
                i32.le_u
                if  ;; label = @7
                  local.get 4
                  local.get 6
                  i32.store8 offset=2
                  local.get 4
                  local.get 7
                  i32.store8 offset=1
                  local.get 4
                  local.get 11
                  i32.const 224
                  i32.or
                  i32.store8
                  local.get 2
                  local.get 5
                  i32.add
                  br 2 (;@5;)
                end
                local.get 4
                local.get 6
                i32.store8 offset=3
                local.get 4
                local.get 7
                i32.store8 offset=2
                local.get 4
                local.get 11
                i32.const 63
                i32.and
                i32.const -128
                i32.or
                i32.store8 offset=1
                local.get 4
                local.get 3
                i32.const 18
                i32.shr_u
                i32.const -16
                i32.or
                i32.store8
                local.get 2
                local.get 5
                i32.add
                br 1 (;@5;)
              end
              local.get 4
              local.get 3
              i32.store8
              local.get 2
              local.get 5
              i32.add
            end
            local.tee 4
            i32.store offset=16
            local.get 8
            local.get 17
            i32.ne
            br_if 0 (;@4;)
          end
        end
        local.get 19
        local.get 9
        i32.load offset=16
        i32.store offset=8
        local.get 19
        local.get 9
        i64.load offset=8 align=4
        i64.store align=4
        local.get 9
        i32.const 32
        i32.add
        global.set 0
        br 1 (;@1;)
      end
      local.get 8
      local.get 1
      call 25
      unreachable
    end
    local.get 16
    i32.load offset=8
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 16
                    i32.load offset=12
                    i32.const 4
                    i32.sub
                    br_table 1 (;@7;) 2 (;@6;) 2 (;@6;) 0 (;@8;) 2 (;@6;)
                  end
                  local.get 3
                  i32.load align=1
                  i32.const 1145980240
                  i32.xor
                  local.get 3
                  i32.const 3
                  i32.add
                  i32.load align=1
                  i32.const 1196312900
                  i32.xor
                  i32.or
                  br_if 1 (;@6;)
                  i32.const 4
                  local.set 2
                  i32.const 4
                  i32.const 1
                  call 24
                  local.tee 4
                  br_if 3 (;@4;)
                  i32.const 1
                  i32.const 4
                  call 25
                  unreachable
                end
                local.get 3
                i32.load align=1
                i32.const 1162760004
                i32.eq
                br_if 1 (;@5;)
              end
              i32.const 20
              local.set 2
              i32.const 20
              i32.const 1
              call 24
              local.tee 4
              i32.eqz
              br_if 2 (;@3;)
              local.get 4
              i32.const 1048592
              i32.load align=1
              i32.store offset=16 align=1
              local.get 4
              i32.const 1048584
              i64.load align=1
              i64.store offset=8 align=1
              local.get 4
              i32.const 1048576
              i64.load align=1
              i64.store align=1
              br 4 (;@1;)
            end
            i32.const 6
            local.set 2
            i32.const 6
            i32.const 1
            call 24
            local.tee 4
            i32.eqz
            br_if 2 (;@2;)
            local.get 4
            i32.const 1048600
            i32.load16_u align=1
            i32.store16 offset=4 align=1
            local.get 4
            i32.const 1048596
            i32.load align=1
            i32.store align=1
            br 3 (;@1;)
          end
          local.get 4
          i32.const 1953064037
          i32.store align=1
          br 2 (;@1;)
        end
        i32.const 1
        i32.const 20
        call 25
        unreachable
      end
      i32.const 1
      i32.const 6
      call 25
      unreachable
    end
    local.get 16
    i32.load offset=4
    local.tee 8
    if  ;; label = @1
      local.get 3
      local.get 8
      call 15
    end
    local.get 1
    if  ;; label = @1
      local.get 0
      local.get 1
      call 15
    end
    local.get 15
    local.get 2
    i32.store offset=4
    local.get 15
    local.get 4
    i32.store
    local.get 16
    i32.const 16
    i32.add
    global.set 0
    local.get 15
    i32.load
    local.get 15
    i32.load offset=4
    local.get 15
    i32.const 16
    i32.add
    global.set 0)
  (func (;23;) (type 6) (param i32)
    unreachable)
  (func (;24;) (type 1) (param i32 i32) (result i32)
    local.get 1
    i32.const 9
    i32.ge_u
    if  ;; label = @1
      local.get 1
      local.get 0
      call 6
      return
    end
    local.get 0
    call 1)
  (func (;25;) (type 0) (param i32 i32)
    local.get 0
    if  ;; label = @1
      local.get 0
      local.get 1
      call 36
      unreachable
    end
    i32.const 1049196
    i32.const 35
    i32.const 1049216
    call 18
    unreachable)
  (func (;26;) (type 0) (param i32 i32)
    local.get 0
    i32.const 93
    local.get 1
    call 18
    unreachable)
  (func (;27;) (type 3) (param i32 i32 i32)
    local.get 1
    if  ;; label = @1
      local.get 0
      local.get 1
      call 15
    end)
  (func (;28;) (type 0) (param i32 i32)
    local.get 0
    i32.const 1048708
    i64.load align=4
    i64.store offset=8 align=4
    local.get 0
    i32.const 1048700
    i64.load align=4
    i64.store align=4)
  (func (;29;) (type 0) (param i32 i32)
    local.get 0
    i32.const 1048692
    i64.load align=4
    i64.store offset=8 align=4
    local.get 0
    i32.const 1048684
    i64.load align=4
    i64.store align=4)
  (func (;30;) (type 2) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 2
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 2))
  (func (;31;) (type 0) (param i32 i32)
    local.get 0
    i32.const 1049108
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func (;32;) (type 1) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call 30)
  (func (;33;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load
    local.set 7
    local.get 0
    i32.load offset=4
    local.set 6
    i32.const 0
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.tee 8
        i32.load offset=8
        local.tee 12
        i32.const 402653184
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 12
                i32.const 268435456
                i32.and
                if  ;; label = @7
                  local.get 1
                  i32.load16_u offset=14
                  local.tee 1
                  br_if 1 (;@6;)
                  i32.const 0
                  local.set 6
                  br 2 (;@5;)
                end
                local.get 6
                i32.const 16
                i32.ge_u
                if  ;; label = @7
                  block (result i32)  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 6
                        local.get 7
                        i32.const 3
                        i32.add
                        i32.const -4
                        i32.and
                        local.tee 1
                        local.get 7
                        i32.sub
                        local.tee 9
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 6
                        local.get 9
                        i32.sub
                        local.tee 11
                        i32.const 2
                        i32.shr_u
                        local.tee 10
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 7
                        i32.ne
                        if  ;; label = @11
                          local.get 7
                          local.get 1
                          i32.sub
                          local.tee 5
                          i32.const -4
                          i32.le_u
                          if  ;; label = @12
                            loop  ;; label = @13
                              local.get 0
                              local.get 2
                              local.get 7
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
                              local.set 0
                              local.get 2
                              i32.const 4
                              i32.add
                              local.tee 2
                              br_if 0 (;@13;)
                            end
                          end
                          local.get 2
                          local.get 7
                          i32.add
                          local.set 4
                          loop  ;; label = @12
                            local.get 0
                            local.get 4
                            i32.load8_s
                            i32.const -65
                            i32.gt_s
                            i32.add
                            local.set 0
                            local.get 4
                            i32.const 1
                            i32.add
                            local.set 4
                            local.get 5
                            i32.const 1
                            i32.add
                            local.tee 5
                            br_if 0 (;@12;)
                          end
                        end
                        local.get 7
                        local.get 9
                        i32.add
                        local.set 5
                        block  ;; label = @11
                          local.get 11
                          i32.const 3
                          i32.and
                          local.tee 1
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 5
                          local.get 11
                          i32.const 2147483644
                          i32.and
                          i32.add
                          local.tee 2
                          i32.load8_s
                          i32.const -65
                          i32.gt_s
                          local.set 3
                          local.get 1
                          i32.const 1
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 3
                          local.get 2
                          i32.load8_s offset=1
                          i32.const -65
                          i32.gt_s
                          i32.add
                          local.set 3
                          local.get 1
                          i32.const 2
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 3
                          local.get 2
                          i32.load8_s offset=2
                          i32.const -65
                          i32.gt_s
                          i32.add
                          local.set 3
                        end
                        local.get 0
                        local.get 3
                        i32.add
                        local.set 2
                        loop  ;; label = @11
                          local.get 5
                          local.set 1
                          local.get 10
                          i32.eqz
                          br_if 2 (;@9;)
                          i32.const 192
                          local.get 10
                          local.get 10
                          i32.const 192
                          i32.ge_u
                          select
                          local.tee 3
                          i32.const 3
                          i32.and
                          local.set 9
                          block  ;; label = @12
                            local.get 3
                            i32.const 2
                            i32.shl
                            local.tee 5
                            i32.const 1008
                            i32.and
                            local.tee 0
                            i32.eqz
                            if  ;; label = @13
                              i32.const 0
                              local.set 4
                              br 1 (;@12;)
                            end
                            local.get 0
                            local.get 1
                            i32.add
                            local.set 11
                            i32.const 0
                            local.set 4
                            local.get 1
                            local.set 0
                            loop  ;; label = @13
                              local.get 4
                              local.get 0
                              i32.load
                              local.tee 13
                              i32.const -1
                              i32.xor
                              i32.const 7
                              i32.shr_u
                              local.get 13
                              i32.const 6
                              i32.shr_u
                              i32.or
                              i32.const 16843009
                              i32.and
                              i32.add
                              local.get 0
                              i32.const 4
                              i32.add
                              i32.load
                              local.tee 4
                              i32.const -1
                              i32.xor
                              i32.const 7
                              i32.shr_u
                              local.get 4
                              i32.const 6
                              i32.shr_u
                              i32.or
                              i32.const 16843009
                              i32.and
                              i32.add
                              local.get 0
                              i32.const 8
                              i32.add
                              i32.load
                              local.tee 4
                              i32.const -1
                              i32.xor
                              i32.const 7
                              i32.shr_u
                              local.get 4
                              i32.const 6
                              i32.shr_u
                              i32.or
                              i32.const 16843009
                              i32.and
                              i32.add
                              local.get 0
                              i32.const 12
                              i32.add
                              i32.load
                              local.tee 4
                              i32.const -1
                              i32.xor
                              i32.const 7
                              i32.shr_u
                              local.get 4
                              i32.const 6
                              i32.shr_u
                              i32.or
                              i32.const 16843009
                              i32.and
                              i32.add
                              local.set 4
                              local.get 0
                              i32.const 16
                              i32.add
                              local.tee 0
                              local.get 11
                              i32.ne
                              br_if 0 (;@13;)
                            end
                          end
                          local.get 10
                          local.get 3
                          i32.sub
                          local.set 10
                          local.get 1
                          local.get 5
                          i32.add
                          local.set 5
                          local.get 4
                          i32.const 8
                          i32.shr_u
                          i32.const 16711935
                          i32.and
                          local.get 4
                          i32.const 16711935
                          i32.and
                          i32.add
                          i32.const 65537
                          i32.mul
                          i32.const 16
                          i32.shr_u
                          local.get 2
                          i32.add
                          local.set 2
                          local.get 9
                          i32.eqz
                          br_if 0 (;@11;)
                        end
                        block (result i32)  ;; label = @11
                          local.get 1
                          local.get 3
                          i32.const 252
                          i32.and
                          i32.const 2
                          i32.shl
                          i32.add
                          local.tee 0
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
                          local.tee 1
                          local.get 9
                          i32.const 1
                          i32.eq
                          br_if 0 (;@11;)
                          drop
                          local.get 1
                          local.get 0
                          i32.load offset=4
                          local.tee 3
                          i32.const -1
                          i32.xor
                          i32.const 7
                          i32.shr_u
                          local.get 3
                          i32.const 6
                          i32.shr_u
                          i32.or
                          i32.const 16843009
                          i32.and
                          i32.add
                          local.tee 1
                          local.get 9
                          i32.const 2
                          i32.eq
                          br_if 0 (;@11;)
                          drop
                          local.get 0
                          i32.load offset=8
                          local.tee 0
                          i32.const -1
                          i32.xor
                          i32.const 7
                          i32.shr_u
                          local.get 0
                          i32.const 6
                          i32.shr_u
                          i32.or
                          i32.const 16843009
                          i32.and
                          local.get 1
                          i32.add
                        end
                        local.tee 0
                        i32.const 8
                        i32.shr_u
                        i32.const 459007
                        i32.and
                        local.get 0
                        i32.const 16711935
                        i32.and
                        i32.add
                        i32.const 65537
                        i32.mul
                        i32.const 16
                        i32.shr_u
                        local.get 2
                        i32.add
                        local.set 2
                        br 1 (;@9;)
                      end
                      i32.const 0
                      local.get 6
                      i32.eqz
                      br_if 1 (;@8;)
                      drop
                      local.get 6
                      i32.const 3
                      i32.and
                      local.set 4
                      local.get 6
                      i32.const 4
                      i32.ge_u
                      if  ;; label = @10
                        local.get 6
                        i32.const -4
                        i32.and
                        local.set 1
                        loop  ;; label = @11
                          local.get 2
                          local.get 5
                          local.get 7
                          i32.add
                          local.tee 0
                          i32.load8_s
                          i32.const -65
                          i32.gt_s
                          i32.add
                          local.get 0
                          i32.const 1
                          i32.add
                          i32.load8_s
                          i32.const -65
                          i32.gt_s
                          i32.add
                          local.get 0
                          i32.const 2
                          i32.add
                          i32.load8_s
                          i32.const -65
                          i32.gt_s
                          i32.add
                          local.get 0
                          i32.const 3
                          i32.add
                          i32.load8_s
                          i32.const -65
                          i32.gt_s
                          i32.add
                          local.set 2
                          local.get 1
                          local.get 5
                          i32.const 4
                          i32.add
                          local.tee 5
                          i32.ne
                          br_if 0 (;@11;)
                        end
                        local.get 4
                        i32.eqz
                        br_if 1 (;@9;)
                      end
                      local.get 5
                      local.get 7
                      i32.add
                      local.set 0
                      loop  ;; label = @10
                        local.get 2
                        local.get 0
                        i32.load8_s
                        i32.const -65
                        i32.gt_s
                        i32.add
                        local.set 2
                        local.get 0
                        i32.const 1
                        i32.add
                        local.set 0
                        local.get 4
                        i32.const 1
                        i32.sub
                        local.tee 4
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 2
                  end
                  local.set 2
                  br 4 (;@3;)
                end
                local.get 6
                i32.eqz
                br_if 3 (;@3;)
                local.get 6
                i32.const 3
                i32.and
                local.set 0
                local.get 6
                i32.const 4
                i32.ge_u
                if  ;; label = @7
                  local.get 6
                  i32.const 12
                  i32.and
                  local.set 5
                  loop  ;; label = @8
                    local.get 2
                    local.get 3
                    local.get 7
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
                    local.set 2
                    local.get 5
                    local.get 3
                    i32.const 4
                    i32.add
                    local.tee 3
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  local.get 0
                  i32.eqz
                  br_if 4 (;@3;)
                end
                local.get 3
                local.get 7
                i32.add
                local.set 3
                loop  ;; label = @7
                  local.get 2
                  local.get 3
                  i32.load8_s
                  i32.const -65
                  i32.gt_s
                  i32.add
                  local.set 2
                  local.get 3
                  i32.const 1
                  i32.add
                  local.set 3
                  local.get 0
                  i32.const 1
                  i32.sub
                  local.tee 0
                  br_if 0 (;@7;)
                end
                br 3 (;@3;)
              end
              local.get 6
              local.get 7
              i32.add
              local.set 4
              i32.const 0
              local.set 6
              local.get 7
              local.set 3
              local.get 1
              local.set 0
              loop  ;; label = @6
                local.get 3
                local.tee 5
                local.get 4
                i32.eq
                br_if 2 (;@4;)
                local.get 6
                block (result i32)  ;; label = @7
                  local.get 3
                  i32.const 1
                  i32.add
                  local.get 3
                  i32.load8_s
                  local.tee 2
                  i32.const 0
                  i32.ge_s
                  br_if 0 (;@7;)
                  drop
                  local.get 3
                  i32.const 2
                  i32.add
                  local.get 2
                  i32.const -32
                  i32.lt_u
                  br_if 0 (;@7;)
                  drop
                  local.get 3
                  i32.const 4
                  i32.const 3
                  local.get 2
                  i32.const -17
                  i32.gt_u
                  select
                  i32.add
                end
                local.tee 3
                local.get 5
                i32.sub
                i32.add
                local.set 6
                local.get 0
                i32.const 1
                i32.sub
                local.tee 0
                br_if 0 (;@6;)
              end
            end
            i32.const 0
            local.set 0
          end
          local.get 1
          local.get 0
          i32.sub
          local.set 2
        end
        local.get 2
        local.get 8
        i32.load16_u offset=12
        local.tee 0
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.get 2
        i32.sub
        local.set 1
        i32.const 0
        local.set 2
        i32.const 0
        local.set 0
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 12
              i32.const 29
              i32.shr_u
              i32.const 3
              i32.and
              i32.const 1
              i32.sub
              br_table 0 (;@5;) 1 (;@4;) 2 (;@3;)
            end
            local.get 1
            local.set 0
            br 1 (;@3;)
          end
          local.get 1
          i32.const 65534
          i32.and
          i32.const 1
          i32.shr_u
          local.set 0
        end
        local.get 12
        i32.const 2097151
        i32.and
        local.set 4
        local.get 8
        i32.load offset=4
        local.set 5
        local.get 8
        i32.load
        local.set 8
        loop  ;; label = @3
          local.get 2
          i32.const 65535
          i32.and
          local.get 0
          i32.const 65535
          i32.and
          i32.lt_u
          if  ;; label = @4
            i32.const 1
            local.set 3
            local.get 2
            i32.const 1
            i32.add
            local.set 2
            local.get 8
            local.get 4
            local.get 5
            i32.load offset=16
            call_indirect (type 1)
            i32.eqz
            br_if 1 (;@3;)
            br 3 (;@1;)
          end
        end
        i32.const 1
        local.set 3
        local.get 8
        local.get 7
        local.get 6
        local.get 5
        i32.load offset=12
        call_indirect (type 2)
        br_if 1 (;@1;)
        i32.const 0
        local.set 2
        local.get 1
        local.get 0
        i32.sub
        i32.const 65535
        i32.and
        local.set 0
        loop  ;; label = @3
          local.get 2
          i32.const 65535
          i32.and
          local.tee 1
          local.get 0
          i32.lt_u
          local.set 3
          local.get 0
          local.get 1
          i32.le_u
          br_if 2 (;@1;)
          local.get 2
          i32.const 1
          i32.add
          local.set 2
          local.get 8
          local.get 4
          local.get 5
          i32.load offset=16
          call_indirect (type 1)
          i32.eqz
          br_if 0 (;@3;)
        end
        br 1 (;@1;)
      end
      local.get 8
      i32.load
      local.get 7
      local.get 6
      local.get 8
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 2)
      local.set 3
    end
    local.get 3)
  (func (;34;) (type 2) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 1048604
    local.get 1
    local.get 2
    call 4)
  (func (;35;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    i64.load align=4
    i64.store)
  (func (;36;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.store offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    local.get 2
    i32.const 8
    i32.add
    local.tee 0
    i32.load
    local.get 0
    i32.load offset=4
    i32.const 1051316
    i32.load
    local.tee 0
    i32.const 2
    local.get 0
    select
    call_indirect (type 0)
    unreachable)
  (func (;37;) (type 0) (param i32 i32)
    i32.const 1051788
    i32.const 1
    i32.store8)
  (func (;38;) (type 0) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store)
  (func (;39;) (type 5)
    unreachable)
  (table (;0;) 19 19 funcref)
  (table (;1;) 1028 externref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1051264))
  (global (;2;) i32 (i32.const 1051304))
  (export "memory" (memory 0))
  (export "rust_generate_button_text" (func 22))
  (export "__abort_handler" (global 1))
  (export "__instance_terminated" (global 2))
  (export "__wbindgen_externrefs" (table 1))
  (export "__wbindgen_malloc" (func 19))
  (export "__wbindgen_realloc" (func 21))
  (export "__wbindgen_free" (func 27))
  (export "__wbindgen_start" (func 0))
  (elem (;0;) (i32.const 1) func 39 37 23 16 9 34 32 17 31 35 23 20 11 14 38 29 28 33)
  (data (;0;) (i32.const 1048576) "an error has occureddelete\00\00\03\00\00\00\0c\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00\00\00\00\00\08\00\00\00\04\00\00\00\07\00\00\00\08\00\00\00\09\00\00\00\0a\00\00\00\0b\00\00\00\10\00\00\00\04\00\00\00\0c\00\00\00\0d\00\00\00\0e\00\00\00\0f\00\00\00\5c\f6\e9_\dc\02\f6\b9\f1\c1pl\f2a\c1$\da\07\8cIxeL\d3\c2}\8fM\96\9f&\cf\c0\00/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library/std/src/sys/sync/rwlock/no_threads.rs\00/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library/alloc/src/raw_vec/mod.rs\00/rust/deps/dlmalloc-0.2.13/src/dlmalloc.rs\00assertion failed: psize >= size + min_overhead\00\00=\01\10\00*\00\00\00\b1\04\00\00\09\00\00\00assertion failed: psize <= size + max_overhead\00\00=\01\10\00*\00\00\00\b7\04\00\00\0d\00\00\00rwlock overflowed read locks\8e\00\10\00]\00\00\00\15\00\00\00,\00\00\00\00\00\00\00\08\00\00\00\04\00\00\00\10\00\00\00\03\00\00\00\0c\00\00\00\04\00\00\00\11\00\00\00rwlock has not been locked for reading\00\00\8e\00\10\00]\00\00\00>\00\00\00\09\00\00\00capacity overflow\00\00\00\ec\00\10\00P\00\00\00\1c\00\00\00\05\00\00\00\b5\00\00\00\e7\02\e0\00\16\00\e0\ff\f8\00\06\00\e0\ff\ff\00\00\00y\00\01\01.\01\ff\ff1\01\00\00\18\ff3\01\04\01\ff\ff:\01\0e\01\ff\ffK\01,\01\ff\ffz\01\04\01\ff\ff\7f\01\00\00\d4\fe\80\01\00\00\c3\00\83\01\02\01\ff\ff\88\01\00\00\ff\ff\8c\01\00\00\ff\ff\92\01\00\00\ff\ff\95\01\00\00a\00\99\01\00\00\ff\ff\9a\01\00\00\a3\00\9b\01\00\00A\a6\9e\01\00\00\82\00\a1\01\04\01\ff\ff\a8\01\00\00\ff\ff\ad\01\00\00\ff\ff\b0\01\00\00\ff\ff\b4\01\02\01\ff\ff\b9\01\00\00\ff\ff\bd\01\00\00\ff\ff\bf\01\00\008\00\c5\01\00\00\ff\ff\c6\01\00\00\fe\ff\c8\01\00\00\ff\ff\c9\01\00\00\fe\ff\cb\01\00\00\ff\ff\cc\01\00\00\fe\ff\ce\01\0e\01\ff\ff\dd\01\00\00\b1\ff\df\01\10\01\ff\ff\f2\01\00\00\ff\ff\f3\01\00\00\fe\ff\f5\01\00\00\ff\ff\f9\01&\01\ff\ff#\02\10\01\ff\ff<\02\00\00\ff\ff?\02\01\00?*B\02\00\00\ff\ffG\02\08\01\ff\ffP\02\00\00\1f*Q\02\00\00\1c*R\02\00\00\1e*S\02\00\00.\ffT\02\00\002\ffV\02\01\003\ffY\02\00\006\ff[\02\00\005\ff\5c\02\00\00O\a5`\02\00\003\ffa\02\00\00K\a5c\02\00\001\ffd\02\00\00g\a5e\02\00\00(\a5f\02\00\00D\a5h\02\00\00/\ffi\02\00\00-\ffj\02\00\00D\a5k\02\00\00\f7)l\02\00\00A\a5o\02\00\00-\ffq\02\00\00\fd)r\02\00\00+\ffu\02\00\00*\ff}\02\00\00\e7)\80\02\00\00&\ff\82\02\00\00C\a5\83\02\00\00&\ff\87\02\00\00*\a5\88\02\00\00&\ff\89\02\00\00\bb\ff\8a\02\01\00'\ff\8c\02\00\00\b9\ff\92\02\00\00%\ff\9d\02\00\00\15\a5\9e\02\00\00\12\a5E\03\00\00T\00q\03\02\01\ff\ffw\03\00\00\ff\ff{\03\02\00\82\00\ac\03\00\00\da\ff\ad\03\02\00\db\ff\b1\03\10\00\e0\ff\c2\03\00\00\e1\ff\c3\03\08\00\e0\ff\cc\03\00\00\c0\ff\cd\03\01\00\c1\ff\d0\03\00\00\c2\ff\d1\03\00\00\c7\ff\d5\03\00\00\d1\ff\d6\03\00\00\ca\ff\d7\03\00\00\f8\ff\d9\03\16\01\ff\ff\f0\03\00\00\aa\ff\f1\03\00\00\b0\ff\f2\03\00\00\07\00\f3\03\00\00\8c\ff\f5\03\00\00\a0\ff\f8\03\00\00\ff\ff\fb\03\00\00\ff\ff0\04\1f\00\e0\ffP\04\0f\00\b0\ffa\04 \01\ff\ff\8b\044\01\ff\ff\c2\04\0c\01\ff\ff\cf\04\00\00\f1\ff\d1\04^\01\ff\ffa\05%\00\d0\ff\d0\10*\00\c0\0b\fd\10\02\00\c0\0b\f8\13\05\00\f8\ff\80\1c\00\00\92\e7\81\1c\00\00\93\e7\82\1c\00\00\9c\e7\83\1c\01\00\9e\e7\85\1c\00\00\9d\e7\86\1c\00\00\a4\e7\87\1c\00\00\db\e7\88\1c\00\00\c2\89\8a\1c\00\00\ff\ffy\1d\00\00\04\8a}\1d\00\00\e6\0e\8e\1d\00\008\8a\01\1e\94\01\ff\ff\9b\1e\00\00\c5\ff\a1\1e^\01\ff\ff\00\1f\07\00\08\00\10\1f\05\00\08\00 \1f\07\00\08\000\1f\07\00\08\00@\1f\05\00\08\00Q\1f\06\01\08\00`\1f\07\00\08\00p\1f\01\00J\00r\1f\03\00V\00v\1f\01\00d\00x\1f\01\00\80\00z\1f\01\00p\00|\1f\01\00~\00\b0\1f\01\00\08\00\be\1f\00\00\db\e3\d0\1f\01\00\08\00\e0\1f\01\00\08\00\e5\1f\00\00\07\00N!\00\00\e4\ffp!\0f\00\f0\ff\84!\00\00\ff\ff\d0$\19\00\e6\ff0,/\00\d0\ffa,\00\00\ff\ffe,\00\00\d5\d5f,\00\00\d8\d5h,\04\01\ff\ffs,\00\00\ff\ffv,\00\00\ff\ff\81,b\01\ff\ff\ec,\02\01\ff\ff\f3,\00\00\ff\ff\00-%\00\a0\e3'-\00\00\a0\e3--\00\00\a0\e3A\a6,\01\ff\ff\81\a6\1a\01\ff\ff#\a7\0c\01\ff\ff3\a7<\01\ff\ffz\a7\02\01\ff\ff\7f\a7\08\01\ff\ff\8c\a7\00\00\ff\ff\91\a7\02\01\ff\ff\94\a7\00\000\00\97\a7\12\01\ff\ff\b5\a7\0e\01\ff\ff\c8\a7\02\01\ff\ff\cd\a7\0e\01\ff\ff\f6\a7\00\00\ff\ffS\ab\00\00`\fcp\abO\000hA\ff\19\00\e0\ff\df\00S\00S\00\00\00I\01\bc\02N\00\00\00\f0\01J\00\0c\03\00\00\90\03\99\03\08\03\01\03\b0\03\a5\03\08\03\01\03\87\055\05R\05\00\00\96\1eH\001\03\00\00\97\1eT\00\08\03\00\00\98\1eW\00\0a\03\00\00\99\1eY\00\0a\03\00\00\9a\1eA\00\be\02\00\00P\1f\a5\03\13\03\00\00R\1f\a5\03\13\03\00\03T\1f\a5\03\13\03\01\03V\1f\a5\03\13\03B\03\80\1f\08\1f\99\03\00\00\81\1f\09\1f\99\03\00\00\82\1f\0a\1f\99\03\00\00\83\1f\0b\1f\99\03\00\00\84\1f\0c\1f\99\03\00\00\85\1f\0d\1f\99\03\00\00\86\1f\0e\1f\99\03\00\00\87\1f\0f\1f\99\03\00\00\88\1f\08\1f\99\03\00\00\89\1f\09\1f\99\03\00\00\8a\1f\0a\1f\99\03\00\00\8b\1f\0b\1f\99\03\00\00\8c\1f\0c\1f\99\03\00\00\8d\1f\0d\1f\99\03\00\00\8e\1f\0e\1f\99\03\00\00\8f\1f\0f\1f\99\03\00\00\90\1f(\1f\99\03\00\00\91\1f)\1f\99\03\00\00\92\1f*\1f\99\03\00\00\93\1f+\1f\99\03\00\00\94\1f,\1f\99\03\00\00\95\1f-\1f\99\03\00\00\96\1f.\1f\99\03\00\00\97\1f/\1f\99\03\00\00\98\1f(\1f\99\03\00\00\99\1f)\1f\99\03\00\00\9a\1f*\1f\99\03\00\00\9b\1f+\1f\99\03\00\00\9c\1f,\1f\99\03\00\00\9d\1f-\1f\99\03\00\00\9e\1f.\1f\99\03\00\00\9f\1f/\1f\99\03\00\00\a0\1fh\1f\99\03\00\00\a1\1fi\1f\99\03\00\00\a2\1fj\1f\99\03\00\00\a3\1fk\1f\99\03\00\00\a4\1fl\1f\99\03\00\00\a5\1fm\1f\99\03\00\00\a6\1fn\1f\99\03\00\00\a7\1fo\1f\99\03\00\00\a8\1fh\1f\99\03\00\00\a9\1fi\1f\99\03\00\00\aa\1fj\1f\99\03\00\00\ab\1fk\1f\99\03\00\00\ac\1fl\1f\99\03\00\00\ad\1fm\1f\99\03\00\00\ae\1fn\1f\99\03\00\00\af\1fo\1f\99\03\00\00\b2\1f\ba\1f\99\03\00\00\b3\1f\91\03\99\03\00\00\b4\1f\86\03\99\03\00\00\b6\1f\91\03B\03\00\00\b7\1f\91\03B\03\99\03\bc\1f\91\03\99\03\00\00\c2\1f\ca\1f\99\03\00\00\c3\1f\97\03\99\03\00\00\c4\1f\89\03\99\03\00\00\c6\1f\97\03B\03\00\00\c7\1f\97\03B\03\99\03\cc\1f\97\03\99\03\00\00\d2\1f\99\03\08\03\00\03\d3\1f\99\03\08\03\01\03\d6\1f\99\03B\03\00\00\d7\1f\99\03\08\03B\03\e2\1f\a5\03\08\03\00\03\e3\1f\a5\03\08\03\01\03\e4\1f\a1\03\13\03\00\00\e6\1f\a5\03B\03\00\00\e7\1f\a5\03\08\03B\03\f2\1f\fa\1f\99\03\00\00\f3\1f\a9\03\99\03\00\00\f4\1f\8f\03\99\03\00\00\f6\1f\a9\03B\03\00\00\f7\1f\a9\03B\03\99\03\fc\1f\a9\03\99\03\00\00\00\fbF\00F\00\00\00\01\fbF\00I\00\00\00\02\fbF\00L\00\00\00\03\fbF\00F\00I\00\04\fbF\00F\00L\00\05\fbS\00T\00\00\00\06\fbS\00T\00\00\00\13\fbD\05F\05\00\00\14\fbD\055\05\00\00\15\fbD\05;\05\00\00\16\fbN\05F\05\00\00\17\fbD\05=\05\00\00(\04'\00\d8\ff\d8\04#\00\d8\ff\97\05\0a\00\d9\ff\a3\05\0e\00\d9\ff\b3\05\06\00\d9\ff\bb\05\01\00\d9\ff\c0\0c2\00\c0\ffp\0d\15\00\e0\ff\c0\18\1f\00\e0\ff`n\1f\00\e0\ff\bbn\18\00\e5\ff\22\e9!\00\de\ff\00\00\90\02\10\00\b9\00\00\00\e6\06\10\00f\00\00\00\16\0a\10\00\0c\00\00\00\02")
  (data (;1;) (i32.const 1051264) "\01\00\00\00\04")
  (@custom "producers" "\02\08language\01\04Rust\00\0cprocessed-by\03\05rustc\1d1.97.1 (8bab26f4f 2026-07-14)\06walrus\060.27.3\0cwasm-bindgen\070.2.130")
  (@custom "target_features" "\08+\0fmutable-globals+\13nontrapping-fptoint+\0bbulk-memory+\08sign-ext+\0freference-types+\0amultivalue+\0fbulk-memory-opt+\16call-indirect-overlong"))
