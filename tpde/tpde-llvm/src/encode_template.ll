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
; Vector extensions
; --------------------------

define <32 x i8> @sext_v32i1(i32 %0) {
entry:
  %1 = bitcast i32 %0 to <32 x i1>
  %2 = sext <32 x i1> %1 to <32 x i8>
  ret <32 x i8> %2
}

define <32 x i8> @zext_v32i1(i32 %0) {
entry:
  %1 = bitcast i32 %0 to <32 x i1>
  %2 = zext <32 x i1> %1 to <32 x i8>
  ret <32 x i8> %2
}

define <8 x i16> @sext_v8i8(<8 x i8> %0) {
entry:
  %2 = sext <8 x i8> %0 to <8 x i16>
  ret <8 x i16> %2
}

define <8 x i16> @zext_v8i8(<8 x i8> %0) {
entry:
  %2 = zext <8 x i8> %0 to <8 x i16>
  ret <8 x i16> %2
}

define <16 x i16> @sext_v16i8(<16 x i8> %0) {
entry:
  %2 = sext <16 x i8> %0 to <16 x i16>
  ret <16 x i16> %2
}

define <16 x i16> @zext_v16i8(<16 x i8> %0) {
entry:
  %2 = zext <16 x i8> %0 to <16 x i16>
  ret <16 x i16> %2
}

define <4 x i32> @sext_v4i16(<4 x i16> %0) {
entry:
  %2 = sext <4 x i16> %0 to <4 x i32>
  ret <4 x i32> %2
}

define <4 x i32> @zext_v4i16(<4 x i16> %0) {
entry:
  %2 = zext <4 x i16> %0 to <4 x i32>
  ret <4 x i32> %2
}

define <8 x i32> @sext_v8i16(<8 x i16> %0) {
entry:
  %2 = sext <8 x i16> %0 to <8 x i32>
  ret <8 x i32> %2
}

define <8 x i32> @zext_v8i16(<8 x i16> %0) {
entry:
  %2 = zext <8 x i16> %0 to <8 x i32>
  ret <8 x i32> %2
}

define <2 x i64> @sext_v2i32(<2 x i32> %0) {
entry:
  %2 = sext <2 x i32> %0 to <2 x i64>
  ret <2 x i64> %2
}

define <2 x i64> @zext_v2i32(<2 x i32> %0) {
entry:
  %2 = zext <2 x i32> %0 to <2 x i64>
  ret <2 x i64> %2
}
