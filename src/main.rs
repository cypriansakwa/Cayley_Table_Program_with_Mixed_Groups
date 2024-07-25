use num::integer::gcd;

// Function to find elements of Z_n (additive group)
fn zn(n: u32) -> Vec<u32> {
    (0..n).collect()
}

// Function to find elements of Z_m* (multiplicative group)
fn zm_star(m: u32) -> Vec<u32> {
    (1..m).filter(|&x| gcd(x, m) == 1).collect()
}

// Function to compute the direct product of Z_n and Z_m*
fn direct_product(n: u32, m: u32) -> Vec<(u32, u32)> {
    let zn_elements = zn(n);
    let zm_star_elements = zm_star(m);
    
    let mut product = Vec::new();
    for &a in &zn_elements {
        for &b in &zm_star_elements {
            product.push((a, b));
        }
    }
    product
}

// Function to compute the operation (a, b) * (e, f) mod (n, m)
fn operation_mod((a, b): (u32, u32), (e, f): (u32, u32), n: u32, m: u32) -> (u32, u32) {
    ((a + e) % n, (b * f) % m)
}

// Function to print the Cayley table
fn print_cayley_table(n: u32, m: u32) {
    let elements = direct_product(n, m);

    // Print the header row
    print!("       ");
    for &elem in &elements {
        print!("{:?} ", elem);
    }
    println!();

    // Print each row of the table
    for &elem1 in &elements {
        print!("{:?} ", elem1);
        for &elem2 in &elements {
            let result = operation_mod(elem1, elem2, n, m);
            print!("{:?} ", result);
        }
        println!();
    }
}

fn main() {
    let n = 5;
    let m = 6;

    println!("Cayley table for Z_{} x Z_{}^*:", n, m);
    print_cayley_table(n, m);
}

