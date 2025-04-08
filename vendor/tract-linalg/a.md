arm64 
    * 32 gp registers
    * 32 128bits SIMD vector registers
            * 4xf32 vectors
    * v0..v31 (inc)

activation
    * need 3 registers (A, B, C)
    * some more room for house keeping

=> 1 activation registers maps to 8 SIMD vectors: 32 floats
    * A: v0..v7 (inc)
    * B: v8..v15 (inc)
    * C: v16..v23 (inc)
    * v24..v31 (inc) for house keeping

