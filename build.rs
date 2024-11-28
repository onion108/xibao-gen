fn main() {
    pkg_config::probe_library("sdl2").expect("Cannot find sdl2");
}
