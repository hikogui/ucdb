// This file was generated by the cargo-build script.

const EAST_ASIAN_WIDTH_CHUNK_SIZE : usize = 256;
const EAST_ASIAN_WIDTH_COLUMN_BITS : usize = 3;
const EAST_ASIAN_WIDTH_INDEX_LEN : usize = 4352;
const EAST_ASIAN_WIDTH_INDEX_BITS : usize = 6;

const EAST_ASIAN_WIDTH_INDEX_BYTE_OFFSET : usize = 4800;

const EAST_ASIAN_WIDTH_DATA: [u8; 8065] = [
    // Column table
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146,
     36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146,  4,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 80,162, 68,130,
     16, 41,146, 36, 72,146, 32, 73,  0,  0,  8,  0,  0,  0,  2,  0, 64,  2,  0, 72, 18,  0,  8,146, 32,  1,130,  4, 64,146, 32,  8,
     16,  0,  0,  0,  0,  0, 16,  4,  0,  0,  4,  0,  0,  0, 72,  0,  4,  0,144,  4,  0,  2,  0, 64,146, 32,  0,146,  4,  1,128,  4,
      0,  0,  0,  0,  0,  0, 72,  0,  4,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  8,130, 32,  8,130, 32,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 16,  0,
      0,  0,  0,  0, 16,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0, 32, 64,144,  4,  1,  2,  0,  0,146,  4, 65,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36,
     73,146, 36, 73,146, 36, 73,146, 36, 73,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,144, 36, 73,146, 36, 73, 18, 36, 73, 18,
      0,  0,144, 36, 73,146, 36, 73, 18, 36, 73, 18,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     16,  0,  0,  0,  0,  0,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73, 16,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,
    109,219,182,109,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  2, 36,  9, 18, 32,  1,146, 32, 73,  0,  0,  0,130,  4,  1,  0,  4,  8,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 32,  0,  0,  0, 64,144, 36,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 32,
     32,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  4,  1, 16,  0,  0,  0,  4,  8,  0,  0,  0,144,  0,  8,  0,  4,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 36,
      0,  0, 36,  9,146, 36, 73,146,  4,  0,146, 36, 73, 18,  0,  0,  0,  0,  0, 16,  0,  0,146, 36, 73, 18,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0, 18,  0,  0,  0,  0,  0,  0,  0,  0,128, 32,  0,  0,  0,  0,  0,  0, 64,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    130,  4, 64,  2,  4, 64, 16,  0,  1,128,  0, 73,  2,  4, 65,146, 36,  8,  0, 32, 73,  0, 32,  1,  0,  0,  0,  2, 32,  0,128,  0,
      0,  0,  0,  0, 18, 32, 73,128,  4, 72,  0,  0,  0,  0,  0,  0,128,  4, 72,  0,  0,  0,  0,  0,  1, 16,  0,  0,  0,  0,  1,  0,
      0,  0,  0,  0,  0,  0,  0, 64,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,128,  0,  0,192,  6,  0,  0,  0,  0,216,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,216, 54,  0,  3,  6,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146,
     36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73, 18, 36, 73,146, 36, 73,146, 36, 73,
    146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146,  4,  0,146, 36,
     73,146, 36, 73,146, 36, 73,146, 36, 73,146,  4,  0,  0,  0,  0,146, 36, 73,146, 36, 73,128, 36,  1,  0,  0,  0, 18, 36, 73, 18,
      0,  0,128,  4, 72,  0, 32,  1, 18,  0, 72,  2,  4, 72, 18,  0,  0,  0,  0,  0,128, 36,  1,  0,  0, 64,  0,  0,  0,  0,128, 13,
      0,  0,  9, 16,  0, 72,  0,176,  1,  0, 32,  8,  0,  0,  0,  0,  0,  0,219,182,109,  0,  0,  0,130,  0,  0,219,182,109,219,  6,
      0,  0,  0,  0, 18, 36, 65,146, 32, 65,  0,  0,  0,  0,  0, 96,  0,  0,  0,192,182,109,  0,  6,  0,  0,  0, 72, 24,  0,  0,192,
      6,  0,  0,  0,  0,  0,128, 77,  0,176, 73,146, 36, 77,146, 52, 73,146, 36, 73, 18,  4,  0,210, 36, 73,210,166, 73,210,164, 73,
      0,128,  1,192,  6,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  3,  0,  0,  0,  0,  0,  0,  0,  1,  0,  0,  0,  0, 48, 12,  0,182,
     97,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 72,146, 36, 73,  0,  0,  0,  0,  0,  0,  0,128,109,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  3,  0,  0,  0,  0, 96,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 36, 73,146,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,128,  4,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 54,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  3,128,
     73, 18,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,219,182,109,219,182,109,219,182,109, 27,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,  6,  0,  0,  0,  0,
    219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,
    109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,  1,  0,  0,  0,  0,  0,  0,  0,  0,  0,219,182,109,219,182,109,
    221,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182, 13,216,182,109,219,182,109,219,182,
    109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182, 13,216,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,
      0,128,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,216,182,109,219,182,109,219,182,109,219,182,109,219,182,
    109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182, 13,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,  1,  0,  0, 96,219,182,109,219,182,109,
    219,182,109,219,182,109,219,182,109,219,182, 13,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,146, 36, 73,219,182,
    109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,
    219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,
    109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,
    219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,
    109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219, 54,  0,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182, 13,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,219,182,109,219,182,109,219,182,109,219, 54,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,
    109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,  6,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36,
     73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146,
     36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,
    146, 36, 73,146, 36, 73,219,182,109, 27,  0,  0,  0,  0,  0,  0,  0,  0,219,182,109,219,182,109,219,182,109,219,182,109,219,176,
    109,219,182,109,219,182, 13,219,  6,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    104,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182, 37, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36,
     73,146, 36, 73,146, 36, 73, 18,  0, 73,146,  0, 73,146,  0, 73,146,  0, 73,  0,109,219, 22, 36, 73, 18,  0,  0,  0,  0,  0,  1,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,219, 54,  0,  0,  0,  0, 27,  0,  0,  0,  0,  0,
    219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,
    109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,  0,  0,  0,
    219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,
    109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,  1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 96,
    219,182,109,  3,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,219,134,109,219,134, 13,
    219,182,109,219,182,109,219,182,109,219,182,109,219,  0,  0,  0,  0,  0,192,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,219,128,
      1,  0,  0,  0,  0,176,109,  0,  0,  0,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,
    219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,
    109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,  6,  0,
    219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,
     13,  0,  0,  0,219,182,109,219,182,109,219,182, 13,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0, 48,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 96,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    146, 36, 73,146,  0,  0,146, 36, 73,146, 36, 73,146, 36, 73,146, 36,  1,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36,
     73,146, 36, 73,146, 36, 73, 18,  0,  0,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 77,218,182,109,219, 36, 73,146, 36, 73,146,
     36,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    219,  0,  0,  0,  0,  0,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,  6,  0,219,182,109,  3,  0,  0, 27,  0,
      0,  0,  0,  0,219,182,  1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    219,182,109,219,182,109,219,182,109,219,182,109,  3,  0,  0,  0,128,109,219,182, 97,219,182,109,219,182,109,219,182,109,219,182,
    109,219,182,109,219,182,109,219,182,109,219,182,109,219, 54,108,219,182,109,219,182,109,219,  6,  0,  0,  0,  0,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,109,219,  0, 96,219,  6,  0,  0,  0,  0,219,182,109,219,182,109,  3, 48,  0,219,182,109,
    219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182, 13,195,182,109,219,182,109,219,182,
    109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219, 54, 96,
    219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,  1,  0,  0,  0,  0,182, 13,219,182,
    109,219,182,109,219,182,109,  0,  0,  0,  0,  0,  0,192,  0,  0,  0,  0,  0,  0,  0,  0,  0,128, 13,  0,  0,  0,  0, 48,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,182,109,
    219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,  1,  0, 48,  0,219,128,109,  0,176,109,  0,  0,  0,  0, 54,  0,  0,176,109,219, 54,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,219,182,109,219,  6,  0,  3,  0,  0,  0,  0,  0,
      0,  0,  0,  0,176,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,176,109,219,182, 97,219,182,109,219,182,
    109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,219,182,109,219, 54,  0,219,182,109, 27,  0, 96,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182, 13,  0,  0,108,219,182,109,219, 54, 96,219,182,109, 27,  0,  0,219,182,109,  3,  0,  0,
    219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,
    109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,
    182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,109,219,182,  1,
    146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36,
     73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146,
     36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,  0,  0,  0,  0,  0,  0,
    146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36,
     73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146,
     36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36, 73,146, 36,  1,
    // Index table
     64, 32, 12, 68, 81, 20, 69, 81, 20, 69, 81, 20,133, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,  7,146, 40, 11,211, 56,197, 83,
     64, 69, 17, 73, 19, 85, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,
    101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,
    150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89, 87, 81, 20,  5, 86,
     20,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,
    101,101, 69, 81, 20, 69, 81, 20,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,101, 21, 69,177,113,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,116,150,101, 89,150,101, 89,150,101, 89,150,101, 89,
    150,101, 89,150,101,121,150,101, 89, 31, 88, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81,132,150, 56, 22, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,144, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,165,121,162,169,186,178, 69,235, 22, 69, 81, 20,
    150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101,
     89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,
    101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,
    150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101,
     89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,
    101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101,189,
    150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101,
     89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,
    101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,
    150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101,
     89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,
    101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101, 89,150,101,189,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
      5, 92, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
     69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81,
     20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69,
     81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20, 69, 81, 20,
    154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,
    105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,
    166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,
    154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,
    105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,
    166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,197,
    154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,
    105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,
    166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,
    154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,
    105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,
    166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,105,154,166,197,
    // Padding to handle unaligned word reads.
      0,
];

/// The EastAsianWidth attribute for Unicode code-points.
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum EastAsianWidth {
    N = 0,
    Na = 1,
    A = 2,
    W = 3,
    H = 4,
    F = 5,
}

/// Get the EastAsianWidth attribute for a Unicode code-point.
///
/// # Arguments
///  - `code_point` A code-point in the form of a rust `char`.
///
/// # Returns
/// A EastAsianWidth attribute of the Unicode code-point.
#[must_use] pub const fn get_east_asian_width(code_point: char) -> EastAsianWidth
{
    const INDEX_MASK : usize = (1 << EAST_ASIAN_WIDTH_INDEX_BITS) - 1;
    const COLUMN_MASK : usize = (1 << EAST_ASIAN_WIDTH_COLUMN_BITS) - 1;

    let code_point_value = code_point as usize;
    let code_point_lo = code_point_value % EAST_ASIAN_WIDTH_CHUNK_SIZE;
    let mut code_point_hi = code_point_value / EAST_ASIAN_WIDTH_CHUNK_SIZE;
    if code_point_hi > EAST_ASIAN_WIDTH_INDEX_LEN - 1 {
        code_point_hi = EAST_ASIAN_WIDTH_INDEX_LEN - 1;
    }

    let index_offset = code_point_hi * EAST_ASIAN_WIDTH_INDEX_BITS;
    let index_byte_offset = index_offset / 8;
    let index_bit_offset = index_offset % 8;
    let mut index: usize = 0;
    index |= (EAST_ASIAN_WIDTH_DATA[EAST_ASIAN_WIDTH_INDEX_BYTE_OFFSET + index_byte_offset + 1] as usize) << 8;
    index |= (EAST_ASIAN_WIDTH_DATA[EAST_ASIAN_WIDTH_INDEX_BYTE_OFFSET + index_byte_offset + 0] as usize) << 0;
    index >>= index_bit_offset;
    index &= INDEX_MASK;

    let column_offset = (index * EAST_ASIAN_WIDTH_CHUNK_SIZE + code_point_lo) * EAST_ASIAN_WIDTH_COLUMN_BITS;
    let column_byte_offset = column_offset / 8;
    let column_bit_offset = column_offset % 8;

    let mut value: usize = 0;
    value |= (EAST_ASIAN_WIDTH_DATA[column_byte_offset + 1] as usize) << 8;
    value |= (EAST_ASIAN_WIDTH_DATA[column_byte_offset + 0] as usize) << 0;
    value >>= column_bit_offset;
    value &= COLUMN_MASK;

    return match value {
        0 => EastAsianWidth::N,
        1 => EastAsianWidth::Na,
        2 => EastAsianWidth::A,
        3 => EastAsianWidth::W,
        4 => EastAsianWidth::H,
        5 => EastAsianWidth::F,
        _ => EastAsianWidth::N,
    };
}

#[cfg(all(test, not(debug_assertions)))]
#[test]
fn east_asian_width_full_coverage()
{
    for c in '\u{0000}'..='\u{d7ff}' {
        let _ = get_east_asian_width(c);
    }
    for c in '\u{e000}'..='\u{10ffff}' {
        let _ = get_east_asian_width(c);
    }
}

