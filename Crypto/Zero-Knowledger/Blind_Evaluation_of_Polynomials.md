该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

# Blind Evaluation of Polynomials

[<<part I](./Homomorphic_Hidings.md)

In future posts, we will see that blind evaluation is a central tool in SNARK constructions.

:book: 在之后的文章中我们会看到盲估计是SNARK构建的中心工具.

We denote by Fp the field of size p; that is, the elements of Fp are {0,…,p−1} and addition and multiplication are done modp as explained in Part I.

:book: 我们用Fp定义大小为p的字段, 而且Fp的元素是{0,…,p−1}, 加法与乘法如Part I介绍的经过modp.

Recall that a polynomial P of degree d over Fp is an expression of the form

P(X)=a0+a1⋅X+a2⋅X^2+…+ad⋅X^d

, for some a0,…,ad∈Fp.

:book: 回想以下d阶Fp域的多项式P的表达式是如下格式:

P(X)=a0+a1⋅X+a2⋅X^2+…+ad⋅X^d

, 其中a0,…,ad∈Fp.

We can *evaluate* P at a point s∈Fp by substituting s for X, and computing the resultant sum

P(s)=a0+a1⋅s+a2⋅s^2+…+ad⋅s^d

:book: 我们可以估计多项式P在Fp域的s点, 使用s代替X, 那么加法计算式:

P(s)=a0+a1⋅s+a2⋅s^2+…+ad⋅s^d

For someone that knows P, the value P(s) is a *linear combination* of the values 1,s,…,s^d – where linear combination just means “weighted sum”, in the case of P(s) the “weights” are a0,…,ad.

:book: 对于有人知道多项式P, P(s)的值是一个1,s,…,s^d的线性组合, 在这里线性组合以为权值加法, 在P(s)的例子中a0,…,ad是权值.

