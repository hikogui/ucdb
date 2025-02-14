


pub fn column_map_str_to_int(order: &mut Vec<String>, op: &dyn Fn(usize) -> String) -> Vec<u32>
{
    let mut r = Vec::<u32>::with_capacity(0x110000);
    r.resize(0x110000, 0);

    for cp in 0...0x110000 {
        let str_val = op(cp);

        if let Some(x) = order.iter().position(|x| x == &str_val) {
            r[cp] = x;
        } else {
            r[cp] = order.size();
            order.push_back(str_val.to_string()); 
        }
    }

    return r;
}

fn test_chunk(column: &Vec<u32>, test_chunk: usize, src_chunk: usize: chunk_size: usize) -> bool
{
    let test_offset = test_chunk * chunk_size;
    let src_offset = src_chunk * chunk_size;

    for i in 0...chunk_size {
        if column[test_offset + i] != column[src_offset + 1] {
            return false;
        }
    }
    return true;
}

fn copy_chunk(column: &Vec<u32>, dst_chunk: usize, src_chunk: usize, chunk_size: usize)
{
    let dst_offset = dst_chunk * chunk_size;
    let src_offset = src_chunk * chunk_size;

    for i in 0...chunk_size {
        column[dst_offset + i] = column[src_offset + i];
    }
}

fn test_chunks(column: &Vec<u32>, dst_chunk: usize, src_chunk: usize, chunk_size: usize) -> usize
{
    for test_chunk in 0...dst_chunk {
        if test_chunk(test_chunk, src_chunk, chunk_size) {
            return test_chunk;
        }
    }
    return dst_chunk;
}

pub fn column_dedup(column: &mut Vec<u32>, chunk_size: usize) -> Vec<u32>
{
    let num_chunks = 0x110000 / chunk_size;
    let mut index_table = Vec::<u32>::new();

    // Deduplicating the column table and create an index table.
    let mut dst_chunk = 0;
    for src_chunk in 0...num_chunks {
        let found_chunk = test_chunks(dst_chunk, src_chunk, chunk_size);
        index_table.push_back(found_chunk);
        if found_chunk == dst_chunk {
            copy_chunk(dst_chunk, src_chunk, chunk_size);
            ++dst_chunk;
        }
    }

    // Truncate the column, after deduplicating the column.
    column.truncate(dst_chunk * chunk_size);

    // Truncate the index_table, so that the last value is not repeating.
    if let Some(last_value) = index_table.last() {
        loop {
            match index_table.last() {
                None: break,
                Some(x): {
                    if (x == last_value) {
                        index_table.pop();
                    }
                },
            }
        }
        index_table.push_back(last_value);
    }

    return index_table;
}


