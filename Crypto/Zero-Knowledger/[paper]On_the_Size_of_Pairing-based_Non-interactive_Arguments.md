该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

# On the Size of Pairing-based Non-interactive Arguments

这篇文章是可以理解 zk-SNARK 的实现的论文依据的摘要, [文章地址](https://eprint.iacr.org/2016/260.pdf). 我会选择重要的部分进行翻译外加一些个人的注释.

**Abstract**. Non-interactive arguments enable a prover to convince a verifier that a statement is true. Recently there has been a lot of progress both in theory and practice on
constructing highly efficient non-interactive arguments with small size and low verification complexity, so-called succinct non-interactive arguments (SNARGs) and succinct non-interactive arguments of knowledge (SNARKs).

Many constructions of SNARGs rely on pairing-based cryptography. In these constructions a proof consists of a number of group elements and the verification consists of checking a number of pairing product equations. The question we address in this article is how efficient pairing-based SNARGs can be. Our first contribution is a pairing-based (preprocessing) SNARK for arithmetic circuit satisfiability, which is an NP-complete language. In our SNARK we work with asymmetric pairings for higher efficiency, a proof is only 3 group elements, and verification consists of checking a single pairing product equations using 3 pairings in total. Our SNARK is zero-knowledge and does not reveal anything about the witness the prover uses to make the proof.

As our second contribution we answer an open question of Bitansky, Chiesa, Ishai, Ostrovsky and Paneth (TCC 2013) by showing that 2-move linear interactive proofs cannot have a linear decision procedure. It follows from this that SNARGs where the prover and verifier use generic asymmetric bilinear group operations cannot consist of a single group element. This gives the first lower bound for pairing-based SNARGs. It remains an intriguing open problem whether this lower bound can be extended to rule out 2 group element SNARGs, which would prove optimality of our 3 element construction.

:book: **摘要**. 非交互参数使得证明者可以让验证人确信状态是正确的. 近期有大量的理论和实践工作在高效构建非交互论证以较小的大小和更低的验证复杂度, 因此被称作简明非交互证明(SNARGs)以及简明非交互知识论证(SNARKs).

大多数 SNARGs 的构建基于成对的密码学. 在这些构造中, 一个证明由一些组元组成, 而验证则由检查一些配对积方程组成. 本文主要说明基于成对的 SNARGs 能够做到多高效. 我们的第一个贡献是成对 SNARK(预处理) 的算术电路可满足性, 该为 NP 完全语言. 在我们的 SNARK 我们主要工作在非对称成对的更高效, 一个证明知识 3 组元素, 而验证由检查一个成对乘法方程(总共有 3 对)组成. 我们的 SNARK 是零知识证明并不会泄露给验证人任何证明人用于产生证明的信息.

至于我们的第二个贡献是证明了 Bitansky, Chiesa, Ishai, Ostrovsky and Paneth (TCC 2013)  的公开推测 2 步的现行交互证明无法得到一个线性确定的过程. 其基于 SNARGs 中证明人和验证人使用通用的非对称双线性群组操作不能由单个群组元素组成. 这给了基于成对算法的 SNARGs 第一个下界. 这个下限是否可以扩展到排除 2 个组元的 SNARG, 这将证明我们 3 个组元构造的最优性, 这仍然是一个有趣的开放问题.

## 1. introduction

Goldwasser, Micali and Rackoff [GMR89] introduced zero-knowledge proofs that enable a prover to convince a verifier that a statement is true without revealing anything else. They have three core properties:

**Completeness**: Given a statement and a witness, the prover can convince the verifier.

**Soundness**: A malicious prover cannot convince the verifier of a false statement.

**Zero-knowledge**: The proof does not reveal anything but the truth of the statement, in particular it does not reveal the prover’s witness.

:book: Goldwasser, Micali and Rackoff [GMR89] 介绍了零知识证明能够让证明人在不泄露任何信息的情况下使验证人确信状态是正确的. 他们有三个核心特征:

**完整性**: 提供一个状态和目击人, 证明人可以使验证人确信.

**良好**: 一个恶意的证明人无法使验证人确信一个伪造的状态.

**零知识**: 证明除了状态的真相不会泄露其他任何东西, 尤其是不会泄露证明人的目击者.

Blum, Feldman and Micali [BFM88] extended the notion to non-interactive zero-knowledge (NIZK) proofs in the common reference string model. NIZK proofs are useful in the construction of non-interactive cryptographic schemes, e.g., digital signatures and CCA-secure public key encryption.

:book: Blum, Feldman and Micali [BFM88] 扩展了非交互零知识证明(NIZK)的概念以通用参考字符串模型. NIZK 证明在构建非交互密码学方案是有效的, 例如, 数字签名和 CCA 安全密钥加密.











