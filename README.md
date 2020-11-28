# Resarith

Resarith is an equivalent resistance calculator. It computes equivalent resistance for arbitrary complex series or parallel resistor associations.

## Example

In this example, we compute the equivalent resistance for a 1.5 Ω resistor in series with an association of two resistors in parallel (respectively of 2 Ω and 3 Ω):

```
$ resarith "1.5 : (2 | 3)"
2.7
```

## Compile and install

Proceed like you would for any Rust project. There are no dependencies.

## Usage

From the command line :

```
resarith input
```

where `input` is an expression describing the association of resistor for which you want the equivalent resistance. Most of the time, you will need to add double quotes around the input to have a valid command.

You can use the following:

* numbers such as `10`, `1.5`;
* the series operator `:`;
* the parallel operator `|`;
* parenthesis to manage priorities.

**Note 1:** Number recognition is very limited, e.g. notations such as `1e3` are not recognized.

**Note 2:** Associativity is not recognized. Instead of `1 | 2 | 3`, you must write `(1 | 2) | 3` or `1 | (2 | 3)`.

# License

See [LICENSE](LICENSE).
