# SSL/TLS协议安全性研究

1. SSL(Secure Sockets Layer，安全套接层)协议及其继任者一一TLS(Transport Layer Security，传输层安全)协议.
2. SSL/TLS协议建立在TCP层之上，是两层结构的协议，下层为记录层协议，用来封装、压缩和加密上层协议消息以及应用层协议消息。上层为与握手有关的协议。该协议主要提供以下四种安全服务：
   * 同等实体认证(简称认证)：为连接的实体身份提供可信性。
   * 数据保密性：保护数据免于非授权的泄露，防止传输的数据遭受被动攻击。
   * 数据完整性：保证收到的数据的确是授权实体所发出的数据，即数据没有遭受修改、插入、删除或重放。
   * 可用性服务：保证授权的客户端能够获得其要求的服务和资源。
3. 客户端在与服务器通信前先通过SSL/TLS握手协议与服务器进行握手，协商安全策略，认证服务器身份(也可以向服务器证实自己的身份)，与服务器进行密钥交换，以获得对应用数据进行保密性和完整性保护的会话密钥，从而建立安全的通信连接。握手顺利完成后，即可在此安全连接上用协商好的安全参数保护应用层数据。
4. 从公开发布的协议第一版SSL2.0(己弃用)开始，到最新版本TLS1.3为止，SSL/TLS协议一共经历了6个版本，前两个版本(SSL2.0和SSL3.0)都由网景公司开发，而从TLS1.0开始，协议的开发由IETF(国际互联网工程任务组)接手。其中由于SSL2.0的握手协议流程有严重安全漏洞，因此SSL3.0彻底重新设计了协议，而此后三个版本(TLS1.0、TLS1.1以及TLS1.2)在安全性方面的改进主要是取消或增加了对某些密码算法的支持。协议最新版本(TLS1.3)无论在握手流程上还是在支持的密码算法上的变化都相当显著。目前正在使用中的版本是SSL3.0到TLS1.2这四个版本(SSL3.0在逐渐被淘汰)，下文中统称为现行版本；而最新版本TLS1.3目前还是因特网草案，尚未正式投入使用。

![](./(SSL3.0到TLS1.2)的基本握手流程.png)

5. 现行版本的握手框架中，握手接近尾声时会用到Challge Cipher Spec协议，来告知对方随后的记录层消息都将由刚刚协商好的密码规范和密钥加密。如果握手过程中出现错误，也会需要A1ert协议。中标“\*”的消息为在握手流程中不一定出现的消息，[]内是传输密钥保护的消息。Change Cipher Spec消息虽然出现在握手流程中，但不属于握手协议的消息。
6. 现行版本均有Hello Request、Client Hello、Server Hello、Certificate、Server KeyExechange、Certificate Request、Server Hello Done、Client Key Exchange、Cenificate Verify:

* Hello Request消息：该消息可以在任何时刻由服务器发送。其作用是提醒客户端在适当的时刻发送client Hello消息来重新发起会话协商。该消息的消息体为空。
* Client Hello消息：client Hello消息必须是客户端刚与服务器连接时发送的第一条消息。此外，当客户端收到服务器提示进行重协商的Hello Request消息，或者客户端自己想在当前连接下重新协商安全参数时，它也应发送Cliem Hello消息。

``` c
// SSL3.0到TLS1.1版本中，该消息的消息体结构如下：
struct {
	ProtocolVersion client_version;
	Random random;
	SessionID session_id;
	CipherSuite cipher_suites<2..2^16-1>;
	CompressionMethod compression_methods<1..2^8-1>;
} ClientHello;
```

* 消息中的client_version域包含客户端支持的SSL/TLS最高版本号。
* random域包含客户端随机数，该随机数由两部分构成，共32字节。其中一部分是按发送方内部时钟计算的日期和时间，占4字节；另一部分是由安全的随机数发生器产生的28字节的随机数。
* session_id域包含可变长度的会话标识，最短可以为空，最长为32字节。若session_id不为空，表明客户端想重用该session_id标识的会话的安全参数(即会话重用)。
* cipher_suites域包含几个客户端支持的密码套件(即密码算法的组合)，这些密码套件按顺序排列，客户端最希望使用的排在最前面。该变量长度可变，但不能为空。
* compression_methods域包含几个客户端支持的压缩方法，并按照客户端的偏好排序。

在SSL3.0到TLS1.1的RFC文件中，compression_methods域之后没有再定义其他数据域，但为了实现向前兼容，允许该消息中的coInpression methods域之后有其他附加数据。Client Hello消息也是唯一一个可以附加数据的握手消息，其他握手消息中的数据必须完全符合消息的定义。

``` c
// 而TLS1.2的C1ient Hello消息中增加了extension域(现在TLS1.0和TLS1.1版本也可以支持该域)，消息体结构如下：
Struct{
	ProtocolVersion client_version;
	Random random;
	SessionID session_id;
	CipherSuite cipher_suites<2..2^16-1>;
	CompressionMethod compression_methods<1..2^8-1>;
	select(extensions_present) {
		case false:
			struct {};
		case true:
			Extension extensions<o..2^16-1>;
	};
} ClientHello;
```
TLS1.2在compression_methods域之后定义了extension域，该域包含客户端向服务器请求的扩展功能，可以为空。

* Server Hello消息服务器收到Client Hello消息之后，如果能找到可接受的密码套件，则返回Server Hello消息作为回复。如果找不到一组可接受的密码套件，则服务器将返回handshake failure警告并终止握手。

``` c
// TLS1.2的ServerHello消息的消息体如下，而SSL3.0到TLS1.1的该消息中不包含extension域(现在TLS1.0和TLS1.1版本也可以支持该域)。

Struct{
	ProtocolVersion client_version;
	Random random;
	SessionID session_id;
	CipherSuite cipher_suites<2..2^16-1>;
	CompressionMethod compression_methods<1..2^8-1>;
	select(extensions_present) {
		case false:
			struct {};
		case true:
			Extension extensions<o..2^16-1>;
	};
} ServerHello;
```

* session_id域包含与当前连接相关的会话的标识。新会话的会话标识是服务器产生的。该域也可以为空，这表示该会话不会被服务器缓存，也不能用会话标识进行重用。一个会话标识的生命期从完成握手(双方交换了Finished消息)开始，直到时间达到预设的期限，或者与该会话相关的连接中出现了fatal等级的错误警告为止。官方建议SessionD的生命期上限不超过24小时。由于攻击者可以在消息传输过程中截获SessionID的明文，因此服务器不应在SessionID中存放任何机密信息。
* Certificate消息：对服务器而言，如果握手不是匿名的(即商定的密钥交换算法不是匿名的)，则服务器需要发送Certificate消息，并把自己的数字证书(链)包含在该消息中。 对客户端而言，如果服务器发送了Certificate Request消息，则客户端应发送Certificate消息。如果有合适的证书，则客户端通过该消息发送其证书(链)。服务器将用客户端提供的证书来验证Certificate Verify消息，或计算premaster secret(固定参数的DH算法)。如果没有合适的证书，客户端仍需发送该消息，但消息中不包含任何证书。<u>当服务器收到不包含证书的certificate消息或客户端提供的证书链中有一部分证书不能被服务器接受时，如果客户端的证书对完成握手而言不是必须的，则可以继续握手，否则发送handshake failure警告并终止握手。</u>该消息中包含一个X.509v3格式的证书链。发送方的证书必须是证书链的头结点，之后每个结点的证书必须可直接用于验证上一个结点的证书。由于根密钥必须被独立分发，因而自签名证书以及根证书中心的证书可以不必包含在该证书链中。
* Server Key Exchange消息：只有当服务器证书包含的数据不足以使客户端完成预主密钥的交换，或者服务器根本没有发送证书时，Server Key Exchange消息才需要被发送。当密钥交换算法为临时参数的DH算法时，该消息的消息体包含服务器的DH公开参数，以及服务器证书对应的私钥对客户端随机数、服务器随机数和消息中的服务器DH公开参数的散列的签名。
* Certificate Request消息：非匿名的服务器可以通过该消息要求客户端提供证书。SSL3.0到TLS1.1中该消息包含服务器可以接受的客户端证书类型的列表以及证书授权中心(CA)的可识别名称列表。TLS1.2中的该消息增加了服务器支持的散列/签名算法组合的列表。客户端证书的签名算法以及客户端证书的签名密钥都必须符合该列表中提供的某散列/签名算法组合。
* Server Hello Done消息：服务器发送该消息来告知客户端Server Hello以及相关消息已经发送完。客户端收到该消息后，要立刻验证服务器证书的合法性，并检查服务器发来的消息中的参数是否可接受。该消息的消息体为空。
* Client Key Exchange消息：该消息在完整的握手流程中是必须被发送的。当密钥交换算法为RSA算法时，该消息包含服务器RSA公钥加密的预主密钥。<u>.该预主密钥是由clientHello消息中的协议版本号和46字节的安全随机数构成的，将被用于生成主密钥(客户端和服务器的共享密钥，用于生成保护该连接中传输的数据的密钥和其他安全参数)。</u>当密钥交换算法为固定参数的DH算法时，该消息的消息体为空。当密钥交换算法为临时参数的DH算法时，该消息包含客户端的DH公开参数。
* Certificate Verify消息：客户端仅在提供了包含签名密钥的证书的情况下才可以发送该消息。该消息紧接着Client Key Exchange消息发送，用于验证客户端身份是否合法(即客户端是否持有其提供的服务器可接受的证书的私钥)。<u>该消息的消息体中包含客户端证书私钥对之前发送的所有握手消息的散列的签名。</u>
* Finished消息：该消息用于验证密钥交换和身份认证是否成功。该消息的接受者必须验证消息的内容是否正确。通信中的一方发送了自己的Finished消息，且接收并验证了对方的Finished消息后，即可开始发送应用层数据。该消息的消息体中包含验证数据，该验证数据是散列函数(SSL3.0)或PRF(TLS协议)的输出，而输入是主密钥和之前所有的握手消息。TLS1.2之前的版本中，该消息中的验证数据长度固定，而TLS1.2中验证消息的长度取决于密码套件，如果没有显式定义，则默认长度为12字节。
* New Session Ticket消息：TLS协议中，服务器通过该消息传送用仅有自己知道的密钥进行机密性和完整性保护的会话票据给客户端。该消息仅当服务器在Server Hello消息中发送了Session Ticket扩展时才会被发送。服务器应在发送自己的Change Cipher Spec消息之前发送New SessionTicket消息。在完整的握手流程中，服务器应先验证客户端的Finished消息，再发送New Session Ticket消息。该消息应包含在Finished消息需要使用的握手消息的散列中。客户端在验证了服务器的Finished消息之后才确认会话票据有效。
该消息的消息体结构如下：
```c
struct {
	uint32 ticket_lifetime_hint;
	opaque ticket<0..2^16-1>;
}NewSessionTicket;
```
其中ticket_lifetime_hint表示会话票据的生命期(由服务器指定)。当会话票据存在的时间超出了其生命期，则客户端应删除该票据以及相关的会话状态。
