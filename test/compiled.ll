@.helloWorld = private unnamed_addr constant [14 x i8] c"Hello World!\n"
declare i32 @printf(ptr nocapture) nounwind
define i32 @main() {
call i32 @printf(ptr @.helloWorld)
ret i32 0
}