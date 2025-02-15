




/// Copy a chunk from the source-chunk to the destination-chunk.
/// 
/// # Arguments
///  - `column`: The column to deduplicate, this column will be mutated by this function.
///  - `dst_chunk`: The index to the chunk where the data will be copied to.
///  - `src_chunk`: The index to the chunk where the data will be copied from.
///  - `chunk_size`: The size of the chunks.
/// 
fn copy_chunk(column: &mut Vec<usize>, dst_chunk: usize, src_chunk: usize, chunk_size: usize)
{
    let dst_offset = dst_chunk * chunk_size;
    let src_offset = src_chunk * chunk_size;

    for i in 0..chunk_size {
        column[dst_offset + i] = column[src_offset + i];
    }
}

fn test_chunk(column: &Vec<usize>, a: usize, b: usize, chunk_size: usize) -> bool
{
    let a_offset = a * chunk_size;
    let b_offset = b * chunk_size;

    for i in 0..chunk_size {
        if column[a_offset + i] != column[b_offset + 1] {
            return false;
        }
    }
    return true;
}

/// Test if any of the chunks previous from the `dst_chunk` are equal to the
/// `src_chunk`.
/// 
/// # Arguments
///  - `column`: The vector to deduplicate.
///  - `dst_chunk`: The chunk-index to the destination-chunk where the
///                 source-chunk will be copied to; if all chunks from
///                 `0..dst_chunk` are unequal to source-chunk.
///  - `src_chunk`: The chunk-index to the source-chunk.
///  - `chunk_size`: The size of the chunks.
/// 
/// # Returns
/// The index to the first chunk that is equal to the chunk pointed to by
/// `src_chunk`, or `dst_chunk` if no such chunk exists.
/// 
/// The `dst_chunk` is returned so that you can directly copy the source-chunk
/// to the correct position.
/// 
fn test_chunks(column: &Vec<usize>, dst_chunk: usize, src_chunk: usize, chunk_size: usize) -> usize
{
    for a_chunk in 0..dst_chunk {
        if test_chunk(column, a_chunk, src_chunk, chunk_size) {
            return a_chunk;
        }
    }
    return dst_chunk;
}

/// Deduplicate a column.
/// 
/// # Arguments
///  - `column` : The column to deduplicate. The column is modified in-place.
///  - `chunk_size` : The size of the chunks.
/// 
/// # Returns
/// The index-table.
/// 
/// To find a specific entry `i` in the deduplicated column:
///  - `chunk_nr = min(i / chunk_size, index_table.len())`
///  - `offset = index_table[chunk_nr] * chunk_size + i % chunk_size`
///  - `entry = column[offset]`
/// 
pub fn dedup(column: &mut Vec<usize>, chunk_size: usize) -> Vec<usize>
{
    let num_chunks = 0x110000 / chunk_size;
    let mut index_table = Vec::<usize>::new();

    // Deduplicating the column table and create an index table.
    let mut dst_chunk = 0;
    for src_chunk in 0..num_chunks {
        let found_chunk = test_chunks(column, dst_chunk, src_chunk, chunk_size);
        index_table.push(found_chunk);
        if found_chunk == dst_chunk {
            copy_chunk(column, dst_chunk, src_chunk, chunk_size);
            dst_chunk += 1;
        }
    }

    // Truncate the column, after deduplicating the column.
    column.truncate(dst_chunk * chunk_size);

    // Truncate the index_table, so that the last value is not repeating.
    if let Some(&last_value) = index_table.last() {
        loop {
            match index_table.last() {
                None => break,
                Some(&x) => {
                    if x == last_value {
                        index_table.pop();
                    } else {
                        break;
                    }
                },
            }
        }
        index_table.push(last_value);
    }

    return index_table;
}

pub fn map_str_to_int<'a>(order: &mut Vec<String>, op: impl Fn(usize) -> &'a String) -> Vec<usize>
{
    let mut r = Vec::<usize>::with_capacity(0x110000);
    r.resize(0x110000, 0);

    for cp in 0..0x110000 {
        let str_val = op(cp);

        if let Some(x) = order.iter().position(|x| x == str_val) {
            r[cp] = x;
        } else {
            r[cp] = order.len();
            order.push(str_val.to_string()); 
        }
    }

    return r;
}

fn compress_insert_value(bytes: &mut Vec<u8>, offset : usize, mut value : usize)
{
    let mut byte_offset = offset / 8;
    let bit_offset = offset % 8;

    value <<= bit_offset;
    while value != 0 {
        bytes[byte_offset] |= (value & 255) as u8;
        value >>= 8;
        byte_offset += 1;
    }
}

/// Compress and integer tables into tightly packed bytes.
///
/// The byte array will end in 7 zero bytes, to be able to
/// do unaligned reads without running beyond the bounds.
pub fn compress(input: &Vec<usize>, num_bits: usize) -> Vec<u8>
{
    let total_num_bits = num_bits * input.len();
    let total_num_bytes = (total_num_bits + 7) / 8;

    let mut r = Vec::<u8>::with_capacity(total_num_bytes + 7);
    r.resize(total_num_bytes + 7, 0);

    let mut offset : usize = 0;
    for x in input {
        compress_insert_value(&mut r, offset, *x);
        offset += num_bits;
    }

    return r;
}
