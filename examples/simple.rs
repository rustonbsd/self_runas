use self_runas::is_elevated;


fn main() {
    println!("23");
    if !is_elevated() {
        let _ = self_runas::self_runas();
    }
}