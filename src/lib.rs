#[allow(dead_code)]
const MAXIMUM_CAPACITY: usize = 1 << 30;

#[allow(dead_code)]
const RESULT_CAPACITY: usize = 16;

#[allow(dead_code)]
const LOAD_FACTOR: f64 = 0.75;

#[allow(dead_code)]
const MIN_TRANSFER_STRIDE: usize = 16;

#[allow(dead_code)]
const RESIZE_STAMP_BITS: usize = 16;

#[allow(dead_code)]
const MAX_RESIZERS: usize = (1 << (32 - RESIZE_STAMP_BITS)) - 1;

#[allow(dead_code)]
const RESIZE_STAMP_SHIFT: usize = 32 - RESIZE_STAMP_BITS;

mod node;
