This Rust code calculates the order of elements in the multiplicative group $\mathbb{Z}_n^*$ with a given modulus $n$. It determines a smallest positive integer $k$ such that $a^k\equiv 1\bmod n$ for each element $a$ that is coprime with $n$.
## Features

- Calculates the order of elements in $\mathbb{Z}_n^*$ for a given $n$.
- Checks whether elements are coprime with $n$ before calculating their order.
- Prints the order of each element or indicates if the element is not coprime with $n$.

## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository and navigate to the project directory:

## Usage
- Modify the value of $n$ in the main function to represent the modulus $n$ of the group $\mathbb{Z}_n^*$.
- Optionally, adjust the max_element variable to limit the range of elements for which you want to calculate the order.
- Run the program using the following command: cargo run.
- The program will print the order of each element in $\mathbb{Z}_n^*$ or indicate if the element is not coprime with $n$.
## How It Works
- GCD Calculation: The gcd function computes the greatest common divisor (GCD) of two numbers to check if they are coprime.
- Order Calculation:
 - The order_of_element function checks if an element $a$ is coprime with $n$. If it is, the function calculates the smallest integer 
$k$ such that $a^k\equiv 1\bmod n$.
 - If the element is not coprime with $n$, it returns None.
- Output: The main function iterates over a range of elements, calculates their order, and prints the result.
```bash
git clone https://github.com/cypriansakwa/Order_Finder_for_Elements_of_Multiplicative_Group.git
cd Order_Finder_for_Elements_of_Multiplicative_Group
