@.fstr = private constant [2 x i8] c"%d" 
@.init = private unnamed_addr constant i32 8

declare i32 @printf(i8*, ...)

define i32 @main() {
start:
  %0 = load i32, i32* @.init
  %1 = call i32 @fib(i32 %0)
;   call i32 (i8*, ...)* @printf(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.fstr , i32 0, i32 0), i32 %1)
  call i32 (i8*, ...)* @printf(i8* @.fstr, i32 %1)
  ret i32 1
}

define i32 @fib(i32 %n) {
start:
    %0 = add i32 %n, 3
    ret i32 %0
}