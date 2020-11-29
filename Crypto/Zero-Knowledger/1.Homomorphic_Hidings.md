该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

# Homomorphic Hidings

这次我们来看下zk-SNARK的第一个组件[Homomorphic Hidings](https://electriccoin.co/blog/snark-explain/):

If I had to choose **one ingredient** whose role is most prominent, it would be what I will call here *Homomorphic Hiding* (HH) [[1\]](#[1]). In this post we explain what an HH is, and then give an example of why it is useful and how it is constructed.

:book: 如果我们必须选出其中作用最突出的组件, 我将会选择该被称为同态隐藏(Homomorphic Hidings)的组件. 在该文档中我会介绍同态隐藏, 并会解释他为何是有用的以及他如何构建.

An HH *E*(*x*) of a number *x* is a function satisfying the following properties:

- For most *x*’s, given *E*(*x*) it is hard to find *x*.

- Different inputs lead to different outputs – so if *x*≠*y*, then *E*(*x*)≠*E*(*y*).

- If someone knows *E*(*x*) and *E*(*y*), they can generate the HH of *arithmetic expressions in* *x* *and* *y*. For example, they can compute *E*(*x*+*y*) from *E*(*x*) and *E*(*y*).

:book: 同态隐藏的函数*E*(*x*)满足以下特性:

* 对于绝大多数*x*给出的结果*E*(*x*), 是难以倒退出*x*的.
* 不同的输入对应不同的输出, 因此if *x*≠*y*, then *E*(*x*)≠*E*(*y*).
* 如果某人知道*E*(*x*)和*E*(*y*), 那么他可以产生*x* *and* *y*同态隐藏的算数表达式. 例如, 可以通过*E*(*x*)和*E*(*y*)计算*E*(*x*+*y*).

Here’s a toy example of why HH is useful for Zero-Knowledge proofs: Suppose Alice wants to prove to Bob she knows numbers *x*,*y* such that *x*+*y*=7 (Of course, it’s not too exciting knowing such *x*,*y*, but this is a good example to explain the concept with).

1. Alice sends *E*(*x*) and *E*(*y*) to Bob.
2. Bob computes *E*(*x*+*y*) from these values (which he is able to do since *E* is an HH).
3. Bob also computes *E*(7), and now checks whether *E*(*x*+*y*)=*E*(7). He accepts Alice’s proof only if equality holds.

:book: 这里有个玩具的例子说明零知识证明: 加入Alice想要让Bob知道她知道*x*,*y*满足*x*+*y*=7. (当然, 知道*x*,*y*并没有什么值得兴奋的, 但这是一个解释概念很好的例子):

* Alice发送*E*(*x*)和*E*(*y*)给Bob.
* Bob从以上值计算*E*(*x*+*y*)(在同态隐藏下是可计算的).
* Bob业绩算*E*(7), 然后检查是否满足*E*(*x*+*y*)=*E*(7). 只有等式成立他才会接受Alice的证明.

As different inputs are mapped by *E* to different hidings, Bob indeed accepts the proof only if Alice sent hidings of *x*,*y* such that *x*+*y*=7. On the other hand, Bob does not learn *x* and *y*, as he just has access to their hidings [[2\]](#[2]).

:book: 由于函数*E*将不同的输入映射到不同的隐藏中, Bob确实也仅会在*x*,*y*的隐藏满足*x*+*y*=7的情况下才接受Alice的证明. 另一方面, Bob并不知道*x*和*y*, 只是他能接触到他们的隐藏形态.

For a prime *p*, we can use the mod*p* operation to also define a *multiplication* operation over the numbers {1,…,*p*−1} in a way that the multiplication result is also always in the set {1,…,*p*−1} – by performing regular multiplication of integers, and then taking the result mod*p*. [[3\]](#[3]) For example, 2⋅4=1(mod7).

:book: 对于素数p, 我们能够使用mod*p*操作对数字{1,…,*p*−1}进行乘法操作, 该方式下乘法的结果也将总是在集合{1,…,*p*−1}中, 通过普通的整数乘法然后进行mod*p*操作. 例如: 2⋅4=1(mod7).

This set of elements together with this operation is referred to as the group Z∗*p*. Z∗*p* has the following useful properties:

1. It is a *cyclic* group; which means that there is some element *g* in Z∗*p* called a *generator* such that all elements of Z∗*p* can be written as *g**a* for some *a* in {0,..,*p*−2}, where we define *g***0=1.
2. The *discrete logarithm problem* is believed to be hard in Z∗*p*. This means that, when p is large, given an element *h* in Z∗*p* it is difficult to find the integer *a* in 0,..,*p*−2 such that *g**a*=*h*(mod*p*).
3. As ”exponents add up when elements are multiplied”, we have for *a*,*b* in 0,..,*p*−2 *g**a*⋅*g**b*=*g**a*+*b*(mod*p*−1).

:book: 元素集合的如此操作可以称作Z∗*p*. Z∗*p*组, 具有如下特性:

1. 这是一个循环组; 这意味着有些元素在Z∗*p*中称为*generator*, 如此Z∗*p*中的所有元素可被写作*g**a*,  其中*a*在集合{0,..,*p*−2}, 我们定义*g***0=1.
2. 在Z∗*p*离散对数问题很难解决. 这意味着, 当p非常巨大, 给一个Z∗*p*域中的元素*h*, 非常困难在0,..,*p*−2来找到一个整数*a*满足: *g**a*=*h*(mod*p*).
3. 在元素相乘时同样也满足指数相加, 对于0,..,*p*−2中的*a*,*b*满足: *g**a*⋅*g**b*=*g**a*+*b*(mod*p*−1).

Using these properties, we now construct an HH that ”supports addition” – meaning that *E*(*x*+*y*) is computable from *E*(*x*) and *E*(*y*). We assume the input *x* of *E* is from Z*p*−1, so it is in the range {0,…,*p*−2}. We define *E*(*x*)=*g**x* for each such *x*, and claim that *E* is an HH: The first property implies that different *x*’s in Z*p*−1 are mapped to different outputs. The second property implies that given *E*(*x*)=*g**x* it is hard to find *x*. Finally, using the third property, given *E*(*x*) and *E*(*y*) we can compute *E*(*x*+*y*) as *E*(*x*+*y*)=*g**x*+*y*mod*p*−1=*g**x*⋅*g**y*=*E*(*x*)⋅*E*(*y*).

:book: 利用以上特性, 我们构建同态隐藏支持加法运算: 意味着*E*(*x*+*y*)能够从*E*(*x*)和*E*(*y*)计算得到. 我们假设*E*中的*x*是在域Z*p*−1中, 因此其在范围{0,…,*p*−2}中. 我们定义*E*(*x*)=*g**x*对应任意*x*, 并称*E*是一种同态隐藏函数: 第一个性质说明了Z*p*−1中不同的*x*’s映射不同的输出. 第二个性质说明了仅给出*E*(*x*)=*g**x*的情况下难以推导出*x*. 最后一个性质说明给定*E*(*x*)和*E*(*y*)计算*E*(*x*+*y*)的方法为: *E*(*x*+*y*)=*g**(x*+*y)*mod*p*−1=*g**x*⋅*g**y*=*E*(*x*)⋅*E*(*y*).

## Appendix

##### [1] Homomorphic hiding is not a term formally used in cryptography, and is introduced here for didactic purposes. It is similar to but weaker than the well-known notion of a computationally hiding commitment. The difference is that an HH is a deterministic function of the input, whereas a commitment uses additional randomness. As a consequence, HHs essentially ”hide most x’s” whereas commitments ”hide every x”.

:book: 同态隐藏并不是密码学中正式的词条, 是出于教学目的才如此介绍的. 它与众所周知的计算隐含承诺的概念相似但较弱. 不同之处在于同台隐藏是输入的确定性输出, 而隐含承诺包含额外的随机性. 总结来说, 同态隐藏"隐藏"了绝大多数x, 而隐含承诺"隐藏"了每一个x.

##### [2] Bob does learn *some* information about x and y. For example, he can choose a random x’, and check whether x=x’ by computing E(x’). For this reason the above protocol is not really a Zero-Knowledge protocol, and is only used here for explanatory purposes. In fact, as we shall see in later posts, HH is ultimately used in snarks to conceal verifier challenges rather than prover secrets.

:book: Bob确实掌握了一些关于x和y的信息. 例如, 它可以选择一个随机x', 并通过E(x')检查x=x'是否成立. 出于该原因, 上述协议并不是零知识证明协议, 在这里仅用以解释说明. 事实上, 正如我们后文提到的, 同态隐藏在snark中最终是用在隐藏验证人的挑战而非证明人的秘密.

##### [3] When p is not a prime it is problematic to define multiplication this way. One issue is that the multiplication result can be zero even when both arguments are not zero. For example when p=4, we can get 2*2=0 (mod 4).

:book: 当p不是素数的时候在定义乘法的时候可能会有这个问题. 一种是乘法的结果可能会变成0, 在两个成熟都不是0的情况下. 例如: 2*2=0 (mod 4).

