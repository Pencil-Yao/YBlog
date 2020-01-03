ps. 测试环境:
```
Linux Yieazy 4.15.0-72-generic #81-Ubuntu SMP Tue Nov 26 12:20:02 UTC 2019 x86_64 x86_64 x86_64 GNU/Linux
ubuntu18.04
go version: 1.13.5
cpu: Intel(R) Core(TM) i7-4720HQ CPU @ 2.60GHz
memory: 16GB
```
读书笔记基于cosmos最新的ibc开发分支: **cwgoes/ibc-demo-fixes**, 主要是针对ibc相关的代码解读和ibc的实现机制.
ibc-demo中给出了ibc环境的完整部署与ibc交易的调试步骤以及所有的源代码, 但是却为给出证明ibc的有效性以及相关的数学证明, 因此本文将分析ibc的实现机制以及cosmos声称ibc具备跨链操作的原子特性进行分析.
完整的ibc-demo环境, [链接在这里](https://github.com/cosmos/gaia/blob/cwgoes/ibc-demo-fixes/ibc-demo.md)
抛开gaia的初始环境, 直接到文档的IBC Command Sequence, 我们看到:

## 操作一: Client Creation
client creation中不仅传入chain id还传入一个非常重要的共识状态文档(consensus_state.json), 根据官方给出的命令:
```shell
gaiacli --home ibc1/n0/gaiacli q ibc client node-state
```
我们可以同样生成一个, 结果:
```json
{
    "type": "ibc/client/tendermint/ConsensusState",
    "value": {
        "chain_id": "ibc0",
        "height": "71497",
        "root": {
            "type": "ibc/commitment/merkle/Root",
            "value": {
                "hash": "2gi0YPMec1GHn4WNR0urWPpf2kABOTKBT5OaFfATtFQ="
            }
        },
        "next_validator_set": {
            "validators": [
                {
                    "address": "F6009D93FEC4CACDD273225E78DDAF07B582FC35",
                    "pub_key": {
                        "type": "tendermint/PubKeyEd25519",
                        "value": "6yUYsrbCi1CY7NBSy8Zxx2CKA+Tc1YsUX+w57OFGM58="
                    },
                    "voting_power": "100",
                    "proposer_priority": "0"
                }
            ],
            "proposer": {
                "address": "F6009D93FEC4CACDD273225E78DDAF07B582FC35",
                "pub_key": {
                    "type": "tendermint/PubKeyEd25519",
                    "value": "6yUYsrbCi1CY7NBSy8Zxx2CKA+Tc1YsUX+w57OFGM58="
                },
                "voting_power": "100",
                "proposer_priority": "0"
            }
        }
    }
}
```
源码中的结构定义:
```go
// ConsensusState defines a Tendermint consensus state
type ConsensusState struct {
    ChainID string `json:"chain_id" yaml:"chain_id"`
    Height uint64 `json:"height" yaml:"height"` // NOTE: defined as 'sequence' in the spec
    Root commitment.RootI `json:"root" yaml:"root"`
    NextValidatorSet *tmtypes.ValidatorSet `json:"next_validator_set" yaml:"next_validator_set"` // contains the PublicKey
}
```
 这个json是其中一条链的共识信息, 其中包含了块高信息, 接下来的验证人集信息以及一个root信息, 从root的type可以看出, root中的hash与当前块高merkle树相关(**为当前块的AppHash**).
client创建的命令:
```shell
# client for chain ibc1 on chain ibc0
echo -e "12345678\n" | gaiacli --home ibc0/n0/gaiacli \
  tx ibc client create ibconeclient \
  $(gaiacli --home ibc1/n0/gaiacli q ibc client node-state) \
  --from n0 -y -o text

# client for chain ibc0 on chain ibc1
echo -e "12345678\n" | gaiacli --home ibc1/n0/gaiacli \
  tx ibc client create ibczeroclient \
  $(gaiacli --home ibc0/n0/gaiacli q ibc client node-state) \
  --from n1 -y -o text
```
该命令的执行结果, 会将consensus state保存的信息将以key为clients/{chain-id}/consensusState的存入数据库中, 其中的root信息又会再次以key为clients/{chain-id}/roots/{height}存入数据库中.
创建的client的结构:

```go
// State is a type that represents the state of a client.
// Any actor holding the Stage can access on and modify that client information.
type State struct {
    // Client ID
    id string `json:"id" yaml:"id"`
    // Boolean that states if the client is frozen when a misbehaviour proof is
    // submitted in the event of an equivocation.
    Frozen bool `json:"frozen" yaml:"frozen"`
}
```
*ps. 该结构与具体节点和地址无关, 显然client的创建是以链本身为单位, 而链的识别仅仅以chain-id, 对client的操作仅有冻结选项.*
完成了几项数据库的写入之后, key为clients/{chain-id}/consensusState和key为clients/{chain-id}/roots/{height}是写入操作中最重要的两项, 其他具体写入项请参看HandleMsgCreateClient函数, 在此不赘述.
之后会发出client创建成功的events:
```go
ctx.EventManager().EmitEvents(sdk.Events{
   sdk.NewEvent(
      EventTypeCreateClient,
      sdk.NewAttribute(AttributeKeyClientID, msg.ClientID),
   ),
   sdk.NewEvent(
      sdk.EventTypeMessage,
      sdk.NewAttribute(sdk.AttributeKeyModule, AttributeValueCategory),
      sdk.NewAttribute(sdk.AttributeKeySender, msg.Signer.String()),
   ),
})
```
## 操作二: Connection Creation
>In order to send transactions using IBC there are two different handshakes that must be performed. First there is a connection created between the two chains. Once the connection is created, an application specific channel handshake is performed which allows the transfer of application specific data. Examples of applications are token transfer, cross-chain validation, cross-chain accounts, and in this tutorial ibc-mock.

为了使用IBC发送交易那么必须执行两种不同握手. 首先, 两个链之间建立了连接, 将执行特定于应用程序的通道握手, 从而可以传输特定于应用程序的数据.  应用程序示例包括令牌转移, 跨链验证, 跨链帐户以及本教程中的ibc-mock.
创建connection的命令:
```shell
gaiacli \
  --home ibc0/n0/gaiacli \
  tx ibc connection handshake \
  connectionzero ibconeclient $(gaiacli --home ibc1/n0/gaiacli q ibc client path) \
  connectionone ibczeroclient $(gaiacli --home ibc0/n0/gaiacli q ibc client path) \
  --chain-id2 ibc1 \
  --from1 n0 --from2 n1 \
  --node1 tcp://localhost:26657 \
  --node2 tcp://localhost:26557
```
其中产生path chain的命令:
```shell
gaiacli --home ibc1/n0/gaiacli q ibc client path
```
得到的结果:
```json
{
    "type": "ibc/commitment/merkle/Prefix",
    "value": {
        "key_prefix": "aWJj"
    }
}
```
connection create的流程:

* connection_open_init操作: 
&emsp;1. 在ibc0链上记录ConnectionEnd状态为init, 并且以connections/connectionzero和clients/ibconeclient/connections为key存储该ConnectionEnd.
* update_client操作: 
&emsp;1. 会做对client是否frozen的检查, 以及块高检查, client是否tendermint client检查(是否为同构链), 以及检查ibc0最新consensus state, 
&emsp;2. 最后更新ibc1上的client: ibczeroclient的consensus state中的root, height以及nextvalidatorset字段(除了chain-id, 都更新到最新状态), 其状态为ibc0的最新consensus state状态, *奇怪的是打印却是ibconeclient更新*. 
&emsp;3. 数据库更新client state和root状态.

`ConnectionEnd`结构体在源码中的定义:

```go
// ConnectionEnd defines a stateful object on a chain connected to another separate
// one.
// NOTE: there must only be 2 defined ConnectionEnds to stablish a connection
// between two chains.
type ConnectionEnd struct {
    State State `json:"state" yaml:"state"`
    ClientID string `json:"client_id" yaml:"client_id"`

    // Counterparty chain associated with this connection.
    Counterparty Counterparty `json:"counterparty" yaml:"counterparty"`
    // Version is utilised to determine encodings or protocols for channels or
    // packets utilising this connection.
    Versions []string `json:"versions" yaml:"versions"`
}
```
* connection_open_try: 
&emsp;1. 会做版本检查
&emsp;2. 验证conection state
&emsp;3. 验证consensus state(被注释)
&emsp;4. 创建一个状态为tryopen的连接
&emsp;5. 连接的connectionEnd写数据库

步骤2的验证过程, 会将ibc0链上的proof传送过ibc1链, 包含两条proof信息其中一条为ibc0链的connectionzero的**ConnectionEnd**状态信息, 另一条为ibc0当前块高-1的AppHash的multistore信息, **完整**信息示例如下
```json
{
    "proof":{
        "StoreInfos":[
            {
                "Name":"staking",
                "Core":{
                    "CommitID":{
                        "Version":38,
                        "Hash":"OyAHcb9W4SvGpTv/fX6VEqWicaOM+9qTXwquGsiwT2o="
                    }
                }
            },
            {
                "Name":"acc",
                "Core":{
                    "CommitID":{
                        "Version":38,
                        "Hash":"xHw/kuGR12u6nN+27zsiZ2T6DJ+lZDdPoBj8ZiM1QvU="
                    }
                }
            },
            {
                "Name":"ibc",
                "Core":{
                    "CommitID":{
                        "Version":38,
                        "Hash":"wFqTYG72fimvvKU/t7/EF6JSfelTJxslDEbw8vZdHtU="
                    }
                }
            },
            {
                "Name":"params",
                "Core":{
                    "CommitID":{
                        "Version":38,
                        "Hash":"3PrT6GfafJIT6djcrepbPNe1pxBCwErffxJdEuW3gBU="
                    }
                }
            },
            {
                "Name":"slashing",
                "Core":{
                    "CommitID":{
                        "Version":38,
                        "Hash":"h7c4wbsuEsYYgjhJhCztAGBPREKDWc2M88JZaXOnQao="
                    }
                }
            },
            {
                "Name":"main",
                "Core":{
                    "CommitID":{
                        "Version":38,
                        "Hash":"Llfvk2BxBfQuVDx2CcfTIaMDoD6Y6I0m+E3VKdOR0WA="
                    }
                }
            },
            {
                "Name":"supply",
                "Core":{
                    "CommitID":{
                        "Version":38,
                        "Hash":"DyyskpRi7Miac9UxgSDUlFwjlmKYawF/myIKoLZK60A="
                    }
                }
            },
            {
                "Name":"distribution",
                "Core":{
                    "CommitID":{
                        "Version":38,
                        "Hash":"/awR8FmcgY8vcy4MKPmZ9o95rNBBtFEx95KOH2yvtsw="
                    }
                }
            },
            {
                "Name":"mint",
                "Core":{
                    "CommitID":{
                        "Version":38,
                        "Hash":"5zTw4N/uFMgbxIZB7wRoAZ78zbMrwQNaE62Mv+0uPmg="
                    }
                }
            },
            {
                "Name":"gov",
                "Core":{
                    "CommitID":{
                        "Version":38,
                        "Hash":"YM0TfBli7KxhY4nWgDSDPykhUJwtKFql9RU5l86WinQ="
                    }
                }
            }
        ]
    }
}
```
该信息会与另一条链的connection state状态一起放入`VerifyMembership`作校验, 该校验函数会放在[ics003](./ics003-connection.md#验证函数)的读书笔记中讲解.

还有一条与共识相关的ConsensusState Proof信息, 其中也包含两条proof信息, 第一条正是ibc1链在ibc0链上的子状态信息, 信息结构为**ConsensusState**上文提供了源码的定义, 另一条与前一个proof一样是AppHash的multistore信息. 

* ibc0链上client updateclient
* connection_open_ack:
&emsp;1. 会做版本检查
&emsp;2. 验证链上已有的连接状态是否为init
&emsp;3. 验证conection state
&emsp;4. 验证consensus state(被注释)
&emsp;5. 创建一个状态为open的连接
&emsp;6. 连接的connectionEnd写数据库
* ibc1链上client updateclient
* connection_open_confirm:
&emsp;1. 会做版本检查
&emsp;2. 验证链上已有的连接状态是否为tryopen
&emsp;3. 验证conection state
&emsp;4. 创建一个状态为open的连接
&emsp;5. 连接的connectionEnd写数据库

Connection建立后, ibc0链与ibc1链上均具备状态为open的connection, 可通过该命令查看通道状态:
```
gaiacli --home ibc0/n0/gaiacli q ibc connection end connectionzero --indent --trust-node
```
得到结果:
```json
{
    "type": "ibc/connection/ConnectionEnd",
    "value": {
        "state": "OPEN",
        "client_id": "ibconeclient",
        "counterparty": {
            "client_id": "ibczeroclient",
            "connection_id": "connectionone",
            "prefix": {
                "type": "ibc/commitment/merkle/Prefix",
                "value": {
                    "key_prefix": "aWJj"
                }
            }
        },
        "versions": [
            "1.0.0"
        ]
    }
}
```

