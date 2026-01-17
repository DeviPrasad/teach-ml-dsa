<head>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.27/dist/katex.min.css" integrity="sha384-Pu5+C18nP5dwykLJOhd2U4Xen7rjScHN/qusop27hdd2drI+lL5KvX7YntvT8yew" crossorigin="anonymous">
    <!-- The loading of KaTeX is deferred to speed up page rendering -->
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.27/dist/katex.min.js" integrity="sha384-2B8pfmZZ6JlVoScJm/5hQfNS2TI/6hPqDZInzzPc8oHpN5SgeNOf4LzREO6p5YtZ" crossorigin="anonymous"></script>
    <!-- To automatically render math in text elements, include the auto-render extension: -->
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.27/dist/contrib/auto-render.min.js" integrity="sha384-hCXGrW6PitJEwbkoStFjeJxv+fSOOQKOPbJxSfM6G5sWZjAyWhXiTIIAmQqnlLlh" crossorigin="anonymous"
        onload="renderMathInElement(document.body);"></script>
    <link rel="stylesheet" type="text/css" href="https://tikzjax.com/v1/fonts.css">
    <script src="https://tikzjax.com/v1/tikzjax.js"></script>
</head>

# A Few Identities involving $\zeta$ in Tiny DSA
Tiny DSA defines the following values:

$$
\begin{array}{rll}
& n & 32\\
&q & \text{The prime number} \ q = 2^{10} - 2^8 + 1 = 769.\\
&\mathbb{Z}_q & \text{The ring of integers modulo} \ q \ \text{whose set of elements is} \ \{0, 1, \ldots, q-1\}.\\
& \zeta & \text{The} \ primitive \ 64^{th} \ root \ \ of \ unity \ \text{in} \ \mathbb{Z}_q. \\
    && \zeta = 12 \\
    && \zeta^{32} \equiv -1 \ \text{mod} \ q\\
    && \zeta^{64} \equiv 1 \ \text{mod} \ q\\
    && \zeta^k \ \not\equiv 1 \ \text{mod} \ q \ \ \ for \ all \ k < 64.\\
\end{array}
$$

We start with the two importatnt equalitites:
$$
\begin{array}{lllll}
    \zeta^{2n} & = & \zeta^{64} & = & 1 \\
    \zeta^{n} & = & \zeta^{32} & = & -1 \\
\end{array}
$$

From these we can derive another useful equality. Recall that in Tiny DSA, $n = 32$:
$$
\begin{array}{lll}
    \zeta^{n}/\zeta^{2n} & = & -1/1\\
    \zeta^{n - 2n} & = & -1\\
    \zeta^{-n} & = & -1 \\
    & = & \zeta^{n}
\end{array}
$$

In summary,
$$
    \zeta^{n}  = \zeta^{-n} = -1
$$

We can now use this result to find another equality: $\zeta^{-m} = -\zeta^{n-m}$
$$
\begin{array}{lll}
    \zeta^{-m} \\
    & = \zeta^{0} \ \zeta^{-m} \\
    & = \zeta^{n} \ \zeta^{-n} \ \zeta^{-m} \\
    & = \zeta^{n} \ \zeta^{-m} \ \zeta^{-n} \\
    & = \zeta^{n-m} \ \, \zeta^{-n} \\
    & = -\zeta^{n-m} & \because \ \zeta^{-n} = -1 \\
\end{array}
$$

The result $\zeta^{-m} = -\,\zeta^{32-m}$ comes handy while evaluating $NTT^{-1}$ function.

## Simple Python Code to Verify the Equalities
```{.number-lines .python}
# python3
n = 32
q = 769
z = 12
assert pow(z, 2*n, q) == 1
assert pow(z, n, q) == q-1
# for all k < 64, z^k != 1 mod q.
# in other words, there is no k in [1, 63] such that z^k = 1 mod q
assert not any([pow(z, k, q)==1 for k in range(1, 2*n)])
##
# check the main result of this section
for m in range(0, 32):
    assert pow(z, -m, q) == q-pow(z, 32-m, q)

```


# Application in ML DSA
Recall that the definition of Tiny DSA closely mimics the structure of ML DSA. Therefore, the main equalities shown in the previous section are also applicable to ML DSA.

The values of $n$, $q$, and $\zeta$, however, are different in ML DSA:

$$
\begin{array}{rll}
& n & 256\\
&q & \text{The prime number} \ q = 2^{23} - 2^{10} + 1 = 8380417.\\
& \zeta & \text{The} \ primitive \ 512^{th} \ root \ \ of \ unity \ \text{in} \ \mathbb{Z}_q. \\
    && \zeta = 1753 \\
    && \zeta^{256} \equiv -1 \ \text{mod} \ q\\
    && \zeta^{512} \equiv 1 \ \text{mod} \ q\\
    && \zeta^k \ \not\equiv 1 \ \text{mod} \ q \ \ \ for \ all \ k < 512.\\
\end{array}
$$

### Exercise
Modify the identifiers in the Python code shown above to match the definition of ML DSA. Run the code and verify that all assertions hold.

$\colorbox{gray}{}$
