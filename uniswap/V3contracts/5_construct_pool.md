该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

---

本系列文章会对UniswapV3的合约进行剖析，作为学习笔记，会手术刀分析合约的执行，文中会提供合约的源码，方便大家学习。

前面的四篇文章:

* [pool部署](./1_pool.md)
* [NFT合约的部署](./2_nft.md)
* [factory合约的部署](./3_factory+poolDeployer.md)
* [nftDescriptor合约部署](./4_nftDescriptor.md)

现在让我们来尝试在私有环境上构造uniswap V3的交易对.

## 1. 启动一条Ethereum的私链

这里有很多方法可以实现, 不过这里我推荐使用 [ganache-ui](https://github.com/trufflesuite/ganache-ui), 操作简单方便还带有ui界面, 非常贴心但不专业.(可能不行, 还是要用geth)

## 2. 编译合约

这里的话使用remix就可以了, 由于代码是在一个工程包, 我们可以采用remix中带有`connect to local filesystem`功能链接你的工程所在目录. 但这个功能同时需要在你的电脑后台启动`remixd`才能正常工作. [安装文档](https://github.com/ethereum/remix-project/tree/master/libs/remixd)

安装remixd:

```
npm install -g @remix-project/remixd
```

本机启动remixd后台任务:

```
remixd -s <path-to-the-shared-folder> -u <remix-ide-instance-URL>
```

## 3. 部署合约

完成以上两步之后部署合约也会是一件非常容易的事情

   * 调用updateMasterMinter, 将自己升级为masterMinter
   * 调用configureMinter, 调整自己能够mint的数量
   * 调用mint, 给自己mint一定数量的usdc, 比如 1个小目标
2. 部署WETH9合约
   * 调用deposit方法, 传入2000 eth
3. 部署Factory合约
4. 部署NftDescriptor合约
5. 部署NftPositionManager合约
6. 对NftPositionManager合约, approve 足够数量的 usdc.
7. 最后调用创建pool的三个方法: `createAndInitializePoolIfNecessary`, `mint`, `refundETH`

以上三个方法需要非常合理的编排数据, 和高度对合约的理解能力才能完成 uniswapV3Pool 的创建, 有疑问的话可以好好重温一下[pool部署](./1_pool.md).

