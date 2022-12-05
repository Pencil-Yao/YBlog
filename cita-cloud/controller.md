# Controller Develop Guide

`controller`在 cita-cloud 中犹如消息路由器。除了`consensus`和`network`的网络消息外，其余微服务之间都消息都需要先发送给`controller`，之后依照执行逻辑进行相应的消息处理；由于微服务之间的通信选用了`grpc`，因此`cita-cloud`天然的采用**pull**方式来推动所有逻辑。

## 配置说明

* 初始化`ControllerConfig`结构体

`ControllerConfig`结构体本身含有大量配置项：

```rust
/// ControllerConfig define majority of controller conduction: micro-server port, reconnect,
/// discovery other node, sync, log config, kms related
#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct ControllerConfig {
    /// controller service port
    pub controller_port: u16,
    /// network service port
    pub network_port: u16,
    /// consensus service port
    pub consensus_port: u16,
    /// storage service port
    pub storage_port: u16,
    /// kms service port
    pub kms_port: u16,
    /// executor service port
    pub executor_port: u16,
    /// self node address
    pub node_address: String,
    /// audit blocks epoch length
    pub block_limit: u64,
    /// block contains txs upper-limit
    pub package_limit: u64,
    /// address length from kms
    pub address_len: u32,
    /// hash length from kms
    pub hash_len: u32,
    /// signature length from kms
    pub signature_len: u32,
    /// account index in kmsdb
    pub key_id: u64,
    /// other micro-serv reconnect interval
    pub server_retry_interval: u64,
    /// discovery other nodes interval
    pub origin_node_reconnect_interval: u64,
    /// controller log4rs config file name
    pub log_file: String,
    /// switch of tx forward
    pub enable_forward: bool,
    /// sync block height interval
    pub sync_interval: u64,
    /// sync block request times
    pub sync_req: u64,
}
```

`ControllerConfig`管理了`controller`的诸多行为：其他微服务的端口和重连，其他节点的发现以及同步行为以及日志的配置和密码学相关配置。`ControllerConfig`默认读取节点根目录（后称`node_dir`）的`config.toml`，但往往看到的配置如下：

```toml
[controller]
consensus_port = 51001
controller_port = 51004
executor_port = 51002
key_id = 1
kms_port = 51005
network_port = 51000
node_address = '3c43fa797ed114544efff8e92287ccbffcc9f907'
package_limit = 30000
storage_port = 51003
```

与`ControllerConfig`数量想去甚远，因为我们定义了`#[serde(default)]`和`impl Default for ControllerConfig`其他未被定义的字段会使用**default**配置，因而不会影响使用。

```
controller_port: controller server的port
network_port： network server的port
consensus_port： consensus server的port
storage_port：storage server的port
kms_port：kms server的port
executor_port：executor server的port
node_address：节点的地址
block_limit：区块查重窗口的长度，默认100
address_len：地址的长度信息，来源kms
hash_len：hash的长度信息，来源kms
signature_len：签名的长度信息，来源kms
key_id：节点私钥在kms_db中的index
server_retry_interval：服务活性重试时间间隔。默认3s
origin_node_reconnect_interval：广播chain_state_init消息的时间间隔，默认3600s
log_file：controller的log4rs配置文件目录
enable_forward：消息转发开关
sync_interval：同步消息的区块间隔，即1次同步请求请求多少个块
sync_req：单次同步时机发送几次请求数量，ps. 对于一次同步时间总共同步的区块数量=sync_interval x sync_req
```

* 初始化`GenesisBlock`结构体

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct GenesisBlock {
    /// the timestamp of genesis block
    pub timestamp: u64,
    /// the previous_block hash for genesis block, default 0x000..000
    pub prevhash: String,
}
```

`GenesisBlock`相对来说比较简单，根据直面意思和注释就能明白各个字段的含义。
