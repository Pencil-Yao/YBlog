该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

文章原文[cosmos_v1.2](https://gateway.pinata.cloud/ipfs/QmdC3YuZBUq5b9mEr3bKTDRq4XLcxafe3LHqDNFUgUoa61)

本文标注的是COSMOSv1.2版本实际上就是大家口中的ATOM2.0, 其提出的目的是针对ATOM的价值捕获能力太差, 同时每年的以几乎20%的幅度无限增发; 主要基于上述问题提出了COSMOSv1.2, 但是最终结果是[链接](https://www.mintscan.io/cosmos/proposals/82)被社区reject了. 本文会抓去文章中一下关键的观点和文字进行翻译分析来窥探当前Cosmos社区的主要方向.

> The vision of the Cosmos Network, as laid out in 2016, has been realized. The creation of a secure software stack for building and connecting application specific blockchains has catalyzed a thriving ecosystem of sovereign interoperable communities and an increasingly dynamic inter-blockchain communication economy. 

:book: Cosmos网络最早在2016年提出, 至今Cosmos网络已经实现了. 用于构建和连接特定应用程序区块链的安全软件堆栈的创建催生了一个繁荣的主权互操作社区生态系统和一个日益活跃的区块链间通信经济.

:bulb: 笔者以为文章的作者显然有点过于乐观了, Cosmos网络还是一个方兴未艾的阶段, 尤其是terra轰然倒塌以后.

> • **Interchain Scheduler:** The IBC economy is a patchwork of asynchronous markets, introducing an enormous variety of cross-domain maximum extractable value (MEV) opportunities. This market can be made more efficient, more secure, and more lucrative for Cosmos chains and their users. The interchain needs a secure block space market to avoid off-chain cartelization and more options for chains seeking to optimize the use of block space.
>
> • **Interchain Allocator:** Sustaining the rapid growth of the interchain will require new approaches to on-chain economic coordination. The Interchain Allocator is a platform for delegated parties to grow and align ATOM-based markets, facilitating multi-chain trust and coordination. The integration of sovereign political economies through transparent col lateralized agreements will unlock higher-order economic power and capital efficiency.

:book: Interchain Scheduler: IBC 经济是异步市场的穿针线, 引入了种类繁多的跨领域最大可提取价值 (MEV) 机会. 对于 Cosmos 链及其用户来说, 这个市场可以变得更高效、更安全、更有利可图. 跨链需要一个安全的区块空间市场来避免链下卡特尔化, 并为寻求优化区块空间使用的链提供更多选择.

Interchain Allocator: 维持跨链的快速增长将需要新的链上经济协调方法. Interchain Allocator 是一个平台, 供委托方发展和调整基于 ATOM 的市场, 促进多链信任和协调. 通过透明的抵押协议整合主权政治经济将释放更高阶的经济实力和资本效率.

> The Interchain Scheduler and the Interchain Allocator drive the growth of the Cosmos Network, together creating a flywheel whereby: 
> 1. Cosmos Hub collects revenues from interchain economic activity by creating a secure block space market, the Interchain Scheduler, and charging a matching fee. 
> 2. Revenue is used to drive long term ecosystem alignment and add promising new projects to the Cosmos Hub’s holdings via the Interchain Allocator. In turn, these projects expand the Scheduler’s addressable market. 

:book: Interchain Scheduler 和 Interchain Allocator 推动了 Cosmos 网络的发展，共同创造了一个飞轮，从而:

1. Cosmos Hub 通过创建安全的区块空间市场从链间经济活动中收取收入, Interchain Scheduler并收取匹配费用.
2. 收益是用来驱动长期经济调整和通过Interchain Allocator将有前景的新项目添加到Cosmos Hub’s holdings. 反过来, 这些项目扩大了调度程序的潜在市场.

:bulb: 这就是cosmos v1.2的核心功能, 通过Interchain Scheduler获得MEV收益, 通过Interchain Allocator提高资本利用率, 并还提供验证人服务(这点有点模仿polkadot), cosmos的体系完全有别于polkadot, 很难说真的有项目需要付费使用cosmos验证人为其增加安全性; 因为链级的安全性已经通过tendermint提供了, 而且cosmos生态本来就是主权独立, 这难免让人感觉是在开倒车.

The Cosmos Hub is the most secure pillar of the interchain.

:book: Cosmos Hub 是跨链最安全的支柱.

:bulb: 真就没明白文章开头说链间网络实现了, 现在又说cosmos是最安全的跨链支柱? 实在是觉得文章的作者太过于乐观了, 至此笔者认为文章没有太大的翻译价值, 就这场会议吹嘘atom要上1000, 我已感觉到cosmos变得过于浮躁了.
