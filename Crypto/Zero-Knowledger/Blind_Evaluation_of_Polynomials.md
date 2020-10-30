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

:book: 一种是Alice发送多项式P给Bob, 由他自己计算出E(P(s)); 另一种是Bob发送s给Alice;她计算出了E(P(s)),并将值传给Bob.

However, in the *blind evaluation problem* we want Bob to learn E(P(s)) without learning P – which precludes the first option; and, most importantly, we don’t want Alice to learn s, which rules out the second [[1\]]((#[1])).

:book: 然而在盲估中我们希望Bop在不知道P的情况下知道E(P(s)) - 这样排除了方案1; 而且更重要的是, 我们也不想要让Alice知道s, 因此排除了方案2.

Using HH, we can perform blind evaluation as follows.

1. Bob sends to Alice the hidings E(1),E(s),…,E(s^d).
2. Alice computes E(P(s)) from the elements sent in the first step, and sends E(P(s)) to Bob. (Alice can do this since E supports linear combinations, and P(s) is a linear combination of 1,s,…,s^d.)

Note that, as only hidings were sent, neither Alice learned ss [[2\]](https://electriccoin.co/blog/snark-explain2/#id5), nor Bob learned P.

:book: 使用同态隐藏, 我们能够执行以下估计:

1. Bob发送Alice隐藏值: E(1),E(s),…,E(sd).
2. Alice从第一步中计算E(P(s)), 然后发送E(P(s))给Bob. (Alice能够做到因为E具备线性组合, 而P(s)正是1,s,…,s^d的线性组合.)

Subsequent posts will go into more detail as to how blind evaluation is used in SNARKs. The rough intuition is that the verifier has a “correct” polynomial in mind, and wishes to check the prover knows it. Making the prover blindly evaluate their polynomial at a random point not known to them, ensures the prover will give the wrong answer with high probability if their polynomial is not the correct one. This, in turn, relies on the Schwartz-Zippel Lemma stating that “different polynomials are different at most points”.

:book: 一系列的文档将会深入介绍盲估计在SNARKs中的应用. 一个简单的情景: 验证人有一个正确的多项式, 并希望检查验证人是知道该多项式的. 让证明者在他们不知道的随机点上盲目评估他们的多项式，确保如果他们的多项式不正确，证明者将很可能给出错误的答案。反过来, 这依赖于Schwartz-Zippel Lemma所指出的"不同的多项式在大多数点是不同的".

## Appendix

##### [1] The main reason we don’t want to send P to Bob, is simply that it is large – (d+1) elements, where, for example, d~2000000 in the current Zcash protocol; this ultimately has to do with the “Succinct” part of SNARKs. It is true that the sequence of hidings Bob is sending to Alice above is just as long, but it will turn out this sequence can be “hard-coded” in the parameters of the system, whereas Alice’s message will be different for each SNARK proof.

:book: 我们不希望发送P给Bob的主要原因是, 多项式P非常大有(d+1)个元素, 然而在Zcash的协议中d大约为2000000, 这最终会影响到SNARKs的简短. 的确, 上文Bob发送给Alice隐藏序列也会一样长, 但事实说明在系统参数中这可以进行"硬编码", 而对于每个SNARK证明, Alice的消息都会有所不同.

##### [2] Actually, the hiding property only guarantees s not being recoverable from E(s), but here we want to claim it is also not recoverable from the sequence E(s),…,E(s^d) that potentially contains more information about s. This follows from the d-power Diffie-Hellman assumption, which is needed in several SNARK security proofs.

:book: 事实上, 隐藏属性是保证无法重E(s)倒推出s, 但是我们想要说明从E(s),…,E(s^d)恢复信息s同样是做不到的. 这结论来源于d阶Diffie-Hellman假设, 该假设用于一些SNARK的安全证明中.