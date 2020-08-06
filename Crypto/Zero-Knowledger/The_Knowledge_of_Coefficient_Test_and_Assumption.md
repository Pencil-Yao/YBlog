该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

## The Knowledge of Coefficient Test and Assumption

[<<part II](./Blind_Evaluation_of_Polynomials.md)

the fact that Alice is *able* to compute E(P(s)) does not guarantee she will indeed send E(P(s)) to Bob, rather than some completely unrelated value.

:book: 事实是Alice可以计算出E(P(s))但不能保证Alice一定会发送E(P(s))给Bob, 而不是发送一下完全不相关的值.

For α∈F∗p [[1\]](#[1]), let us call a pair of elements (a,b) in G an α-pair if a,b≠0 and b=α⋅a.

The KC Test proceeds as follows.

1. Bob chooses random α∈F∗p and a∈G. He computes b=α⋅a.
2. He sends to Alice the "challenge" pair (a,b). Note that (a,b) is an α-pair.
3. Alice must now respond with a *different* pair (a′,b′) that is also an α-pair.
4. Bob accepts Alice’s response only if (a′,b′) is indeed an α-pair. (As he knows α he can check if b′=α⋅a′.)

:book: 对于α∈F∗p, 我们称在G域中的元素(a,b)在满足a,b≠0 and b=α⋅a时为α-pair

系数测试知识流程如下所示:

1. Bob从域中选择随机数α∈F∗p以及a∈G. 他计算了b=α⋅a.
2. 他发送Alice挑战对(a,b). 注意到 (a,b)是一个α-pair.
3. Alice必须使用不同的α-pair (a′,b′)来进行回应.
4. Bob接受了Alice的回应如果(a′,b′)确实也是α-pair(我们知道α可以用于检查, 如果b′=α⋅a′)







## Appendix

##### [1] F∗p denotes the non-zero elements of Fp. It is the same as Z∗p described in Part I.

:book: ​F∗p是Fp的无零域, 它与part I中提到的Z*p是一样的.