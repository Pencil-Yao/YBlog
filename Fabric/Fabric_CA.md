### Sample CSR(Certificate Signing Request)
```shell
cn: fabric-ca-server
names:
   - C: US
     ST: "North Carolina"
     L:
     O: Hyperledger
     OU: Fabric
hosts:
  - host1.example.com
  - localhost
ca:
   expiry: 131400h
   pathlength: 1
```

>All of the fields above pertain to the X.509 signing key and certificate which is generated by the `fabric-ca-server init`. This corresponds to the `ca.certfile` and `ca.keyfile` files in the server’s configuration file. The fields are as follows:

所有的字段都会被囊括到X.509的签名秘钥与证书中, 这些都会通过`fabric-ca-server init`命令生成. 产生的文件`ca.certfile`和`ca.keyfile`都会对应于服务器的配置文件中. 这些字段为:

- **cn** is the Common Name
- **O** is the organization name
- **OU** is the organizational unit
- **L** is the location or city
- **ST** is the state
- **C** is the country

>The fabric-ca-server init command generates a self-signed CA certificate unless the -u <parent-fabric-ca-server-URL> option is specified. If the -u is specified, the server’s CA certificate is signed by the parent Fabric CA server. In order to authenticate to the parent Fabric CA server, the URL must be of the form <scheme>://<enrollmentID>:<secret>@<host>:<port>, where <enrollmentID> and <secret> correspond to an identity with an ‘hf.IntermediateCA’ attribute whose value equals ‘true’. The fabric-ca-server init command also generates a default configuration file named fabric-ca-server-config.yaml in the server’s home directory.

fabric-ca-server初始命令产生自签的CA证书, 除非指定了-u <parent-fabric-ca-server-URL>配置项. 如果- u的配置项被指定了, 那么该CA服务器将会被父Fabric CA签名. 为了得到父Fabric CA服务器的授权, 该url的形式必须为 <scheme>://<enrollmentID>:<secret>@<host>:<port>, 其中 <enrollmentID> 和<secret> 制定了一个对象, 并且该对象的‘hf.IntermediateCA’属性必须为'true'. fabric-ca-server的初始化命令同时生成一个名为fabric-ca-server-config.yaml的配置文件在服务器的home目录下.

>Unless the Fabric CA server is configured to use LDAP, it must be configured with at least one pre-registered bootstrap identity to enable you to register and enroll other identities. The -b option specifies the name and password for a bootstrap identity.

除非fabric ca服务器配置了使用LDAP, 那么他必须至少配置一个启动身份用来能够注册其他子身份. -b操作项正是用来定义启动身份的名字与密码的.

### crypto-related
在fabric-ca中server端使用到公私钥算法与椭圆曲线由client端发送的csr所用到的公私钥算法与椭圆曲线决定。而签名算法是由fabric-ca中的server端的ca证书决定（由启动参数`--ca.certfile`引入的证书或`FABRIC_CA_SERVER_CA_CERTFILE`指向的证书）。tls的证书会用于tls的监听中：

```go
server.go
listener, err = tls.Listen("tcp", addr, config)
```

### enroll和register的区别
enroll的账户会生成CA端用RootCA签发的签名证书，证书放在目录signcerts下，而rootca会放在cacert目录下，enroll会设置有效期
register的账户CA端不会签发证书，但会将账户存储入数据库，并在该过程中为账户生成密码并返回。
register操作后的账户还需要通过enroll申请CA端用RootCA签发的签名证书。关于enroll和register可以理解为：
1. fabric-ca-server服务起来时需要预置一个admin账户（等价于预register一个管理员账户）
2. 管理账户需要通过enroll来激活账户，激活过程需要用到账户名与账户密码，激活账户可以得到该账户的由CA端用RootCA签发的签名证书。
3. 使用上一步admin账户签发的证书，可以用来注册一个用户账号user，该过程不会签发证书，但会将账户存储入数据库，并在该过程中为账户生成密码并返回。
4. 使用上一步注册的用户user账户与账户名对账户进行激活，类似与第一步会得到该账户的CA签发的证书。

### 国密TLS与国际TLS的选择

交给用户来选择，用户发送的`ClientHello`消息中带有`SupportedCurves`字段，选择加密所用的曲线。服务器端这边同样有一个支持的加密曲线列表`defaultCurvePreferences`，列表中按顺序第一个与`ClientHello`消息中带有`SupportedCurves`匹配的加密曲现决定了TLS的版本：
`SM2P256v1` -> 国密TLS
others（`X25519`, `CurveP256`, `CurveP384`, `CurveP521`） -> 国际TLS

同时客户端发送的证书如果是国密证书的话，服务器端默认选择`SM2P256v1`

待解决的问题：
1. fabcar中的javascript脚本的证书为何是国际证书
2. DH与DHE，ECDH与ECDHE 的区别
