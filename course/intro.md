It took me a little more than a week to implement the ML-DSA algorithm. Before that, I had already spent many days trying to understand and implement the Number Theoretic Transform (NTT) as described in the ML-KEM (https://csrc.nist.gov/pubs/fips/203/final) and ML-DSA (https://csrc.nist.gov/pubs/fips/204/final) specifications. I read Amber Sprenkels’ excellent article (https://electricdusk.com/ntt.html), studied the publication by Ardianto Satriawan, Relia Mareta, and Hanho Lee ((https://eprint.iacr.org/2024/585)), and watched Prof. Alfred Menezes’ outstanding lectures (https://cryptography101.ca/kyber-dilithium/). I went through the same materials multiple times and experimented with various versions of Python code to develop a basic understanding of the principles.

The learning curve was steep for me, and it took time to piece everything together (I'm not sharp at math - nothing comes easily to me). I am writing this article in the hope that it will help others get up to speed with the technical details of the ML-KEM (Kyber) and ML-DSA (Dilithium) algorithms. Keeping beginners in mind, I have tried to make this as self-contained as possible. Therefore, let's get started from the beginning.

Background
The round 1 NIST submission package of CRYSTALS-Dilithium

In section 1.2 of the article, a note titled **Implementation Considerations** summarizes the reason for using *Number Theoretic Transform*:

"The main algebraic operation performed in the scheme
is a multiplication of a matrix $\mathbf{A}$, whose elements are polynomials in $Z_q[X]/(X^{256} + 1)$
by a vector of such polynomials. In our recommended parameter setting, $\mathbf{A}$ is a 5 x 4
matrix and therefore consists of 20 polynomials. Thus the multiplication $\mathbf{Av}$ involves 20
polynomial multiplications. As in most lattice-based schemes that are based on operations
over polynomial rings, we have chosen our ring so that the multiplication operation has
a very efficient implementation via the Number Theoretic Transform (NTT), which is
just a version of FFT that works over the finite field $Z_q$ rather than over the complex
numbers. To enable the NTT, we needed to choose a prime q so that the group $Z_q$ has
an element of order 2n = 512; or equivalently $ q \equiv 1 (\mathrm{mod} \ 512)$. If $r$ is such an element,
then $X^{256} + 1 = (X−r)(X−r^3)···(X−r^{511})$ and thus one can equivalently represent
any polynomial $a \in Z_q[X]/(X^{256} + 1)$ in its CRT (Chinese Remainder Theorem) form as
$(a(r),a(r^3),...,a(r^{2n−1}))$."

We will gradually unpack a bunch of key ideas from the above paragraph:

1. The scheme uses polynomials of degree $n=256$.

2. The prime $q = 2^{23}-2^{13}+1$ (i.e, $q = 8380417$).

3. The *polynomial ring* is $Z_q[X]/(X^{256} + 1)$.

4. The scheme defines matrices of polynomials and multiplication with vectors of polynomials.

5. NTT helps in efficient implementation of polynomial multiplication.

6. Dilithium has chosen prime $q$ such that there is an element $r$ of order 512 (= $2n$) in $Z_q$. This means $ q \equiv 1 (\mathrm{mod} \ 512)$ and that $r^{512} \equiv 1 (\mathrm{mod} \ q)$. In ML DSA, $r = 1753$.

7. Polynomials have a **Chinese Remainder Theorem** form.




## Number Theoretic Transform (NTT)
The schoolbook method of multiplying two $n$-degree polynomials involves $n^2$ coefficient multiplications. This simple method becomes computationally expensive especially in algorithms that generate signing keys, produce and verify signatures. The following table shows the number of polynomial multiplications performed in ML DSA algorithms across three flavors:

|             | Key Generation       | Signature Generation       | Signature Verification |
|:------------|:---------------------|:---------------------|:-----------------------|
| ML-DSA-44   | 16 multiplications   | 28 multiplications   | 20 multiplications     |
|             | $16*(256*256)$ terms | $28*(256*256)$ terms | $28*(256*256)$ terms   |
| ML-DSA-65   | 30 multiplications   | 47 multiplications   | 36 multiplications     |
|             | $30*(256*256)$ terms | $47*(256*256)$ terms | $47*(256*256)$ terms   |
| ML-DSA-87   | 56 multiplications   | 79 multiplications   | 64 multiplications     |
|             | $56*(256*256)$ terms | $79*(256*256)$ terms | $64*(256*256)$ terms   |


NTT is a clever method for representing polynomials enabling efficient multiplication of two fixed-degree polynomials. When the polynomial ring is specially crafted, the computational complexity of polynomial multiplication reduces to $O(n \ log \ n)$.

In ML DSA, all polynomials are single-variable polynomials. The identifier $X$ represents the variable. The polynomial ring, denoted by $Z_q[X]/(x^{256}+1)$, is defined over $Z_q$ modulo $X^{256} + 1$. This means the coefficients of polynomials in $R_q$ are in the set $\{0, 1, 2, ... q-1\}$, denoted by $Z_q$. Formally, $Z_q$ represents the ring of integers modulo $m$ whose set of elements is $\{0, 1, 2, ... q-1\}$. The members of the ring $R_q$ are polynomials reduced by $(x^{256}+1)$. The term $(x^{256}+1)$ is called the *cyclomatic modulus* of the ring.



## Polynomial Remainder Theorem
We will develop the constructive proof mentioned in the wikipedia article (https://en.wikipedia.org/wiki/Polynomial_remainder_theorem).


Let
$$
f(x) = a_n x^n + a_{n-1} x^{n-1} + \dots + a_1 x + a_0
$$

be a polynomial with coefficients in a ring, and let $r$ be any element of the ring. Then there exists a polynomial $Q(x)$ of degree at most $(n-1)$ such that
$$
f(x) = (x - r) \ Q(x) \ + \ f(r).
$$

The remainder of the division of $f(x)$ by the linear polynomial $(x - r)$ is exactly $f(r)$.

---

### Proof

1. **The polynomial and its evaluation at $r$.**

   Let
   $$
   f(x) = a_n x^n + a_{n-1} x^{n-1} + \dots + a_1 x + a_0.
   $$

   Evaluate $f(r)$:
   $$
   f(r) = a_n r^n + a_{n-1} r^{n-1} + \dots + a_1 r + a_0.
   $$

2. **The difference $f(x) - f(r)$.**

   $$
   f(x) - f(r)
   = (a_n x^n + a_{n-1} x^{n-1} + \dots + a_1 x + a_0)
     - (a_n r^n + a_{n-1} r^{n-1} + \dots + a_1 r + a_0).
   $$

    Grouping and rearranging terms yields
   $$
   f(x) - f(r)
   = a_n(x^n - r^n)
     + a_{n-1}(x^{n-1} - r^{n-1})
     + \dots
     + a_1(x - r)
   $$


3. **The algebraic identity.**

   For an integer $k \ge 1$
   $$
   x^k - r^k
   = (x - r)\bigl( x^{k-1} + x^{k-2}r + \dots + xr^{k-2} + r^{k-1} \bigr).
   $$

4. **Apply the identity to each term in $f(x) - f(r)$.**

   For each $k = 1, \dots, n$:
   $$
   a_k(x^k - r^k)
   = (x - r)\,a_k\bigl( x^{k-1} + x^{k-2}r + \dots + r^{k-1} \bigr).
   $$

5. **Factor out $(x - r)$.**

   Apply the algebraic identity in the final expression of step 2:
   $$
   \begin{aligned}
   f(x) - f(r)
   &= (x - r)\,a_n( x^{n-1} + x^{n-2}r + \dots + r^{\,n-1} ) \\
   &\quad + (x - r)\,a_{n-1}( x^{n-2} + x^{n-3}r + \dots + r^{\,n-2} ) \\
   &\quad + \dots + (x - r)\,a_1.
   \end{aligned}
   $$

   Factor $(x - r)$ from the entire sum (the right-hand side expression):

   $$
   f(x) - f(r)
   = (x - r)\Bigl(
     a_n( x^{n-1} + x^{n-2}r + \dots + r^{\,n-1} )
     + \dots + a_1
     \Bigr).
   $$

6. **Define the polynomial $Q(x)$ and Complete the Proof**

    Let
   $$
    Q(x) = a_n( x^{n-1} + x^{n-2}r + \dots + r^{\,n-1} )
      + a_{n-1}( x^{n-2} + x^{n-3}r + \dots + r^{\,n-2} )
      + \dots + a_1.
   $$
    where
    $$\deg(Q) \le n - 1$$


   We use polynomial $Q$, and write
   $$
   f(x) - f(r) = (x - r) \, Q(x).
   $$

    from which it follows that
   $$
   f(x) = (x - r) \, Q(x) + f(r).
   $$

   $\Box$




```
Rings - Basic Definitions and Concepts.
https://math.libretexts.org/Bookshelves/Combinatorics_and_Discrete_Mathematics/Applied_Discrete_Structures_(Doerr_and_Levasseur)/16%3A_An_Introduction_to_Rings_and_Fields/16.01%3A_Rings_Basic_Definitions_and_Concepts

CRYSTALS-Kyber – Submission to the NIST post-quantum project.
Roberto Avanzi, Joppe Bos, Léo Ducas, Eike Kiltz, Tancrède Lepoint, Vadim Lyubashevsky, John M. Schanck, Peter Schwabe, Gregor Seiler, and Damien Stehlé.
Specification document (part of the submission package). 2017-11-30

CRYSTALS-Dilithium – Algorithm Specifications and Supporting Documentation
Léo Ducas, Eike Kiltz, Tancrède Lepoint, Vadim Lyubashevsky, Peter Schwabe, Gregor Seiler, and Damien Stehlé.
Specification document (part of the submission package). 2017-11-30

```