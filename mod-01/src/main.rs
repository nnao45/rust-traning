mod no_sub;
mod lib;

fn main() {
    no_sub::no_sub_mods();
    lib::world::hello_mods();
    lib::hoge::hoge_mods();
    lib::huga::huga_mods();
}
