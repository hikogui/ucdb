


pub fn generate_enum_table(code_dir: &std::path::Path, name : &str, enum_values: &Vec<String>, column: &Vec<usize>, index: &Vec<usize>) -> Result<(), String>
{
    let upper_name = name.to_string().to_uppercase();

    let index_max_value = match find_max(index) {
        Some(v) => v,
        None => 0,
    };

    let index_bits = (index_max_value + 1).next_power_of_two().trailing_zeros();
    let column_bits = enum_values.len().next_power_of_two().trailing_zeros();

    let column_bytes = column.compress(column, column_bits);
    let index_bytes = column.compress(index, index_bits);

    let mut s = String::new();
    s += format("const {}_COLUMN_BITS : usize = {};\n", upper_name, column_bits);
    s += format("const {}_INDEX_BITS : usize = {};\n", upper_name, index_bits);
    s += "\n";

    s += format("enum {}_type {\n");
    for (i, v) in enum_values.iter().enumerate() {
        s += format("    {} = {},\n", v, i);
    }
    s += "};\n\n";

    s += format("const {}_COLUMN: [u8; {}] = {", upper_name, column_bytes.len());
    for (i, v) in column_bytes.iter().enumerate() {
        if (v % 16 == 0) {
            s += "\n     ";
        }
        s += format("{},", v);
    }
    s += "\n};\n\n";

    s += format("const {}_INDEX: [u8; {}] = {", upper_name, index_bytes.len());
    for (i, v) in index_bytes.iter().enumerate() {
        if (v % 16 == 0) {
            s += "\n     ";
        }
        s += format("{},", v);
    }
    s += "\n};\n\n";




    return Ok(());
}
