该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

# zk-SNARK

先看下zcash对zk-SNARK的[介绍](https://z.cash/technology/zksnarks/), 以下是是一部重点语段的翻译.

The acronym zk-SNARK stands for “Zero-Knowledge Succinct Non-Interactive Argument of Knowledge,” and refers to a proof construction where one can prove possession of certain information, e.g. a secret key, without revealing that information, and without any interaction between the prover and verifier.

:book: zk-SNARK是"Zero-Knowledge Succinct Non-Interactive Argument of Knowledge"的缩写, 表示可以构建对特定信息的证明, eg. 密钥, 不暴露信息并且证明者与验证者之间不需要交互.

“Succinct” zero-knowledge proofs can be verified within a few milliseconds, with a proof length of only a few hundred bytes even for statements about programs that are very large. In the first zero-knowledge protocols, the prover and verifier had to communicate back and forth for multiple rounds, but in “non-interactive” constructions, the proof consists of a single message sent from prover to verifier. Currently, the most efficient known way to produce zero-knowledge proofs that are non-interactive and short enough to publish to a block chain is to have an initial setup phase that generates a common reference string shared between prover and verifier. We refer to this common reference string as the public parameters of the system.

:book: "简明"零知识证明可在数微妙之内被验证， 仅需要几百字节的证明甚至对于非常巨大程序的状态. 在最初的零知识证明协议中, 证明人与验证人之间必须来回通信多次, 但是在"非交互"构建下, 该证明包含了单条消息由证明人发给验证人. 当前, 当前最有效的产生零知识证明的方式是非交互的, 最有效的发布在区块链的方式是初始启动阶段在证明人与验证人之间产生共同的引用字符串. 我们将公共的引用字符串视为系统的公共参数.

<img src="../images/arithmetic-circuit.png" style="zoom: 67%;" />

Looking at such a circuit, we can think of the input values a, b, c as “traveling” left-to-right on the wires towards the output wire. Our next step is to build what is called a Rank 1 Constraint System, or R1CS, to check that the values are “traveling correctly”. In this example, the R1CS will confirm, for instance, that the value coming out of the multiplication gate where b and c went in is b*c.

:book: 观察如上逻辑电路, 我们可以认为输入数据a, b, c是沿着线路从左移动到右边的输出线路. 我们下一步是构建被称为1级限制系统, 或叫R1CS, 是用于校验该线路的正确性. 在该例子中R1CS将会校验从乘法门电路出来的是b*c该值由b和c输入得到.

In this R1CS representation, the verifier has to check many constraints — one for almost every wire of the circuit. (For technical reasons, it turns out we only have a constraint for wires coming out of multiplication gates.) In a 2012 [paper on the topic](https://eprint.iacr.org/2012/215.pdf), Gennaro, Gentry, Parno and Raykova presented a nice way to “bundle all these constraints into one”. This method uses a representation of the circuit called a Quadratic Arithmetic Program (QAP). The single constraint that needs to be checked is now between polynomials rather than between numbers. The polynomials can be quite large, but this is alright because when an identity does not hold between polynomials, it will fail to hold at most points. Therefore, you only have to [check that the two polynomials match at one randomly chosen point](https://z.cash/blog/snark-explain2) in order to correctly verify the proof with high probability.

:book: 在R1CS的表达式中, 验证人必须验证很多限制—几乎逻辑电路中的每一条连线. (处于技术原因, 原来从乘法门出来的连线我们只有一个约束.)  在2012年的[主题论文](https://eprint.iacr.org/2012/215.pdf)Gennaro, Gentry, Parno and Raykova展示了一个很好的方式"捆绑这些限制成为一个限制". 该方法使用了一种被成为Quadratic Arithmetic Program (QAP)的逻辑电路表现形式. 该单一限制需要在多项式之间检查而非数字. 该多项式可以非常巨大, 但这并没问题, 因为当多项式之间的恒等式无法成立时, 大多数点也将难以保持. 因此, 你只需要[校验两个多项式在一个随机点的情况](https://z.cash/blog/snark-explain2)用来验证该证明是高度可能的.

If the prover knew in advance which point the verifier would choose to check, they might be able to craft polynomials that are invalid, but still satisfy the identity at that point. With zk-SNARKs, sophisticated mathematical techniques such as [homomorphic encryption](https://en.wikipedia.org/wiki/Homomorphic_encryption) and [pairings](https://en.wikipedia.org/wiki/Pairing-based_cryptography) of elliptic curves are used to evaluate polynomials “blindly” – i.e. without knowing which point is being evaluated. The public parameters described above are used to determine which point will be checked, but in encrypted form so that neither the prover nor the verifier know what it is.

:book: 如果证明人事先知道验证人会选择哪个点用于检查, 那么他可以建立一个错误的多项式, 但用已知点仍然能够通过恒等式. 对于zk-SNARKs应用的复杂数学技术例如同态加密和椭圆曲线对用于盲目的评估多项式 即不知道哪个点会被用于评估. 上文提到的公开参数用于决定哪个点会被用于评估, 但是在的加密形式下所以不论证明人或者验证人都不会知道这个点是哪个.

The description so far has mainly addressed how to get the S and N in “SNARKs” — how to get a short, non-interactive, single message proof — but hasn’t addressed the “zk” (zero-knowledge) part which allows the prover to maintain the confidentiality of their secret inputs. It turns out that at this stage, the “zk” part can be easily added by having the prover use “random shifts” of the original polynomials that still satisfy the required identity.

For a step-by-step, in-depth explanation of key concepts behind zk-SNARKs in Zcash, see our SNARKs Explainer series with posts on:

1. [Homomorphic Hiding](https://electriccoin.co/blog/snark-explain1/)
2. [Blind Evaluation of Polynomials](https://electriccoin.co/blog/snark-explain2/)
3. [The Knowledge of Coefficient Test and Assumption](https://electriccoin.co/blog/snark-explain3/)
4. [How to make Blind Evaluation of Polynomials Verifiable](https://electriccoin.co/blog/snark-explain4/)
5. [From Computations to Polynomials](https://z.cash/blog/snark-explain5)
6. [The Pinocchio Protocol](https://z.cash/blog/snark-explain6)
7. [Pairings of Elliptic Curves](https://z.cash/blog/snark-explain7)

:book: 到目前为止的描述主要是在"SNARK"如何体现"S"和"N"上 — 如何得到一个简短的, 非交互的, 单一消息证明 — 并没有定位"zk-"(零知识)的部分, 该部分允许证明人保持自己输入数据的隐私性. 事实证明, 在该阶段"zk"的部分可以简单的添加通过"随机移位"初始的多项式并且该多项式任能够满足恒等式成立.

为了解释zk-SNARK我们需要先阐释一些基本的组件:

1. [同态隐藏(Homomorphic Hidings)](./1.Homomorphic_Hidings.md)
2. [多项式的盲估计(Blind Evaluation of Polynomials)](./2.Blind_Evaluation_of_Polynomials.md)
3. [系数测试和假设知识(The Knowledge of Coefficient Test and Assumption)](./3.The_Knowledge_of_Coefficient_Test_and_Assumption.md)
4. [如何构建多项式盲估计的验证(How to make Blind Evaluation of Polynomials Verifiable)](./4.How_to_make_Blind_Evaluation_of_Polynomials_Verifiable.md)
5. [从计算到多项式(From Computations to Polynomials)](./5.From_Computations_to_Polynomials.md)
6. [匹诺曹协议(The Pinocchio Protocol)](./6.The_Pinocchio_Protocol.md)
7. [椭圆曲线对(Pairings of Elliptic Curves)](./7.Pairings_of_Elliptic_Curves.md)

