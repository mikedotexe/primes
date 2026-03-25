Prove the classical base-10 prime filter in the form that matches this
repository's maintained Agda example.

Target statement:

For every natural number `n`, if `n` is prime and `n > 10`, then `gcd(n, 10) = 1`.

Equivalent concrete reading:

- every prime greater than `10` ends in `1`, `3`, `7`, or `9`
- no prime greater than `10` is divisible by `2` or `5`

Repo alignment:

- this matches `agda-proofs/Examples/Base10ResidueFilter.agda`
- the goal is a small, rigorous proof surface that can serve as an OpenProver
  pilot task

Success criteria:

- produce a concise proof
- make the dependence on primality and `n > 10` explicit
- explain how this theorem is a base-10 instance of the larger coprimality
  filter story used elsewhere in the repo

Do not:

- claim anything about template-specific prime density from this theorem alone
- drop the `n > 10` condition
