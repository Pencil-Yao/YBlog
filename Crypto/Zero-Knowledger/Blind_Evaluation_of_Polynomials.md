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

In the last post, we saw the HH E defined by E(x)=g^x where g was a generator of a group with a hard discrete log problem. We mentioned that this HH “supports addition” in the sense that E(x+y) can be computed from E(x) and E(y). We note here that it also “supports linear combinations”; meaning that, given a,b,E(x),E(y) we can compute E(ax+by). This is simply because

E(ax+by)=g^(ax+by)=g^ax⋅g^by=(g^x)a⋅(g^y)b=E(x)^a⋅E(y)^b.

:book: 在上一篇文档, 我们提到了同态隐藏支持加法从某种意义上说E(x+y)能够从E(x)和E(y)计算得到. 我们注意到同态隐藏也"支持线性组合"; 意味着提供a,b,E(x),E(y)我们能够计算E(ax+by):

E(ax+by)=g^(ax+by)=g^ax⋅g^by=(g^x)a⋅(g^y)b=E(x)^a⋅E(y)^b.

### Blind evaluation of a polynomial

Suppose Alice has a polynomial P of degree d, and Bob has a point s∈Fp that he chose randomly. Bob wishes to learn E(P(s)), i.e., the HH of the evaluation of P at s. Two simple ways to do this are:

:book: 假设Alice拥有一个d阶多项式P, 而Bob在Fp域上随机选择一个点s. Bob想要知道E(P(s))的值, 例如P在s的估计值的同态隐藏值. 有两个简单的方法来实施:

- Alice sends P to Bob, and he computes E(P(s)) by himself.
- Bob sends s to Alice; she computes E(P(s)) and sends it to Bob.

:book: 一种是Alice发送多项式P给Bob, 由他自己计算出E(P(s)); 另一种