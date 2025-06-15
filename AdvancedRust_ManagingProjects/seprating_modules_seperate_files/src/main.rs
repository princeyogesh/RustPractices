mod  greeting;
fn main() {

    greeting::description(); //greeting messages
    greeting::formal::english();
    greeting::formal::spanish();

    greeting::casual::english();
}
