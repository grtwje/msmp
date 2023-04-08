# msmp

Minimal Sparse Matrix Packing

Derived from:

> Marshall D. Brain, and Alan L. Tharp.
> "Perfect Hashing Using Sparse Matrix Packing."
> *Information Systems*, vol. 15, no. 3, 1990, pp. 281-290.

This is small project for me to practice Rust.

The sparse matrix packing for perfect hashing described in the paper is interesting,
but I'm *guessing* that the state of the art in this area has moved well beyond that
of 1990.  The paper is short and worth reading.  It provides a detailed description
of the algorithm. Below I give only a brief synopsis.

The input is a list of items that you wish to have unique hash values for.

**Limitation**: Meant to be used on a static unchanging list of items.

For each item in the list a function is called to generate a row index, and a
different function is called to generate a column index. These are used as
coordinates into a 2D sparse array where the item's index in the original list
is stored. After all the items have been added to the 2D sparse array, the 2D
array is converted into a packed 1D array (no index gaps) along with a row lookup
table that contains an offset of where to find where rows from the original 2D
array start.

For example, following one of the examples from the paper, given this list of
words: AND, BEGIN, CHAR, CONST, ELSE, END, ENTER, EOF. And using functions for
determining the row and column indices that simply subtract ord('A') from the first and
last letter of the word. You end up with hash values of:

- AND -> 4
- BEGIN -> 7
- CHAR -> 3
- CONST -> 5
- ELSE -> 1
- END -> 0
- ENTER -> 6
- EOF -> 2

which are computed by the following pseudo code:

```
row_lookup_table = [1, -6, -14, 0, -3]

row_index =
    val = 0
    for x in word_letters[0..0]:
        val += (val * 26) + ord(x) - ord('A')

col_index =
    val = 0
    for x in word_letters[-1..-1]:
        val += (val * 26) + ord(x) - ord('A')

hash_value = (row_lookup_table[row_index] + col_index) % 8
```

In its current form my library is a only a basic implementation of the algorithm.  While
it does allow more than using just the first and last letters for determining the
indices (using just 1 letter quickly runs into collisions), it is still not at all
general purpose. For instance it assumes ASCII uppercase text as input.

In tests/simple_tests.rc an example of using the library can bee seen. It builds the
word list by reading from a file. The algorithm for generating the row and column indices
is pluggable. The test creates an instance of the ElcAlgorithm (end letter count) using the
defaults of 1 letter from each end of the word and 26 columns each row of the 2D array. The
word list and algorithm are passed to the generate_hash function which returns:

- a pseudo code text string of the code needed to implement the hash,
- and a closure that can be called on words from the list to get their hash value.
