   公钥基础设施（Public Key Infrastructure，简称PKI）是目前网络安全建设的基础与核心，是电子商务安全实施的基本保障，因 此，对PKI技术的研究和开发成为目前信息安全领域的热点。本文对PKI技术进行了全面的分析和总结，其中包括PKI组成、证书认证机构CA、PKI应用、应用编程接口和PKI标准等，并对CA的开发做了简要分析。本文对PKI，特别是CA的开发、应用和普及具有一定的促进作用。

   PKI发展的一个重要方面就是标准化问题，它也是建立互操作性的基础。目前，PKI标准化主要有两个方面：一是RSA公司的公钥加密标准PKCS（Public Key Cryptography Standards）,它定义了许多基本PKI部件，包括数字签名和证书请求格式等；二是由Internet工程任务组IETF（Internet Engineering Task Force）和PKI工作组PKIX（Public Key Infrastructure Working Group）所定义的一组具有互操作性的公钥基础设施协议。在今后很长的一段时间内，PKCS和PKIX将会并存，大部分的PKI产品为保持兼容性，也将会对这两种标准进行支持。







PKCS是由美国RSA数据安全公司及其合作伙伴制定的一组公钥密码学标准，其中包括证书申请、证书更新、证书作废表发布、扩展证书内容以及数字签名、数字信封的格式等方面的一系列相关协议。到1999年底，PKCS已经公布了以下标准：

* PKCS#1：定义RSA公开密钥算法加密和签名机制，主要用于组织PKCS#7中所描述的数字签名和数字信封。 
* PKCS#3：定义Diffie-Hellman密钥交换协议。 
* PKCS#5：描述一种利用从口令派生出来的安全密钥加密字符串的方法。使用MD2或MD5 从口令中派生密钥，并采用DES-CBC模式加密。主要用于加密从一个计算机传送到另一个计算机的私人密钥，不能用于加密消息。 
* PKCS#6：描述了公钥证书的标准语法，主要描述X.509证书的扩展格式。 
* PKCS#7：定义一种通用的消息语法，包括数字签名和加密等用于增强的加密机制，PKCS#7与PEM兼容，所以不需其他密码操作，就可以将加密的消息转换成PEM消息。 
* PKCS#8：描述私有密钥信息格式，该信息包括公开密钥算法的私有密钥以及可选的属性集等。 
* PKCS#9：定义一些用于PKCS#6证书扩展、PKCS#7数字签名和PKCS#8私钥加密信息的属性类型。 
* PKCS#10：描述证书请求语法。 
* PKCS#11：称为Cyptoki，定义了一套独立于技术的程序设计接口，用于智能卡和PCMCIA卡之类的加密设备。 
* PKCS#12：描述个人信息交换语法标准。描述了将用户公钥、私钥、证书和其他相关信息打包的语法。 
* PKCS#13：椭圆曲线密码体制标准。 
* PKCS#14：伪随机数生成标准。 
* PKCS#15：密码令牌信息格式标准。