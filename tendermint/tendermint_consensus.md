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

> Note that the orthogonal approach for reducing message complexity in order to improve scalability and decentralization (number of processes) of BFT consensus algorithms is using advanced cryptography (for example Boneh-Lynn-Shacham (BLS) signatures [14]) as done for example in SBFT [15]

值得注意的是为了降低BFT共识算法的信息复杂度以提升可扩展性以及分散性(可容纳的处理单元数量)的正交方法目前正在使用高级的加密方法(例如Boneh-Lynn-Shacham (BLS) 签名, 该方法就用于SBFT共识中)

>Processes in our model are not part of a single administration domain; therefore we cannot enforce a direct network connectivity between all processes. Instead, we assume that each process is connected to a subset of processes called peers, such that there is an indirect communication channel between all correct processes

处理单元在我们的模型中并不一定只是单一管理域的一部分; 因此我们不能强制网络为一个直连网络: 连接了所有处理单元. 相反, 我们只能假设每一个处理单元知识与一个子集的处理单元(谓之peer)相连, 因此存在非直接通信的通道连接所有正确的处理单元.

### 定义
规定&theta;为系统的执行时间上限; GST(Global Stabilization Time): 消息在正确的处理单元中通信所需要经过多少时间后才被认为可靠(消息传播时间t > GST, 消息才被认为可靠, 总时间T = t + &theta;).

>We assume that process steps (which might include sending and receiving messages) take zero time. Processes are equipped with clocks so they can measure local timeouts. All protocol messages are signed, i.e., when a correct process q receives a signed message m from its peer, the process q can verify who was the original sender of the message m.

我们假设处理某些步骤(包括接收和发送消息)耗时为零. 所有处理单元都配备时钟, 所以他们可以推断本地超时. 所有的协议消息都是被签名的, 例如, 当正确的处理单元q从peer接收了签名的消息m, 处理单元q能够验证谁是这个消息m原始的发送人.

> Gossip communication: If a correct process p receives some message m at time t, all correct processes will receive m before max{t, GST } + &theta.

gossip通信: 如果正确的处理单元p接收到消息m的时间为t, 所有正确的处理单元接收到m的时间会小于max{t, GST } + &theta.

>State machine replication (SMR) is a general approach for replicating services modeled as a deterministic state machine [1], [2]. The key idea of this approach is to guarantee that all replicas start in the same state and then apply requests from clients in the same order, thereby guaranteeing that the replicas’ states will not diverge. 

复制状态机(SMR)是用于复制建模服务为了确定性状态机的通用方法. 其关键思想是保证所有复制都起始于一个相同状态然后以相同顺序执行来自客户端一致顺序的请求, 因此保证复制的状态不会分叉.

>Tendermint solves state machine replication by sequentially executing consensus instances to agree on each block of transactions that are then executed by the service being replicated. We consider a variant of the Byzantine consensus problem called Validity Predicate-based Byzantine consensus that is motivated by blockchain systems [17]. The problem is defined by an agreement, a termination, and a validity property.
>* Agreement: No two correct processes decide on different values.
>* Termination: All correct processes eventually decide on a value.
>* Validity: A decided value is valid, i.e., it satisfies the predefined predicate denoted valid().

tendermint通过顺序执行共识实例来校验区块中的交易, 然后由复制的服务来执行, 以解决状态复制的问题. 我们考虑拜占庭共识问题的变种, 它是由区块链系统驱动的基于有效性凭证的拜占庭共识. 该问题由一致性, 最终性以及有效性定义.
* 一致性: 不存在正确的处理单元选择不同的值.
* 最终性: 所有正确的处理单元最终会选择同一个值.
* 有效性: 决定的值都是有效的, 例如, 它能通过预定义为valid()的校验.

>For example, the condition 2f + 1<**PRECOMMIT**, h<sub>p</sub>, r, id(v)>, evaluates to true upon reception of **PRECOMMIT** messages for height h<sub>p</sub>, a round r and with value equal to id(v) whose senders have aggregate voting power at least equal to 2f + 1

例如,  2f + 1<**PRECOMMIT**, h<sub>p</sub>, r, id(v)>的条件, 执行为true依据PRECOMMIT消息具备接收的区块高度h<sub>p</sub>, 轮数为r以及值与id(v)一致, 并且消息的合计投票权重至少达到2f + 1.

>The variables with index p are process local state variables, while variables without index p are value placeholders. The sign * denotes any value.

变量有p下标为过程局部变量, 而没有p下标变量为占位符. 标记\*表示任意值.

>We denote with n the total voting power of processes in the system, and we assume that the total voting power of faulty processes in the system is bounded with a system parameter f. The algorithm assumes that n > 3f, i.e., it requires that the total voting power of faulty processes is smaller than one third of the total voting power. For simplicity we present the algorithm for the case n = 3f + 1.

伪翻译: 文章中指定了投票权值要达到n, 并且n = 3f + 1, 其中f定义为拜占庭节点的投票权值总和.

>The algorithm proceeds in rounds, where each round has a dedicated proposer. The mapping of rounds to proposers is known to all processes and is given as a function *proposer*(h, round), returning the proposer for the round round in the consensus instance h. We assume that the proposer selection function is weighted roundrobin, where processes are rotated proportional to their voting power7. The internal protocol state transitions are triggered by message reception and by expiration of timeouts. There are three timeouts in Algorithm 1: timeoutPropose, timeoutPrevote and timeoutPrecommit. The timeouts prevent the algorithm from blocking and waiting forever for some condition to be true, ensure that processes continuously transition between rounds, and guarantee that eventually (after GST) communication between correct processes is timely and reliable so they can decide. The last role is achieved by increasing the timeouts with every new round r, i.e, timeoutX(r) = initTimeoutX + r * timeoutDelta; they are reset for every new height (consensus instance).

算法以轮进行处理, 每一轮都会有一个专门的提议人. 每一轮对应的提议人对所有处理单元来说都是已知的并由函数*proposer*(h, round), 该函数返回共识实例h下round轮的提议者. 我们假设提议者选择为加权轮询, 该过程按其投票权成比例轮换. 内部协议状态转换由消息接收和超时到期触发. 在算法1中有3种超时: timeoutPropose, timeoutPrevote以及timeoutPrecommit. 超时是为了防止算法因为等待某些条件为true导致堵塞甚至永久等待, 以及保证每一轮之间连续地处理转换, 并且最终保证(GST时间之后)正确的处理单元之间通信是及时的和可靠地因此需要定义这些超时. 最后一个角色是通过在每个新回合r中增加超时来实现的, 例如 timeoutX(r) = initTimeoutX + r*timeoutDelta; 该时间会在每个新高度重置(共识实例).

>Processes exchange the following messages in Tendermint: PROPOSAL, PREVOTE and PRECOMMIT. The PROPOSAL message is used by the proposer of the current round to suggest a potential decision value, while PREVOTE and PRECOMMIT are votes for a proposed value. According to the classification of consensus algorithms from [10], Tendermint, like PBFT [7] and DLS [6], belongs to class 3, so it requires two voting steps (three communication exchanges in total) to decide a value. The Tendermint consensus algorithm is designed for the blockchain context where the value to decide is a block of transactions (ie. it is potentially quite large, consisting of many transactions). Therefore, in the Algorithm 1 (similar as in [7]) we are explicit about sending a value (block of transactions) and a small, constant size value id (a unique value identifier, normally a hash of the value, i.e., if id(v) = id(v′), then v = v′). The PROPOSAL message is the only one carrying the value; PREVOTE and PRECOMMIT messages carry the value id. A correct process decides on a value v in Tendermint upon receiving the PROPOSAL for v and 2f +1 voting-power equivalent PRECOMMIT messages for id(v) in some round r. In order to send PRECOMMIT message for v in a round r, a correct process waits to receive the PROPOSAL and 2f +1 of the corresponding PREVOTE messages in the round r. Otherwise, it sends PRECOMMIT message with a special nil value. This ensures that correct processes can PRECOMMIT only a single value (or nil) in a round. As proposers may be faulty, the proposed value is treated by correct processes as a suggestion (it is not blindly accepted), and a correct process tells others if it accepted the PROPOSAL for value v by sending PREVOTE message for id(v); otherwise it sends PREVOTE message with the special nil value.

tendermint中处理单元交换以下消息: **PROPOSAL**, **PREVOTE**以及**PRECOMMIT**. **PROPOSAL**是提议者用于在当前轮建议一个可能的决定值, 而**PROPOSAL**和**PRECOMMIT**消息携带着这个值的ID. 一个正确的处理单元


Algorithm 1 Tendermint consensus algorithm
---

```typescript
Initialization:
    hp := 0 /* current height, or consensus instance we are currently executing */
    roundp := 0 /* current round number */
    stepp ∈ {propose, prevote, precommit}
    decisionp[] := nil
    lockedValuep := nil
    lockedRoundp := −1
    validValuep := nil
    validRoundp := −1
upon start do StartRound(0)
Function StartRound(round) :
    roundp ← round
    stepp ← propose
    if proposer(hp, roundp) = p then
        if validV aluep 6= nil then
            proposal ← validV aluep
        else
            proposal ← getV alue()
        broadcast hPROPOSAL, hp, roundp, proposal, validRoundpi
    else
        schedule OnT imeoutP ropose(hp, roundp) to be executed after timeoutP ropose(roundp)
upon hPROPOSAL, hp, roundp, v, −1i from proposer(hp, roundp) while stepp = propose do
    if valid(v) ∧ (lockedRoundp = −1 ∨ lockedV aluep = v) then
        broadcast hPREVOTE, hp, roundp, id(v)i
    else
        broadcast hPREVOTE, hp, roundp, nili
    stepp ← prevote
upon hPROPOSAL, hp, roundp, v, vri from proposer(hp, roundp) AND 2f + 1 hPREVOTE, hp, vr, id(v)i while stepp = propose ∧ (vr ≥ 0 ∧ vr < roundp) do
    if valid(v) ∧ (lockedRoundp ≤ vr ∨ lockedV aluep = v) then
        broadcast hPREVOTE, hp, roundp, id(v)i
    else
        broadcast hPREVOTE, hp, roundp, nili
    stepp ← prevote
upon 2f + 1 hPREVOTE, hp, roundp, ∗i while stepp = prevote for the first time do
    schedule OnT imeoutP revote(hp, roundp) to be executed after timeoutP revote(roundp)
upon hPROPOSAL, hp, roundp, v, ∗i from proposer(hp, roundp) AND 2f + 1 hPREVOTE, hp, roundp, id(v)i while valid(v) ∧ stepp ≥ prevote for the first time do
    if stepp = prevote then
        lockedV aluep ← v
        lockedRoundp ← roundp
        broadcast hPRECOMMIT, hp, roundp, id(v))i
        stepp ← precommit
    validV aluep ← v
    validRoundp ← roundp
upon 2f + 1 hPREVOTE, hp, roundp, nili while stepp = prevote do
    broadcast hPRECOMMIT, hp, roundp, nili
    stepp ← precommit
upon 2f + 1 hPRECOMMIT, hp, roundp, ∗i for the first time do
    schedule OnT imeoutP recommit(hp, roundp) to be executed after timeoutP recommit(roundp)
upon hPROPOSAL, hp, r, v, ∗i from proposer(hp, r) AND 2f + 1 hPRECOMMIT, hp, r, id(v)i while decisionp[hp] = nil do
    if valid(v) then
        decisionp[hp] = v
        hp ← hp + 1
        reset lockedRoundp, lockedV aluep, validRoundp and validV aluep to initial values and empty message log
        StartRound(0)
upon f + 1 h∗, hp, round, ∗, ∗i with round > roundp do
    StartRound(round)
Function OnT imeoutP ropose(height, round) :
    if height = hp ∧ round = roundp ∧ stepp = propose then
        broadcast hPREVOTE, hp, roundp, nili
        stepp ← prevote
Function OnT imeoutP revote(height, round) :
    if height = hp ∧ round = roundp ∧ stepp = prevote then
        broadcast hPRECOMMIT, hp, roundp, nili
        stepp ← precommit
Function OnT imeoutP recommit(height, round) :
    if height = hp ∧ round = roundp then
        StartRound(roundp + 1)
```

