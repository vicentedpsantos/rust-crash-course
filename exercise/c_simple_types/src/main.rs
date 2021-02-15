// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);

    ding_machine::print_difference(coords.0, coords.1);   // Uncomment and finish this line


    // 2. We want to use the `print_array` function to print coords...but coords isn't an array!
    // Create an array of type [f32; 2] and initialize it to contain the
    // information from coords.  Uncomment the print_array line and run the code.
    //
    let coords_arr = [coords.0, coords.1];
    ding_machine::print_array(coords_arr);


    let series = [1, 1, 2, 3, 5, 8, 13];
    // 3. Make the `ding` function happy by passing it the value 13 out of the `series` array.
    // Use array indexing.  Done correctly, `cargo run` will produce the additional output
    // "Ding, you found 13!"
    //
    ding_machine::ding(series[series.len() - 1]);


    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    ding_machine::on_off(mess.2[1].0);

    ding_machine::print_distance(coords);
}
