; --------------------------
; 128-bit overflow operations
; --------------------------

; Multiplication
declare { i128, i1 } @llvm.umul.with.overflow.i128(i128, i128)
declare { i128, i1 } @llvm.smul.with.overflow.i128(i128, i128)

define { i128, i1 } @of_mul_u128(i128 %0, i128 %1) {
entry:
  %2 = call { i128, i1 } @llvm.umul.with.overflow.i128(i128 %0, i128 %1)
  ret { i128, i1 } %2
}

define { i128, i1 } @of_mul_i128(i128 %0, i128 %1) {
entry:
  %2 = call { i128, i1 } @llvm.smul.with.overflow.i128(i128 %0, i128 %1)
  ret { i128, i1 } %2
}

; Addition
declare { i128, i1 } @llvm.uadd.with.overflow.i128(i128, i128)
declare { i128, i1 } @llvm.sadd.with.overflow.i128(i128, i128)

define { i128, i1 } @of_add_u128(i128 %0, i128 %1) {
entry:
  %2 = call { i128, i1 } @llvm.uadd.with.overflow.i128(i128 %0, i128 %1)
  ret { i128, i1 } %2
}

define { i128, i1 } @of_add_i128(i128 %0, i128 %1) {
entry:
  %2 = call { i128, i1 } @llvm.sadd.with.overflow.i128(i128 %0, i128 %1)
  ret { i128, i1 } %2
}

; Subtraction
declare { i128, i1 } @llvm.usub.with.overflow.i128(i128, i128)
declare { i128, i1 } @llvm.ssub.with.overflow.i128(i128, i128)

define { i128, i1 } @of_sub_u128(i128 %0, i128 %1) {
entry:
  %2 = call { i128, i1 } @llvm.usub.with.overflow.i128(i128 %0, i128 %1)
  ret { i128, i1 } %2
}

define { i128, i1 } @of_sub_i128(i128 %0, i128 %1) {
entry:
  %2 = call { i128, i1 } @llvm.ssub.with.overflow.i128(i128 %0, i128 %1)
  ret { i128, i1 } %2
}

; --------------------------
; 1-bit vector operations
; --------------------------

declare i1 @llvm.vector.reduce.and.v16i1(<16 x i1>)

define i1 @reduce_and_v16i1(<16 x i1> %0) {
entry:
  %1 = call i1 @llvm.vector.reduce.and.v16i1(<16 x i1> %0)
  ret i1 %1
}
