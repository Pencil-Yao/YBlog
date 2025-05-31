该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

---

本系列文章会对UniswapV3的合约进行剖析，作为学习笔记，会手术刀分析合约的执行，文中会提供合约的源码，方便大家学习。

# 交易对的创建

UniswapV3本身代码量非常庞大, 无论是从toekn端开始亦或者交易端开始难免显得冗长, 本文打算从代码的中端切入, 并且已一个个合约如何部署来展开, 先来看一下交易对是如何创建的.

我们来观察一个交易对创建：[Virtual-WETH](https://etherscan.io/tx/0x4b9ae122e5451bfda960cb332fb62b9c6fa6bf72c0da5acef5b194bf3bfd18aa)

调用对象: [Uniswap V3: Positions NFT](https://etherscan.io/address/0xC36442b4a4522E871399CD717aBDD847Ab11FE88)

这笔交易: 涉及 49,999,999.999999999999880385 virtual (295be96e64066971fe2cc1) 217.859582274229073488 eth (bcf68b7cf12697650)

调用的code:

```
Function: multicall(bytes[] data)

MethodID: 0xac9650d8
[0]:  0000000000000000000000000000000000000000000000000000000000000020
[1]:  0000000000000000000000000000000000000000000000000000000000000003
[2]:  0000000000000000000000000000000000000000000000000000000000000060
[3]:  0000000000000000000000000000000000000000000000000000000000000120
[4]:  00000000000000000000000000000000000000000000000000000000000002c0
[5]:  0000000000000000000000000000000000000000000000000000000000000084
[6]:  13ead56200000000000000000000000044ff8620b8ca30902395a7bd3f2407e1
[7]:  a091bf73000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead908
[8]:  3c756cc200000000000000000000000000000000000000000000000000000000
[9]:  0000271000000000000000000000000000000000000000000088cc92434c3213
[10]: eb1523ab00000000000000000000000000000000000000000000000000000000
[11]: 0000000000000000000000000000000000000000000000000000000000000164
[12]: 8831645600000000000000000000000044ff8620b8ca30902395a7bd3f2407e1
[13]: a091bf73000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead908
[14]: 3c756cc200000000000000000000000000000000000000000000000000000000
[15]: 00002710ffffffffffffffffffffffffffffffffffffffffffffffffffffffff
[16]: fff2766000000000000000000000000000000000000000000000000000000000
[17]: 000d89a0000000000000000000000000000000000000000000295be96e640669
[18]: 71fe2cc100000000000000000000000000000000000000000000000bcf68b7cf
[19]: 1269765000000000000000000000000000000000000000000029418a79240a29
[20]: 45c0aaf200000000000000000000000000000000000000000000000bc7d74fb8
[21]: eb285519000000000000000000000000f1c429b0ce94ef9893ef110d2cc10020
[22]: 1dce71c800000000000000000000000000000000000000000000000000000000
[23]: 65884d9300000000000000000000000000000000000000000000000000000000
[24]: 0000000000000000000000000000000000000000000000000000000000000004
[25]: 12210e8a00000000000000000000000000000000000000000000000000000000
```

对上述code进行解析:

```
[13ead56200000000000000000000000044ff8620b8ca30902395a7bd3f2407e1a091bf73000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000000000000000000000000000000000000000271000000000000000000000000000000000000000000088cc92434c3213eb1523ab,
8831645600000000000000000000000044ff8620b8ca30902395a7bd3f2407e1a091bf73000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000000000000000002710fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2766000000000000000000000000000000000000000000000000000000000000d89a0000000000000000000000000000000000000000000295be96e64066971fe2cc100000000000000000000000000000000000000000000000bcf68b7cf1269765000000000000000000000000000000000000000000029418a79240a2945c0aaf200000000000000000000000000000000000000000000000bc7d74fb8eb285519000000000000000000000000f1c429b0ce94ef9893ef110d2cc100201dce71c80000000000000000000000000000000000000000000000000000000065884d93,
12210e8a]

方法一: 13ead562 - createAndInitializePoolIfNecessary
Creates a new pool if it does not exist, then initializes if not initialized

方法二: 88316456 - mint
Creates a new position wrapped in a NFT

方法三: 12210e8a - refundETH
Refunds any ETH balance held by this contract to the `msg.sender`
```

首先`13ead562`是一个叫做`Multicall`的调用, 其代码并不复杂:

```solidity
/// @title Multicall
/// @notice Enables calling multiple methods in a single call to the contract
abstract contract Multicall is IMulticall {
    /// @inheritdoc IMulticall
    function multicall(bytes[] calldata data) external payable override returns (bytes[] memory results) {
        results = new bytes[](data.length);
        for (uint256 i = 0; i < data.length; i++) {
            (bool success, bytes memory result) = address(this).delegatecall(data[i]);

            if (!success) {
                // Next 5 lines from https://ethereum.stackexchange.com/a/83577
                if (result.length < 68) revert();
                assembly {
                    result := add(result, 0x04)
                }
                revert(abi.decode(result, (string)));
            }

            results[i] = result;
        }
    }
}
```

这个函数干的事情就像`@notice`写的是为了在一次`call`进行多次`calling`. 其核心就是`address(this).delegatecall(data[i])`这个**delegatecall**.

现在我们进一步来看下方法一的具体参数:

```
createAndInitializePoolIfNecessary
[
  {
    "address"(token0): "44ff8620b8ca30902395a7bd3f2407e1a091bf73"
  },
  {
    "address"(token1): "c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
  },
  {
    "uint24"(fee): "2710"
  },
  {
    "uint160"(sqrtPriceX96): "88cc92434c3213eb1523ab"
  }
]
```

这个调用在合约`PoolInitializer`中, 合约逻辑要较为简单:

```solidity
/// @title Creates and initializes V3 Pools
abstract contract PoolInitializer is IPoolInitializer, PeripheryImmutableState {
    /// @inheritdoc IPoolInitializer
    function createAndInitializePoolIfNecessary(
        address token0,
        address token1,
        uint24 fee,
        uint160 sqrtPriceX96
    ) external payable override returns (address pool) {
        require(token0 < token1);
        pool = IUniswapV3Factory(factory).getPool(token0, token1, fee);

        if (pool == address(0)) {
            pool = IUniswapV3Factory(factory).createPool(token0, token1, fee);
            IUniswapV3Pool(pool).initialize(sqrtPriceX96);
        } else {
            (uint160 sqrtPriceX96Existing, , , , , , ) = IUniswapV3Pool(pool).slot0();
            if (sqrtPriceX96Existing == 0) {
                IUniswapV3Pool(pool).initialize(sqrtPriceX96);
            }
        }
    }
}
```

可以看到每个pool是以`token0`, `token1`和`fee`来作为tag, 在这个`UniswapV3Factory`防止重复的. `UniswapV3Factory`是`NonfungiblePositionManager`建立时传入的. 正常逻辑下请求建立的pool是不存在的, 这时`createPool`会创建一个new pool并且该调用会返回pool的地址. 并且会对该pool设置一个初始价格**sqrtPriceX96**(这是价格的开方数, sqrt(price), 而后面的**X96**表示, 这个数后96位位小数, 前64位为整数, 记作**Q64.96**), 这个函数工作完成.

然后是方法而的具体参数:

```
mint
[
  {
    "address"(token0): "44ff8620b8ca30902395a7bd3f2407e1a091bf73"
  },
  {
    "address"(token1): "c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
  },
  {
    "uint24"(fee): "2710"
  },
  {
    "int24"(tickLower): "f27660" // -88720
  },
  {
    "int24"(tickUpper): "d89a0" // 887200
  },
  {
    "uint256"(amount0Desired): "295be96e64066971fe2cc1"
  },
  {
    "uint256"(amount1Desired): "bcf68b7cf12697650"
  },
  {
    "uint256"(amount0Min): "29418a79240a2945c0aaf2"
  },
  {
    "uint256"(amount1Min): "bc7d74fb8eb285519"
  },
  {
    "address"(recipient): "f1c429b0ce94ef9893ef110d2cc100201dce71c8"
  },
  {
    "uint256"(deadline): "65884d93"
  }
]
```

下面进入较为复杂的`mint`调用, 这里面有大量的参数, 从名字上可以看出来很多都是pool的配置参数, 注意`tickLower`是负数要算补码, 我已经注释了.  源码如下:

```solidity
    /// @inheritdoc INonfungiblePositionManager
    function mint(MintParams calldata params)
        external
        payable
        override
        checkDeadline(params.deadline)
        returns (
            uint256 tokenId,
            uint128 liquidity,
            uint256 amount0,
            uint256 amount1
        )
    {
        IUniswapV3Pool pool;
        (liquidity, amount0, amount1, pool) = addLiquidity(
            AddLiquidityParams({
                token0: params.token0,
                token1: params.token1,
                fee: params.fee,
                recipient: address(this),
                tickLower: params.tickLower,
                tickUpper: params.tickUpper,
                amount0Desired: params.amount0Desired,
                amount1Desired: params.amount1Desired,
                amount0Min: params.amount0Min,
                amount1Min: params.amount1Min
            })
        );

        _mint(params.recipient, (tokenId = _nextId++));

        bytes32 positionKey = PositionKey.compute(address(this), params.tickLower, params.tickUpper);
        (, uint256 feeGrowthInside0LastX128, uint256 feeGrowthInside1LastX128, , ) = pool.positions(positionKey);

        // idempotent set
        uint80 poolId =
            cachePoolKey(
                address(pool),
                PoolAddress.PoolKey({token0: params.token0, token1: params.token1, fee: params.fee})
            );

        _positions[tokenId] = Position({
            nonce: 0,
            operator: address(0),
            poolId: poolId,
            tickLower: params.tickLower,
            tickUpper: params.tickUpper,
            liquidity: liquidity,
            feeGrowthInside0LastX128: feeGrowthInside0LastX128,
            feeGrowthInside1LastX128: feeGrowthInside1LastX128,
            tokensOwed0: 0,
            tokensOwed1: 0
        });

        emit IncreaseLiquidity(tokenId, liquidity, amount0, amount1);
    }
```

其中最重要的操作都在`addLiquidity`方法内, 源码:

```solidity
    /// @notice Add liquidity to an initialized pool
    function addLiquidity(AddLiquidityParams memory params)
        internal
        returns (
            uint128 liquidity,
            uint256 amount0,
            uint256 amount1,
            IUniswapV3Pool pool
        )
    {
        PoolAddress.PoolKey memory poolKey =
            PoolAddress.PoolKey({token0: params.token0, token1: params.token1, fee: params.fee});

        pool = IUniswapV3Pool(PoolAddress.computeAddress(factory, poolKey));

        // compute the liquidity amount
        {
            (uint160 sqrtPriceX96, , , , , , ) = pool.slot0();
            uint160 sqrtRatioAX96 = TickMath.getSqrtRatioAtTick(params.tickLower);
            uint160 sqrtRatioBX96 = TickMath.getSqrtRatioAtTick(params.tickUpper);

            liquidity = LiquidityAmounts.getLiquidityForAmounts(
                sqrtPriceX96,
                sqrtRatioAX96,
                sqrtRatioBX96,
                params.amount0Desired,
                params.amount1Desired
            );
        }

        (amount0, amount1) = pool.mint(
            params.recipient,
            params.tickLower,
            params.tickUpper,
            liquidity,
            abi.encode(MintCallbackData({poolKey: poolKey, payer: msg.sender}))
        );

        require(amount0 >= params.amount0Min && amount1 >= params.amount1Min, 'Price slippage check');
    }
```

就像上文说的, pool基于`token0`, `token1`和`fee`查重, 之后计算liquidity中使用了`sqrtPriceX96`和`tick`等概念是uniswap中提出的, 这各概念是uniswapV2(貌似, 未考证)提出的, 也有相当丰富的资料讲解其机制, 大家可以网上查找资料. 简单说: 就是uniswap将整个xy=k的价格曲线拆分称多个`tick`, 然后liquidity privider是基于tick注入流动性. 

在这里的逻辑:

1. 调用`getLiquidityForAmounts`计算出基于设置tick的上下限和当前的价格和注入的流动性换算出一个流动值`liquidity`
2. 调用`mint`则会真正的注入流动性

其中token virtual是提前通过aprove调用允许`transferFrom`的, 我也找到了原始[approve交易](https://etherscan.io/tx/0x6a2b16afdd96aa19e8ea3d99b3308b23cc3898b2de225be77983a86372d0de70)

而eth的流动性是交易中直接transfer的, 注入流动性的核心代码:

```solidity
    /// @param token The token to pay
    /// @param payer The entity that must pay
    /// @param recipient The entity that will receive payment
    /// @param value The amount to pay
    function pay(
        address token,
        address payer,
        address recipient,
        uint256 value
    ) internal {
        if (token == WETH9 && address(this).balance >= value) {
            // pay with WETH9
            IWETH9(WETH9).deposit{value: value}(); // wrap only what is needed to pay
            IWETH9(WETH9).transfer(recipient, value);
        } else if (payer == address(this)) {
            // pay with tokens already in the contract (for the exact input multihop case)
            TransferHelper.safeTransfer(token, recipient, value);
        } else {
            // pull payment
            TransferHelper.safeTransferFrom(token, payer, recipient, value);
        }
    }
```

可以看到transfer的eth会被wrap称weth, 并注入pool中成为流动性.

## pool合约源码

[在这里](./assert/UniswapV3Pool.zip)

