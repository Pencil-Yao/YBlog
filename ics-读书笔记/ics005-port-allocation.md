读书笔记是摘取ibc的文档ics协议中有趣的部分(可能也是关键部分)的翻译和源码对照, 意在理清ibc实现背后的原理和主要逻辑. 该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

# [ics005-port-allocation](https://github.com/cosmos/ics/tree/master/spec/ics-005-port-allocation)

## 大纲

> This standard specifies the port allocation system by which modules can bind to uniquely named ports allocated by the IBC handler. Ports can then be used to open channels and can be transferred or later released by the module which originally bound to them.

该标准规范了端口申请系统用于IBC handler申请特定端口绑定模块. 随后端口可用来创建通道和包传递亦或之后由绑定的模块将其释放.

### 动机

> Conventions may emerge as to what kind of module logic is bound to a particular port name, such as "bank" for fungible token handling or "staking" for interchain collateralisation. This is analogous to port 80's common use for HTTP servers — the protocol cannot enforce that particular module logic is actually bound to conventional ports, so users must check that themselves. Ephemeral ports with pseudorandom identifiers may be created for temporary protocol handling.

约定可以如此体现当模块绑定特定的端口, 例如将'bank'用于非同质化token, 或'stake'用于链间抵押. 这有点类似与80端口用于http服务, 但规范不能强迫特定的模块逻辑必须绑定符合逻辑的端口, 所以用户必须检查这些信息. 可以创建具有伪随机标识符的临时端口用以临时协议的处理.

> Modules may bind to multiple ports and connect to multiple ports bound to by another module on a separate machine. Any number of (uniquely identified) channels can utilise a single port simultaneously. Channels are end-to-end between two ports, each of which must have been previously bound to by a module, which will then control that end of the channel.

模块可能绑定多个端口以及连接绑定了多个端口的通道, 在一个独立的机器上. 任意数量具备唯一标识符的通道可以同时使用单个端口. 通道是两个端口之间的端到端，每个端口必须事先已被模块绑定，然后模块才可以控制通道的该端。

### 定义

>A *port* is a particular kind of identifier which is used to permission channel opening and usage to modules.
>
>A *module* is a sub-component of the host state machine independent of the IBC handler. Examples include Ethereum smart contracts and Cosmos SDK & Substrate modules. The IBC specification makes no assumptions of module functionality other than the ability of the host state machine to use object-capability or source authentication to permission ports to modules.

`port`端口是特定的标识符用于许可通道打开和模块使用.

`module`是主机的子模块独立于IBC handler. 示例包括以太坊智能合约以及Cosmos SDK和Substrate模块. IBC规范不对功能模块作任何假设, 除了主机状态机使用功能或对模块许可端口的源身份验证的方式除外

### 需要的特性

>* Once a module has bound to a port, no other modules can use that port until the module releases it
>* A module can, on its option, release a port or transfer it to another module
>* A single module can bind to multiple ports at once
>* Ports are allocated first-come first-serve and "reserved" ports for known modules can be bound when the chain is first started

* 一旦某个端口被模块绑定, 除了该模块释放该端口那么其他模块均无法使用该端口
* 模块可以选择释放端口或通过端口发送
* 一个模块可以绑定多个端口
* 端口的申请为先到者先得并且保留端口对于已知模块能够绑定当链初启动时.

与tcp的类比:

| IBC Concept          | TCP/IP Concept           | Differences                                                  |
| -------------------- | ------------------------ | ------------------------------------------------------------ |
| IBC                  | TCP                      | Many, see the architecture documents describing IBC          |
| Port (e.g. "bank")   | Port (e.g. 80)           | No low-number reserved ports, ports are strings              |
| Module (e.g. "bank") | Application (e.g. Nginx) | Application-specific                                         |
| Client               | -                        | No direct analogy, a bit like L2 routing and a bit like TLS  |
| Connection           | -                        | No direct analogy, folded into connections in TCP            |
| Channel              | Connection               | Any number of channels can be opened to or from a port simultaneously |

## 技术规范

### 绑定端口

IBC handler必须实现`bindPort`, `bindPort`绑定一个未分配的端口, 如果该端口已被分配将会调用失败.

```typescript
function bindPort(id: Identifier) {
    abortTransactionUnless(validatePortIdentifier(id))
    abortTransactionUnless(privateStore.get(portPath(id)) === null)
    key = generate()
    privateStore.set(portPath(id), key)
    return key
}
```

### 转移端口所有权

如果主机状态机支持对象功能, 则不需要附加协议, 因为端口引用是承载功能. 如果不是可以实现以下函数:

```typescript
function transferPort(id: Identifier) {
    abortTransactionUnless(authenticate(privateStore.get(portPath(id))))
    key = generate()
    privateStore.set(portPath(id), key)
}
```

### 释放端口

IBC handler必须实现`releasePort`函数, 其允许模块释放绑定到其他模块上的端口.

```typescript
function releasePort(id: Identifier) {
    abortTransactionUnless(authenticate(privateStore.get(portPath(id))))
    privateStore.delete(portPath(id))
}
```

