use validaten::creditcard;

fn main() {
    println!("4844161459546175 => {:?}", creditcard::which_card("4844161459546175"));
    println!("6796265520244 => {:?}", creditcard::which_card("6796265520244"));
    println!("6007221111111110 => {:?}", creditcard::which_card("6007221111111110"));
    println!("5019118545073184 => {:?}", creditcard::which_card("5019118545073184"));
    println!("4035300539804083 => {:?}", creditcard::which_card("4035300539804083"));
    println!("5463113589982388 => {:?}", creditcard::which_card("5463113589982388"));
    println!("370789709084107 => {:?}", creditcard::which_card("370789709084107"));
    println!("3022143741431999 => {:?}", creditcard::which_card("3022143741431999"));
    println!("30043277253245 => {:?}", creditcard::which_card("30043277253245"));
    println!("3860847190349 => {:?}", creditcard::which_card("3860847190349"));
    println!("6011575126600688 => {:?}", creditcard::which_card("6011575126600688"));
    println!("62600094752489242 => {:?}", creditcard::which_card("62600094752489242"));
    println!("3588337499926343 => {:?}", creditcard::which_card("3588337499926343"));
}