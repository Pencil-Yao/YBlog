该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

---

本文是对 uniswap 基础数学论文的翻译, [论文连接](https://github.com/runtimeverification/verified-smart-contracts/blob/uniswap/uniswap/x-y-k.pdf), 当然我主要针对重要部分进行翻译. 

## 1. Formal Overview of x × y = k Model

Consider a decentralized exchange that trades two tokens X and Y. Let x and y be the number of tokens X and Y, respectively, that the exchange currently reserves. The token exchange price is determined by the ratio of x and y so that the product ![](https://latex.codecogs.com/gif.latex?x{\times}y) is preserved. That is, when you sell ![](https://latex.codecogs.com/gif.latex?{\Delta}x) tokens, you will get ![](https://latex.codecogs.com/gif.latex?{\Delta}y) tokens such that ![](https://latex.codecogs.com/gif.latex?x{\times}y=(x+{\Delta}x){\times}(y+{\Delta}y)). Thus, the price ![](https://latex.codecogs.com/gif.latex?({\Delta}x/{\Delta}y)) is the function of ![](https://latex.codecogs.com/gif.latex?x/y). Specifically, when you trade ![](https://latex.codecogs.com/gif.latex?{\Delta}x) with ![](https://latex.codecogs.com/gif.latex?{\Delta}y), the exchange token reserves are updated as follows:

![](https://latex.codecogs.com/gif.latex?x'=x+{\Delta}x=(1+{\alpha})x=\frac{1}{1-{\beta}}x)

![](https://latex.codecogs.com/gif.latex?y'=y-{\Delta}y=\frac{1}{1+{\alpha}}y=(1-{\beta})y)

Where ![](https://latex.codecogs.com/gif.latex?{\alpha}=\frac{{\Delta}x}{x}) and ![](https://latex.codecogs.com/gif.latex?{\beta}=\frac{{\Delta}y}{y}) Also, we have:

![](https://latex.codecogs.com/gif.latex?{\Delta}x=\frac{{\beta}}{1-{\beta}}x)

![](https://latex.codecogs.com/gif.latex?{\Delta}y=\frac{{\alpha}}{1+{\alpha}}y)

:book: 考虑一个去中心化交易所交易两种代币 X 和 Y. 使用 x 和 y 表示代币代币 X 和 Y 的数量, 相应的表示交易所当前的代币存量. 代币的交易价格是由 x 和 y 的比例决定的, 因此说 ![](https://latex.codecogs.com/gif.latex?x{\times}y) 是维持固定的. 这就是为什么, 当你卖出 ![](https://latex.codecogs.com/gif.latex?{\Delta}x) 的代币你将会获得  ![](https://latex.codecogs.com/gif.latex?{\Delta}y) 的代币并且满足 ![](https://latex.codecogs.com/gif.latex?x{\times}y=(x+{\Delta}x){\times}(y+{\Delta}y)).  因此说实际的价格函数是 ![](https://latex.codecogs.com/gif.latex?({\Delta}x/{\Delta}y)). 尤其是, 当你交易  ![](https://latex.codecogs.com/gif.latex?{\Delta}x) 获得 ![](https://latex.codecogs.com/gif.latex?{\Delta}y), 交易所得存量会如下更新：

![](https://latex.codecogs.com/gif.latex?x'=x+{\Delta}x=(1+{\alpha})x=\frac{1}{1-{\beta}}x)

![](https://latex.codecogs.com/gif.latex?y'=y-{\Delta}y=\frac{1}{1+{\alpha}}y=(1-{\beta})y)

其中



![](https://latex.codecogs.com/gif.latex?{\Delta}x=\frac{{\beta}}{1-{\beta}}x)

![](https://latex.codecogs.com/gif.latex?{\Delta}y=\frac{{\alpha}}{1+{\alpha}}y)



Now consider a fee for each token trade. Let ![](https://latex.codecogs.com/gif.latex?0{\le}{\rho}<1) be a fee, e.g., ![](https://latex.codecogs.com/gif.latex?{\rho}=0.003) for 0.3% fee schedule.

![](https://latex.codecogs.com/gif.latex?x_{\rho}^{'}=x+{\Delta}x=(1+{\alpha})x=\frac{1+{\beta}(\frac{1}{\gamma})}{1-{\beta}}x)

![](https://latex.codecogs.com/gif.latex?y_{\rho}^{'}=y+{\Delta}y=\frac{1}{1+{\alpha}{\gamma}}y=(1-{\beta})y)

Where ![](https://latex.codecogs.com/gif.latex?{\alpha}=\frac{{\Delta}x}{x}) and ![](https://latex.codecogs.com/gif.latex?{\beta}=\frac{{\Delta}y}{y}), and ![](https://latex.codecogs.com/gif.latex?{\gamma}=1-{\rho}). Also, we have:

![](https://latex.codecogs.com/gif.latex?{\Delta}x=\frac{{\beta}}{1-{\beta}}{\cdot}\frac{1}{\gamma}x)

![](https://latex.codecogs.com/gif.latex?{\Delta}y=\frac{{\alpha}{\gamma}}{1+{\alpha}{\gamma}}y)

:book: 我们现在考虑一笔交易费. 设定 ![](https://latex.codecogs.com/gif.latex?0{\le}{\rho}<1) 作为一个交易费,  例如 ![](https://latex.codecogs.com/gif.latex?{\rho}=0.003) 对应 0.3% 费率. 那么：



![](https://latex.codecogs.com/gif.latex?y_{\rho}^{'}=y+{\Delta}y=\frac{1}{1+{\alpha}{\gamma}}y=(1-{\beta})y)

其中 ![](https://latex.codecogs.com/gif.latex?{\alpha}=\frac{{\Delta}x}{x}),  ![](https://latex.codecogs.com/gif.latex?{\beta}=\frac{{\Delta}y}{y}) 以及 ![](https://latex.codecogs.com/gif.latex?{\gamma}=1-{\rho}). 因此我们可以得到：

![](https://latex.codecogs.com/gif.latex?{\Delta}x=\frac{{\beta}}{1-{\beta}}{\cdot}\frac{1}{\gamma}x)

![](https://latex.codecogs.com/gif.latex?{\Delta}y=\frac{{\alpha}{\gamma}}{1+{\alpha}{\gamma}}y)

Note that we have the same formula with the previous one if there is no fee, i.e., ![](https://latex.codecogs.com/gif.latex?{\gamma}=1). Also, note that the product of x and y slightly increases for each trade due to the fee. That is, ![](https://latex.codecogs.com/gif.latex?x_{\rho}^{'}{\times}y_{\rho}^{'}>x{\times}y) when ![](https://latex.codecogs.com/gif.latex?{\rho}>0), while ![](https://latex.codecogs.com/gif.latex?x_{\rho}^{'}{\times}y_{\rho}^{'}=x{\times}y) when ![](https://latex.codecogs.com/gif.latex?{\rho}=0) (no fee).

:book: 我们注意到当没有费率的时候这个共识与前面的是一样, 例如 ![](https://latex.codecogs.com/gif.latex?{\gamma}=1). 同时, 注意到 x 和 y 的乘积略微大于没有费率时的乘积. 也就是 ![](https://latex.codecogs.com/gif.latex?x_{\rho}^{'}{\times}y_{\rho}^{'}>x{\times}y) 当 ![](https://latex.codecogs.com/gif.latex?{\rho}>0) 时, 而 ![](https://latex.codecogs.com/gif.latex?x_{\rho}^{'}{\times}y_{\rho}^{'}=x{\times}y) 当 ![](https://latex.codecogs.com/gif.latex?{\rho}=0) 没有费率时. 

**State Transition System** We formalize the market maker model as a state transition system, where the state represents the current asset of the exchange, and the transition represents how each function updates the state.
We define the exchange state as a tuple (*e, t, l*), where *e* is the amount of Ether (in wei), *t* is the number of (exchange) tokens, and *l* is the amount of total liquidity (i.e., the total supply of UNI tokens).

:book: **State Transition System** 我们定义市场模型类似一个状态装换系统, 其中状态代表当前交易所的资产量, 而转换代表每一个动作如何更新这个状态. 

我们定义交易所的状态类似一个 tuple (*e, t, l*), 其中 *e* 代表 Ether 的数量（单位 wei）,  *t* 代表代币的交易池数量, 而 *l* 达标总共的流动交易量. 

## 2. Updating Liquidity

We formalize two functions **addLiquidity** and **removeLiquidity** that mints and burns the liquidity, respectively. We first formalize their mathematical definition, **addLiquidity<sub>spec</sub>** and **removeLiquidity<sub>spec</sub>**, that uses the real arithmetic. Then, we formalize their implementation, **addLiquidity<sub>code</sub>** and **removeLiquidity<sub>code</sub>** , that uses the integer arithmetic, and analyze the approximation errors due to the integer rounding.

:book: 我们定义两个函数 **addLiquidity** 和 **removeLiquidity** 分别代表流动性挖矿和销毁流动性. 我们首先公式化他们的数学定义, **addLiquidity<sub>spec</sub>** 和 **removeLiquidity<sub>spec</sub>**, 代表真实算术结果. 然后我们公式化定义整数算术, **addLiquidity<sub>code</sub>** 和 **removeLiquidity<sub>code</sub>** 会存在整数计算的小偏差. 

### 2.1 Minting Liquidity

#### 2.1.1 **addLiquidity<sub>spec</sub>**

**Definition 1.** **addLiquidity<sub>spec</sub>** takes as input ![](https://latex.codecogs.com/gif.latex?{\bigtriangleup}e>0) and updates the state as follows:

![](https://latex.codecogs.com/gif.latex?(e,t,l)\overset{_{addLiquidity_{spec}({\bigtriangleup}e})}{\rightarrow}(e',t',l'))

where

![](https://latex.codecogs.com/gif.latex?e'=(1+{\alpha})e)

![](https://latex.codecogs.com/gif.latex?t'=(1+{\alpha})t)

![](https://latex.codecogs.com/gif.latex?l'=(1+{\alpha})l)

and ![](https://latex.codecogs.com/gif.latex?{\alpha}=\frac{{\Delta}e}{e}).

:book: 定义 **addLiquidity<sub>spec</sub>** 增加流动性  ![](https://latex.codecogs.com/gif.latex?{\bigtriangleup}e>0) 更新状态如下:

其中

![](https://latex.codecogs.com/gif.latex?e'=(1+{\alpha})e)

![](https://latex.codecogs.com/gif.latex?t'=(1+{\alpha})t)

![](https://latex.codecogs.com/gif.latex?l'=(1+{\alpha})l)

而且 ![](https://latex.codecogs.com/gif.latex?{\alpha}=\frac{{\Delta}e}{e}).

Here, an investor deposits both ![](https://latex.codecogs.com/gif.latex?{\bigtriangleup}e) ether (wei) and ![](https://latex.codecogs.com/gif.latex?{\bigtriangleup}t=t'-t) tokens, and mints ![](https://latex.codecogs.com/gif.latex?{\bigtriangleup}l=l'-l) liquidity. The invariant is that the ratio of e : t : l is preserved, and k = e × t increases, as formulated in the following theorem.

**Theorem 1.** Let ![](https://latex.codecogs.com/gif.latex?(e,t,l)\overset{_{addLiquidity_{spec}({\bigtriangleup}e})}{\rightarrow}(e',t',l')). Let ![](https://latex.codecogs.com/gif.latex?k=e{\times}t) and ![](https://latex.codecogs.com/gif.latex?{k}'={e}'{\times}{t}'). Then, we have the following:

1 ![](https://latex.codecogs.com/gif.latex?(e:t:l)=({e}':{t}':{l}'))
2 ![](https://latex.codecogs.com/gif.latex?k<k')
3 ![](https://latex.codecogs.com/gif.latex?\frac{k'}{k}=(\frac{l'}{l})^{2})

:book: 其中, 一个投资者同时存入了 ![](https://latex.codecogs.com/gif.latex?{\bigtriangleup}e) ether (wei) 和 ![](https://latex.codecogs.com/gif.latex?{\bigtriangleup}t=t'-t) 代币. 比例变量 e:t:l 是保持不变的，而 k = e × t 增长了，将以下定理公式化:

**定理1.** 规定 . ![](https://latex.codecogs.com/gif.latex?k=e{\times}t) 和 ![](https://latex.codecogs.com/gif.latex?{k}'={e}'{\times}{t}'). 那么我们可以得出以下结论：



1 ![](https://latex.codecogs.com/gif.latex?(e:t:l)=({e}':{t}':{l}'))
2 ![](https://latex.codecogs.com/gif.latex?k<k')
3 ![](https://latex.codecogs.com/gif.latex?\frac{k'}{k}=(\frac{l'}{l})^{2})

#### 2.1.2 **addLiquidity<sub>code</sub>**

**Definition 2.** **addLiquidity<sub>code</sub>** takes as input an integer ![](https://latex.codecogs.com/gif.latex?{\bigtriangleup}e>0{\in}Z) and updates the state as follows:

![](https://latex.codecogs.com/gif.latex?(e,t,l){\in}Z^{3}\overset{_{addLiquidity_{code}({\bigtriangleup}e})}{\rightarrow}({e}'',{t}'',{l}''){\in}Z^{3})

![](https://latex.codecogs.com/gif.latex?e''=e+{\Delta}e=(1+\alpha)e)

![](https://latex.codecogs.com/gif.latex?t''=t+\left\lfloor{\frac{{\Delta}e{\times}t}{e}}\right\rfloor=\left\lfloor{(1+{\alpha}t)}\right\rfloor+1)

![](https://latex.codecogs.com/gif.latex?l''=l+\left\lfloor{\frac{{\Delta}e{\times}t}{e}}\right\rfloor=\left\lfloor{(1+{\alpha}t)}\right\rfloor)

and ![](https://latex.codecogs.com/gif.latex?\alpha=\frac{{\bigtriangleup}e}{e}).

:book: 定义2. **addLiquidity<sub>code</sub>** 增加了新的整数  ![](https://latex.codecogs.com/gif.latex?{\bigtriangleup}e>0{\in}Z) 然后状态如下：

![](https://latex.codecogs.com/gif.latex?e''=e+{\Delta}e=(1+\alpha)e)

![](https://latex.codecogs.com/gif.latex?t''=t+\left\lfloor{\frac{{\Delta}e{\times}t}{e}}\right\rfloor=\left\lfloor{(1+{\alpha}t)}\right\rfloor+1)

![](https://latex.codecogs.com/gif.latex?l''=l+\left\lfloor{\frac{{\Delta}e{\times}t}{e}}\right\rfloor=\left\lfloor{(1+{\alpha}t)}\right\rfloor)

其中 ![](https://latex.codecogs.com/gif.latex?\alpha=\frac{{\bigtriangleup}e}{e}).

Theorem 2. Let ![](https://latex.codecogs.com/gif.latex?(e,t,l)\overset{addLiquidity_{spec}({\Delta}e)}{\rightarrow}({e}',{t}',{l}')). Let ![](https://latex.codecogs.com/gif.latex?(e,t,l)\overset{addLiquidity_{code}({\Delta}e)}{\rightarrow}({e}'',{t}'',{l}'')). Let ![](https://latex.codecogs.com/gif.latex?k=e{\times}t), ![](https://latex.codecogs.com/gif.latex?{k}'={e}'{\times}{t}') and ![](https://latex.codecogs.com/gif.latex?({k}''={e}''{\times}{t}'')). The, we have:

![](https://latex.codecogs.com/gif.latex?e''=e{'})

![](https://latex.codecogs.com/gif.latex?t''=\left\lfloor{t'}\right\rfloor+1)

![](https://latex.codecogs.com/gif.latex?l''=\left\lfloor{l'}\right\rfloor)

and

1. ![](https://latex.codecogs.com/gif.latex?e<e{'}=e{''})
2. ![](https://latex.codecogs.com/gif.latex?t<t'<t''{\le}t'+1)
3. ![](https://latex.codecogs.com/gif.latex?l'-1<l''{\le}l')
4. ![](https://latex.codecogs.com/gif.latex?k<k'<k'')
5. ![](https://latex.codecogs.com/gif.latex?(\frac{l''}{l})^{2}<\frac{k''}{k})

:book: 定理2. 使 ![](https://latex.codecogs.com/gif.latex?(e,t,l)\overset{addLiquidity_{spec}({\Delta}e)}{\rightarrow}({e}',{t}',{l}')). 使 ![](https://latex.codecogs.com/gif.latex?(e,t,l)\overset{addLiquidity_{code}({\Delta}e)}{\rightarrow}({e}'',{t}'',{l}'')). 使  ![](https://latex.codecogs.com/gif.latex?k=e{\times}t), ![](https://latex.codecogs.com/gif.latex?{k}'={e}'{\times}{t}') 和 ![](https://latex.codecogs.com/gif.latex?({k}''={e}''{\times}{t}'')). 那么我们可以得到：



![](https://latex.codecogs.com/gif.latex?t''=\left\lfloor{t'}\right\rfloor+1)

![](https://latex.codecogs.com/gif.latex?l''=\left\lfloor{l'}\right\rfloor)

以及

1. ![](https://latex.codecogs.com/gif.latex?e<e{'}=e{''})
2. ![](https://latex.codecogs.com/gif.latex?t<t'<t''{\le}t'+1)
3. ![](https://latex.codecogs.com/gif.latex?l'-1<l''{\le}l')
4. ![](https://latex.codecogs.com/gif.latex?k<k'<k'')
5. ![](https://latex.codecogs.com/gif.latex?(\frac{l''}{l})^{2}<\frac{k''}{k})

### 2.2 Burning Liquidity

#### 2.2.1 removeLiquidity<sub>spec</sub>

取出流动性本质上来说和注入流动性刚好相反，我这里就不在赘述了，原文写的实在有些啰嗦了。

### 总结

核心思想是 xy=k，然后因为计算机是处理整数的，本质上币也是整数的，因此就不得不处理存在小数的情况，虽然论文写得很复杂，但本质上来讲，在增加流动性方面大家只要抓住 eth 本位原则，也就是 eth 肯定是整数，而由 token 来补足小数造成的损失。而减少流动性方面是以 l 为本位的，那么 eth 和 token 的小数损失由用户承担。























