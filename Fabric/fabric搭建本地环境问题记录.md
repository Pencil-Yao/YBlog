##  Fabric 本地调试

fabric本地调用可以使得遇到的fabric问题能够通过更清晰的 debug 方式查看, 但想要实现这一个过程难免会存在一些坑, 本文主要受到[多多-江北残刀](https://blog.csdn.net/beijihukk/article/details/103656730#commentBox)的启发, 其实大部分问题文章已经提到, 但未突出导致后来还是走了不少弯路, 这里将一些问题在重点拿出来.

### 问题背景

基于 fabric-samples 的 first-network 场景构建一个完整的网络, 其中除了 peer0.org1 节点以外均已过去 docker 的方式运行, 而 peer0.org1 以本地的方式进行运行, 因此 peer0.org1 为实际的调试节点.
完成 peer0.org1 相关环境本地化:
```
CORE_PEER_ID=peer0.org1.example.com
CORE_PEER_ADDRESS=peer0.org1.example.com:7051
CORE_PEER_LISTENADDRESS=0.0.0.0:7051
CORE_PEER_CHAINCODEADDRESS=<local ip not localhost>:7052
CORE_PEER_CHAINCODELISTENADDRESS=0.0.0.0:7052
CORE_PEER_GOSSIP_BOOTSTRAP=peer1.org1.example.com:8051CORE_PEER_GOSSIP_EXTERNALENDPOINT=peer0.org1.example.com:7051
CORE_PEER_LOCALMSPID=Org1MSP
FABRIC_LOGGING_SPEC=DEBUG
CORE_PEER_TLS_ENABLED=false
CORE_PEER_GOSSIP_USELEADERELECTION=true
CORE_PEER_GOSSIP_ORGLEADER=false
CORE_PEER_PROFILE_ENABLED=true
CORE_PEER_TLS_CERT_FILE=<fabric-samples dir>/fabric-samples/first-network/crypto-config/peerOrganizations/org1.example.com/peers/peer0.org1.example.com/tls/server.crt
CORE_PEER_TLS_KEY_FILE=<fabric-samples dir>/first-network/crypto-config/peerOrganizations/org1.example.com/peers/peer0.org1.example.com/tls/server.key
CORE_PEER_TLS_ROOTCERT_FILE=<fabric-samples dir>/fabric-samples/first-network/crypto-config/peerOrganizations/org1.example.com/peers/peer0.org1.example.com/tls/ca.crt
CORE_VM_DOCKER_HOSTCONFIG_NETWORKMODE=net_byfn
FABRIC_CFG_PATH=/home/ypf/ypf-app/goapp/src/github.com/hyperledger/fabric/sampleconfig
```

### IP 问题 or 证书问题

但当你启动整个脚本的时候会发现在 peer create channel 这一步就走不下去了, 因为过去是 docker 的执行环境, 节点之间的通信可以之间通过节点名字进行访问, 而 peer0.org1 在本地运行, 那么 peer0.org1 不知道其他节点的 ip, 其他节点也不知道 peer0.org1 的 ip.

**解决方法**

1. peer0.org1 发现其他节点,  在 `/etc/hosts` 中添加节点 "ip 节点名"
2. 其他节点发现 peer0.org1, 在 docker-compose 每个需要发现 peer0.org1 节点添加 extra_hosts: - peer0.org1.example.com ip(该 ip 不能为 localhost)

*聪明的朋友可能会想到修改 cli 中节点名称为 ip 的方式, 但是很可惜的是修改了名称以后, 证书校验无法通过, 因为证书是对节点名称进行背书, 更换为 ip 后还是会走不通*

### chaincode 容器

修改完了上述问题, 当你再次启动时候

查看 chaincode 容器的 log 会看到
```
error trying to connect to local peer
github.com/hyperledger/fabric/core/chaincode/shim.userChaincodeStreamGetter
 /opt/gopath/src/github.com/hyperledger/fabric/core/chaincode/shim/chaincode.go:112
github.com/hyperledger/fabric/core/chaincode/shim.Start
 /opt/gopath/src/github.com/hyperledger/fabric/core/chaincode/shim/chaincode.go:151
main.main
 /chaincode/input/src/github.com/chaincode/chaincode_example02/go/chaincode_example02.go:195
runtime.main
        /opt/go/src/runtime/proc.go:200
runtime.goexit
        /opt/go/src/runtime/asm_amd64.s:1337
Error starting Simple chaincode: error trying to connect to local peer: context deadline exceeded
```

chaincode 容器一样也会遇到发现 peer0.org1 节点的问题, 而 chaincode 容器不是通过 docker-compose 配置产生, 是 fabric 中 golang 代码实现容器生成, 我们可以修改 `CORE_PEER_CHAINCODEADDRESS` 来解决.

```
CORE_PEER_CHAINCODEADDRESS=<local ip not localhost>:7052
```

### MSP 问题

MSP 问来源于 sampleconfig/core.yaml 配置文件中 `mspConfigPath` 配置不正确所致, 修改如下:

```
# Path on the file system where peer will find MSP local configurations
mspConfigPath: <fabric-samples dir>/first-network/crypto-config/peerOrganizations/org1.example.com/peers/peer0.org1.example.com/msp
```