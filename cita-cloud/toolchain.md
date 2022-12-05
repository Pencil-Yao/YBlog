# toolchain

本文介绍`cita-cloud`中自身开发并使用的`toolchain`。

## cloud-code aka status_code

`cloud-code`在`cita-cloud`源码中有一个更常见的名字`status_code`，其作用也类似与`http`协议中的`status_code`，之后为了发送到`crate.io`上面不得已改成了`cloud-code`。

## 使用

`Cargo.toml`中加入：

```
status_code = { package = "cloud-code", version = "0.1" }
```

## 作用

`status_code`可以包含在**rpc**返回的结构体中，其本质是一个**u64**字段，同时提供了**u64**与`status_code`的互换的**trait**方便微服务之间的调用：

```rust
/// The response status code
#[derive(Debug, Clone, Copy, Eq, PartialEq, FromPrimitive)]
pub enum StatusCode {
    /// Success: 0
    Success = 0,
    /// Convert int to status Error
    ConvertIntError,
    ...

    /// controller error, start from 100
    /// node in misbehave list
    MisbehaveNode = 100,
    /// node in ban list
    BannedNode,
    /// address not consistent with record origin
    AddressOriginCheckError,
	...

    /// Consensus from 200
    /// check proposal proof error
    ConsensusServerNotReady = 200,
    /// proof of proposal error
    ProposalProofError,
    ...

    /// Kms from 300
    /// Kms server not ready
    KmsServerNotReady = 300,
    /// hash result is none
    NoneHashResult,
	...

    /// Network from 400
    /// Network server not ready
    NetworkServerNotReady = 400,
    /// send message error
    SendMsgError,
	...

    /// executor from 500
    /// Executor server not ready
    ExecuteServerNotReady = 500,
    /// internal channel disconnected
    InternalChannelDisconnected,
	...

    /// storage from 600
    /// storage server not ready
    StorageServerNotReady = 600,
    /// kv not found
    NotFound,
	...
}
```

可以看到不同的微服务都登记了自身的错误**code**并有明确的注释文字，另外本身**code**也非常长可以通过字面值看出错误的意思。**code**的分布关系：

**Success**为0，往后累加的是系统级错误。

`controller`的**code**：1xx

`consensus`的**code**：2xx

`kms`的**code**：3xx

`network`的**code**：4xx

`execuotor`的**code**：5xx

## cloud-util

cloud-util是cita-cloud的工具集，包含多个微服务都有可能用到的工具函数集合

## 使用

`Cargo.toml`中加入：

```
cloud-util = "0.1"
```

## 作用

正如工程中所包含的文件阐述的一样：

```
$ cd cloud-util/src
$ tree -L 1
.
├── common.rs
├── crypto.rs
├── example
├── lib.rs
├── network.rs
└── storage.rs

1 directory, 5 files
```

其中封装了4类：**common**，**crypto**，**network**，**storage**函数列表，具体函数功能不一一列出了。