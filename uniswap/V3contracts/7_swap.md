该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

---

本系列文章会对UniswapV3的合约进行剖析，作为学习笔记，会手术刀分析合约的执行，文中会提供合约的源码，方便大家学习。

本篇我们来讲讲在 uniswap 中的最终要的一个操作 swap:

由于Pool合约中的swap方法并不会完成inputToken的转入, 只会完成outputToken的转出, inputToken转入发生在回调中, 也就意味着用户是不能直接和swap合约交互的, 因为eoa地址并不实现回调, 回调的时候也就必然revert了. 并且这个回调是一个具体的方法, 名为`uniswapV3SwapCallback`.

最简单的方法就是部署一个uniswap的swapRouter, 该合约提供了丰富的swap请求交易形式

`exactInputSingle`, `exactInput`, `exactOutputSingle`, `exactOutput`

input和output各有两个实际是嵌套使用, 我们一般只要使用`exactInputSingle`或`exactOutputSingle`. input和output的主要区别分别是: input指定输入代币, output指定输出代币. 按照router定义的结构`ExactInputSingleParams`安步输入参数正常就没问题, 值得一说的的是参数`sqrtPriceLimitX96`这个参数是定义`token1/token0`的相对价格的与我们建立Pool的时候类似. 当然我们在UI上操作的时候是不会看到这个参数的, 因为使用一个我们更为熟悉的名字滑点来代替. 滑点是当前价格的偏差, 而`sqrtPriceLimitX96`限制的是完成交易以后Pool最新的价格, 该价格是必须要大于`sqrtPriceLimitX96`, 否则这笔交易只会swap ``sqrtPriceLimitX96`之上价格的部分.

## Router合约源码

[在这里](./assert/SwapRouter.zip)
