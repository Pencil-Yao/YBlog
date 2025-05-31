该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

---

本系列文章会对UniswapV3的合约进行剖析，作为学习笔记，会手术刀分析合约的执行，文中会提供合约的源码，方便大家学习。

第三部分我们来看一下nft合约的前置合约factory合约, 首先factory合约是在pool合约的基础增加了两个合约(factory和pooldeDloyer)

首先可以说一下`pooldeDloyer`就如同其名字一样该合约用来构建pool合约的, 其本身的核心代码:

```solidity
    function deploy(
        address factory,
        address token0,
        address token1,
        uint24 fee,
        int24 tickSpacing
    ) internal returns (address pool) {
        parameters = Parameters({factory: factory, token0: token0, token1: token1, fee: fee, tickSpacing: tickSpacing});
        pool = address(new UniswapV3Pool{salt: keccak256(abi.encode(token0, token1, fee))}());
        delete parameters;
    }
```

可以说朴实无华而简单.

但这段代码也是factory合约创建pool的核心部分:

```solidity
    function createPool(
        address tokenA,
        address tokenB,
        uint24 fee
    ) external override noDelegateCall returns (address pool) {
        require(tokenA != tokenB);
        (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
        require(token0 != address(0));
        int24 tickSpacing = feeAmountTickSpacing[fee];
        require(tickSpacing != 0);
        require(getPool[token0][token1][fee] == address(0));
        pool = deploy(address(this), token0, token1, fee, tickSpacing);
        getPool[token0][token1][fee] = pool;
        // populate mapping in the reverse direction, deliberate choice to avoid the cost of comparing addresses
        getPool[token1][token0][fee] = pool;
        emit PoolCreated(token0, token1, fee, tickSpacing, pool);
    }
```

# factory合约的部署

factory合约部署就是一切的开端了, 他不需要其他前置合约了:

```solidity
    constructor() {
        owner = msg.sender;
        emit OwnerChanged(address(0), msg.sender);

        feeAmountTickSpacing[500] = 10;
        emit FeeAmountEnabled(500, 10);
        feeAmountTickSpacing[3000] = 60;
        emit FeeAmountEnabled(3000, 60);
        feeAmountTickSpacing[10000] = 200;
        emit FeeAmountEnabled(10000, 200);
    }
```

其中合约`owner`的操作无需多言, 但也看到了有关`feeAmountTickSpacing`的操作.

```solidity
    /// @inheritdoc IUniswapV3Factory
    mapping(uint24 => int24) public override feeAmountTickSpacing;

/// @notice Returns the tick spacing for a given fee amount, if enabled, or 0 if not enabled
    /// @dev A fee amount can never be removed, so this value should be hard coded or cached in the calling context
    /// @param fee The enabled fee, denominated in hundredths of a bip. Returns 0 in case of unenabled fee
    /// @return The tick spacing
    function feeAmountTickSpacing(uint24 fee) external view returns (int24);
```

`feeAmountTickSpacing`是用来规定` Returns the tick spacing for a given fee amount`, 说明factory初始化的时候设置了不同费率的tickspacing

| Fee Tier   | tickSpacing |
| ---------- | ----------- |
| 0.05%(500) | 10          |
| 0.3%(3000) | 60          |
| 1%(10000)  | 200         |

tickSpacing越小意味着价格tick越小, 流动性划分的越细. 关于uniswap V3流动性配置有一套复杂的算法, 后续文档会详细分析. 但是越细的划分也将导致需要的手续费增加.