本文为tendermint paper: [The latest gossip on BFT consensus](https://www.researchgate.net/publication/326412260_The_latest_gossip_on_BFT_consensus)的读书笔记, 本文旨在理清论文中所讲的BFT共识. 如果您在阅读过程中有任何意见可以发起ISSUE, 如果喜欢的话可以点击`star`.

## 引言

>Consensus is one of the most fundamental problems in distributed computing. It is important because of it’s role in State Machine Replication (SMR), a generic approach for replicating services that can be modeled as a deterministic state machine [1], [2]. The key idea of this approach is that service replicas start in the same initial state, and then execute requests (also called transactions) in the same order; thereby guaranteeing that replicas stay in sync with each other. The role of consensus in the SMR approach is ensuring that all replicas receive transactions in the same order. Traditionally, deployments of SMR based systems are in data-center settings (local area network), have a small number of replicas (three to seven) and are typically part of a single administration domain (e.g., Chubby [3]); therefore they handle benign (crash) failures only, as more general forms of failure (in particular, malicious or Byzantine faults) are considered to occur with only negligible probability.

共识是最基础的问题在分布式计算中. 由于共识在状态机复制(SMR)中扮演重要角色, 复制服务的通用方法可以将其建模为确定行状态机. 方法的关键点是拷贝服务启始与一个相同的初始状态, 然后以一致的顺序执行请求(也可被称为交易); 因此保证复制状态保持同步. 共识在在SMR的方法中作用是保证接收的交易按照按照一致的顺序. 传统地, 基于SMR系统的部署是用在数据中心(本地局域网), 只有比较小的拷贝数(3-7)并且通常是单个管理域的一部分; 因此他们仅处理良性故障(奔溃), 对于更普遍形式的故障(尤其是恶意的, 或拜占庭故障)被认为发生的可能性极小.

## 正文

*正文部分不会做全翻译, 但会对重要的特性单独拿出来翻译*

> The Tendermint consensus algorithm is inspired by the PBFT SMR algorithm [8] and the DLS algorithm for authenticated faults (the Algorithm 2 from [6]).

tendermint的共识所发受到pbft smr算法与dls算法中授权故障部分的启发. 

>Tendermint is not presented in the basic round model of [6]. Furthermore, we use the term round differently than in [6]; in Tendermint a round denotes a sequence of communication steps instead of a single communication step in [6]

tendermint并不是DLS基本的回合模型. 更进一步, 我们使用了round模型有别于DLS. tendermint中每一回合代表一系列的通信步骤而非DLS中的单一通信步骤.

>To ensure that a proposed value is accepted by all correct processes a proposer will 1) build the global state by receiving messages from other processes, 2) select the safe value to propose and 3) send the selected value together with the signed messages received in the first step to support it. The value v<sub>i</sub> that a correct process sends to the next proposer normally corresponds to a value the process considers as acceptable for a decision:
>* in PBFT [8] and DLS [6] it is not the value itself but a set of 2f + 1 signed messages with the same value id,
>* in Fast Byzantine Paxos [11] the value itself is being sent.

为了确保提案被所有正确处理单元接收, 提议者需要做到:
1. 建立全局状态通过接收来自其他所有处理单元的消息
2. 选择安全的值进行提议
3. 发送挑选的值与支持该值的在第一步接收到的签名消息.
按照正确的过程发送给下一个提议者的值v<sub>i</sub>通常对应于该过程认为可以接受的值:
* 在pbft和dls中, 提议值并非数值本身而是指被2f+1签名的值;
* 在快速拜占庭paxos中该值正是被发送的值.

![](./proposer_change.png)

>In both cases, using this mechanism in our system model (ie. high number of nodes over gossip based network) would have high communication complexity that increases with the number of processes: in the first case as the message sent depends on the total number of processes, and in the second case as the value (block of transactions) is sent by each process. The set of messages received in the first step are normally piggybacked on the proposal message (in the Figure 1 denoted with [v<sub>1..4</sub>]) to justify the choice of the selected value x. Note that sending this message also does not scale with the number of processes in the system.

两方面共同的, 在我们的系统模型中使用该机制(也就大量节点使用gossip的基础网络)会随着处理单元数目的增加在交流上变得格外复杂: 第一种情况该值的消息发送依赖于处理单元的数目, 而第二种情况该值(块中的交易)是由该轮的处理单元发送. 这一类在第一轮接收的消息通常会包含在提议的消息中(图1中的v<sub>1..4</sub>)为了证明选择值x的合理性. 请注意, 发送此消息也不会随着系统中处理单元的数量增长而扩展.

>We designed a novel termination mechanism for Tendermint that better suits the system model we consider. It does not require additional communication (neither sending new messages nor piggybacking information on the existing messages) and it is fully based on the communication pattern that is very similar to the normal case in PBFT [8]. Therefore, there is only a single mode of execution in Tendermint, i.e., there is no separation between the normal and the recovery mode, which is the case in other PBFT-like protocols (e.g., [8], [12] or [13]). We believe this makes Tendermint simpler to understand and implement correctly

我们为tendermint设计了新型的终止机制, 我们认为其更适合系统模型. 该机制不需要额外的通信(既不发送新消息也不需要在当前存在的消息中附带信息)并且完全是依据非常类似与pbft的通信模式. 因此, 在tendermint只有一种执行类型, 例如, 普通模式和恢复模式没有区别, 这与其他类pbft协议不同. 我们相信这使tendermint更简单去理解与实现更正确.

