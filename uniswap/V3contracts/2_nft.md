该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

---

本系列文章会对UniswapV3的合约进行剖析，作为学习笔记，会手术刀分析合约的执行，文中会提供合约的源码，方便大家学习。

该系列的第一篇文章提到了[pool部署](./1_pool.md)

下面我们来讲完成pool部署所调用nft合约的部署

# NFT合约的部署

当前v3使用的nft合约: [Uniswap V3: Positions NFT](https://etherscan.io/address/0xC36442b4a4522E871399CD717aBDD847Ab11FE88)

合约本身部署并不复杂, 其constructor函数如下:

```solidity
    constructor(
        address _factory,
        address _WETH9,
        address _tokenDescriptor_
    ) ERC721Permit('Uniswap V3 Positions NFT-V1', 'UNI-V3-POS', '1') PeripheryImmutableState(_factory, _WETH9) {
        _tokenDescriptor = _tokenDescriptor_;
    }
```

对[部署交易](https://etherscan.io/tx/0xc5eabeff36dc4593e58ede208838105815106e5a11aa725638d72b43f88e5fb2)的code进行解析, 上述三个参数:

```
[
  {
    "_factory": "1f98431c8ad98523631ae4a59f267346ea31f984"
  },
  {
    "_WETH9": "c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
  },
  {
    "_tokenDescriptor_": "ee6a57ec80ea46401049e92587e52f5ec1c24785"
  }
]
```

这三个都是提前部署的合约, 分别是工厂合约, eth-warp合约和token descriptor合约

## nft合约源码

[在这里](./assert/NonfungiblePositionManager.zip)