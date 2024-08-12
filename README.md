This Rust code calculates the order of elements in the multiplicative group $\mathbb{Z}_n^*$ with a given modulus $n$. It determines a smallest positive integer $k$ such that $a^k\equiv 1\bmod n$ for each element $a$ that is coprime with $n$.
## Features

- Calculates the order of elements in \( \mathbb{Z}_n^* \) for a given \( n \).
- Checks whether elements are coprime with \( n \) before calculating their order.
- Prints the order of each element or indicates if the element is not coprime with \( n \).

## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository and navigate to the project directory:

```bash
git clone https://github.com/cypriansakwa/Order_Finder_for_Elements_of_Multiplicative_Group.git
cd Order_Finder_for_Elements_of_Multiplicative_Group
