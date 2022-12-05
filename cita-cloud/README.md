# cita-cloud

cita-cloud 组织地址：[这里](https://github.com/cita-cloud)

cita-cloud 是新一代区块链底层架构，区别于过于过去区块链是一个整体服务，集成了：共识、校验、存储以及执行器等功能；cita-cloud对这些大功能进行状态的抽象和拆分，做成了微服务架构，微服务之间又规范了 grpc 接口：[cita-cloud-proto](https://github.com/cita-cloud/cita_cloud_proto)；由此 cita-cloud 正式开辟出了由6个微服务：controller，consensus，executor，kms，network，storage分别提供不同功能，同时组合起来具备企业级使用的区块链系统，各个微服务功能简单介绍：

* controller：负责链上逻辑的执行和微服务间消息的发送，例如我要对用户提交的交易进行检查，controller会对交易内容处理，然后发送给 kms 进行延签......
* consensus：负责对最新区块达成共识
* executor：负责对收到的连续区块的执行工作，或者说解释工作，例如 evm
* kms：负责保管用户的私钥，以及使用该私钥进行签名和验签工作
* network：负责节点之间的连接，以及提供微服务之间的注册接口，使得不同节点间的相同微服务可以互相发送消息
* storage：负责存储和读取数据。