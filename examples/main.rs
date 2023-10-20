use cpp_streams::{cin, cout, endl};

fn main() {
    let mut input = String::new();

    cin >> &mut input;

    cout << "Hello, world!\n" << "Other string\n" << input << "\n" << 1 << 1.0 << endl;
}