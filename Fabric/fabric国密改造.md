## SM2 椭圆曲线

sm2 椭圆曲线是指 sm2p256k1, 该曲线基于 ANSI X9.62 elliptic curve prime256v1 (aka secp256r1, NIST P-256)<sup>[1]</sup>改造.
sm2p256k1 椭圆曲线参数:
```go
p256Sm2Params.P, _ = new(big.Int).SetString("FFFFFFFEFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF00000000FFFFFFFFFFFFFFFF", 16)
	p256Sm2Params.N, _ = new(big.Int).SetString("FFFFFFFEFFFFFFFFFFFFFFFFFFFFFFFF7203DF6B21C6052B53BBF40939D54123", 16)
	p256Sm2Params.B, _ = new(big.Int).SetString("28E9FA9E9D9F5E344D5A9E4BCF6509A7F39789F515AB8F92DDBCBD414D940E93", 16)
	p256Sm2Params.Gx, _ = new(big.Int).SetString("32C4AE2C1F1981195F9904466A39C9948FE30BBFF2660BE1715A4589334C74C7", 16)
	p256Sm2Params.Gy, _ = new(big.Int).SetString("BC3736A2F4F6779C59BDCEE36B692153D0A9877CC62A474002DF32E52139F0A0", 16)
	p256Sm2Params.BitSize = 256
```

sm2p256k1 的 asn1 oid: 1 2 156 10197 1 301
该 oid 编码后的结果: 2A 81 1C CF 55 01 82 2D<sup>[2]</sup>

SM2 的公私钥生成算法与 ECDSA 一致均为 ANSI X9.62 id_ecPublicKey
asn1 oid: 1 2 840 10045 2 1







## 参考资料

[1] https://www.jianshu.com/p/f0d26ff12325
[2] [asn1 oid 的编码规则](https://blog.csdn.net/xiao628945/article/details/8006092)