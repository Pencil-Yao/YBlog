本文为solana白皮书: [Solana: A new architecture for a high performance blockchain v0.8.14](https://www.researchgate.net/publication/224165674_Generic_Construction_of_Consensus_Algorithms_for_Benign_and_Byzantine_Faults)的读书笔记, 本文旨在理清索拉纳提出新型POH共识机制, 本文写作时间较早很多是我的个人感想, 多处引用可能也不严谨. 如果您在阅读过程中有任何意见可以发起ISSUE, 如果喜欢的话可以点击`star`.

*solana<sup>1</sup>是众多区块链中非常耀眼的一位"新人", 号称实现了710k tps<sup>2</sup>(在1gb网络环境下)而收获了大量的目光. 众所周知区块链技术的tps与其共识机制有着密不可分的关系, 今天笔者透过solana的白皮书来和大家唠叨一下solana's PoH.*

# Solana White Paper Digest

## Solana Proof of History

*总所周知索拉纳采用的是PofH(历史证明)共识, 是一种历史数据运算长期累加的一种证明方式, 与大多数区块链项目一样PoH也是**一种可验证**, **确定性**的共识方式, 由于poh验证特性索拉纳项目方要求验证节点需要配备gpu这一昂贵的硬件, 下面我将结合白皮书以及其书面材料来尽量简单明了的说明索拉纳的poh共识机制:*

![solana-poh.png](./solana-poh.png)