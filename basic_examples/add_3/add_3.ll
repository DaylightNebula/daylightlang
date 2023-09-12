@.fstr = private constant [2 x i8] c"%d"            ; format string
@.init = private unnamed_addr constant i32 8        ; initial value

declare i32 @printf(i8*, ...)                       ; external printf

define i32 @main() {                                ; define main function
start:                                              ; llvm required branch label
  %0 = load i32, i32* @.init                        ; get init (llvm thing)
  %1 = call i32 @test(i32 %0)                       ; call test function from loaded value
  call i32 (i8*, ...)* @printf(i8* @.fstr, i32 %1)  ; call printf with format string and test value
  ret i32 1                                         ; return ok (llvm thing)
}

define i32 @test(i32 %n) {                          ; define test function
start:                                              ; llvm required branch label
    %0 = add i32 %n, 3                              ; add 3 to input
    ret i32 %0                                      ; return new value
}