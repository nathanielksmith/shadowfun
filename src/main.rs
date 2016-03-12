mod dice;

fn main() {
    println!("/ / / S H A D O W  F U N \\ \\ \\");

    println!("d6: {}", dice::d6());
    println!("nd6: {}", dice::nd6(4));
    println!("roll: {:?}", dice::roll(12,20));

}
