该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

---

本系列文章会对UniswapV3的合约进行剖析，作为学习笔记，会手术刀分析合约的执行，文中会提供合约的源码，方便大家学习。

# NFT Descriptor合约

这是nftManager合约的前置合约, 描述也很简单就是`Describes NFT token positions`

合约的创建也很简单:

```solidity
    constructor(address _WETH9) {
        WETH9 = _WETH9;
    }
```

不过其最核心的作用是生成`tokenURI`

原版描述: `Produces the URI describing a particular token ID for a position manager`

翻一下就是针对每一个交易对生成一个唯一且特定的uri, 在这里每一个交易对都特化成了nft.