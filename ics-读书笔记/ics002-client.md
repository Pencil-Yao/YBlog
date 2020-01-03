读书笔记是摘取ibc的文档ics协议中有趣的部分(可能也是关键部分)的翻译和源码对照, 意在理清ibc实现背后的原理和主要逻辑.
# [ics002-client](https://github.com/cosmos/ics/tree/master/spec/ics-002-client-semantics)
## Motivation
>Alice, a module on a machine, wants to introduce Bob, a second module on a second machine who Alice knows (and who knows Alice), to Carol, a third module on a third machine, who Alice knows but Bob does not. Alice must utilise an existing channel to Bob to communicate the canonically-serialisable validity predicate for Carol, with which Bob can then open a connection and channel so that Bob and Carol can talk directly. If necessary, Alice may also communicate to Carol the validity predicate for Bob, prior to Bob's connection attempt, so that Carol knows to accept the incoming request.

alice作为一台机器上的一个module, 想要将bo(第二台机器上的第二个模块, bob和alice互相人事)介绍给carol(第三台机器上的第三个模块, carol仅alice认识). alice必须使用现存的与bob的通道来发送规范的串行序列, 序列用于证明carol的有效性. 通过这个序列bob能够与carol开启一个channel和connection介于此bob和carol可以直接通信. 如有必要alice同样可以与carol通信发送另一个规范的串行序列, 序列用于证明bob的有效性, 这个操作在bob尝试与carol建立链接之前, 因此carol会接受与bob传来的请求.
>State machines implementing the IBC protocol can support any number of client types, and each client type can be instantiated with different initial consensus states in order to track different consensus instances. In order to establish a connection between two machines (see ICS 3), the machines must each support the client type corresponding to the other machine's consensus algorithm.

实现IBC协议的状态机可以支持任意的客户端的成员, 并且每一种客户端都可以被不同的初始共识状态初始化以用来追踪不同共识的实例. 为了在两个机器集群间建立connection连接(ICS 3), 一方机器集群必须支持另一方拥有不同共识算法的机器集群对应的客户端.
>Consensus is a Header generating function which takes the previous ConsensusState with the messages and returns the result.
```
type Consensus = (ConsensusState, [Message]) => Header
```
这显然是跨链协议中两条链互相同步对方的关键操作, ConsensusState是以往收到的另一条链的状态, *[Messge]数组为以往状态到当前状态间发生的交易*, 通过这两个参数, 我们可以相信至少平行链具备互相校验的能力, 然而查阅源码却没找到函数的原型, 能看到一个如下接口:
``` go
// Blockchain is consensus algorithm which generates valid Headers. It generates
// a unique list of headers starting from a genesis ConsensusState with arbitrary messages.
// This interface is implemented as defined in https://github.com/cosmos/ics/tree/master/spec/ics-002-client-semantics#blockchain.
type Blockchain interface {
    Genesis() ConsensusState // Consensus state defined in the genesis
    Consensus() Header // Header generating function
}
```
而[官方文档](https://github.com/cosmos/ics/tree/master/spec/ics-002-client-semantics#blockchain)指出:
>A blockchain is a consensus algorithm which generates valid Headers. It generates a unique list of headers starting from a genesis ConsensusState with arbitrary messages.

但很可惜没有实现Blockchain接口的结构体, 相信还在开发中.
在client update过程中有一个关键函数:
```
type checkValidityAndUpdateState = (Header) => Void
```
>If the provided header was valid, the client MUST also mutate internal state to store now-finalised consensus roots and update any necessary signature authority tracking (e.g. changes to the validator set) for future calls to the validity predicate.
>如果提供的header是有效的, client MUST更改内部状态: 保存当前确认的共识根以及更新必要的签名权限跟踪(即更新验证人集合), 为了之后的有效断言调用.

该函数已在[ibc源码](https://github.com/cosmos/gaia/tree/cwgoes/ibc-demo-fixes)中实现, 其中关键函数:
``` go
// VerifyFutureCommit will check to see if the set would be valid with a different
// validator set.
//
// vals is the old validator set that we know. Over 2/3 of the power in old
// signed this block.
//
// In Tendermint, 1/3 of the voting power can halt or fork the chain, but 1/3
// can't make arbitrary state transitions. You still need > 2/3 Byzantine to
// make arbitrary state transitions.
//
// To preserve this property in the light client, we also require > 2/3 of the
// old vals to sign the future commit at H, that way we preserve the property
// that if they weren't being truthful about the validator set at H (block hash
// -> vals hash) or about the app state (block hash -> app hash) we can slash
// > 2/3. Otherwise, the lite client isn't providing the same security
// guarantees.
//
// Even if we added a slashing condition that if you sign a block header with
// the wrong validator set, then we would only need > 1/3 of signatures from
// the old vals on the new commit, it wouldn't be sufficient because the new
// vals can be arbitrary and commit some arbitrary app hash.
//
// newSet is the validator set that signed this block. Only votes from new are
// sufficient for 2/3 majority in the new set as well, for it to be a valid
// commit.
//
// NOTE: This doesn't check whether the commit is a future commit, because the
// current height isn't part of the ValidatorSet. Caller must check that the
// commit height is greater than the height for this validator set.
func (vals *ValidatorSet) VerifyFutureCommit(newSet *ValidatorSet, chainID string,
    blockID BlockID, height int64, commit *Commit) error {
    oldVals := vals

    // Commit must be a valid commit for newSet.
    err := newSet.VerifyCommit(chainID, blockID, height, commit)
    if err != nil {
        return err
    }

    // Check old voting power.
    oldVotingPower := int64(0)
    seen := map[int]bool{}
    round := commit.Round()

    for idx, precommit := range commit.Precommits {
        if precommit == nil {
            continue
        }
        if precommit.Height != height {
            return errors.Errorf("Blocks don't match - %d vs %d", round, precommit.Round)
        }
        if precommit.Round != round {
            return errors.Errorf("Invalid commit -- wrong round: %v vs %v", round, precommit.Round)
        }
        if precommit.Type != PrecommitType {
            return errors.Errorf("Invalid commit -- not precommit @ index %v", idx)
        }
        // See if this validator is in oldVals.
        oldIdx, val := oldVals.GetByAddress(precommit.ValidatorAddress)
        if val == nil || seen[oldIdx] {
            continue // missing or double vote...
        }
        seen[oldIdx] = true

        // Validate signature.
        precommitSignBytes := commit.VoteSignBytes(chainID, idx)
        if !val.PubKey.VerifyBytes(precommitSignBytes, precommit.Signature) {
            return errors.Errorf("Invalid commit -- invalid signature: %v", precommit)
        }
        // Good precommit!
        if blockID.Equals(precommit.BlockID) {
            oldVotingPower += val.VotingPower
        }
        // else {
        // It's OK that the BlockID doesn't match. We include stray
        // precommits to measure validator availability.
        // }
    }

    if oldVotingPower <= oldVals.TotalVotingPower()*2/3 {
        return errTooMuchChange{oldVotingPower, oldVals.TotalVotingPower()*2/3 + 1}
    }
    return nil
}
```
该函数有详尽的函数解释, 大意为:
1.会验证Header.Commit中的签名, 但要求签名人在Old_ConsensusState的验证人集合中.
2.符合第一点的签名人的vote_power占据Old_ConsensusState验证人集合的vote_power总和的2/3以上(ps. 当前验证人以及Old_ConsensusState的验证人的vote_power的视图均依据Old_ConsensusState的时间点, 意味着当前验证人的当前vote_power并不会影响校验结果, *该方式显然是经验性的, 官方的解释也是为了light client可验证的需要, 在验证人集合以及验证人vote_power大幅度激变的情况下, ibc在该light client下无法实施, 但如果ibc交易的发生与ibc前置配置工作紧密进行, 第二点验证要求的满足是极大概率通过的, 从这个环节可以发现最大渎职行为会发生在light client第一个传送的*ConsensusState的验证, 这或许需要多light client的分布式验证???[大雾])
VerifyFutureCommit()函数方能通过, 之后更新client的ConsensusState状态.
>Proxy clients
>Proxy clients verify another (proxy) machine's verification of the target machine, by including in the proof first a proof of the client state on the proxy machine, and then a secondary proof of the sub-state of the target machine with respect to the client state on the proxy machine. This allows the proxy client to avoid storing and tracking the consensus state of the target machine itself, at the cost of adding security assumptions of proxy machine correctness.

代理客户端: 代理客户端验证代理机器对于目标机器的验证, 通过一份证明, 该证明包括一份代理机器上的客户状态的证明, 另一份代理机器人的目标机器子状态对应的客户状态证明. 这允许代理客户端避免存储和跟踪目标机器的共识状态, 但这一切建立在增加代理机器的安全风险上面.
*代理人也是ibc中关键中间人, 这段翻译我理解为代理客户端需要代理机器人本身的客户状态的有效性证明以及目标机器的客户状态在代理机器上的有效性证明, 猜测目标机器的客户端对于本地机器也是代理客户端与目标机器的关系, 那么代理机器的责任真是非常重大, 因为他可以做出伪造数据一类渎职行为.*
>Utilising past roots
>To avoid race conditions between client updates (which change the state root) and proof-carrying transactions in handshakes or packet receipt, many IBC handler functions allow the caller to specify a particular past root to reference, which is looked up by height. IBC handler functions which do this must ensure that they also perform any requisite checks on the height passed in by the caller to ensure logical correctness.

使用过去的roots: 为了避免client更新状态(区块高度增长, 会影响到根状态)和携带证明的交易握手过程或包接收过程之间存在的竞态, 很多IBC的处理函数会允许调用者指定特定高度的根状态作为参考. IBC处理函数必须确保他们对调用者传入的高度执行了必要的检查, 以保证逻辑的正确性.

## Create Client的伪代码
```
function createClient(
  id: Identifier,
  clientType: ClientType,
  consensusState: ConsensusState) {
    abortTransactionUnless(validateClientIdentifier(id))
    abortTransactionUnless(privateStore.get(clientStatePath(id)) === null)
    abortSystemUnless(provableStore.get(clientTypePath(id)) === null)
    clientState = clientType.initialize(consensusState)
    privateStore.set(clientStatePath(id), clientState)
    provableStore.set(clientTypePath(id), clientType)
}
```
与源码实现的函数比较:
``` go
// CreateClient creates a new client state and populates it with a given consensus
// state as defined in https://github.com/cosmos/ics/tree/master/spec/ics-002-client-semantics#create
func (k Keeper) CreateClient(
    ctx sdk.Context, clientID string,
    clientType exported.ClientType, consensusState exported.ConsensusState,
) (types.State, error) {
    _, found := k.GetClientState(ctx, clientID)
    if found {
        return types.State{}, types.ErrClientExists(k.codespace, clientID)
    }

    _, found = k.GetClientType(ctx, clientID)
    if found {
        panic(fmt.Sprintf("consensus type is already defined for client %s", clientID))
    }

    clientState := k.initialize(ctx, clientID, consensusState)
    k.SetVerifiedRoot(ctx, clientID, consensusState.GetHeight(), consensusState.GetRoot())
    k.SetClientState(ctx, clientState)
    k.SetClientType(ctx, clientID, clientType)
    k.Logger(ctx).Info(fmt.Sprintf("client %s created at height %d", clientID, consensusState.GetHeight()))
    return clientState, nil
}
```
**k.SetVerifiedRoot(ctx, clientID, consensusState.GetHeight(), consensusState.GetRoot())似乎未出现在伪代码中**, *是不是忘了?*
## Update Client的伪代码
```
function updateClient(
  id: Identifier,
verifyMembership
  header: Header) {
    clientType = provableStore.get(clientTypePath(id))
    abortTransactionUnless(clientType !== null)
    clientState = privateStore.get(clientStatePath(id))
    abortTransactionUnless(clientState !== null)
    clientType.checkValidityAndUpdateState(clientState, header)
}
```
源码实现:
``` go
// UpdateClient updates the consensus state and the state root from a provided header
func (k Keeper) UpdateClient(ctx sdk.Context, clientID string, header exported.Header) error {

    clientType, found := k.GetClientType(ctx, clientID)
    if !found {
        return sdkerrors.Wrap(types.ErrClientTypeNotFound(k.codespace), "cannot update client")
    }

    // check that the header consensus matches the client one
    if header.ClientType() != clientType {
        return sdkerrors.Wrap(types.ErrInvalidConsensus(k.codespace), "cannot update client")
    }

    clientState, found := k.GetClientState(ctx, clientID)
    if !found {
        return sdkerrors.Wrap(types.ErrClientNotFound(k.codespace, clientID), "cannot update client")
    }

    if clientState.Frozen {
        return sdkerrors.Wrap(types.ErrClientFrozen(k.codespace, clientID), "cannot update client")
    }

    consensusState, found := k.GetConsensusState(ctx, clientID)
    if !found {
        return sdkerrors.Wrap(types.ErrConsensusStateNotFound(k.codespace), "cannot update client")
    }

    if header.GetHeight() < consensusState.GetHeight() {
        return sdkerrors.Wrap(
            sdk.ErrInvalidSequence(
                fmt.Sprintf("header height < consensus height (%d < %d)", header.GetHeight(), consensusState.GetHeight()),
            ),
            "cannot update client",
        )
    }

    consensusState, err := consensusState.CheckValidityAndUpdateState(header)
    if err != nil {
        return sdkerrors.Wrap(err, "cannot update client")
    }

    k.SetConsensusState(ctx, clientID, consensusState)
    k.SetVerifiedRoot(ctx, clientID, consensusState.GetHeight(), consensusState.GetRoot())
    k.Logger(ctx).Info(fmt.Sprintf("client %s updated to height %d", clientID, consensusState.GetHeight()))
    return nil
}
```
**k.SetConsensusState(ctx, clientID, consensusState), k.SetVerifiedRoot(ctx, clientID, consensusState.GetHeight(), consensusState.GetRoot())与create client类似, 伪代码中也缺失,** (看来是有意想要这么设置的), **其中核心函数应该是CheckValidityAndUpdateState()的方法, 上文有该函数的分析**, 和我的个人看法.